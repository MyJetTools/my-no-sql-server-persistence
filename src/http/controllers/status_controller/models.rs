use crate::app::AppContext;
use my_http_server_swagger::*;
use rust_extensions::ApplicationStates;
use serde::{Deserialize, Serialize};

use super::{non_initialized::NonInitializedModel, status_bar_model::StatusBarModel};

#[derive(Serialize, Deserialize, Debug, MyHttpObjectStructure)]
pub struct TableModel {
    pub name: String,
    #[serde(rename = "partitionsCount")]
    pub partitions_count: usize,
    #[serde(rename = "dataSize")]
    pub data_size: usize,
    #[serde(rename = "recordsAmount")]
    pub records_amount: usize,
    #[serde(rename = "lastUpdateTime")]
    pub last_update_time: i64,
    #[serde(rename = "lastPersistTime")]
    pub last_persist_time: Option<i64>,
    #[serde(rename = "lastPersistDuration")]
    pub last_persist_duration: Vec<usize>,
    #[serde(rename = "nextPersistTime")]
    pub next_persist_time: Option<i64>,
    #[serde(rename = "persistAmount")]
    pub persist_amount: usize,
}

#[derive(Serialize, Deserialize, Debug, MyHttpObjectStructure)]
pub struct StatusModel {
    #[serde(rename = "notInitialized", skip_serializing_if = "Option::is_none")]
    not_initialized: Option<NonInitializedModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    initialized: Option<InitializedModel>,
    #[serde(rename = "statusBar")]
    status_bar: StatusBarModel,
}

impl StatusModel {
    pub async fn new(app: &AppContext) -> Self {
        let tables = app.db.get_tables().await;

        let mut tables_model = Vec::new();

        for table in &tables {
            let metrics = crate::operations::get_table_metrics(table.as_ref()).await;

            let last_persist_time = if let Some(last_persist_time) = metrics.last_persist_time {
                Some(last_persist_time.unix_microseconds)
            } else {
                None
            };

            let table_model = TableModel {
                name: table.name.clone(),
                partitions_count: metrics.partitions_amount,
                data_size: metrics.table_size,
                records_amount: metrics.records_amount,
                last_update_time: metrics.last_update_time.unix_microseconds,
                last_persist_time,
                persist_amount: metrics.persist_amount,
                last_persist_duration: metrics.last_persist_duration,
                next_persist_time: if let Some(next_persist_time) = metrics.next_persist_time {
                    Some(next_persist_time.unix_microseconds)
                } else {
                    None
                },
            };

            tables_model.push(table_model);
        }

        if app.states.is_initialized() {
            return Self {
                not_initialized: None,
                initialized: Some(InitializedModel::new(tables_model)),
                status_bar: StatusBarModel::new(app, tables.len()),
            };
        }

        return Self {
            not_initialized: Some(NonInitializedModel::new(app).await),
            initialized: None,
            status_bar: StatusBarModel::new(app, tables.len()),
        };
    }
}
#[derive(Serialize, Deserialize, Debug, MyHttpObjectStructure)]
pub struct InitializedModel {
    pub tables: Vec<TableModel>,
}

impl InitializedModel {
    pub fn new(tables: Vec<TableModel>) -> Self {
        Self { tables }
    }
}
