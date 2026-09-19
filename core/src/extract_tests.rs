use super::*;

fn build_zip_fixture(entries: &[(&str, &[u8])]) -> Vec<u8> {

    let mut writer = zip::ZipWriter::new(std::io::Cursor::new(Vec::new()));

    let options = zip::write::SimpleFileOptions::default();

    for (name, content) in entries {

        writer.start_file(*name, options).expect("start zip entry");

        std::io::Write::write_all(&mut writer, content).expect("write zip entry");

    }

    writer.finish().expect("finish zip").into_inner()

}

#[test]
fn install_plugin_places_root_level_dll_into_bepinex_plugins() {

    let dir = tempfile::tempdir().expect("tempdir");

    let archive_path = dir.path().join("MyPlugin.dll");

    std::fs::write(&archive_path, b"MZ-raw-dll-bytes").expect("write dll fixture");

    let game_dir = dir.path().join("game");

    std::fs::create_dir_all(&game_dir).expect("make game dir");

    let count = install_plugin(archive_path.to_str().unwrap(), game_dir.to_str().unwrap()).expect("install");

    assert_eq!(count, 1);

    let placed = game_dir.join("BepInEx/plugins/MyPlugin.dll");

    assert_eq!(std::fs::read(placed).expect("dll placed"), b"MZ-raw-dll-bytes");

}

#[test]
fn install_plugin_extracts_zip_skips_meta_files_case_insensitively() {

    let dir = tempfile::tempdir().expect("tempdir");

    let zip_bytes = build_zip_fixture(&[
        ("manifest.json", b"{}"),
        ("ICON.PNG", b"binary"),
        ("MyPlugin.dll", b"dll-bytes"),
        ("config/settings.cfg", b"key=value"),
    ]);

    let archive_path = dir.path().join("plugin.zip");

    std::fs::write(&archive_path, zip_bytes).expect("write zip fixture");

    let game_dir = dir.path().join("game");

    std::fs::create_dir_all(&game_dir).expect("make game dir");

    let count = install_plugin(archive_path.to_str().unwrap(), game_dir.to_str().unwrap()).expect("install");

    assert_eq!(count, 2);

    assert!(game_dir.join("BepInEx/plugins/MyPlugin.dll").exists());

    assert_eq!(std::fs::read_to_string(game_dir.join("config/settings.cfg")).unwrap(), "key=value");

    assert!(!game_dir.join("manifest.json").exists());

    assert!(!game_dir.join("ICON.PNG").exists());

}

#[test]
fn install_plugin_routes_rar_magic_to_archive_extraction_not_zip_parsing() {

    let dir = tempfile::tempdir().expect("tempdir");

    let archive_path = dir.path().join("plugin.rar");

    let mut fake_rar = b"Rar!".to_vec();

    fake_rar.extend_from_slice(b"not-a-real-archive");

    std::fs::write(&archive_path, fake_rar).expect("write rar fixture");

    let game_dir = dir.path().join("game");

    std::fs::create_dir_all(&game_dir).expect("make game dir");

    let error = install_plugin(archive_path.to_str().unwrap(), game_dir.to_str().unwrap()).unwrap_err();

    assert!(!error.starts_with("open zip"), "fake rar bytes must not fall through to zip parsing: {}", error);

}

#[test]
fn place_extracted_skips_meta_files_and_specializes_root_level_dll() {

    let dir = tempfile::tempdir().expect("tempdir");

    let source = dir.path().join("source");

    std::fs::create_dir_all(source.join("Sub")).expect("make source tree");

    std::fs::write(source.join("manifest.json"), b"{}").unwrap();

    std::fs::write(source.join("ReadMe.MD"), b"docs").unwrap();

    std::fs::write(source.join("RootPlugin.dll"), b"root-dll").unwrap();

    std::fs::write(source.join("Sub/Config.txt"), b"cfg").unwrap();

    std::fs::write(source.join("Sub/Nested.dll"), b"nested-dll").unwrap();

    let game_dir = dir.path().join("game");

    std::fs::create_dir_all(&game_dir).expect("make game dir");

    let count = place_extracted(source.to_str().unwrap(), game_dir.to_str().unwrap()).expect("place extracted");

    assert_eq!(count, 3);

    assert_eq!(std::fs::read(game_dir.join("BepInEx/plugins/RootPlugin.dll")).unwrap(), b"root-dll");

    assert_eq!(std::fs::read(game_dir.join("Sub/Nested.dll")).unwrap(), b"nested-dll");

    assert_eq!(std::fs::read(game_dir.join("Sub/Config.txt")).unwrap(), b"cfg");

    assert!(!game_dir.join("manifest.json").exists());

    assert!(!game_dir.join("ReadMe.MD").exists());

}

#[test]
fn apply_fix_repair_returns_zero_when_no_rar_files_present() {

    let dir = tempfile::tempdir().expect("tempdir");

    let title_folder = dir.path().join("Title");

    let repair_dir = title_folder.join("Fix Repair");

    std::fs::create_dir_all(&repair_dir).expect("make repair dir");

    std::fs::write(repair_dir.join("readme.txt"), b"not an archive").unwrap();

    std::fs::write(repair_dir.join("notes.ini"), b"[section]").unwrap();

    let game_dir = dir.path().join("game");

    std::fs::create_dir_all(&game_dir).expect("make game dir");

    let applied = apply_fix_repair(title_folder.to_str().unwrap(), game_dir.to_str().unwrap()).expect("apply fix repair");

    assert_eq!(applied, 0);

}

#[test]
fn apply_fix_repair_matches_rar_extension_case_insensitively() {

    let dir = tempfile::tempdir().expect("tempdir");

    let title_folder = dir.path().join("Title");

    let repair_dir = title_folder.join("Fix Repair");

    std::fs::create_dir_all(&repair_dir).expect("make repair dir");

    std::fs::write(repair_dir.join("patch.RAR"), b"not-a-real-archive").unwrap();

    let game_dir = dir.path().join("game");

    std::fs::create_dir_all(&game_dir).expect("make game dir");

    let error = apply_fix_repair(title_folder.to_str().unwrap(), game_dir.to_str().unwrap()).unwrap_err();

    assert!(!error.starts_with("read"), "uppercase .RAR must be picked up for extraction, got: {}", error);

}
