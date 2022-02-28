use std::env;

fn main() {
    let target_os = env::var("CARGO_CFG_TARGET_OS");

    if let Ok("macos") = target_os.as_deref() {
        println!("cargo:rustc-link-lib=framework=IOKit")
    }
}
