fn main() {
    println!("cargo:rustc-link-lib=mylib");
    println!("cargo:rustc-link-search=./c_lib");
}
