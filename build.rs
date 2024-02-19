//use cc::Build;
//use std::path::Path;

fn main() {
    /*
    let fsst = Path::new("vendor/fsst");

    Build::new()
        .cpp(true)
        .std("c++17")
        .flag("-W")
        .flag("-Wall")
        .flag("-g")
        //.flag("-01")
        .flag("-march=native")
        .flag("-c")
        .file(fsst.join("fsst_avx512.cpp"))
        .compile("fsst_avx512");

    Build::new()
        .cpp(true)
        .std("c++17")
        .flag("-W")
        .flag("-Wall")
        .flag("-g")
        //.flag("-01")
        .flag("-march=native")
        .flag("-c")
        .file(fsst.join("libfsst.cpp"))
        .compile("libfsst")
    */
    println!("cargo:rustc-link-search=.");
    println!("cargo:rustc-link-lib=fsst");
}
