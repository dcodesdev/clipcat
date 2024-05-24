use std::fs;
use std::io::Read;
use std::path::Path;

pub fn read_directory_contents(dir: &Path) -> anyhow::Result<String> {
    let mut combined_content = String::new();

    if dir.is_file() {
        read_file(dir, &mut combined_content)?;
    } else {
        read_directory_contents_recursive(dir, dir, &mut combined_content)?;
    }

    Ok(combined_content)
}

pub fn read_file(path: &Path, content: &mut String) -> anyhow::Result<()> {
    let file_content = std::fs::read_to_string(path)?;

    *content = file_content;

    Ok(())
}

fn read_directory_contents_recursive(
    base_path: &Path,
    current_path: &Path,
    combined_content: &mut String,
) -> anyhow::Result<()> {
    for entry in fs::read_dir(current_path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            let relative_path = path.strip_prefix(base_path).unwrap().to_string_lossy();
            combined_content.push_str(&format!("File: {}\n", relative_path));

            let mut file_content = String::new();
            let mut file = fs::File::open(&path)?;

            if let Err(_) = file.read_to_string(&mut file_content) {
                println!("Skipping non UTF-8 file: {}", relative_path);
                return Ok(());
            }

            combined_content.push_str(&file_content);
            combined_content.push('\n');
        } else if path.is_dir() {
            read_directory_contents_recursive(base_path, &path, combined_content)?;
        }
    }

    Ok(())
}
