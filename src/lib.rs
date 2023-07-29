pub mod error;
pub mod header;
pub mod elements;
pub mod documents;



#[cfg(test)]
mod tests {

    use crate::documents::{DriversLicense, Reader};

    #[test]
    fn test_asni() {
        let file_name = "./img.jpeg";

        let results = rxing::helpers::detect_multiple_in_file(file_name).expect("decodes");
        
        for result in results {
            let (header, dl) = DriversLicense::new(result.getText().to_string()).unwrap();
            println!("{:#?}", header);
        }
    }
}