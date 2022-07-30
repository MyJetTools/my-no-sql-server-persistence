use crate::app::AppContext;
use my_http_server_swagger::*;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, MyHttpObjectStructure)]
pub struct LocationModel {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug, MyHttpObjectStructure)]
pub struct StatusBarModel {
    pub location: LocationModel,
    #[serde(rename = "persistAmount")]
    persist_amount: usize,
    #[serde(rename = "tablesAmount")]
    pub tables_amount: usize,
    #[serde(rename = "masterNode")]
    pub master_node: Option<String>,
}

impl StatusBarModel {
    pub fn new(app: &AppContext, tables_amount: usize) -> Self {
        Self {
            master_node: None,
            location: LocationModel {
                id: app.settings.location.to_string(),
            },
            persist_amount: app.get_persist_amount(),

            tables_amount,
        }
    }
}
