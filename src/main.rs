mod user_info;

// Functions for getting user info
pub use crate::user_info::user_info_funcs;

fn main() {
    println!("Rustedo!");
    let subtitle = "a simple Rust notepad.";
    println!("{}",subtitle);

    let version : f32 = 1.0; 
    println!("Version {:.1}",version);

    let user_info = user_info_funcs::get_user_info();
    println!("User: {}", user_info);

}
