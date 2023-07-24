pub mod asni;
pub mod error;
pub mod header;

#[cfg(test)]
mod tests {
    use crate::asni::ASNI;

    #[test]
    fn test_asni() {
        let file_name = "./img.jpeg";

        let results = rxing::helpers::detect_multiple_in_file(file_name).expect("decodes");
    
        for result in results {
            let reader = ASNI::from(result.getText().to_string()).unwrap();
            println!("{}", reader.header);
            println!("{:?}", reader);
        }
    }
}