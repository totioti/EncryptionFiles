use std::io;
use std::fs;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

use walkdir::WalkDir;


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

pub fn read_txt_files(folders_path: &str) -> io::Result<HashMap<String, String>>
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

pub fn write_txt_file(file_path: &str, file_content: &str) -> io::Result<()>
{
    let mut file = File::create(file_path)?;
    file.write_all(file_content.as_bytes())?;

    Ok(())
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

        let result = read_txt_files(folders_path)?;

        for (key, value) in result
        {
            println!("{}: {}", key, value);
        }

        Ok(())
    }

    #[test]
    fn test_write_txt() -> io::Result<()>
    {
        let file_path: &str = "Assets\\TestFiles\\test_writing.txt";
        let content: &str = "Some Text for Writing.";

        let _ = write_txt_file(file_path, content);

        Ok(())
    }
}
