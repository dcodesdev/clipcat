use std::fs;
use std::io::Read;
use std::path::Path;

pub fn read_directory_contents(dir: &str) -> anyhow::Result<String> {
    let mut combined_content = String::new();
    let base_path = Path::new(dir);
    read_directory_contents_recursive(base_path, base_path, &mut combined_content)?;
    Ok(combined_content)
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
            file.read_to_string(&mut file_content)?;

            combined_content.push_str(&file_content);
            combined_content.push('\n');
        } else if path.is_dir() {
            read_directory_contents_recursive(base_path, &path, combined_content)?;
        }
    }
    Ok(())
}
