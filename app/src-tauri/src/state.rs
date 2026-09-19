use librqbit::Session;

use std::sync::Arc;

#[derive(serde::Serialize, Clone)]

#[serde(rename_all = "camelCase")]

pub struct DownloadProgress {

    pub title: String,

    pub downloaded_bytes: u64,

    pub total_bytes: u64,

    pub state: String,

}

pub struct DownloadEngine {

    pub session: Arc<Session>,

    pub cancels: std::sync::Mutex<std::collections::HashMap<String, std::sync::Arc<std::sync::atomic::AtomicBool>>>,

}

