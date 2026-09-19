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
