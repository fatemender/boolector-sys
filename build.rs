fn main() {
    println!("cargo:rustc-link-lib=boolector");
    println!("cargo:rustc-link-lib=lgl");
}
