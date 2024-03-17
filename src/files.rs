use std::io;
use std::fs;
use std::collections::HashMap;
use std::fs::{File};
use std::io::Write;
use std::path::PathBuf;

use walkdir::WalkDir;
use lopdf::Document;
use rayon::prelude::*;

#[derive(Debug)]
struct Files
{
    paths: Vec<PathBuf>,
    files: HashMap<PathBuf, String>,
}

impl Files
{
    fn new(root: &str) -> Files
    {
        let paths: Vec<PathBuf> = WalkDir::new(root)
            .into_iter()
            .par_bridge()
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.file_type().is_file())
            .map(|entry| entry.path().to_owned())
            .collect();

        let files: HashMap<PathBuf, String> = HashMap::new();

        return Files { paths, files };
    }

    pub fn print_files(&self) -> io::Result<()>
    {
        let _ = self.paths
            .iter()
            .par_bridge()
            .for_each(|entry| println!("{}", entry.display()));

        Ok(())
    }

    pub fn read_txt_files(&mut self) -> io::Result<()>
    {
        let result: HashMap<PathBuf, String> = self.paths
            .iter()
            .filter(|path| path.extension().unwrap_or_default() == "txt")
            .par_bridge()
            .map(|path| -> (PathBuf, String)
                {
                    let content= fs::read_to_string(path).unwrap();

                    return (path.to_owned(), content);
                }).collect();

        self.files.extend(result);

        Ok(())
    }

    pub fn write_txt_file(&mut self, file_path: &str, file_content: &str) -> io::Result<()>
    {
        let mut file = File::create(file_path)?;
        file.write_all(file_content.as_bytes())?;

        self.paths.push(PathBuf::from(file_path));

        Ok(())
    }

    pub fn read_pdf_files(&mut self) -> io::Result<()>
    {
        let result: HashMap<PathBuf, String> = self.paths
            .iter()
            .filter(|path| path.extension().unwrap_or_default() == "pdf")
            .par_bridge()
            .map(|path| -> (PathBuf, String)
                {
                    let document = Document::load(path).unwrap();

                    let content = document
                        .get_pages()
                        .iter()
                        .map(|(&number, _)|
                            {
                                let number = number;
                                return document.extract_text(&[number]).unwrap_or_default();
                            }).collect();

                    return(path.to_owned(), content);
                }).collect();

        self.files.extend(result);

        Ok(())
    }
}


#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_print() -> io::Result<()>
    {
        let root = "Assets\\TestFiles";
        let files = Files::new(root);

        let _ = files.print_files()?;

        Ok(())
    }
    #[test]
    fn test_write_txt() -> io::Result<()>
    {
        let root = "Assets\\TestFiles\\";
        let path = "Assets\\TestFiles\\test_writing.txt";
        let content= fs::read_to_string("Assets\\poetry.txt").unwrap();

        let mut files = Files::new(root);

        let _ = files.write_txt_file(path, content.as_str())?;

        Ok(())
    }
    #[test]
    fn read_txt_files() -> io::Result<()>
    {
        let root = "Assets\\TestFiles";
        let mut files = Files::new(root);

        let _ = files.read_txt_files()?;

        for (path, content) in files.files
        {
            println!("{}: {}", path.display(), content);
        }

        Ok(())
    }
    #[test]
    fn read_pdf_files() -> io::Result<()>
    {
        let root = "Assets\\TestFiles";
        let mut files = Files::new(root);

        let _ = files.read_pdf_files()?;

        for (path, content) in files.files
        {
            println!("{}: {}", path.display(), content);
        }

        Ok(())
    }
}
