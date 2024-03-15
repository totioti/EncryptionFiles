use std::io;
use std::fs;
use std::collections::HashMap;

use walkdir::WalkDir;


pub fn files_hello()
{
    println!("Hello, From files.rs")
}

pub fn print_files(folders_path: &str) -> io::Result<()>
{
    for file_entry in WalkDir::new(folders_path)
    {
        let entry = match file_entry {
            Ok(entry) => { entry }
            Err(error) =>
                {
                    println!("[ERROR] {}", error);

                    continue;
                }
        };

        println!("{}", entry.path().display());
    }

    Ok(())
}

pub fn read_files(folders_path: &str) -> io::Result<HashMap<String, String>>
{
    let mut result: HashMap<String, String> = HashMap::new();

    for file_entry in WalkDir::new(folders_path)
    {
        let entry = match file_entry {
            Ok(entry) => { entry }
            Err(error) =>
                {
                    println!("[ERROR] {}", error);

                    continue;
                }
        };

        let file_path = entry.path();
        let file_content= match fs::read_to_string(file_path)
        {
            Ok(content) => { content }
            Err(error) =>
                {
                    println!("[ERROR] {}", error);

                    continue;
                }
        };

        let file_path_string = String::from(file_path.to_str().unwrap());
        result.insert(file_path_string, file_content);
    }

    Ok(result)
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_print_files()
    {
        let folders_path: &str = "Assets\\TestFiles";

        let _ = print_files(folders_path);
    }

    #[test]
    fn test_reading() -> io::Result<()>
    {
        let folders_path: &str = "Assets\\TestFiles";

        let result = read_files(folders_path)?;

        for (key, value) in result
        {
            println!("{}: {}", key, value);
        }

        Ok(())
    }
}
