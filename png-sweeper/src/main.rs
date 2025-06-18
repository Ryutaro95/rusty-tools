use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

fn main() -> io::Result<()> {
    let desktop_path = get_desktop_path()?;
    let target_dir = get_target_directory()?;

    ensure_target_directory(&target_dir)?;

    let png_files = find_png_files(&desktop_path)?;

    if png_files.is_empty() {
        println!("デスクトップにPNGファイルが見つかりませんでした。");
        return Ok(());
    }

    println!("{}個のPNGファイルを移動します...", png_files.len());

    for file in png_files {
        move_file(&file, &target_dir)?;
    }

    println!("すべてのPNGファイルの移動が完了しました。");
    Ok(())
}

fn get_desktop_path() -> io::Result<PathBuf> {
    let home = env::var("HOME")
        .map_err(|_| io::Error::new(io::ErrorKind::NotFound, "HOME環境変数が見つかりません"))?;
    Ok(Path::new(&home).join("Desktop"))
}

fn get_target_directory() -> io::Result<PathBuf> {
    let home = env::var("HOME")
        .map_err(|_| io::Error::new(io::ErrorKind::NotFound, "HOME環境変数が見つかりません"))?;
    Ok(Path::new(&home).join("Pictures").join("ScreenShot"))
}

fn ensure_target_directory(target_dir: &Path) -> io::Result<()> {
    if !target_dir.exists() {
        fs::create_dir_all(target_dir)?;
        println!("ディレクトリを作成しました: {}", target_dir.display());
    }
    Ok(())
}

fn find_png_files(desktop_path: &Path) -> io::Result<Vec<PathBuf>> {
    let mut png_files = Vec::new();

    if !desktop_path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!(
                "デスクトップディレクトリが見つかりません: {}",
                desktop_path.display()
            ),
        ));
    }

    for entry in fs::read_dir(desktop_path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            if let Some(extension) = path.extension() {
                if extension.to_string_lossy().to_lowercase() == "png" {
                    png_files.push(path);
                }
            }
        }
    }

    Ok(png_files)
}

fn move_file(source: &Path, target_dir: &Path) -> io::Result<()> {
    let file_name = source
        .file_name()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "ファイル名が取得できません"))?;

    let target_path = target_dir.join(file_name);

    fs::rename(source, &target_path)?;
    println!(
        "移動しました: {} -> {}",
        source.display(),
        target_path.display()
    );

    Ok(())
}
