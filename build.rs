fn main() {
    println!("cargo:warning=Your message here");
    println!("cargo:rustc-link-lib=dylib=raylib");
}
