use tauri::Manager;

pub(crate) const LANE_AUTO_CLICK: &str = r#"

(function () {

  const timer = setInterval(() => {

    const candidates = Array.from(document.querySelectorAll('a, button, div, span')).filter((el) => {

      return el.textContent && el.textContent.trim() === 'Download' && el.children.length === 0;

    });

    if (candidates.length > 0) {

      clearInterval(timer);

      candidates[0].click();

    }

  }, 1000);

})();

"#;

#[tauri::command]
pub fn open_download_window(app: tauri::AppHandle, url: String) -> Result<(), String> {

    use tauri::WebviewUrl;

    use tauri::WebviewWindowBuilder;

    let parsed = tauri::Url::parse(&url).map_err(|error| error.to_string())?;

    if let Some(existing) = app.get_webview_window("lane-test") {

        let _ = existing.close();

    }

    WebviewWindowBuilder::new(&app, "lane-test", WebviewUrl::External(parsed))

        .title("FIXED lane")

        .inner_size(520.0, 640.0)

        .initialization_script(LANE_AUTO_CLICK)

        .on_download(|_webview, event| match event {

            tauri::webview::DownloadEvent::Requested { url, destination } => {

                eprintln!("[LANE-TEST] REQUESTED: {} -> {:?}", url, destination);

                false

            }

            tauri::webview::DownloadEvent::Finished { url, path, success } => {

                eprintln!("[LANE-TEST] FINISHED: {} {:?} ok={}", url, path, success);

                true

            }

            _ => true,

        })

        .build()

        .map_err(|error| error.to_string())?;

    Ok(())

}
