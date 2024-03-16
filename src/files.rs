use std::io;
use std::fs;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use rayon::prelude::*;

use walkdir::WalkDir;
use lopdf::Document;


pub fn print_files(folders_path: &str) -> io::Result<()>
{
    let entries = WalkDir::new(folders_path)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_file());

    let _ = entries
        .par_bridge()
        .for_each(|entry|
            {
                println!("{}", entry.path().display());
            });

    Ok(())
}

pub fn read_txt_files(folders_path: &str) -> io::Result<HashMap<String, String>>
{
    let entries = WalkDir::new(folders_path)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_file() &&
            entry.path().extension().unwrap_or_default() == "txt");


    let result = entries
        .par_bridge()
        .map(|entry| -> (String, String)
            {
                let path = entry.path();
                let content= fs::read_to_string(path).unwrap();

                let string_path = String::from(path.to_str().to_owned().unwrap());

                return (string_path, content)
            })
        .collect();

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
    let _ = read_txt_files(folders_path)?
        .iter().for_each(|(path, content)|
        {
            write_txt_file(path.as_str(), function(content.as_str()).unwrap()).unwrap();
        });

    Ok(())
}

pub fn read_pdf_files(folders_path: &str) -> io::Result<HashMap<String, String>>
{
    let entries = WalkDir::new(folders_path)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_file() &&
            entry.path().extension().unwrap_or_default() == "pdf");

    let result = entries
        .par_bridge()
        .map(|entry| -> (String, String)
            {
                let file_path = entry.path();
                let document = Document::load(file_path).unwrap();

                let content = document
                    .get_pages()
                    .iter()
                    .map(|(&number, _)|
                        {
                            let number = number;
                            return document.extract_text(&[number]).unwrap_or_default();
                        }).collect();

                let file_path_string = String::from(file_path.to_str().unwrap());
                return (file_path_string, content);
            })
        .collect();

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
        let _ = read_pdf_files(folders_path)?
            .iter()
            .for_each(|(path, _)|
                {
                    println!("{}", path);
                });

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

                Ok("Test:

Shall I compare thee to a summer's day?
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
