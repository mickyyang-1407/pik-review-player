fn main() {
    println!("cargo:rerun-if-changed=src/player/atmos_wrapper.m");
    // Need macOS 11.0+ for allowedAudioSpatializationFormats
    std::env::set_var("MACOSX_DEPLOYMENT_TARGET", "11.0");
    cc::Build::new()
        .file("src/player/atmos_wrapper.m")
        .flag("-fobjc-arc")
        .flag("-mmacosx-version-min=11.0")
        .compile("atmos_wrapper");
    // Link required Apple frameworks
    println!("cargo:rustc-link-lib=framework=AVFoundation");
    println!("cargo:rustc-link-lib=framework=CoreMedia");
    println!("cargo:rustc-link-lib=framework=CoreAudio");
    println!("cargo:rustc-link-lib=framework=MediaToolbox");
    println!("cargo:rustc-link-lib=framework=Accelerate");
    tauri_build::build()
}
