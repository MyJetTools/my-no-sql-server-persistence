use std::sync::Arc;

use my_no_sql_sdk::core::db::DbTableAttributes;
use rust_extensions::date_time::DateTimeAsMicroseconds;

use crate::{app::AppContext, serializers::TableMetadataFileContract};

use super::FilesToDownloadList;

pub async fn start(app: &Arc<AppContext>) {
    println!("Loading list of tables");
    let started = DateTimeAsMicroseconds::now();

    if app.persist_io.is_none() {
        println!("Skipping legacy initialization");
        return;
    }

    let tables_to_load = app.persist_io.as_ref().unwrap().get_list_of_tables().await;

    {
        let mut init_state = app.init_state.lock().await;
        init_state.tables_to_load = tables_to_load;
    }
    let duration = DateTimeAsMicroseconds::now()
        .duration_since(started)
        .as_positive_or_zero();
    println!("Loaded list of tables in {:?}", duration);
    let mut load_table_processes = Vec::new();

    for _ in 0..app.settings.init_threads_amount {
        let app = app.clone();
        let task = tokio::spawn(async move { load_table_process(app).await });
        load_table_processes.push(task);
    }

    for task in load_table_processes {
        task.await.unwrap();
    }

    let duration = DateTimeAsMicroseconds::now()
        .duration_since(started)
        .as_positive_or_zero();
    println!("Initialized everything in {:?}", duration);
}

async fn load_table_process(app: Arc<AppContext>) {
    loop {
        let (table_to_load, files_to_download) = {
            let mut init_state = app.init_state.lock().await;
            let table_to_load = init_state.tables_to_load.pop();

            let files_to_download = Arc::new(FilesToDownloadList::new());
            if let Some(table_to_load) = table_to_load.clone() {
                init_state
                    .tables_loading
                    .insert(table_to_load.clone(), files_to_download.clone());
            }

            (table_to_load, files_to_download)
        };

        if table_to_load.is_none() {
            return;
        }
        let table_name = table_to_load.unwrap();

        println!("Loading table: {}", table_name);

        // Starting loading files process
        let table_spawned = table_name.clone();
        let app_spawned: Arc<AppContext> = app.clone();
        let files_to_download_spawned = files_to_download.clone();
        tokio::spawn(async move {
            app_spawned
                .persist_io
                .as_ref()
                .unwrap()
                .get_table_files(table_spawned.as_str(), files_to_download_spawned.as_ref())
                .await;
        });

        // Loading Content While Loading List of Files.
        while let Some(file_name) = files_to_download.get_next_file_to_download().await {
            let content = app
                .persist_io
                .as_ref()
                .unwrap()
                .load_table_file(&table_name, &file_name)
                .await;

            match file_name {
                crate::persist_io::TableFile::TableAttributes => match content {
                    Some(content) => {
                        let meta_data: TableMetadataFileContract =
                            serde_json::from_slice(content.as_slice()).unwrap();
                        app.tables
                            .restore_table(&table_name, meta_data.into())
                            .await;
                    }
                    None => {
                        app.tables
                            .restore_table(&table_name, DbTableAttributes::create_default())
                            .await;
                    }
                },
                crate::persist_io::TableFile::DbPartition(partition_key) => {
                    if let Some(content) = content {
                        let result = crate::serializers::deserialize_records(content.as_slice());

                        match result {
                            Ok(db_rows) => {
                                app.tables.restore_records(&table_name, db_rows).await;
                            }
                            Err(err) => {
                                if !app.settings.skip_broken_partitions {
                                    panic!(
                                        "Error loading partition {}/{}: Err:{:?}",
                                        table_name,
                                        partition_key.as_str(),
                                        err
                                    );
                                }
                            }
                        }
                    }
                }
            }

            if let Some(table) = app.tables.get_table(table_name.as_str()).await {
                table.flush_data_to_cache().await;
            }
        }

        {
            let mut init_state = app.init_state.lock().await;
            init_state.tables_loading.remove(&table_name);
            init_state.tables_loaded.push(table_name);
        }
    }
}
