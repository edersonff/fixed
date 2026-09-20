use super::*;

#[test]
fn tasklist_reports_running_true_on_a_real_match_line() {

    let output = "steam.exe                    1234 Console                    1    123,456 K";

    assert!(tasklist_reports_running(output, "steam.exe"));

}

#[test]
fn tasklist_reports_running_false_on_the_no_tasks_message() {

    let output = "INFO: No tasks are running which match the specified criteria.";

    assert!(!tasklist_reports_running(output, "steam.exe"));

}

#[test]
fn tasklist_reports_running_false_on_empty_output() {

    assert!(!tasklist_reports_running("", "steam.exe"));

}

#[test]
fn tasklist_reports_running_is_case_insensitive() {

    let output = "STEAM.EXE                    1234 Console                    1    123,456 K";

    assert!(tasklist_reports_running(output, "steam.exe"));

}

#[test]
fn strip_bundle_paths_removes_appimage_entries_and_keeps_system_paths() {

    let markers = vec![String::from("squashfs-root"), String::from(".mount_")];

    let polluted = "/home/eder/.cache/appimage-x/squashfs-root/usr/bin:/usr/bin:/home/eder/.local/bin";

    let clean = strip_bundle_paths(polluted, &markers);

    assert_eq!(clean, "/usr/bin:/home/eder/.local/bin");

}

#[test]
fn strip_bundle_paths_keeps_a_clean_path_intact() {

    let markers = vec![String::from("squashfs-root")];

    let clean_path = "/usr/local/bin:/usr/bin";

    assert_eq!(strip_bundle_paths(clean_path, &markers), clean_path);

}

#[test]
fn strip_bundle_paths_empties_a_fully_polluted_value() {

    let markers = vec![String::from(".mount_")];

    let polluted = "/tmp/.mount_fixedAbCd/usr/lib::/tmp/.mount_fixedAbCd/usr/bin";

    assert_eq!(strip_bundle_paths(polluted, &markers), "");

}

#[test]
fn strip_bundle_paths_strips_every_ld_library_entry_of_the_bundle() {

    let markers = vec![String::from("/home/eder/.cache/appimage-x/squashfs-root")];

    let polluted = "/home/eder/.cache/appimage-x/squashfs-root/usr/lib/:/home/eder/.cache/appimage-x/squashfs-root/lib/:/opt/cuda/lib64";

    let clean = strip_bundle_paths(polluted, &markers);

    assert_eq!(clean, "/opt/cuda/lib64");

}

#[test]
fn carries_bundle_path_detects_pythonhome_pointing_into_bundle() {

    let markers = vec![String::from("squashfs-root"), String::from(".mount_")];

    let polluted = "/home/eder/.cache/pub-e2e/squashfs-root/usr/";

    assert!(carries_bundle_path(polluted, &markers));

}

#[test]
fn carries_bundle_path_false_for_system_values() {

    let markers = vec![String::from("squashfs-root"), String::from(".mount_")];

    assert!(!carries_bundle_path("/usr/share:/home/eder/.local/share", &markers));

    assert!(!carries_bundle_path("", &markers));

}
