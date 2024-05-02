use std::env;
use std::fs;
use std::os::unix::fs::symlink;
use std::path::Path;

fn main() {
    // 获取当前的工作目录
    let out_dir = env::var("OUT_DIR").unwrap();

    // 构建源文件和目标文件的路径
    let src_exe_path = Path::new(&out_dir)
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("greetings");
    let dst_exe_path = Path::new(&env::var("HOME").unwrap())
        .join("bin")
        .join("greetings");

    // 构建源文件和目标文件的路径
    let src_txt_path = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap())
        .join("src")
        .join("pithy.txt");
    let dst_txt_path = Path::new(&env::var("HOME").unwrap())
        .join("bin")
        .join("pithy.txt");

    // 检查是否正在运行cargo clean命令
    if env::var("CARGO_CLEAN").unwrap_or_default() == "1" {
        // 删除符号链接
        fs::remove_file(&dst_exe_path).unwrap_or_else(|err| {
            eprintln!("Failed to remove symlink {:?}: {}", dst_exe_path, err)
        });
        fs::remove_file(&dst_txt_path).unwrap_or_else(|err| {
            eprintln!("Failed to remove symlink {:?}: {}", dst_txt_path, err)
        });
    } else {
        // 删除旧的符号链接
        let _ = fs::remove_file(&dst_exe_path);
        let _ = fs::remove_file(&dst_txt_path);

        // 创建新的符号链接
        symlink(&src_exe_path, &dst_exe_path).unwrap_or_else(|err| {
            panic!(
                "Failed to create symlink from {:?} to {:?}: {}",
                src_exe_path, dst_exe_path, err
            )
        });
        symlink(&src_txt_path, &dst_txt_path).unwrap_or_else(|err| {
            panic!(
                "Failed to create symlink from {:?} to {:?}: {}",
                src_txt_path, dst_txt_path, err
            )
        });
    }
}
