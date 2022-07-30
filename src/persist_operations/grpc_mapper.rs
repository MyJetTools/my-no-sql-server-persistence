use my_no_sql_core::db::DbRow;
use rust_extensions::date_time::DateTimeAsMicroseconds;

use crate::mynosqlserverpersistence_grpc::UpdateTableEntityGrpcModel;

impl Into<DbRow> for UpdateTableEntityGrpcModel {
    fn into(self) -> DbRow {
        let expires: Option<DateTimeAsMicroseconds> = if self.expires == 0 {
            None
        } else {
            Some(DateTimeAsMicroseconds::new(self.expires))
        };

        DbRow::restore(
            self.partition_key,
            self.row_key,
            self.data,
            expires,
            self.timespan,
        )
    }
}
