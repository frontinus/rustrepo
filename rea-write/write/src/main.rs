use std::fs::File;
use std::io::Write;

fn write_file(file_path: &str, contents: &str) -> std::io::Result<()> {
    let mut file = File::create(file_path)?;
    file.write_all(contents.as_bytes())?;
    Ok(())
}

fn main() {
    
    write_file("file.txt","mi cago addosso");
}
