fn compile() -> String {
    let build_type = if Ok("release".to_owned()) == std::env::var("PROFILE") {
        "Release"
    } else {
        "Debug"
    };

    let mut conf = cmake::Config::new("GameNetworkingSockets");

    let dst = conf
        .define("CMAKE_BUILD_TYPE", build_type)
        .build();

    dst.display().to_string()
}

fn generate_bindings(out_dir: String) {

    let include_dirs = vec![
        format!("{}/include/GameNetworkingSockets/steam", out_dir),
    ];

    let build = autocxx_build::Builder::new("src/lib.rs", &include_dirs)
        .extra_clang_args(&["-std=c++14"])
        .expect_build();
    build.compile("gns-lib");

    println!("cargo:rerun-if-changed=src/lib.rs");

    println!("cargo:rustc-link-search={}/lib", out_dir);
    println!("cargo:rustc-link-lib=GameNetworkingSockets_s");

    let gns_deps = vec![
        "protobuf",
        "openssl",
    ];

    for dep in gns_deps {
        for lib in pkg_config::probe_library(dep).unwrap().libs {
            println!("cargo:rustc-link-lib={}", lib)
        }
    }
}

fn main() {
    let out_dir = compile();
    generate_bindings(out_dir);
}
