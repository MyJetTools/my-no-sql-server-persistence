use my_sqlite::{sql_where::NoneWhereModel, SqlLiteConnectionBuilder};

use super::{DbRowDto, TableMetaDataDto, WhereModelAll};

pub const TABLE_NAME_ROWS: &str = "rows";
pub const TABLE_NAME_METADATA: &str = "metadata";

pub struct TableRepo {
    pub connection: my_sqlite::SqlLiteConnection,
}

impl TableRepo {
    pub async fn new(file_path: String) -> Self {
        Self {
            connection: SqlLiteConnectionBuilder::new(file_path)
                .create_table_if_no_exists::<DbRowDto>(TABLE_NAME_ROWS)
                .create_table_if_no_exists::<TableMetaDataDto>(TABLE_NAME_METADATA)
                .build()
                .await
                .unwrap(),
        }
    }

    pub async fn get_all(&self) -> Vec<DbRowDto> {
        let result = self
            .connection
            .query_rows(TABLE_NAME_ROWS, NoneWhereModel::new())
            .await
            .unwrap();

        result
    }

    pub async fn clear_all_records(&self) {
        self.connection
            .delete_db_entity(TABLE_NAME_ROWS, &WhereModelAll {})
            .await
            .unwrap();
    }

    pub async fn bulk_insert_or_update(&self, rows: &[DbRowDto]) {
        if rows.len() == 0 {
            return;
        }
        self.connection
            .bulk_insert_or_update(rows, TABLE_NAME_ROWS)
            .await
            .unwrap();
    }

    pub async fn update_meta_data(&self, meta_data: impl Into<TableMetaDataDto>) {
        let meta_data = meta_data.into();
        self.connection
            .insert_or_update_db_entity(TABLE_NAME_METADATA, &meta_data)
            .await
            .unwrap();
    }

    pub async fn get_table_attribute(&self) -> Option<TableMetaDataDto> {
        self.connection
            .query_single_row(TABLE_NAME_METADATA, NoneWhereModel::new())
            .await
            .unwrap()
    }
}
