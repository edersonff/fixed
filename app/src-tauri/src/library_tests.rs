use super::*;

struct HomeFixture {

    _home: tempfile::TempDir,

    previous_home: Option<String>,

}

impl Drop for HomeFixture {

    fn drop(&mut self) {

        match self.previous_home.take() {
            Some(home) => std::env::set_var("HOME", home),
            None => std::env::remove_var("HOME"),
        }

    }

}

fn fixture_with_games_dir() -> HomeFixture {

    let previous_home = std::env::var("HOME").ok();

    let home = tempfile::tempdir().expect("home tempdir");

    std::env::set_var("HOME", home.path());

    std::fs::create_dir_all(home.path().join("games")).expect("create games root");

    HomeFixture { _home: home, previous_home }

}

#[test]
fn uninstall_game_rejects_a_title_containing_dot_dot() {

    let _guard = crate::test_support::HOME_ENV_LOCK.lock().unwrap_or_else(|poisoned| poisoned.into_inner());

    let _fixture = fixture_with_games_dir();

    let result = uninstall_game(String::from(".."), false);

    assert!(result.is_err());

}

#[test]
fn uninstall_game_rejects_a_title_containing_a_path_separator() {

    let _guard = crate::test_support::HOME_ENV_LOCK.lock().unwrap_or_else(|poisoned| poisoned.into_inner());

    let _fixture = fixture_with_games_dir();

    let result = uninstall_game(String::from("/etc/passwd"), false);

    assert!(result.is_err());

}

#[test]
fn uninstall_game_rejects_a_missing_folder() {

    let _guard = crate::test_support::HOME_ENV_LOCK.lock().unwrap_or_else(|poisoned| poisoned.into_inner());

    let _fixture = fixture_with_games_dir();

    let result = uninstall_game(String::from("DoesNotExist"), false);

    assert!(result.is_err());

}

#[cfg(unix)]
#[test]
fn uninstall_game_rejects_a_symlink_and_deletes_nothing() {

    let _guard = crate::test_support::HOME_ENV_LOCK.lock().unwrap_or_else(|poisoned| poisoned.into_inner());

    let _fixture = fixture_with_games_dir();

    let real_target = tempfile::tempdir().expect("real target tempdir");

    std::fs::write(real_target.path().join("marker.txt"), b"keep me").expect("write marker");

    let games_root = std::env::var("HOME").map(std::path::PathBuf::from).unwrap().join("games");

    std::os::unix::fs::symlink(real_target.path(), games_root.join("Foo")).expect("create symlink");

    let result = uninstall_game(String::from("Foo"), false);

    assert!(result.is_err());

    assert!(real_target.path().join("marker.txt").exists());

}

#[test]
fn uninstall_game_deletes_the_folder_when_removal_from_steam_is_not_requested() {

    let _guard = crate::test_support::HOME_ENV_LOCK.lock().unwrap_or_else(|poisoned| poisoned.into_inner());

    let _fixture = fixture_with_games_dir();

    let games_root = std::env::var("HOME").map(std::path::PathBuf::from).unwrap().join("games");

    let game_dir = games_root.join("Friendly Steps");

    std::fs::create_dir_all(&game_dir).expect("create game dir");

    std::fs::write(game_dir.join("Friendly Steps.exe"), b"stub").expect("write stub exe");

    let result = uninstall_game(String::from("Friendly Steps"), false);

    assert!(result.is_ok());

    assert!(!game_dir.exists());

}
