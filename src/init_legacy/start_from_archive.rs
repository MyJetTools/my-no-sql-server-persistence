use std::{io::Read, sync::Arc};

use rust_extensions::{date_time::DateTimeAsMicroseconds, str_utils::StrUtils};

use crate::{
    app::AppContext, init_legacy::InitFromArchiveContent, persist_io::TableFile,
    serializers::TableMetadataFileContract,
};

pub async fn start_from_archive(app: &Arc<AppContext>) {
    if app.settings.legacy_zip_archive.is_none() {
        println!("Skipping loading from legacy zip archive");
        return;
    }

    let legacy_zip_archive =
        rust_extensions::file_utils::format_path(app.settings.legacy_zip_archive.as_ref().unwrap())
            .to_string();

    let started = DateTimeAsMicroseconds::now();
    let content = tokio::fs::read(legacy_zip_archive.as_str()).await;

    if let Err(err) = &content {
        println!(
            "Error reading legacy zip archive {}. Err: {:?}",
            legacy_zip_archive, err
        );
        return;
    }

    let mut archive = zip::ZipArchive::new(std::io::Cursor::new(content.unwrap())).unwrap();

    let mut zip_names = Vec::new();
    for zip_name in archive.file_names() {
        let splitted = zip_name.split_exact_to_2_lines("/");
        if splitted.is_none() {
            println!("Skipped file: {}", zip_name);
        }

        let (table_name, partition_file_name) = splitted.unwrap();

        let partition_file = TableFile::from_file_name(partition_file_name).unwrap();

        zip_names.push((zip_name.to_string(), table_name.to_string(), partition_file))

        //println!("File: {}", itm);
    }

    for (zip_name, table_name, partition_file) in zip_names {
        let mut file = archive.by_name(zip_name.as_str()).unwrap();
        let mut content = Vec::new();
        file.read_to_end(&mut content).unwrap();

        let mut write_access = app.init_from_archive.lock().await;

        if !write_access.to_load.contains_key(&table_name) {
            write_access
                .to_load
                .insert(table_name.to_string(), Vec::new());
        }

        write_access
            .to_load
            .get_mut(&table_name)
            .unwrap()
            .push(InitFromArchiveContent {
                file: partition_file,
                content,
            });
    }

    let mut tasks = Vec::new();
    for _ in 0..app.settings.init_threads_amount {
        let task = tokio::spawn(load_table(app.clone()));
        tasks.push(task);
    }

    for task in tasks {
        task.await.unwrap();
    }

    /*
    let tables = app.tables.get_tables().await;

    for table in tables {
        println!("Flushing table {}", table.table_name);
        table.flush_data_to_cache().await;
    }
     */

    let init_state = app.init_state.lock().await;

    let now = DateTimeAsMicroseconds::now();
    println!(
        "Loaded tables {} in {:?}",
        init_state.tables_loaded.len(),
        now.duration_since(started).as_positive_or_zero()
    );
}

async fn load_table(app: Arc<AppContext>) {
    loop {
        let (table_name, contents) = {
            let mut write_access = app.init_from_archive.lock().await;

            let item = write_access.to_load.pop_first();
            if item.is_none() {
                return;
            }

            item.unwrap()
        };

        for itm in contents {
            match itm.file {
                TableFile::TableAttributes => {
                    let meta_data: TableMetadataFileContract =
                        serde_json::from_slice(itm.content.as_slice()).unwrap();
                    app.tables.restore_table(&table_name, meta_data).await;
                }
                TableFile::DbPartition(_) => {
                    let db_rows =
                        crate::serializers::deserialize_records(itm.content.as_slice()).unwrap();
                    app.tables.restore_records(&table_name, db_rows).await;
                }
            }
        }

        let table = app.tables.get_table(table_name.as_str()).await.unwrap();
        table.flush_data_to_cache().await;

        let mut write_access = app.init_from_archive.lock().await;

        write_access.loaded.push(table_name);
    }
}
