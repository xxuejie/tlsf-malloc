use cc::Build;

fn main() {
    println!("cargo:rerun-if-changed=./tlsf/tlsf.h");
    println!("cargo:rerun-if-changed=./tlsf/tlsf.c");

    let mut build = Build::new();
    build
        .file("./tlsf/tlsf.c")
        .include("./tlsf")
        .static_flag(true)
        .flag("-O3")
        .flag("-g")
        .flag("-std=c99")
        .flag("-nostdlib")
        .flag("-Wall")
        .flag("-Werror")
        .flag("-Wextra")
        .flag("-fno-builtin-printf")
        .flag("-fno-builtin-memcmp")
        .flag("-fdata-sections")
        .flag("-ffunction-sections")
        .flag("-Dtlsf_assert(...)=")
        .compile("tlsf");
}
