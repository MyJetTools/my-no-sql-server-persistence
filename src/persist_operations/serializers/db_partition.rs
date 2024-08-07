use std::sync::Arc;

use my_json::json_reader::array_parser::ArrayToJsonObjectsSplitter;
use my_no_sql_sdk::core::{
    db::DbPartition,
    db_json_entity::{DbJsonEntity, JsonTimeStamp},
};

pub fn deserialize(raw: &[u8]) -> Result<DbPartition, String> {
    let mut db_partition = DbPartition::new();

    for db_entity_json_result in raw.split_array_json_to_objects() {
        if let Err(err) = db_entity_json_result {
            return Err(format!("Can not split to array of json objects: {:?}", err));
        }

        let db_entity_json = db_entity_json_result.unwrap();

        match DbJsonEntity::parse(db_entity_json) {
            Ok(db_entity) => {
                let db_row = if let Some(time_stamp) = db_entity.time_stamp {
                    let time_stamp = JsonTimeStamp::parse_or_now(time_stamp);
                    db_entity.restore_db_row(&time_stamp)
                } else {
                    let time_stamp = JsonTimeStamp::now();
                    db_entity.to_db_row(&time_stamp)
                };

                db_partition.insert_row(Arc::new(db_row), None);
            }
            Err(err) => {
                println!("Skipping entity. Reason {:?}", err);
            }
        }
    }
    Ok(db_partition)
}
