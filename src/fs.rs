use std::fs;
use std::io::Read;
use std::path::Path;

pub fn read_contents(path: &Path) -> anyhow::Result<String> {
    let mut combined_content = String::new();

    if path.is_file() {
        read_file(path, &mut combined_content)?;
    } else {
        read_directory_contents_recursive(path, path, &mut combined_content)?;
    }

    Ok(combined_content)
}

pub fn read_file(path: &Path, combined_content: &mut String) -> anyhow::Result<()> {
    combined_content.push_str(&format!("File: {}\n", path.display()));

    let mut file_content = String::new();
    let mut file = fs::File::open(&path)?;

    if let Err(_) = file.read_to_string(&mut file_content) {
        return Ok(());
    }

    combined_content.push_str(&file_content);
    combined_content.push('\n');

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
            read_file(&path, combined_content)?;
        } else if path.is_dir() {
            read_directory_contents_recursive(base_path, &path, combined_content)?;
        }
    }

    Ok(())
}
