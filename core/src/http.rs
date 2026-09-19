use crate::USER_AGENT;
use crate::*;

pub fn mirror_download_url(hosters_url: &str) -> Result<Option<String>, String> {

    let bytes = fetch_bytes(hosters_url)

        .map_err(|error| format!("hosters listing {}: {}", hosters_url, error))?;

    let html = String::from_utf8_lossy(&bytes).to_string();

    let mut best: Option<String> = None;

    for word in html.split('"').chain(html.split('\'')) {

        let trimmed = word.trim();

        if let Some(rest) = trimmed.strip_prefix("https://pixeldrain.com/u/") {

            let id: String = rest.chars().take_while(|c| c.is_ascii_alphanumeric()).collect();

            if id.len() >= 6 {

                best = Some(format!("https://pixeldrain.com/api/file/{}", id));

            }

        }

    }

    Ok(best)

}

pub fn http_size(url: &str) -> u64 {

    match ureq::head(url)

        .set("User-Agent", USER_AGENT)

        .timeout(std::time::Duration::from_secs(15))

        .call()

    {

        Ok(response) => response

            .header("Content-Length")

            .and_then(|value| value.parse().ok())

            .unwrap_or(0),

        Err(_) => 0,

    }

}

pub fn http_download(url: &str, dest_path: &str, cancel: &std::sync::atomic::AtomicBool, progress: &dyn Fn(u64, u64)) -> Result<(), String> {

    let response = ureq::get(url)

        .set("User-Agent", USER_AGENT)

        .timeout(std::time::Duration::from_secs(90))

        .call()

        .map_err(|error| format!("http get: {}", error))?;

    let total: u64 = response

        .header("Content-Length")

        .and_then(|value| value.parse().ok())

        .unwrap_or(0);

    let mut reader = response.into_reader();

    let mut file = std::fs::File::create(dest_path).map_err(|error| format!("create {}: {}", dest_path, error))?;

    let mut buffer = vec![0u8; 262144];

    let mut done: u64 = 0;

    loop {

        if cancel.load(std::sync::atomic::Ordering::Relaxed) {

            return Err("cancelled".to_string());

        }

        let read = reader

            .read(&mut buffer)

            .map_err(|error| format!("read: {}", error))?;

        if read == 0 {

            break;

        }

        std::io::Write::write_all(&mut file, &buffer[..read]).map_err(|error| format!("write: {}", error))?;

        done += read as u64;

        progress(done, total);

    }

    Ok(())

}

pub fn download_partial(url: &str, dest: &str, limit: u64) -> Result<u64, String> {

    let response = ureq::get(url)

        .set("User-Agent", USER_AGENT)

        .timeout(std::time::Duration::from_secs(30))

        .call()

        .map_err(|error| format!("request {}: {}", url, error))?;

    let mut reader = response.into_reader().take(limit);

    let mut file = std::fs::File::create(dest).map_err(|error| format!("create {}: {}", dest, error))?;

    let mut buffer = [0u8; 65536];

    let mut written: u64 = 0;

    loop {

        let read = reader.read(&mut buffer).map_err(|error| format!("read: {}", error))?;

        if read == 0 {

            break;

        }

        file.write_all(&buffer[..read]).map_err(|error| format!("write: {}", error))?;

        written += read as u64;

    }

    Ok(written)

}

pub fn fetch_bytes(url: &str) -> Result<Vec<u8>, String> {

    let response = ureq::get(url)

        .set("User-Agent", USER_AGENT)

        .timeout(std::time::Duration::from_secs(30))

        .call()

        .map_err(|error| format!("request {}: {}", url, error))?;

    let mut bytes = Vec::new();

    response.into_reader().take(8_000_000).read_to_end(&mut bytes).map_err(|error| error.to_string())?;

    Ok(bytes)

}

