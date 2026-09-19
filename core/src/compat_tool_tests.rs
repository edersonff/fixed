use super::*;

const SAMPLE: &str = "\t\t\t\t\"CompatToolMapping\"\n\t\t\t\t{\n\t\t\t\t\t\"3299031949\"\n\t\t\t\t\t{\n\t\t\t\t\t\t\"name\"\t\t\"proton_hotfix\"\n\t\t\t\t\t\t\"config\"\t\t\"\"\n\t\t\t\t\t\t\"priority\"\t\t\"250\"\n\t\t\t\t\t}\n\t\t\t\t}\n";

#[test]
fn has_mapping_finds_an_existing_appid() {

    assert!(has_mapping(SAMPLE, 3299031949));

    assert!(!has_mapping(SAMPLE, 3361250326));

}

#[test]
fn insert_mapping_adds_the_appid_with_the_named_tool() {

    let out = insert_mapping(SAMPLE, 3361250326, DEFAULT_COMPAT_TOOL).expect("inserted");

    assert!(has_mapping(&out, 3361250326));

    assert!(has_mapping(&out, 3299031949));

    assert!(out.contains("\"name\"\t\t\"proton_experimental\""));

}

#[test]
fn insert_mapping_returns_none_when_the_appid_is_already_mapped() {

    assert!(insert_mapping(SAMPLE, 3299031949, DEFAULT_COMPAT_TOOL).is_none());

}

#[test]
fn insert_mapping_keeps_every_byte_of_the_original_body() {

    let out = insert_mapping(SAMPLE, 3361250326, DEFAULT_COMPAT_TOOL).expect("inserted");

    let stripped: String = out.chars().filter(|c| !c.is_whitespace()).collect();

    let original: String = SAMPLE.chars().filter(|c| !c.is_whitespace()).collect();

    let added: String = "\"3361250326\"{\"name\"\"proton_experimental\"\"config\"\"\"\"priority\"\"250\"}"
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect();

    assert_eq!(stripped.len(), original.len() + added.len());

    assert!(stripped.contains(&added));

}

#[test]
fn insert_mapping_returns_none_when_the_section_is_absent() {

    assert!(insert_mapping("\"InstallConfigStore\"\n{\n}\n", 1, DEFAULT_COMPAT_TOOL).is_none());

}

#[test]
fn remove_mapping_removes_only_the_target_appid() {

    let out = remove_mapping(SAMPLE, 3299031949).expect("removed");

    assert!(!has_mapping(&out, 3299031949));

    assert_eq!(out, "\t\t\t\t\"CompatToolMapping\"\n\t\t\t\t{\n\t\t\t\t}\n");

}

#[test]
fn remove_mapping_leaves_every_other_appid_and_byte_untouched() {

    let with_two = insert_mapping(SAMPLE, 3361250326, DEFAULT_COMPAT_TOOL).expect("inserted");

    let out = remove_mapping(&with_two, 3361250326).expect("removed");

    assert!(has_mapping(&out, 3299031949));

    assert!(!has_mapping(&out, 3361250326));

    assert_eq!(out, SAMPLE);

}

#[test]
fn remove_mapping_returns_none_when_the_appid_is_absent() {

    assert!(remove_mapping(SAMPLE, 1).is_none());

}

#[test]
fn remove_mapping_returns_none_when_the_section_is_absent() {

    assert!(remove_mapping("\"InstallConfigStore\"\n{\n}\n", 3299031949).is_none());

}

#[test]
fn remove_compat_tool_writes_a_rollback_copy_and_removes_the_mapping() {

    let dir = tempfile::tempdir().expect("tempdir");

    std::fs::create_dir_all(dir.path().join("config")).expect("mkdir config");

    let config_path = config_vdf_path(dir.path().to_str().unwrap());

    std::fs::write(&config_path, SAMPLE).expect("write fixture");

    let removed = remove_compat_tool(dir.path().to_str().unwrap(), 3299031949).expect("remove");

    assert!(removed);

    let backup = std::fs::read_to_string(format!("{}.bak-fixed", config_path)).expect("read backup");

    assert_eq!(backup, SAMPLE);

    let updated = std::fs::read_to_string(&config_path).expect("read config");

    assert!(!has_mapping(&updated, 3299031949));

}

#[test]
fn remove_compat_tool_returns_false_and_writes_no_backup_when_the_appid_is_absent() {

    let dir = tempfile::tempdir().expect("tempdir");

    std::fs::create_dir_all(dir.path().join("config")).expect("mkdir config");

    let config_path = config_vdf_path(dir.path().to_str().unwrap());

    std::fs::write(&config_path, SAMPLE).expect("write fixture");

    let removed = remove_compat_tool(dir.path().to_str().unwrap(), 1).expect("remove call");

    assert!(!removed);

    assert!(!std::path::Path::new(&format!("{}.bak-fixed", config_path)).exists());

}
