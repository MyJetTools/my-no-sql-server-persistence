use crate::{app::AppContext, db::DbTableWrapper};

pub async fn create_table(app: &AppContext, db_table_wrapper: &DbTableWrapper) {
    app.persist_io
        .create_table_folder(db_table_wrapper.name.as_str())
        .await;

    let attrs = db_table_wrapper.get_table_attributes().await;

    super::save_table_attributes(app, db_table_wrapper, &attrs).await;

    let mut write_access = db_table_wrapper.data.write().await;

    let attrs = write_access.db_table.attributes.clone();

    write_access.persisted_table_data.create_table(attrs);
}
