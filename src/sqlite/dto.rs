use my_no_sql_sdk::core::db::DbTableAttributes;
use my_sqlite::macros::*;
use rust_extensions::date_time::DateTimeAsMicroseconds;

use crate::serializers::TableMetadataFileContract;
#[derive(TableSchema, InsertDbEntity, UpdateDbEntity, SelectDbEntity, Debug)]
pub struct DbRowDto {
    #[primary_key(0)]
    #[generate_where_model("WhereByPartitionKey")]
    #[generate_where_model("WhereByPartitionKeyAndRowKey")]
    pub partition_key: String,
    #[primary_key(1)]
    #[generate_where_model("WhereByPartitionKeyAndRowKey")]
    pub row_key: String,
    pub content: String,
}
#[derive(TableSchema, InsertDbEntity, UpdateDbEntity, SelectDbEntity, Debug)]
pub struct TableMetaDataDto {
    #[primary_key(0)]
    pub id: i64,
    pub persist: bool,
    pub max_partitions_amount: Option<u64>,
    pub max_rows_per_partition: Option<u64>,
    pub created: Option<String>,
}

impl Into<DbTableAttributes> for TableMetaDataDto {
    fn into(self) -> DbTableAttributes {
        let created = if let Some(created) = self.created.as_ref() {
            match DateTimeAsMicroseconds::from_str(created.as_str()) {
                Some(value) => value,
                None => DateTimeAsMicroseconds::now(),
            }
        } else {
            DateTimeAsMicroseconds::now()
        };

        DbTableAttributes {
            persist: self.persist,
            max_partitions_amount: self.max_partitions_amount.map(|itm| itm as usize),
            max_rows_per_partition_amount: self.max_rows_per_partition.map(|itm| itm as usize),
            created,
        }
    }
}

impl<'s> Into<TableMetaDataDto> for &'s DbTableAttributes {
    fn into(self) -> TableMetaDataDto {
        TableMetaDataDto {
            id: 0,
            persist: self.persist,
            max_partitions_amount: self.max_partitions_amount.map(|x| x as u64),
            max_rows_per_partition: self.max_rows_per_partition_amount.map(|x| x as u64),
            created: Some(self.created.to_rfc3339()),
        }
    }
}

impl Into<TableMetadataFileContract> for TableMetaDataDto {
    fn into(self) -> TableMetadataFileContract {
        TableMetadataFileContract {
            persist: self.persist,
            max_partitions_amount: self.max_partitions_amount.map(|x| x as usize),
            max_rows_per_partition_amount: self.max_rows_per_partition.map(|x| x as usize),
            created: self.created,
        }
    }
}

#[derive(WhereDbModel)]
pub struct WhereModelAll {}
