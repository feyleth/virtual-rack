use std::env;

fn main() {
    if let Ok(profile) = env::var("PROFILE") {
        println!("cargo:rustc-check-cfg=cfg(build,values(\"debug\",\"release\"))");
        println!("cargo:rustc-cfg=build={:?}", profile);
    }
}
