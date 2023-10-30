use std::{path::Path, fs::{self, File, rename}, io::{Read, Write}};

use walkdir::WalkDir;
use zip::{ZipWriter, write::FileOptions, CompressionMethod};

pub fn copy_directory(src: &Path, dst: &Path) -> std::io::Result<()> {
    if src.is_dir() {
        fs::create_dir_all(dst)?;
        for entry in fs::read_dir(src)? {
            let entry = entry?;
            let entry_path = entry.path();
            let new_dst = dst.join(entry_path.file_name().unwrap());
            copy_directory(&entry_path, &new_dst)?;
        }
    } else {
        fs::copy(src, dst)?;
    }
    Ok(())
}

pub fn create_pptx(src: &str) -> std::io::Result<()> {
    let archive_name = "archive.zip";

    let archive_file = File::create(archive_name)?;
    let mut archive = ZipWriter::new(archive_file);

    for entry in WalkDir::new(src).into_iter().filter_map(|e| e.ok()) {
        if !entry.file_type().is_file() {
            continue; // Ignore directories
        }

        let file_name_in_zip = entry.path().strip_prefix(src).unwrap().to_string_lossy().into_owned();
        let file_content = File::open(entry.path())?;

        let options = FileOptions::default()
            .compression_method(CompressionMethod::Stored); // Preserve file permissions

        archive.start_file(file_name_in_zip, options)?;
        let mut buffer = Vec::new();
        file_content.take(u64::MAX).read_to_end(&mut buffer)?;

        archive.write_all(&buffer)?;
    }

    rename(archive_name, "presentation.pptx")?;
    Ok(())
}