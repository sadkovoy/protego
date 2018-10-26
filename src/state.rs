use std::sync::Arc;
use std::sync::Mutex;

use std::collections::HashMap;

pub struct AppState {
    pub entries: Arc<Mutex<HashMap<String, i32>>>,
}

pub fn prepare_state() -> Arc<Mutex<HashMap<String, i32>>> {
    Arc::new(Mutex::new(HashMap::new()))
}
