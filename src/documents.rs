use crate::elements::{JurisdictionSpecificVehicleClass, Element, JurisdictionSpecificRestrictionCodes, JurisdictionSpecificEndorsementCodes, DocumentExpirationDate, CustomerFirstName, CustomerFamilyName, DocumentIssueDate, CustomerMiddleNames, DateOfBirth, PhysicalDescriptionSex, PhysicalDescriptionEyeColor, PhysicalDescriptionHeight, AddressStreet1, AddressCity, AddressJurisdictionCode, AddressPostalCode, CustomerIdNumber, DocumentDescriminator, CountryIdentification, FamilyNameTruncation, FirstNameTruncation, MiddleNameTruncation, AddressStreet2, HairColor, PlaceOfBirth, AuditInformation, InventoryControlNumber, AliasFamilyName, AliasGivenName, AliasSuffixName, NameSuffix, PhysicalDescriptionWeightRange, RaceEthnicity, StandardVehicleClassification, StandardEndorsementCode, StandardRestrictionCode, JurisdictionSpecificVehicleClassificationDescription, JurisdictionSpecificEndorsementCodeDescription, JurisdictionSpecificRestrictionCodeDescription, DHSComplianceType, DHSCardRevisionDate, HAZMATEndorcementExpirationDate, LimitedDurationDocumentIndicator, WeightPounds, WeightKilograms, Under18Until, Under19Until, Under21Until, OrganDonorIndicator, VeteranIndicator, by_id};
use crate::header::Header;
use crate::error::Error;

pub trait Reader {

    fn get_header_and_lines(file: String) -> Result<(Header, Vec<String>), Error> {
        let mut lines: Vec<String> = file.split("\n").map(|f| f.to_string()).collect();

        if &lines[0] != "@" {
            return Err(Error::NonCompliance)
        }

        lines.remove(0);

        let headline = &lines[0];

        let header = Header::from(headline.to_string())?;
        
        lines.remove(0);

        Ok((
            header,
            lines
        ))
    }

    fn new(file: String) -> Result<(Header, Self), Error> where Self: Sized;
}

#[derive(Debug)]
pub struct DriversLicense {
    pub jurisdiction_specific_vehicle_class:                    Option<String>,
    pub jurisdiction_specific_restriction_codes:                Option<String>,
    pub jurisdiction_specific_endorsement_codes:                Option<String>,
    pub document_expiration_date:                               Option<String>,
    pub customer_family_name:                                   Option<String>,
    pub customer_first_name:                                    Option<String>,
    pub document_issue_date:                                    Option<String>,
    pub customer_middle_names:                                  Option<String>,
    pub date_of_birth:                                          Option<String>,
    pub physical_description_sex:                               Option<String>,
    pub physical_description_eye_color:                         Option<String>,
    pub physical_description_height:                            Option<String>,
    pub address_street_1:                                       Option<String>,
    pub address_city:                                           Option<String>,
    pub address_jurisdiction_code:                              Option<String>,
    pub address_postal_code:                                    Option<String>,
    pub customer_id_number:                                     Option<String>,
    pub document_descriminator:                                 Option<String>,
    pub country_identification:                                 Option<String>,
    pub family_name_truncation:                                 Option<String>,
    pub first_name_truncation:                                  Option<String>,
    pub middle_name_truncation:                                 Option<String>,
    pub address_street_2:                                       Option<String>,
    pub hair_color:                                             Option<String>,
    pub place_of_birth:                                         Option<String>,
    pub audit_information:                                      Option<String>,
    pub inventory_control_number:                               Option<String>,
    pub alias_family_name:                                      Option<String>,
    pub alias_given_name:                                       Option<String>,
    pub alias_suffix_name:                                      Option<String>,
    pub name_suffix:                                            Option<String>,
    pub physical_description_weight_range:                      Option<String>,
    pub race_ethnicity:                                         Option<String>,
    pub standard_vehicle_classification:                        Option<String>,
    pub standard_endorsement_code:                              Option<String>,
    pub standard_restriction_code:                              Option<String>,
    pub jurisdiction_vehicle_classification_description:        Option<String>,
    pub jurisdiction_specific_edorsement_code_description:      Option<String>,
    pub jurisdiction_specific_restriction_code_description:     Option<String>,
    pub compliance_type:                                        Option<String>,
    pub card_revision_date:                                     Option<String>,
    pub hazmat_endorsement_expiration:                          Option<String>,
    pub limited_duration_document_indicator:                    Option<String>,
    pub weight_pounds:                                          Option<String>,
    pub weight_kilograms:                                       Option<String>,
    pub under_18_until:                                         Option<String>,
    pub under_19_until:                                         Option<String>,
    pub under_21_until:                                         Option<String>,
    pub organ_donor_indicator:                                  Option<String>,
    pub veteran_indicator:                                      Option<String>,    
}



impl Reader for DriversLicense {
    fn new(file: String) -> Result<(Header, DriversLicense), Error> {

        let (header, lines): (Header, Vec<String>) = Self::get_header_and_lines(file)?;

        for line in &lines {
            let (prefix, value): (&str, &str) = line.split_at(3);
            match by_id(line.split_at(3).0.to_string()) {
                Err(_err) => {
                    println!("Warning invalid Element ID: '{prefix}' WITH VALUE: '{value}' please submit an issue to have the Element added.")
                },
                Ok(_) => {}
            } 
        }

        Ok((header, DriversLicense {
            jurisdiction_specific_vehicle_class:                JurisdictionSpecificVehicleClass::no_error_find_in_lines(&lines),
            jurisdiction_specific_restriction_codes:            JurisdictionSpecificRestrictionCodes::no_error_find_in_lines(&lines),
            jurisdiction_specific_endorsement_codes:            JurisdictionSpecificEndorsementCodes::no_error_find_in_lines(&lines),
            document_expiration_date:                           DocumentExpirationDate::no_error_find_in_lines(&lines),
            customer_family_name:                               CustomerFamilyName::no_error_find_in_lines(&lines),
            customer_first_name:                                CustomerFirstName::no_error_find_in_lines(&lines),
            document_issue_date:                                DocumentIssueDate::no_error_find_in_lines(&lines),
            customer_middle_names:                              CustomerMiddleNames::no_error_find_in_lines(&lines),
            date_of_birth:                                      DateOfBirth::no_error_find_in_lines(&lines),
            physical_description_sex:                           PhysicalDescriptionSex::no_error_find_in_lines(&lines),
            physical_description_eye_color:                     PhysicalDescriptionEyeColor::no_error_find_in_lines(&lines),
            physical_description_height:                        PhysicalDescriptionHeight::no_error_find_in_lines(&lines),
            address_street_1:                                   AddressStreet1::no_error_find_in_lines(&lines),
            address_city:                                       AddressCity::no_error_find_in_lines(&lines),
            address_jurisdiction_code:                          AddressJurisdictionCode::no_error_find_in_lines(&lines),
            address_postal_code:                                AddressPostalCode::no_error_find_in_lines(&lines),
            customer_id_number:                                 CustomerIdNumber::no_error_find_in_lines(&lines),
            document_descriminator:                             DocumentDescriminator::no_error_find_in_lines(&lines),
            country_identification:                             CountryIdentification::no_error_find_in_lines(&lines),
            family_name_truncation:                             FamilyNameTruncation::no_error_find_in_lines(&lines),
            first_name_truncation:                              FirstNameTruncation::no_error_find_in_lines(&lines),
            middle_name_truncation:                             MiddleNameTruncation::no_error_find_in_lines(&lines),
            address_street_2:                                   AddressStreet2::no_error_find_in_lines(&lines),
            hair_color:                                         HairColor::no_error_find_in_lines(&lines),
            place_of_birth:                                     PlaceOfBirth::no_error_find_in_lines(&lines),
            audit_information:                                  AuditInformation::no_error_find_in_lines(&lines),
            inventory_control_number:                           InventoryControlNumber::no_error_find_in_lines(&lines),
            alias_family_name:                                  AliasFamilyName::no_error_find_in_lines(&lines),
            alias_given_name:                                   AliasGivenName::no_error_find_in_lines(&lines),
            alias_suffix_name:                                  AliasSuffixName::no_error_find_in_lines(&lines),
            name_suffix:                                        NameSuffix::no_error_find_in_lines(&lines), 
            physical_description_weight_range:                  PhysicalDescriptionWeightRange::no_error_find_in_lines(&lines),
            race_ethnicity:                                     RaceEthnicity::no_error_find_in_lines(&lines),
            standard_vehicle_classification:                    StandardVehicleClassification::no_error_find_in_lines(&lines),
            standard_endorsement_code:                          StandardEndorsementCode::no_error_find_in_lines(&lines),
            standard_restriction_code:                          StandardRestrictionCode::no_error_find_in_lines(&lines),
            jurisdiction_vehicle_classification_description:    JurisdictionSpecificVehicleClassificationDescription::no_error_find_in_lines(&lines),
            jurisdiction_specific_edorsement_code_description:  JurisdictionSpecificEndorsementCodeDescription::no_error_find_in_lines(&lines),
            jurisdiction_specific_restriction_code_description: JurisdictionSpecificRestrictionCodeDescription::no_error_find_in_lines(&lines),
            compliance_type:                                    DHSComplianceType::no_error_find_in_lines(&lines),
            card_revision_date:                                 DHSCardRevisionDate::no_error_find_in_lines(&lines),
            hazmat_endorsement_expiration:                      HAZMATEndorcementExpirationDate::no_error_find_in_lines(&lines),
            limited_duration_document_indicator:                LimitedDurationDocumentIndicator::no_error_find_in_lines(&lines),
            weight_pounds:                                      WeightPounds::no_error_find_in_lines(&lines),
            weight_kilograms:                                   WeightKilograms::no_error_find_in_lines(&lines),
            under_18_until:                                     Under18Until::no_error_find_in_lines(&lines),
            under_19_until:                                     Under19Until::no_error_find_in_lines(&lines),
            under_21_until:                                     Under21Until::no_error_find_in_lines(&lines),
            organ_donor_indicator:                              OrganDonorIndicator::no_error_find_in_lines(&lines),
            veteran_indicator:                                  VeteranIndicator::no_error_find_in_lines(&lines)
        }))
    }   
}

pub struct IdentificationCard {
    pub document_expiration_date:            Option<String>,
    pub customer_family_name:                Option<String>,
    pub customer_first_name:                 Option<String>,
    pub document_issue_date:                 Option<String>,
    pub customer_middle_names:               Option<String>,
    pub date_of_birth:                       Option<String>,
    pub physical_description_sex:            Option<String>,
    pub physical_description_eye_color:      Option<String>,
    pub physical_description_height:         Option<String>,
    pub address_street_1:                    Option<String>,
    pub address_city:                        Option<String>,
    pub address_jurisdiction_code:           Option<String>,
    pub address_postal_code:                 Option<String>,
    pub customer_id_number:                  Option<String>,
    pub document_descriminator:              Option<String>,
    pub country_identification:              Option<String>,
    pub family_name_truncation:              Option<String>,
    pub first_name_truncation:               Option<String>,
    pub middle_name_truncation:              Option<String>,
    pub address_street_2:                    Option<String>,
    pub hair_color:                          Option<String>,
    pub place_of_birth:                      Option<String>,
    pub audit_information:                   Option<String>,
    pub inventory_control_number:            Option<String>,
    pub alias_family_name:                   Option<String>,
    pub alias_given_name:                    Option<String>,
    pub alias_suffix_name:                   Option<String>,
    pub name_suffix:                         Option<String>,
    pub physical_description_weight_range:   Option<String>,
    pub race_ethnicity:                      Option<String>,
    pub compliance_type:                     Option<String>,
    pub card_revision_date:                  Option<String>,
    pub limited_duration_document_indicator: Option<String>,
    pub weight_pounds:                       Option<String>,
    pub weight_kilograms:                    Option<String>,
    pub under_18_until:                      Option<String>,
    pub under_19_until:                      Option<String>,
    pub under_21_until:                      Option<String>,
    pub organ_donor_indicator:               Option<String>,
    pub veteran_indicator:                   Option<String>,
}

impl Reader for IdentificationCard {
    fn new(file: String) -> Result<(Header, IdentificationCard), Error> {
        let (header, lines): (Header, Vec<String>) = Self::get_header_and_lines(file)?;

        for line in &lines {
            let (prefix, value): (&str, &str) = line.split_at(3);
            match by_id(line.split_at(3).0.to_string()) {
                Err(_err) => {
                    println!("Warning invalid Element ID: '{prefix}' WITH VALUE: '{value}' please submit an issue to have the Element added.")
                },
                Ok(_) => {}
            } 
        }

        Ok((header, IdentificationCard {
            document_expiration_date:           DocumentExpirationDate::no_error_find_in_lines(&lines),
            customer_family_name:               CustomerFamilyName::no_error_find_in_lines(&lines),
            customer_first_name:                CustomerFirstName::no_error_find_in_lines(&lines),
            document_issue_date:                DocumentIssueDate::no_error_find_in_lines(&lines),
            customer_middle_names:              CustomerMiddleNames::no_error_find_in_lines(&lines),
            date_of_birth:                      DateOfBirth::no_error_find_in_lines(&lines),
            physical_description_sex:           PhysicalDescriptionSex::no_error_find_in_lines(&lines),
            physical_description_eye_color:     PhysicalDescriptionEyeColor::no_error_find_in_lines(&lines),
            physical_description_height:        PhysicalDescriptionHeight::no_error_find_in_lines(&lines),
            address_street_1:                   AddressStreet1::no_error_find_in_lines(&lines),
            address_city:                       AddressCity::no_error_find_in_lines(&lines),
            address_jurisdiction_code:          AddressJurisdictionCode::no_error_find_in_lines(&lines),
            address_postal_code:                AddressPostalCode::no_error_find_in_lines(&lines),
            customer_id_number:                 CustomerIdNumber::no_error_find_in_lines(&lines),
            document_descriminator:             DocumentDescriminator::no_error_find_in_lines(&lines),
            country_identification:             CountryIdentification::no_error_find_in_lines(&lines),
            family_name_truncation:             FamilyNameTruncation::no_error_find_in_lines(&lines),
            first_name_truncation:              FirstNameTruncation::no_error_find_in_lines(&lines),
            middle_name_truncation:             MiddleNameTruncation::no_error_find_in_lines(&lines),
            address_street_2:                   AddressStreet2::no_error_find_in_lines(&lines),
            hair_color:                         HairColor::no_error_find_in_lines(&lines),
            place_of_birth:                     PlaceOfBirth::no_error_find_in_lines(&lines),
            audit_information:                  AuditInformation::no_error_find_in_lines(&lines),
            inventory_control_number:           InventoryControlNumber::no_error_find_in_lines(&lines),
            alias_family_name:                  AliasFamilyName::no_error_find_in_lines(&lines),
            alias_given_name:                   AliasGivenName::no_error_find_in_lines(&lines),
            alias_suffix_name:                  AliasSuffixName::no_error_find_in_lines(&lines),
            name_suffix:                        NameSuffix::no_error_find_in_lines(&lines),
            physical_description_weight_range:  PhysicalDescriptionWeightRange::no_error_find_in_lines(&lines),
            race_ethnicity:                     RaceEthnicity::no_error_find_in_lines(&lines),
            compliance_type:                    DHSComplianceType::no_error_find_in_lines(&lines),
            card_revision_date:                 DHSCardRevisionDate::no_error_find_in_lines(&lines),
            limited_duration_document_indicator:LimitedDurationDocumentIndicator::no_error_find_in_lines(&lines),
            weight_pounds:                      WeightPounds::no_error_find_in_lines(&lines),
            weight_kilograms:                   WeightKilograms::no_error_find_in_lines(&lines),
            under_18_until:                     Under18Until::no_error_find_in_lines(&lines),
            under_19_until:                     Under19Until::no_error_find_in_lines(&lines),
            under_21_until:                     Under21Until::no_error_find_in_lines(&lines),
            organ_donor_indicator:              OrganDonorIndicator::no_error_find_in_lines(&lines),
            veteran_indicator:                  VeteranIndicator::no_error_find_in_lines(&lines),
        }))
    }

    
}