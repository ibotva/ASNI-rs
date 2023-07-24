use chrono::{DateTime, Utc};

use crate::{error::Error, header::Header};


pub struct ASNI {
    header: Header
}

impl ASNI {


    pub fn from(file: String) -> Result<ASNI, Error> {
        let mut line_index = 0;
        let mut lines: Vec<String> = file.split("\n").map(|f| f.to_string()).collect();

        if &lines[0] != "@" {
            return Err(Error::NonCompliance)
        }

        lines.remove(0);

        let headline = &lines[0];

        let header = Header::from(headline.to_string())?;
        lines.remove(0);


        for line in lines.to_owned() {

            let mut jurisdiction_restriction_codes = None;
            let mut jurisdiction_endorsement_codes: Option<&str> = None;
            let mut id_expiration_date: Option<DateTime<Utc>> = None;

            // DCB Is juristiction specific restriction codes.
            if line.starts_with("DCB") {
                jurisdiction_restriction_codes = Some(line.split_at(3).1);
                
            } 
            // DCD Is jurisdiction endorsement codes
            else if line.starts_with("DCD") {
                jurisdiction_endorsement_codes = Some(line.split_at(3).1);
            }
            // DBA is ID expiration date, MMDDYYYY for usa and  and YYYYMMDD for canada
            else if line.starts_with("DBA") {
                
            }


            
            println!("{}", line);
            

            line_index = line_index + 1;
        };



        Ok(ASNI { header })
    }
}