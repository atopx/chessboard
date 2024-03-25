fn main() {
    println!("cargo:rustc-link-arg=-fapple-link-rtlib");
    tauri_build::build()
}
