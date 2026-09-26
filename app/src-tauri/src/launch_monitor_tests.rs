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

#[cfg(not(windows))]
#[test]
#[ignore = "needs python3, ss and outbound https reach (live network)"]
fn connection_signal_detects_browser_named_established_socket() {

    let mut simulator = std::process::Command::new("python3")

        .arg("-c")

        .arg("import socket, time\nopen('/proc/self/comm', 'w').write('chrome')\nwhile True:\n    socks = []\n    for info in socket.getaddrinfo('online-fix.me', 443, socket.AF_INET, socket.SOCK_STREAM):\n        try:\n            socks.append(socket.create_connection(info[4], timeout=5))\n        except OSError:\n            pass\n    time.sleep(5)\n    for s in socks:\n        try:\n            s.close()\n        except OSError:\n            pass")

        .stdout(std::process::Stdio::null())

        .stderr(std::process::Stdio::null())

        .spawn()

        .expect("python3 available");

    std::thread::sleep(std::time::Duration::from_secs(5));

    let detected = super::fix_site_connection_open();

    let _ = simulator.kill();

    let _ = simulator.wait();

    assert!(detected, "expected the chrome-named established socket to online-fix.me to be detected");

}
