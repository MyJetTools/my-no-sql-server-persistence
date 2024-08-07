use my_no_sql_sdk::core::db::DbTableAttributes;
use rust_extensions::date_time::DateTimeAsMicroseconds;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TableMetadataFileContract {
    #[serde(rename = "Persist")]
    #[serde(default = "default_persist")]
    pub persist: bool,
    #[serde(rename = "MaxPartitionsAmount")]
    pub max_partitions_amount: Option<usize>,
}

impl TableMetadataFileContract {
    pub fn parse(content: &[u8]) -> Self {
        let parse_result = serde_json::from_slice::<TableMetadataFileContract>(content);

        match parse_result {
            Ok(res) => res,
            Err(_) => TableMetadataFileContract {
                max_partitions_amount: None,
                persist: true,
            },
        }
    }
}

fn default_persist() -> bool {
    true
}

impl Into<DbTableAttributes> for TableMetadataFileContract {
    fn into(self) -> DbTableAttributes {
        DbTableAttributes {
            created: DateTimeAsMicroseconds::now(),
            max_partitions_amount: self.max_partitions_amount,
            persist: self.persist,
            max_rows_per_partition_amount: todo!(),
        }
    }
}

pub fn serialize(attrs: &DbTableAttributes) -> Vec<u8> {
    let contract = TableMetadataFileContract {
        max_partitions_amount: attrs.max_partitions_amount,
        persist: attrs.persist,
    };

    serde_json::to_vec(&contract).unwrap()
}
