use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=kernels/score.cu");

    let out_dir = env::var("OUT_DIR").unwrap();
    let ptx_path = PathBuf::from(&out_dir).join("score.ptx");

    let status = Command::new("nvcc")
        .arg("--ptx")
        .arg("-o")
        .arg(&ptx_path)
        .arg("kernels/score.cu")
        .arg("-arch=sm_75") // Or a different architecture if you need
        .arg("--allow-unsupported-compiler") // Allow MSVC compatibility
        .status()
        .expect("Failed to run nvcc");

    if !status.success() {
        panic!("nvcc failed to compile CUDA kernel");
    }

    println!(
        "cargo:rustc-env=KERNEL_SCORE_PTX={}",
        ptx_path.to_str().unwrap()
    );
}
