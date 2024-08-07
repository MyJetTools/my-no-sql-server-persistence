use my_sqlite::macros::*;

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

impl<'s> Into<TableMetaDataDto> for &'s TableMetadataFileContract {
    fn into(self) -> TableMetaDataDto {
        let created = self.created.as_ref().map(|itm| itm.to_string());
        TableMetaDataDto {
            id: 0,
            persist: self.persist,
            max_partitions_amount: self.max_partitions_amount.map(|x| x as u64),
            max_rows_per_partition: self.max_rows_per_partition_amount.map(|x| x as u64),
            created,
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
