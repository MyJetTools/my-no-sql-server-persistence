pub enum PersistEvent {
    PersistTableMetadata,
    PersistPartition {
        partition_key: String,
    },
    PersistRow {
        partition_key: String,
        row_key: String,
    },
}
