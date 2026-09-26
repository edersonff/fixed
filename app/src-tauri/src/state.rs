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

    pub session: std::sync::Mutex<Option<Arc<Session>>>,

    pub cancels: std::sync::Mutex<std::collections::HashMap<String, std::sync::Arc<std::sync::atomic::AtomicBool>>>,

    pub torrents: std::sync::Mutex<std::collections::HashMap<String, std::sync::Arc<librqbit::ManagedTorrent>>>,

}

impl DownloadEngine {

    pub fn is_busy(&self) -> bool {

        let http = self.cancels.lock().map(|active| !active.is_empty()).unwrap_or(false);

        let torrent = self.torrents.lock().map(|active| !active.is_empty()).unwrap_or(false);

        http || torrent

    }

}

#[derive(serde::Serialize, Clone)]

#[serde(rename_all = "camelCase")]

pub struct LaunchProgress {

    pub title: String,

    pub phase: String,

    pub detail: String,

}

