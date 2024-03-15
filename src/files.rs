use walkdir::WalkDir;
use std::io;

pub fn files_hello()
{
    println!("Hello, From files.rs")
}

pub fn print_files(folder_path: &str) -> io::Result<()>
{
    for file_entry in WalkDir::new(folder_path)
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
}
