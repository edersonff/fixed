use encoding_rs::WINDOWS_1251;

use scraper::Html;

use scraper::Selector;

use serde::Serialize;

use std::io::Read;

use std::io::Write;

use std::path::Path;

pub(crate) const HOME_FIXTURE: &[u8] = include_bytes!("../tests/fixtures/home.html");

pub(crate) const USER_AGENT: &str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0 Safari/537.36 FIXED/0.1";

#[derive(Serialize)]

#[serde(rename_all = "camelCase")]

pub struct GameEntry {

    pub title: String,

    pub page_url: String,

    pub poster_url: String,

    pub category: String,

    pub published_at: String,

    pub views: u64,

    pub comments: u64,

}

#[derive(Serialize)]

#[serde(rename_all = "camelCase")]

pub struct DownloadLane {

    pub kind: String,

    pub url: String,

}

#[derive(Serialize)]

#[serde(rename_all = "camelCase")]

pub struct GameDetail {

    pub title: String,

    pub build: String,

    pub steam_ext_url: String,

    pub lanes: Vec<DownloadLane>,

    pub mentions_fix_repair: bool,

    pub video_id: String,

}


mod assets;
mod extract;
mod http;
mod parsers;
mod site;
mod steam;

pub use assets::*;
pub use extract::*;
pub use http::*;
pub use parsers::*;
pub use site::*;
pub use steam::*;

#[derive(Serialize)]

#[serde(rename_all = "camelCase")]

pub struct GamesPage {

    pub source: String,

    pub games: Vec<GameEntry>,

}

#[derive(Serialize)]

#[serde(rename_all = "camelCase")]

pub struct LaneProbe {

    pub parts: Vec<String>,

    pub first_part_bytes: u64,

}
