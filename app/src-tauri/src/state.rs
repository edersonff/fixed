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

    pub extracting: std::sync::Mutex<std::collections::HashSet<String>>,

}

impl DownloadEngine {

    pub fn is_busy(&self) -> bool {

        let http = self.cancels.lock().map(|active| !active.is_empty()).unwrap_or(false);

        let torrent = self.torrents.lock().map(|active| !active.is_empty()).unwrap_or(false);

        let extracting = self.extracting.lock().map(|active| !active.is_empty()).unwrap_or(false);

        http || torrent || extracting

    }

}

// Both download pipelines drop their http/torrent registry entry before extraction starts, so a
// window close mid-extraction saw is_busy() == false and killed a half-written install. This is
// the one place that tracks the gap between "download done" and "extract done".
pub async fn extract_tracked(engine: &DownloadEngine, safe_title: String, title: String, folder: String) -> Result<u32, String> {

    if let Ok(mut extracting) = engine.extracting.lock() {

        extracting.insert(safe_title.clone());

    }

    let result = tokio::task::spawn_blocking(move || crate::game_files::extract_and_verify(&title, &folder))
        .await
        .unwrap_or_else(|error| Err(format!("join: {}", error)));

    if let Ok(mut extracting) = engine.extracting.lock() {

        extracting.remove(&safe_title);

    }

    result

}

#[derive(serde::Serialize, Clone)]

#[serde(rename_all = "camelCase")]

pub struct LaunchProgress {

    pub title: String,

    pub phase: String,

    pub detail: String,

}

