fn main() {
    if let Some(datastax_dir) = option_env!("CASSANDRA_SYS_LIB_PATH") {
        for p in datastax_dir.split(";") {
            println!("cargo:rustc-link-search={}", p);
        }
    }

    println!("cargo:rustc-flags=-l static=cassandra_static");
    println!("cargo:rustc-link-search={}", "/usr/lib/x86_64-linux-gnu");
    println!("cargo:rustc-link-search={}", "/usr/local/lib/x86_64-linux-gnu");
    println!("cargo:rustc-link-search={}", "/usr/local/lib64");
    println!("cargo:rustc-link-search={}", "/usr/local/lib");
    println!("cargo:rustc-link-search={}", "/usr/lib64/");
    println!("cargo:rustc-link-search={}", "/usr/lib/");
}
