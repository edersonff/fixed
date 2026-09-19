use super::exe_basename;
use super::poll_until;
use super::poll_until_some;
use super::TitleGuard;
use std::time::Duration;

#[test]
fn exe_basename_strips_the_directory() {

    assert_eq!(exe_basename("/home/user/games/BOMBANANA/BOMBANANA.exe"), "BOMBANANA.exe");

}

#[test]
fn exe_basename_keeps_a_bare_filename() {

    assert_eq!(exe_basename("BOMBANANA.exe"), "BOMBANANA.exe");

}

#[test]
fn title_guard_rejects_a_second_acquire_for_the_same_title() {

    let first = TitleGuard::acquire("bombanana-dedup-test").expect("first acquire succeeds");

    assert!(TitleGuard::acquire("bombanana-dedup-test").is_none());

    drop(first);

    assert!(TitleGuard::acquire("bombanana-dedup-test").is_some());

}

#[test]
fn poll_until_returns_true_immediately_when_the_check_already_passes() {

    assert!(poll_until(Duration::from_millis(50), Duration::from_millis(10), || true));

}

#[test]
fn poll_until_times_out_and_returns_false() {

    assert!(!poll_until(Duration::from_millis(30), Duration::from_millis(10), || false));

}

#[test]
fn poll_until_some_returns_the_value_once_the_check_finds_it() {

    let mut calls = 0;

    let result = poll_until_some(Duration::from_millis(100), Duration::from_millis(10), || {

        calls += 1;

        if calls >= 2 {

            Some(calls)

        } else {

            None

        }

    });

    assert_eq!(result, Some(2));

}
