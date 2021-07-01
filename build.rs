use std::env;

fn main() {
    let target_os = env::var("CARGO_CFG_TARGET_OS");
    if let Ok("macos") = target_os.as_ref().map(|x| x.as_str()) {
        println!("cargo:rustc-link-lib=framework=IOKit")
    }
}
