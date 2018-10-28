use std::sync::Arc;
use std::sync::Mutex;

use std::collections::HashMap;
use actix::Addr;
use actix_redis::RedisActor;

pub struct AppState {
    pub entries: Arc<Mutex<HashMap<String, i32>>>,
    pub redis: Arc<Addr<RedisActor>>
}

pub fn prepare_entries() -> Arc<Mutex<HashMap<String, i32>>> {
    Arc::new(Mutex::new(HashMap::new()))
}
