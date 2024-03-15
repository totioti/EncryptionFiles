pub fn encryption_hello()
{
    println!("Hello, From encryption.rs")
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test()
    {
        encryption_hello()
    }
}