fn main() {
    println!("cargo:rustc-link-search=native=.");
    println!("cargo:rustc-link-lib=add");
    println!("cargo:rustc-link-lib=sub");
}
