use my_json::json_reader::array_iterator::JsonArrayIterator;
use my_no_sql_sdk::core::db_json_entity::DbJsonEntity;
use rust_extensions::array_of_bytes_iterator::SliceIterator;

pub struct DbRowFromFile {
    pub partition_key: String,
    pub row_key: String,
    pub content: String,
}

pub fn deserialize_records(src: &[u8]) -> Result<Vec<DbRowFromFile>, String> {
    let mut result = Vec::new();

    let slice_iterator = SliceIterator::new(src);

    let mut json_array_iterator = JsonArrayIterator::new(slice_iterator);

    while let Some(json) = json_array_iterator.get_next() {
        let json = json.map_err(|err| format!("Error reading partition: {:?}", err))?;

        let json_entity =
            DbJsonEntity::restore_into_db_row(json.unwrap_as_object(&json_array_iterator).unwrap())
                .map_err(|err| format!("Error reading partition: {:?}", err))?;
        let db_entity = DbRowFromFile {
            partition_key: json_entity.get_partition_key().to_string(),
            row_key: json_entity.get_row_key().to_string(),
            content: String::from_utf8(json_entity.to_vec())
                .map_err(|err| format!("Error reading partition: {:?}", err))?,
        };

        result.push(db_entity);
    }
    return Ok(result);
}
