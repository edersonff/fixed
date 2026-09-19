fn main() {

    println!("HOME={:?}", std::env::var("HOME").ok());

    println!("USERPROFILE={:?}", std::env::var("USERPROFILE").ok());

    println!("fixed_home={:?}", fix_core::fixed_home());

}
