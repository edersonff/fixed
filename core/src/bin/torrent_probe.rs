use std::path::Path;

use bytes::Bytes;

use librqbit::AddTorrent;

use librqbit::AddTorrentOptions;

use librqbit::Session;

fn dir_size(path: &str) -> u64 {

    fn walk(entry: &Path, total: &mut u64) {

        if entry.is_file() {

            if let Ok(meta) = entry.metadata() {

                *total += meta.len();

            }

        } else if let Ok(entries) = std::fs::read_dir(entry) {

            for child in entries.flatten() {

                walk(&child.path(), total);

            }

        }

    }

    let mut total: u64 = 0;

    walk(Path::new(path), &mut total);

    total

}

#[tokio::main]

async fn main() -> anyhow::Result<()> {

    std::fs::create_dir_all("/tmp/opencode/torrent-test")?;

    let session = Session::new("/tmp/opencode/torrent-test".into()).await?;

    let torrent_bytes = std::fs::read("/tmp/opencode/ron.torrent")?;

    let mut opts = AddTorrentOptions::default();

    opts.only_files_regex = Some(String::from("part08"));

    let response = session

        .add_torrent(AddTorrent::TorrentFileBytes(Bytes::from(torrent_bytes)), Some(opts))

        .await?;

    match response.into_handle() {

        Some(_) => eprintln!("torrent added, downloading part08 only"),

        None => {

            eprintln!("not added (already managed or list-only)");

            return Ok(());

        }

    }

    for tick in 1..=6 {

        tokio::time::sleep(std::time::Duration::from_secs(5)).await;

        eprintln!("tick {}: {} MB on disk", tick, dir_size("/tmp/opencode/torrent-test") / 1_000_000);

    }

    Ok(())

}
