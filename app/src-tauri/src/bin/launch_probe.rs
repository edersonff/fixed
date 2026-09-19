use app_lib::steam_ipc;

fn main() {

    let title = std::env::args().nth(1).unwrap_or_else(|| String::from("BOMBANANA!"));

    let t0 = std::time::Instant::now();

    println!("[PROBE] launch probe for {} at {:?}", title, t0);

    steam_ipc::ensure_cef_flag().expect("cef flag");

    if !steam_ipc::cef_port_open() {

        println!("[PROBE] port closed, starting steam -silent");

        std::process::Command::new("steam")

            .arg("-silent")

            .stdin(std::process::Stdio::null())

            .stdout(std::process::Stdio::null())

            .stderr(std::process::Stdio::null())

            .spawn()

            .expect("spawn steam");

        let mut tries = 0;

        while !steam_ipc::cef_port_open() && tries < 45 {

            println!("[PROBE] waiting for cef port, {}s", tries * 2);

            std::thread::sleep(std::time::Duration::from_secs(2));

            tries += 1;

        }

    }

    if !steam_ipc::cef_port_open() {

        println!("[PROBE] FAILED: cef port never opened after 90s");

        std::process::exit(2);

    }

    println!("[PROBE] cef port open at {}ms", t0.elapsed().as_millis());

    let folder = format!("{}/games/{}", std::env::var("HOME").unwrap_or_default(), title);

    let exe = fix_core::find_game_exe(&folder).expect("game exe");

    let vdf = app_lib::find_shortcuts_vdf().expect("shortcuts vdf");

    let appid = fix_core::find_shortcut_appid_by_exe(&vdf, &exe)

        .or_else(|| fix_core::find_shortcut_appid(&vdf, &title))

        .expect("shortcut appid");

    println!("[PROBE] appid {} for {}", appid, title);

    let runtime = tokio::runtime::Builder::new_current_thread()

        .enable_all()

        .build()

        .expect("tokio runtime");

    match runtime.block_on(steam_ipc::cef_run_game(appid)) {

        Ok(gid) => println!("[PROBE] RunGame fired, gid {}", gid),

        Err(error) => {

            println!("[PROBE] RunGame FAILED: {}", error);

            std::process::exit(3);

        }

    }

    println!("[PROBE] done at {}ms", t0.elapsed().as_millis());

}
