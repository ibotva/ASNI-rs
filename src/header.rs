use std::fmt;
use crate::error::Error;

#[derive(Debug)]
pub struct Header {
    file_type: String,
    issuer_identification_number: String,
    aamva_version_number: String,
    jurisdiction_version_number: String,
    number_of_entries: String,
    // A subfile type of DL means that its a drivers license, ID means state ID, and EN means enhanced drivers license
    subfile_type: String,
    offset: String,
    length: String,
    extra_characters: String
}

impl Header {
    
    pub fn from(header: String) -> Result<Header, Error>{
        let mut characters = header;

        if let Some('\x1e') = characters.chars().next() {
            characters.remove(0);
        } else {
            return Err(Error::Formmating { violation: "Record separator not found".to_string() });
        };

        if let Some('\x0d') = characters.chars().next() {
            characters.remove(0);
        } else {
            return Err(Error::Formmating { violation: "Segment terminator not found".to_string() });
        };

        let file_type: String = characters.drain(00..05).collect();

        if file_type != "ANSI ".to_string() {
            return Err(Error::InvalidFileType);
        }

        let issuer_identification_number: String = characters.drain(00..06).collect();

        // Todo: Ensure the below three values are between 0 and 99 or return err result
        let aamva_version_number: String = characters.drain(00..02).collect();
        let jurisdiction_version_number: String = characters.drain(00..02).collect();
        let number_of_entries: String = characters.drain(00..02).collect();
        let subfile_type: String = characters.drain(00..02).collect();
        let offset: String = characters.drain(00..04).collect();
        let length: String = characters.drain(00..04).collect();


        Ok(Header { 
            extra_characters: characters,
            file_type, 
            issuer_identification_number, 
            aamva_version_number, 
            jurisdiction_version_number, 
            number_of_entries, 
            subfile_type, 
            offset, 
            length
        })
    }    
}

impl fmt::Display for Header {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Header {{
    File Type:               '{}'
    Issuer Identification #: '{}'
    AAMVA Version #:         '{}'
    Jurisdiction Version #:  '{}'
    Number of Entries:       '{}'
    Subfile Type:            '{}'
    Offset:                  '{}'
    Length:                  '{}'
    Extra Characters         '{}'
}}", &self.file_type, &self.issuer_identification_number, &self.aamva_version_number, &self.jurisdiction_version_number, &self.number_of_entries, &self.subfile_type, &self.offset, &self.length, &self.extra_characters)
    }
}