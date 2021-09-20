fn main() {
    #[cfg(not(target_os = "windows"))]
    {
        println!("cargo:rustc-cdylib-link-arg=-Wl,-undefined,dynamic_lookup");
    }
}
