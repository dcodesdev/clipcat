use std::fs;
use std::io::Read;

pub fn read_directory_contents(dir: &str) -> anyhow::Result<String> {
    let mut combined_content = String::new();

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            let file_name = path.file_name().unwrap().to_string_lossy();
            combined_content.push_str(&format!("File: {}\n", file_name));

            let mut file_content = String::new();
            let mut file = fs::File::open(&path)?;
            file.read_to_string(&mut file_content)?;

            combined_content.push_str(&file_content);
            combined_content.push('\n');
        }
    }

    Ok(combined_content)
}
