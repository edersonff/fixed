use librqbit::Session;

use std::sync::Arc;

use crate::LANE_AUTOClick;
use crate::add_game_to_steam;
use crate::delete_installers;
use crate::extract_first_rar;
use crate::find_shortcuts_vdf;
use crate::log_fail;
use serde::Serialize;
use tauri::Emitter;
use tauri::Manager;

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

