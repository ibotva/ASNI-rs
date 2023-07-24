use crate::{error::Error, header::Header};

#[derive(Debug)]
pub struct ASNI {
    // This provides details about the Ansi file
    pub header: Header,
    // ElementID: DCA, the allowed types of vehicle a license holder can drive.
    pub jurisdiction_vehicle_class: Option<String>,
    // ElementID: DCP, description for DCA (jurisdiction vehicle class)
    pub jurisdiction_vehicle_class_desc: Option<String>,
    // ElementID: DCB, the restrictions to a license.
    pub jurisdiction_restriction_codes: Option<String>,
    // ElementID: DCR, description for jurisdiction restriction codes
    pub jurisdiction_restriction_codes_desc: Option<String>,
    // ElementID: DCD, the additional priviledges granted to a license holder.
    pub jurisdiction_endorsement_codes: Option<String>,
    // ElementID: DCQ, description for DCD
    pub jurisdiction_endorsement_codes_desc: Option<String>,
    // ElementID: DBA, the date that the license expires
    pub document_expiration_date: Option<String>,
    // ElementID: DCS, the license holders Last Name
    pub last_name: Option<String>,
    // ElementID: DAC, the license holders first name
    pub first_name: Option<String>,
    // ElementID: DAD, the license holders middle name, separated with commas if there is multiple
    pub middle_name: Option<String>,
    // ElementID: DBD, the license issue date.
    pub license_issued_date: Option<String>,
    // ElementID: DBB, the license holders date of birth
    pub date_of_birth: Option<String>,
    // ElementID: DBC, the license holders gender, 1 means male, 2 means female, 9 means unspecified.
    pub gender: Option<String>,
    // ElementID: DAY, the license holders eye color. (ANSI D-20 codes)
    pub eye_color: Option<String>,
    // ElementID: DAU, the license holders height.  (3 digits, a space, and then a measurement like inches or cenimeters)
    pub height: Option<String>,
    // ElementID: DAG, the license holders street of residence.
    pub address_street: Option<String>,
    // ElementID: DAH, a second field for license holders street of residence.
    pub address_street_2: Option<String>,
    // ElementID: DAI, the license holders city of residence.
    pub address_city: Option<String>,
    // ElementID: DAJ, the license holders Jurisdiction Code.
    pub address_state: Option<String>,
    // ElementID: DAK, zip code
    pub address_zip: Option<String>,
    // ElementID: DAQ, customer id number
    pub customer_id_number: Option<String>,
    // ElementID: DCF, document discriminator
    pub document_descriminator: Option<String>,
    // ElementID: DCG, country where license was issued.
    pub issuing_country: Option<String>,

    // These next few all have the same value of "T", "N", or "U" to represent whether its trunctuated or not, or unknown, respectively.
    // ElementID: DDE, shows whether family name is trunctuated
    pub family_name_trunctuated: Option<String>,
    // ElementID: DDF, shows whether first name is trunctuated
    pub first_name_trunctuated: Option<String>,
    // ElementID: DDG, shows whether middle name is trunctuated
    pub middle_name_trunctuated: Option<String>,

    // ElementID: DAZ, shows the hair color of license holder
    pub hair_color: Option<String>,
    // ElementID: DCI, shows the license holders place of birth
    pub birth_location: Option<String>,
    // ElementID: DCJ, audit information verifying card.
    pub audit_information: Option<String>,
    // ElementID: DCK, inventory control number
    pub inventory_control_number: Option<String>,
    // ElementID: DBN, alias family name of license holder
    pub family_name: Option<String>,
    // ElementID: DBG, given name of license holder
    pub given_name: Option<String>,
    // ElementID: DBS, other suffix/alias the license holder is known by.
    pub suffix_name: Option<String>,
    // ElementID: DCU, suffix of the name
    pub name_suffix: Option<String>,
    // ElementID: DCL, race or ethnicity of license holder
    pub race: Option<String>,

    // The following are placeholders for future efforts to standardize from AAMVA
    // ElementID: DCM
    pub standard_vehicle_classification: Option<String>,
    // ElementID: DCN, standard enforcement code
    pub standard_endorsement_code: Option<String>,
    // ElementID: DCO, standard restriction code
    pub standard_restriction_code: Option<String>,
    
    // ElementID: DDA, indicates compliance, F = compliant, N = non-compliant
    pub compliance_type: Option<String>,
    // ElementID: DDB, card revision date
    pub card_revision_date: Option<String>,
    // ElementID: DDC, Hazmat endorsement expiration date
    pub hazmat_endorsement_expiration: Option<String>,
    // ElementID: DDD, indicates whether a document holder has temporary lawful status, 1 if true.
    pub temporary_lawful_status: Option<String>,
    // ElementID: DAW, the weight in pounds of the license holder
    pub weight_in_pounds: Option<String>,
    // ElementID DAX, the weight in kilograms of the license holder
    pub weight_in_kilograms: Option<String>,
    // ElementID DDH, date when license holder will be 18 years or older
    pub under_18_until: Option<String>,
    // ElementID DDI, date when license holder will be 19 years or older
    pub under_19_until: Option<String>,
    // ElementID DDJ, date when license holder will be 21 years or older
    pub under_21_until: Option<String>,
    // ElementID DDK, indicates whether the license holder is an organ donor
    pub organ_donor: Option<String>,
    // ElementID DDL, indicates whether the license holder is a veteran
    pub veteran: Option<String>,
    

}

impl ASNI {


    pub fn from(file: String) -> Result<ASNI, Error> {
        let mut lines: Vec<String> = file.split("\n").map(|f| f.to_string()).collect();

        if &lines[0] != "@" {
            return Err(Error::NonCompliance)
        }

        lines.remove(0);

        let headline = &lines[0];

        let header = Header::from(headline.to_string())?;
        lines.remove(0);

        let mut jurisdiction_vehicle_class: Option<String> = None;
        let mut jurisdiction_vehicle_class_desc: Option<String> = None;
        let mut jurisdiction_restriction_codes: Option<String> = None;
        let mut jurisdiction_restriction_codes_desc: Option<String> = None;
        let mut jurisdiction_endorsement_codes: Option<String> = None;
        let mut jurisdiction_endorsement_codes_desc: Option<String> = None;
        let mut document_expiration_date: Option<String> = None;
        let mut last_name: Option<String> = None;
        let mut first_name: Option<String> = None;
        let mut middle_name: Option<String> = None;
        let mut license_issued_date: Option<String> = None;
        let mut date_of_birth: Option<String> = None;
        let mut gender: Option<String> = None;
        let mut eye_color: Option<String> = None;
        let mut height: Option<String> = None;
        let mut address_street: Option<String> = None;
        let mut address_street_2: Option<String> = None;
        let mut address_city: Option<String> = None;
        let mut address_state: Option<String> = None;
        let mut address_zip: Option<String> = None;
        let mut customer_id_number: Option<String> = None;
        let mut document_descriminator: Option<String> = None;
        let mut issuing_country: Option<String> = None;
        let mut family_name_trunctuated: Option<String> = None;
        let mut first_name_trunctuated: Option<String> = None;
        let mut middle_name_trunctuated: Option<String> = None;
        let mut hair_color: Option<String> = None;
        let mut birth_location: Option<String> = None;
        let mut audit_information: Option<String> = None;
        let mut inventory_control_number: Option<String> = None;
        let mut family_name: Option<String> = None;
        let mut given_name: Option<String> = None;
        let mut suffix_name: Option<String> = None;
        let mut name_suffix: Option<String> = None;
        let mut race: Option<String> = None;
        let mut standard_vehicle_classification: Option<String> = None;
        let mut standard_endorsement_code: Option<String> = None;
        let mut standard_restriction_code: Option<String> = None;
        let mut compliance_type: Option<String> = None;
        let mut card_revision_date: Option<String> = None;
        let mut hazmat_endorsement_expiration: Option<String> = None;
        let mut temporary_lawful_status: Option<String> = None;
        let mut weight_in_pounds: Option<String> = None;
        let mut weight_in_kilograms: Option<String> = None;
        let mut under_18_until: Option<String> = None;
        let mut under_19_until: Option<String> = None;
        let mut under_21_until: Option<String> = None;
        let mut organ_donor: Option<String> = None;
        let mut veteran: Option<String> = None;
        

        for line in lines.to_owned() {


            let (prefix, value) = line.split_at(3);

         
            if prefix == "DCA" {
                jurisdiction_vehicle_class = Some(value.to_string())
            } else if prefix == "DCP" {
                jurisdiction_vehicle_class_desc = Some(value.to_string())
            } else if prefix == "DCB" {
                jurisdiction_restriction_codes = Some(value.to_string())
            } else if prefix == "DCR" {
                jurisdiction_restriction_codes_desc = Some(value.to_string())
            } else if prefix == "DCD" {
                jurisdiction_endorsement_codes = Some(value.to_string())
            } else if prefix == "DCQ" {
                jurisdiction_endorsement_codes_desc = Some(value.to_string())
            } else if prefix == "DBA" {
                document_expiration_date = Some(value.to_string())
            } else if prefix == "DCS" {
                last_name = Some(value.to_string())
            } else if prefix == "DAC" {
                first_name = Some(value.to_string())
            } else if prefix == "DAD" {
                middle_name = Some(value.to_string())
            } else if prefix == "DBD" {
                license_issued_date = Some(value.to_string())
            } else if prefix == "DBB" {
                date_of_birth = Some(value.to_string())
            } else if prefix == "DBC" {
                gender = Some(value.to_string())
            } else if prefix == "DAY" {
                eye_color = Some(value.to_string())
            } else if prefix == "DAU" {
                height = Some(value.to_string())
            } else if prefix == "DAG" {
                address_street = Some(value.to_string())
            } else if prefix == "DAH" {
                address_street_2 = Some(value.to_string())
            } else if prefix == "DAI" {
                address_city = Some(value.to_string())
            } else if prefix == "DAJ" {
                address_state = Some(value.to_string())
            } else if prefix == "DAK" {
                address_zip = Some(value.to_string())
            } else if prefix == "DAQ" {
                customer_id_number = Some(value.to_string())
            } else if prefix == "DCF" {
                document_descriminator = Some(value.to_string())
            } else if prefix == "DCG" {
                issuing_country = Some(value.to_string())
            } else if prefix == "DDE" {
                family_name_trunctuated = Some(value.to_string())
            } else if prefix == "DDF" {
                first_name_trunctuated = Some(value.to_string())
            } else if prefix == "DDG" {
                middle_name_trunctuated = Some(value.to_string())
            } else if prefix == "DAZ" {
                hair_color = Some(value.to_string())
            } else if prefix == "DCI" {
                birth_location = Some(value.to_string())
            } else if prefix == "DCJ" {
                audit_information = Some(value.to_string())
            } else if prefix == "DCK" {
                inventory_control_number = Some(value.to_string())
            } else if prefix == "DBN" {
                family_name = Some(value.to_string())
            } else if prefix == "DBG" {
                given_name = Some(value.to_string())
            } else if prefix == "DBS" {
                suffix_name = Some(value.to_string())
            } else if prefix == "DCU" {
                name_suffix = Some(value.to_string())
            } else if prefix == "DCL" {
                race = Some(value.to_string())
            } else if prefix == "DCM" {
                standard_vehicle_classification = Some(value.to_string())
            } else if prefix == "DCN" {
                standard_endorsement_code = Some(value.to_string())
            } else if prefix == "DCO" {
                standard_restriction_code = Some(value.to_string())
            } else if prefix == "DDA" {
                compliance_type = Some(value.to_string())
            } else if prefix == "DDB" {
                card_revision_date = Some(value.to_string())
            } else if prefix == "DDC" {
                hazmat_endorsement_expiration = Some(value.to_string())
            } else if prefix == "DDD" {
                temporary_lawful_status = Some(value.to_string())
            } else if prefix == "DAW" {
                weight_in_pounds = Some(value.to_string())
            } else if prefix == "DAX" {
                weight_in_kilograms = Some(value.to_string())
            } else if prefix == "DDH"{
                under_18_until = Some(value.to_string())
            } else if prefix == "DDI" {
                under_19_until = Some(value.to_string())
            } else if prefix == "DDJ" {
                under_21_until = Some(value.to_string())
            } else if prefix == "DDK" {
                organ_donor = Some(value.to_string())
            } else if prefix == "DDL" {
                veteran = Some(value.to_string())
            } else {
                println!("Warning no valid checker for Prefix: {}\nValue: {}", prefix, value)
            }

        }


        Ok(ASNI { header, jurisdiction_vehicle_class, jurisdiction_vehicle_class_desc, jurisdiction_restriction_codes, jurisdiction_restriction_codes_desc, jurisdiction_endorsement_codes, jurisdiction_endorsement_codes_desc, document_expiration_date, last_name, first_name, middle_name, license_issued_date, date_of_birth, gender, eye_color, height, address_street, address_street_2, address_city, address_state, address_zip, customer_id_number, document_descriminator, issuing_country, family_name_trunctuated, first_name_trunctuated, middle_name_trunctuated, hair_color, birth_location, audit_information, inventory_control_number, family_name, given_name, suffix_name, name_suffix, race, standard_vehicle_classification, standard_endorsement_code, standard_restriction_code, compliance_type, card_revision_date, hazmat_endorsement_expiration, temporary_lawful_status, weight_in_pounds, weight_in_kilograms, under_18_until, under_19_until, under_21_until, organ_donor, veteran })
    }
}