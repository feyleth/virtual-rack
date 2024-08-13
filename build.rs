use std::env;

fn main() {
    if let Ok(profile) = env::var("PROFILE") {
        println!("cargo:rustc-check-cfg=build={:?}", profile);
    }
}
