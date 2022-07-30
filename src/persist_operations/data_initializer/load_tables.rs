use std::sync::Arc;

use rust_extensions::StopWatch;

use crate::{app::AppContext, db::DbTableWrapper};

pub async fn load_tables(app: Arc<AppContext>) {
    let table_names = app.persist_io.get_list_of_tables().await;

    app.init_state
        .init_table_names(table_names.clone(), app.logs.as_ref())
        .await;

    tokio::spawn(super::table_list_of_files_loader(app.clone(), table_names));

    let mut sw = StopWatch::new();
    sw.start();

    let mut threads = Vec::new();
    for _ in 0..app.settings.init_threads_amount {
        threads.push(tokio::spawn(super::load_table_files::spawn(app.clone())));
    }

    for thread in threads {
        thread.await.unwrap();
    }

    while let Some(db_table) = app.init_state.get_table_data_result().await {
        let db_table_wrapper = DbTableWrapper::restore_from_blob(db_table);
        app.db.add(db_table_wrapper).await;
    }

    app.states.set_initialized();

    sw.pause();

    app.logs.add_info(
        None,
        crate::app::logs::SystemProcess::Init,
        "init_tables".to_string(),
        format!("All tables initialized in {:?}", sw.duration()),
        None,
    );
}
