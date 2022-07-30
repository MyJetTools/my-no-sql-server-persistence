use std::sync::Arc;

use rust_extensions::{date_time::DateTimeAsMicroseconds, StopWatch};

use crate::{
    app::{logs::SystemProcess, AppContext},
    db::{data_to_persist::PersistResult, DbTableWrapper},
};

pub async fn persist(app: &Arc<AppContext>) {
    let is_shutting_down = app.states.is_shutting_down();

    loop {
        let tables = app.db.get_tables().await;

        let mut has_something_to_persist = false;

        for db_table_wrapper in tables {
            if let Some(persist_result) = db_table_wrapper
                .get_job_to_persist(DateTimeAsMicroseconds::now(), is_shutting_down)
                .await
            {
                has_something_to_persist = true;
                let mut sw = StopWatch::new();
                sw.start();
                let result = tokio::spawn(persist_to_blob(
                    app.clone(),
                    db_table_wrapper.clone(),
                    persist_result,
                ))
                .await;

                sw.pause();

                if result.is_ok() {
                    db_table_wrapper.set_persisted(sw.duration()).await;
                }

                if let Err(err) = result {
                    app.logs.add_fatal_error(
                        Some(db_table_wrapper.name.to_string()),
                        SystemProcess::PersistOperation,
                        "PersistTimer".to_string(),
                        format!("Can not persist messages {:?}", err),
                        None,
                    )
                }
            }
        }

        if !has_something_to_persist {
            break;
        }
    }
}

async fn persist_to_blob(
    app: Arc<AppContext>,
    db_table_wrapper: Arc<DbTableWrapper>,
    persist_result: PersistResult,
) {
    match persist_result {
        PersistResult::PersistAttrs => {
            let attrs = db_table_wrapper.get_table_attributes().await;
            crate::persist_operations::sync::save_table_attributes(
                app.as_ref(),
                db_table_wrapper.as_ref(),
                &attrs,
            )
            .await;
        }
        PersistResult::PersistTable => {
            crate::persist_operations::sync::save_table(app.as_ref(), db_table_wrapper.as_ref())
                .await;
        }
        PersistResult::PersistPartition(partition_key) => {
            crate::persist_operations::sync::save_partition(
                app.as_ref(),
                db_table_wrapper.as_ref(),
                partition_key.as_str(),
            )
            .await;
        }
    }
}
