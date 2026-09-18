use fix_core::decode_cp1251;

use fix_core::parse_detail;

use fix_core::parse_home;

mod assets;

fn main() {

    let home_bytes = std::fs::read("tests/fixtures/home.html").expect("home fixture readable");

    let home_html = decode_cp1251(&home_bytes);

    let entries = parse_home(&home_html);

    println!("home parsed: {}", entries.len());

    let live = fix_core::home_games(1);

    println!("home_games: source={} games={}", live.source, live.games.len());

    let search = fix_core::search_games("Ready or Not");

    println!("search: source={} games={}", search.source, search.games.len());

    match fix_core::find_game_exe("/home/eder/games/Friendly Steps") {

        Some(exe) => println!("game exe: {}", exe),

        None => println!("game exe: NOT FOUND"),

    }

    let vdf = format!("{}/.steam/steam/userdata/256021013/config/shortcuts.vdf", std::env::var("HOME").unwrap_or_default());

    match fix_core::add_steam_shortcut(

        &vdf,

        "Friendly Steps",

        "/home/eder/games/Friendly Steps/Friendly Steps/Friendly Steps.exe",

        "/home/eder/games/Friendly Steps/Friendly Steps/",

        fix_core::ONLINE_FIX_LAUNCH_OPTIONS,

    ) {

        Ok(index) => println!("steam shortcut: index {}", index),

        Err(error) => println!("steam shortcut FAILED: {}", error),

    }

    if let Some(first) = search.games.first() {

        println!("  first: {} | {} | {}", first.title, first.category, first.published_at);

    }

    let probe = fix_core::probe_lane("https://uploads.online-fix.me:2053/uploads/Ready%20or%20Not/");

    println!("probe: parts={} first_part_bytes={}", probe.parts.len(), probe.first_part_bytes);

    for part in probe.parts.iter().take(3) {

        println!("  part: {}", part.rsplit('/').next().unwrap_or(""));

    }

    if let Some(first) = probe.parts.first() {

        match fix_core::download_partial(first, "/tmp/opencode/probe-part.bin", 2_000_000) {

            Ok(written) => println!("download_partial: {} bytes to /tmp/opencode/probe-part.bin", written),

            Err(error) => println!("download_partial FAILED: {}", error),

        }

    }

    for entry in entries.iter().take(2) {

        println!("  {} | {} | views={}", entry.title, entry.category, entry.views);

    }

    let detail_bytes = std::fs::read("tests/fixtures/detail.html").expect("detail fixture readable");

    let detail_html = decode_cp1251(&detail_bytes);

    let detail = parse_detail(&detail_html);

    println!("detail: {} | build={} | fix_repair={}", detail.title, detail.build, detail.mentions_fix_repair);

    println!("  steam_ext: {}", if detail.steam_ext_url.is_empty() { "MISSING" } else { "present" });

    for lane in detail.lanes.iter() {

        println!("  lane {}: {}", lane.kind, lane.url);

    }

    println!("video: {}", detail.video_id);

    match assets::save_app_assets(1144200, "assets") {

        Ok(saved) => for line in saved {

            println!("asset: {}", line);

        },

        Err(error) => println!("asset FAILED: {}", error),

    }

}
