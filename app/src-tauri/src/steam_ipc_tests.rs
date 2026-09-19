use super::run_game_expression;

#[test]
fn run_game_expression_quotes_a_gid_past_max_safe_integer() {

    let expression = run_game_expression(10739533279498600448);

    println!("{}", expression);

    assert_eq!(expression, "SteamClient.Apps.RunGame(\"10739533279498600448\", \"\", -1, 100)");

}

#[test]
fn run_game_expression_quotes_a_small_gid_too() {

    let expression = run_game_expression(0);

    assert_eq!(expression, "SteamClient.Apps.RunGame(\"0\", \"\", -1, 100)");

}
