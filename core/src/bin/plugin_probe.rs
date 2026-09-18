fn main() {

    let title_folder = "/home/eder/games/Friendly Steps";

    let game_dir = "/home/eder/games/Friendly Steps/Friendly Steps";

    match fix_core::install_plugin(

        "/home/eder/Downloads/glarmer-PEAK_Unlimited-4.0.1.zip",

        game_dir,

    ) {

        Ok(count) => println!("plugin installed: {} files", count),

        Err(error) => println!("plugin FAILED: {}", error),

    }

    match fix_core::apply_fix_repair(title_folder, game_dir) {

        Ok(count) => println!("fix repair reapplied: {} archive(s)", count),

        Err(error) => println!("fix repair FAILED: {}", error),

    }

}
