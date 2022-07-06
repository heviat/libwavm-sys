fn main() {
    let wavm_path = std::env::var("WAVM_LIB_PATH").unwrap();
    println!("cargo:rustc-link-lib={}", wavm_path);
 } 