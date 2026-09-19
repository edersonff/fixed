use serde_json::Value;

pub const CEF_DEBUG_PORT: u16 = 8080;

pub fn cef_flag_path() -> Option<std::path::PathBuf> {

    crate::steam_client::steam_root().map(|root| root.join(".cef-enable-remote-debugging"))

}

pub fn ensure_cef_flag() -> Result<bool, String> {

    let path = cef_flag_path().ok_or_else(|| String::from("steam root not found"))?;

    if path.exists() {

        return Ok(false);

    }

    std::fs::File::create(&path).map_err(|error| format!("create cef flag: {}", error))?;

    eprintln!("[CEF] flag created at {}", path.display());

    Ok(true)

}

pub fn cef_port_open() -> bool {

    let url = format!("http://127.0.0.1:{}/json", CEF_DEBUG_PORT);

    fix_core::fetch_bytes(&url).is_ok()

}

fn shared_js_ws_url() -> Result<String, String> {

    let url = format!("http://127.0.0.1:{}/json", CEF_DEBUG_PORT);

    let body = fix_core::fetch_bytes(&url).map_err(|error| format!("cef /json: {}", error))?;

    let text = String::from_utf8_lossy(&body).to_string();

    let targets: Value = serde_json::from_str(&text).map_err(|error| format!("cef /json parse: {}", error))?;

    let list = targets.as_array().ok_or_else(|| String::from("cef /json: expected array"))?;

    for target in list {

        let title = target.get("title").and_then(Value::as_str).unwrap_or("");

        if title.contains("SharedJSContext") {

            if let Some(ws) = target.get("webSocketDebuggerUrl").and_then(Value::as_str) {

                return Ok(String::from(ws));

            }

        }

    }

    Err(String::from("cef /json: SharedJSContext target not found"))

}

fn json_quote(value: &str) -> String {

    serde_json::to_string(value).unwrap_or_else(|_| String::from("\"\""))

}

async fn cdp_evaluate(expression: &str) -> Result<Value, String> {

    let ws_url = shared_js_ws_url()?;

    let expr = String::from(expression);

    tokio::task::spawn_blocking(move || cdp_evaluate_blocking(&ws_url, &expr))

        .await

        .map_err(|error| format!("cdp join: {}", error))?

}

fn cdp_evaluate_blocking(ws_url: &str, expression: &str) -> Result<Value, String> {

    use tungstenite::Message;

    let (mut socket, _response) = tungstenite::connect(ws_url)

        .map_err(|error| format!("cef ws connect: {}", error))?;

    let request = serde_json::json!({

        "id": 1,

        "method": "Runtime.evaluate",

        "params": {

            "expression": expression,

            "awaitPromise": true,

            "returnByValue": true,

        },

    });

    socket

        .send(Message::Text(request.to_string()))

        .map_err(|error| format!("cef ws send: {}", error))?;

    loop {

        let message = socket

            .read()

            .map_err(|error| format!("cef ws read: {}", error))?;

        let Message::Text(text) = message else {

            continue;

        };

        let parsed: Value = serde_json::from_str(&text).map_err(|error| format!("cef ws parse: {}", error))?;

        if parsed.get("id").and_then(Value::as_i64) != Some(1) {

            continue;

        }

        socket.close(None).ok();

        if let Some(error) = parsed.get("error") {

            return Err(format!("cef evaluate: {}", error));

        }

        if let Some(exception) = parsed.pointer("/result/exceptionDetails") {

            return Err(format!("cef js exception: {}", exception));

        }

        return parsed

            .pointer("/result/result/value")

            .cloned()

            .ok_or_else(|| String::from("cef evaluate: no value"));

    }

}

pub async fn cef_add_shortcut(name: &str, exe: &str) -> Result<u32, String> {

    let expression = format!(

        "SteamClient.Apps.AddShortcut({}, {}, '', '')",

        json_quote(name),

        json_quote(exe),

    );

    let value = cdp_evaluate(&expression).await?;

    let appid = value

        .as_u64()

        .ok_or_else(|| format!("cef AddShortcut returned non-number: {:?}", value))?;

    eprintln!("[CEF] AddShortcut {} -> appid {}", name, appid);

    Ok(appid as u32)

}

pub async fn cef_set_launch_options(appid: u32, launch_options: &str) -> Result<(), String> {

    let expression = format!(

        "SteamClient.Apps.SetShortcutLaunchOptions({}, {})",

        appid,

        json_quote(launch_options),

    );

    cdp_evaluate(&expression).await?;

    eprintln!("[CEF] SetShortcutLaunchOptions {} ok", appid);

    Ok(())

}

pub async fn cef_run_game(appid: u32) -> Result<u64, String> {

    let gid = ((appid as u64) << 32) | 0x02000000;

    let expression = format!("SteamClient.Apps.RunGame({}, \"\", -1, 100)", gid);

    cdp_evaluate(&expression).await?;

    eprintln!("[CEF] RunGame gid {} ok", gid);

    Ok(gid)

}
