use std::io;
use std::fs;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

use walkdir::WalkDir;
use lopdf::Document;


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
        if file_path.extension().unwrap_or_default() != "txt"
        {
            continue;
        }

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

pub fn apply_to_files_content(folders_path: &str, function: fn(file_content: &str) -> io::Result<&str>) -> io::Result<()>
{
    let files_content = read_txt_files(folders_path)?;

    for (file_path, file_content) in files_content
    {
        let _ = write_txt_file(file_path.as_str(), function(file_content.as_str())?);
    }

    Ok(())
}

pub fn read_pdf_files(folders_path: &str) -> io::Result<HashMap<String, String>>
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
        if file_path.extension().unwrap_or_default() == "pdf"
        {
            match Document::load(file_path)
            {
                Ok(document) =>
                    {
                        let mut content = String::new();

                        for (page_number, _) in document.get_pages()
                        {
                            let page_number = page_number as u32;
                            let extracted = document.extract_text(&[page_number]).unwrap_or_default();

                            content.push_str(extracted.as_str());
                        }

                        let file_path_string = String::from(file_path.to_str().unwrap());
                        result.insert(file_path_string, content);
                    }
                Err(error) =>
                    {
                        println!("[ERROR] {}", error);

                        continue;
                    }
            }
        }
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

        let result = read_txt_files(folders_path)?;

        for (key, value) in result
        {
            println!("{}: {}", key, value);
        }

        Ok(())
    }

    #[test]
    fn test_reading_pdf() -> io::Result<()>
    {
        let folders_path: &str = "Assets\\TestFiles";

        let result = read_pdf_files(folders_path)?;

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

    #[test]
    fn test_apply_to_files_content()
    {
        let function = |file_content: &str| -> io::Result<&str>
            {
                _ = file_content;

                Ok("Shall I compare thee to a summer's day?
Thou art more lovely and more temperate:
Rough winds do shake the darling buds of May,
And summer's lease hath all too short a date:

Sometime too hot the eye of heaven shines,
And often is his gold complexion dimmed;
And every fair from fair sometime declines,
By chance or nature's changing course untrimmed;

But thy eternal summer shall not fade
Nor lose possession of that fair thou owest;
Nor shall Death brag thou wanderest in his shade,
When in eternal lines to time thou growest:

So long as men can breathe or eyes can see,
So long lives this, and this gives life to thee.")
            };

        let folder_path = "Assets\\TestFiles";

        let _ = apply_to_files_content(folder_path, function);
    }
}
