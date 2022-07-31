use std::{collections::HashMap, time::Duration};

use rust_extensions::{date_time::DateTimeAsMicroseconds, lazy::LazyVec};
use tokio::sync::Mutex;

use crate::mynosqlserverpersistence_grpc::*;

pub struct GrpcPersistProcess {
    pub created: DateTimeAsMicroseconds,
    pub table_name: String,
    pub persist_moment: DateTimeAsMicroseconds,
    pub items: Vec<TableEntityGrpcModel>,
}

impl GrpcPersistProcess {
    pub fn new(table_name: String, persist_moment: DateTimeAsMicroseconds) -> Self {
        GrpcPersistProcess {
            created: DateTimeAsMicroseconds::now(),
            table_name,
            items: Vec::new(),
            persist_moment,
        }
    }
}

pub struct GrpcPersistProcesses {
    pub processes: Mutex<HashMap<i64, GrpcPersistProcess>>,
}

impl GrpcPersistProcesses {
    pub fn new() -> Self {
        Self {
            processes: Mutex::new(HashMap::new()),
        }
    }

    //TODO - GC
    pub async fn add(
        &self,
        process_id: i64,
        table_name: String,
        persist_moment: DateTimeAsMicroseconds,
    ) {
        let mut write_access = self.processes.lock().await;
        write_access.insert(
            process_id,
            GrpcPersistProcess::new(table_name, persist_moment),
        );
    }

    pub async fn get(&self, process_id: i64) -> GrpcPersistProcess {
        let mut write_access = self.processes.lock().await;
        let result = write_access.remove(&process_id);

        if result.is_none() {
            panic!("GrpcPersistProcess with id {} is not found", process_id);
        }

        result.unwrap()
    }

    pub async fn gc(&self, gc_timeout: Duration) {
        let mut write_access = self.processes.lock().await;
        let now = DateTimeAsMicroseconds::now();

        let to_gc = {
            let mut to_gc = LazyVec::new();
            for (process_id, process) in &*write_access {
                if now.duration_since(process.created).as_positive_or_zero() > gc_timeout {
                    to_gc.add(*process_id);
                }
            }

            to_gc.get_result()
        };

        if let Some(to_gc) = to_gc {
            for process_id in to_gc {
                write_access.remove(&process_id);
            }
        }
    }
}
