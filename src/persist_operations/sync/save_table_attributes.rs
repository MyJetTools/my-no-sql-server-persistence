use my_no_sql_core::db::DbTableAttributes;

use super::super::serializers;
use crate::{app::AppContext, db::DbTableWrapper, persist_io::TableFile};
pub async fn save_table_attributes(
    app: &AppContext,
    db_table_wrapper: &DbTableWrapper,
    attrs: &DbTableAttributes,
) {
    app.persist_io
        .create_table_folder(db_table_wrapper.name.as_str())
        .await;

    let content = serializers::table_attrs::serialize(attrs);

    app.persist_io
        .save_table_file(
            db_table_wrapper.name.as_str(),
            &TableFile::TableAttributes,
            content,
        )
        .await;

    let mut write_access = db_table_wrapper.data.write().await;

    write_access
        .persisted_table_data
        .update_table_attributes(attrs.clone());
}
