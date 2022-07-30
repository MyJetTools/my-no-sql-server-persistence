use std::sync::Arc;

use my_no_sql_core::db::{DbTable, DbTableAttributes};

use crate::{app::AppContext, db::DbTableWrapper};

pub async fn get_table(app: &AppContext, table_name: &str) -> Arc<DbTableWrapper> {
    let db_table_wrapper = app.db.get_table(table_name).await;

    if db_table_wrapper.is_some() {
        return db_table_wrapper.unwrap();
    }

    let db_table = DbTable::new(table_name.to_string(), DbTableAttributes::create_default());

    let db_table_wrapper = DbTableWrapper::new(db_table);

    app.db.add(db_table_wrapper.clone()).await;
    db_table_wrapper
}
