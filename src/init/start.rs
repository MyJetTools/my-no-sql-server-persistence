use std::sync::Arc;

use rust_extensions::date_time::DateTimeAsMicroseconds;

use crate::app::AppContext;

pub const DB_FILE_EXTENSION: &'static str = ".sqlite";

pub async fn start(app: &Arc<AppContext>) {
    let mut persistence_dest = app.settings.get_persistence_dest();
    let mut read_dir = tokio::fs::read_dir(persistence_dest.as_str())
        .await
        .unwrap();

    let mut files = Vec::new();

    let started = DateTimeAsMicroseconds::now();

    while let Some(entry) = read_dir.next_entry().await.unwrap() {
        let file_name = entry.file_name().into_string().unwrap();

        if file_name.ends_with(DB_FILE_EXTENSION) {
            files.push(file_name);
        }
    }

    if !persistence_dest.ends_with(std::path::MAIN_SEPARATOR) {
        persistence_dest.push(std::path::MAIN_SEPARATOR);
    }

    for file in &files {
        let mut persistence_dest = persistence_dest.clone();
        persistence_dest.push_str(file);

        app.tables
            .restore_table_from_sqlite(file.as_str(), persistence_dest)
            .await;
    }

    let now = DateTimeAsMicroseconds::now();

    println!(
        "Files: {:?} in {:?}",
        files.len(),
        now.duration_since(started).as_positive_or_zero()
    );
}
