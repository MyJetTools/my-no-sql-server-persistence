use my_no_sql_sdk::core::db::DbTableAttributes;
use rust_extensions::date_time::DateTimeAsMicroseconds;

use crate::my_no_sql_server_persistence_grpc::UpdateTableAttributesGrpcModel;

impl Into<DbTableAttributes> for UpdateTableAttributesGrpcModel {
    fn into(self) -> DbTableAttributes {
        let created = DateTimeAsMicroseconds::from(self.created);
        DbTableAttributes {
            persist: self.persist,
            max_partitions_amount: self.max_partitions_amount.map(|x| x as usize),
            max_rows_per_partition_amount: self.max_rows_per_partition.map(|x| x as usize),
            created,
        }
    }
}
