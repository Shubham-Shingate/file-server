
use std::fs;
use std::io;

pub fn make_dir(dir_name: &str) -> io::Result<()> {
    fs::create_dir_all(dir_name)?;
    println!("NewDir directory created successfully\n");
    Ok(())
}