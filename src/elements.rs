use lazy_static::lazy_static;

use crate::error::Error;

#[derive(Debug)]
pub struct ElementValidationData {
    pub id:             &'static str,
    pub alpha:          &'static bool,
    pub numeric:        &'static bool,
    pub special:        &'static bool,
    pub fixed_length:   &'static bool,
    pub length:         &'static i8,
    pub document_type:  &'static str
}



pub trait Element {
    fn data() -> ElementValidationData;

    fn remove_padding(value: String) -> String {
        let split_values: Vec<&str> = value.split(" ").collect();

        let mut new_values: Vec<&str> = vec![];

        for val in split_values {
            if val != "" && val != "\r" {
                new_values.push(val);
            }
        }

        new_values.join("")
    }

    fn no_error_validate(element_line: &String) -> Option<String> {
        match Self::validate(element_line) {
            Ok(value) => Some(value),
            Err(_err) => None
        }
    }

    fn validate(element_line: &String) -> Result<String, Error> {
        // Here is where we will validate that the data matches AAVMA specification.
        let data = Self::data();

        let (prefix, value) = element_line.split_at(3);
        

        if prefix != data.id {
            Err(Error::MatchError { var_1: prefix.to_string(), var_2: data.id.to_string() })
        } else { 
            Ok(value.to_string())
        }
    }

    fn find_in_lines(lines: &Vec<String>) -> Result<String, Error> {
        for line in lines {
            match Self::validate(line) {
                Ok(value) => {
                    return Ok(value);
                }, 
                Err(_) => {} 
            }
        }
        return  Err(Error::ElementNotFound { element_name: Self::data().id.to_string() });
    }

    fn no_error_find_in_lines(lines: &Vec<String>) -> Option<String> {
        match Self::find_in_lines(lines) {
            Ok(value)  => {
                Some(Self::remove_padding(value))
            }, Err(_) => {
                None
            }
        }
    }
}

#[derive(Clone, Copy)]
pub struct JurisdictionSpecificVehicleClass;

impl Element for JurisdictionSpecificVehicleClass {
    fn data() -> ElementValidationData {
        ElementValidationData { id: "DCA", alpha: &true, numeric: &true, special: &true, fixed_length: &false, length: &6, document_type: "DL"}
    }
}

pub struct JurisdictionSpecificRestrictionCodes;

impl Element for JurisdictionSpecificRestrictionCodes {
    fn data() -> ElementValidationData {
        ElementValidationData { id: "DCB", alpha: &true, numeric: &true, special: &true, fixed_length: &false, length: &12, document_type: "DL" }
    }
}

pub struct JurisdictionSpecificEndorsementCodes;

impl Element for JurisdictionSpecificEndorsementCodes {
    fn data() -> ElementValidationData {
        ElementValidationData { id: "DCD", alpha: &true, numeric: &true, special: &true, fixed_length: &false, length: &5, document_type: "DL" }
    }
}

pub struct DocumentExpirationDate;

impl Element for DocumentExpirationDate {
    fn data() -> ElementValidationData {
        ElementValidationData { id: "DBA", alpha: &false, numeric: &true, special: &false, fixed_length: &true, length: &8, document_type: "Both" }
    }
}

pub struct CustomerFamilyName;

impl Element for CustomerFamilyName {
    fn data() -> ElementValidationData {
        ElementValidationData { id: "DCS", alpha: &true, numeric: &true, special: &true, fixed_length: &false, length: &40, document_type: "Both" }
    }
}

pub struct CustomerFirstName;

impl Element for CustomerFirstName {
    fn data() -> ElementValidationData {
        ElementValidationData { id: "DAC", alpha: &true, numeric: &true, special: &true, fixed_length: &false, length: &40, document_type: "Both" }
    }
}

pub struct CustomerMiddleNames;

impl Element for CustomerMiddleNames {
    fn data() -> ElementValidationData {
        ElementValidationData { id: "DAD", alpha: &true, numeric: &true, special: &true, fixed_length: &false, length: &40, document_type: "Both" }
    }
}

pub struct DocumentIssueDate;

impl Element for DocumentIssueDate {
    fn data() -> ElementValidationData {
        ElementValidationData { id: "DBD", alpha: &false, numeric: &true, special: &false, fixed_length: &true, length: &8, document_type: "Both" }
    }
}

pub struct DateOfBirth;

impl Element for DateOfBirth {
    fn data() -> ElementValidationData {
        ElementValidationData { id: "DBB", alpha: &false, numeric: &true, special: &false, fixed_length: &true, length: &8, document_type: "Both" }
    }
}

pub struct PhysicalDescriptionSex;

impl Element for PhysicalDescriptionSex {
    fn data() -> ElementValidationData {
        ElementValidationData { id: "DBC", alpha: &false, numeric: &true, special: &false, fixed_length: &true, length: &1, document_type: "Both"}
    }
}

pub struct PhysicalDescriptionEyeColor;

impl Element for PhysicalDescriptionEyeColor {
    fn data() -> ElementValidationData {
        ElementValidationData { id: "DAY", alpha: &true, numeric: &false, special: &false, fixed_length: &true, length: &3, document_type: "Both" }
    }
}

pub struct PhysicalDescriptionHeight;

impl Element for PhysicalDescriptionHeight {
    fn data() -> ElementValidationData {
        ElementValidationData { id: "DAU", alpha: &true, numeric: &true, special: &true, fixed_length: &true, length: &6, document_type: "Both" }
    }
}

pub struct AddressStreet1;

impl Element for AddressStreet1 {
    fn data() -> ElementValidationData {
        ElementValidationData { id: "DAG", alpha: &true, numeric: &true, special: &true, fixed_length: &false, length: &35, document_type: "Both" }
    }
}

pub struct AddressCity;

impl Element for AddressCity {
    fn data() -> ElementValidationData {
        ElementValidationData { id: "DAI", alpha: &true, numeric: &true, special: &true, fixed_length: &false, length: &20, document_type: "Both" }
    }
}

pub struct AddressJurisdictionCode;

impl Element for AddressJurisdictionCode {
    fn data() -> ElementValidationData {
        ElementValidationData { id: "DAJ", alpha: &true, numeric: &false, special: &false, fixed_length: &true, length: &2, document_type: "Both" }
    }
}

pub struct AddressPostalCode;

impl Element for AddressPostalCode {
    fn data() -> ElementValidationData {
        ElementValidationData { id: "DAK", alpha: &true, numeric: &true, special: &true, fixed_length: &true, length: &11, document_type: "Both" }
    }
}

pub struct CustomerIdNumber;

impl Element for CustomerIdNumber {
    fn data() -> ElementValidationData {
        ElementValidationData { id: "DAQ", alpha: &true, numeric: &true, special: &true, fixed_length: &false, length: &25, document_type: "Both" }
    }
}

pub struct DocumentDescriminator;

impl Element for DocumentDescriminator {
    fn data() -> ElementValidationData {
        ElementValidationData { id: "DCF", alpha: &true, numeric: &true, special: &true, fixed_length: &false, length: &25, document_type: "Both" }
    }
}

pub struct CountryIdentification;

impl Element for CountryIdentification {
    fn data() -> ElementValidationData {
        ElementValidationData { id: "DCG", alpha: &true, numeric: &false, special: &false, fixed_length: &true, length:  &3, document_type: "Both" }
    }
}

pub struct FamilyNameTruncation;

impl Element for FamilyNameTruncation {
    fn data() -> ElementValidationData {
        ElementValidationData { id: "DDE", alpha: &true, numeric: &false, special: &false, fixed_length: &true, length:  &1, document_type: "Both" }
    }
}


pub struct FirstNameTruncation;

impl Element for FirstNameTruncation {
    fn data() -> ElementValidationData {
        ElementValidationData { id: "DDF", alpha: &true, numeric: &false, special: &false, fixed_length: &true, length:  &1, document_type: "Both" }
    }
}


pub struct MiddleNameTruncation;

impl Element for MiddleNameTruncation {
    fn data() -> ElementValidationData {
        ElementValidationData { id: "DDG", alpha: &true, numeric: &false, special: &false, fixed_length: &true, length:  &1, document_type: "Both" }
    }
}

pub struct AddressStreet2;

impl Element for AddressStreet2 {
    fn data() -> ElementValidationData {
        ElementValidationData { id: "DAH", alpha: &true, numeric: &true, special: &true, fixed_length: &false, length: &35, document_type: "Both" }
    }
}

pub struct HairColor;

impl Element for HairColor {
    fn data() -> ElementValidationData {
        ElementValidationData { id: "DAZ", alpha: &true, numeric: &false, special: &false, fixed_length: &false, length: &12, document_type: "Both" }
    }
}

pub struct PlaceOfBirth;

impl Element for PlaceOfBirth {
    fn data() -> ElementValidationData {
        ElementValidationData { id: "DCI", alpha: &true, numeric: &false, special: &false, fixed_length: &false, length: &12, document_type: "Both" }
    }
}

pub struct AuditInformation;

impl Element for AuditInformation {
    fn data() -> ElementValidationData {
        ElementValidationData { id: "DCJ", alpha: &true, numeric: &true, special: &true, fixed_length: &false, length: &25, document_type: "Both" }
    }
}

pub struct InventoryControlNumber;

impl Element for InventoryControlNumber {
    fn data() -> ElementValidationData {
        ElementValidationData { id: "DCK", alpha: &true, numeric: &true, special: &true, fixed_length: &false, length: &25, document_type: "Both" }
    }
}

pub struct AliasFamilyName;

impl Element for AliasFamilyName {
    fn data() -> ElementValidationData {
        ElementValidationData { id: "DBN", alpha: &true, numeric: &true, special: &true, fixed_length: &false, length: &10, document_type: "Both" }
    }
}

pub struct AliasGivenName;

impl Element for AliasGivenName {
    fn data() -> ElementValidationData {
        ElementValidationData { id: "DBG", alpha: &true, numeric: &true, special: &true, fixed_length: &false, length: &15, document_type: "Both" }
    }
}

pub struct AliasSuffixName;

impl Element for AliasSuffixName {
    fn data() -> ElementValidationData {
        ElementValidationData { id: "DBS", alpha: &true, numeric: &true, special: &true, fixed_length: &false, length: &5, document_type: "Both" }
    }
}

pub struct NameSuffix;

impl Element for NameSuffix {
    fn data() -> ElementValidationData {
        ElementValidationData { id: "DCU", alpha: &true, numeric: &true, special: &true, fixed_length: &false, length: &5, document_type: "Both" }
    }
}

pub struct PhysicalDescriptionWeightRange;

impl Element for PhysicalDescriptionWeightRange {
    fn data() -> ElementValidationData {
        ElementValidationData { id: "DCE", alpha: &false, numeric: &true, special: &false, fixed_length: &true, length: &1, document_type: "Both" }
    }
}

pub struct RaceEthnicity;

impl Element for RaceEthnicity {
    fn data() -> ElementValidationData {
        ElementValidationData { id:  "DCL", alpha: &true, numeric: &false, special: &false, fixed_length: &false, length: &3, document_type: "Both" }
    }
}

pub struct StandardVehicleClassification;

impl Element for StandardVehicleClassification {
    fn data() -> ElementValidationData {
        ElementValidationData { id: "DCM", alpha: &true, numeric: &true, special: &false, fixed_length: &true, length: &4, document_type: "DL" }
    }
}

pub struct StandardEndorsementCode;

impl Element for StandardEndorsementCode {
    fn data() -> ElementValidationData {
        ElementValidationData { id: "DCN", alpha: &true, numeric: &true, special: &false, fixed_length: &true, length: &5, document_type: "DL" }
    }
}
pub struct StandardRestrictionCode;

impl Element for StandardRestrictionCode {
    fn data() -> ElementValidationData {
        ElementValidationData { id: "DCO", alpha: &true, numeric: &true, special: &false, fixed_length: &true, length: &12, document_type: "DL" }
    }
}

pub struct JurisdictionSpecificVehicleClassificationDescription;

impl Element for JurisdictionSpecificVehicleClassificationDescription {
    fn data() -> ElementValidationData {
        ElementValidationData { id: "DCP", alpha: &true, numeric: &true, special: &true, fixed_length: &false, length: &50, document_type: "DL" }
    }
}

pub struct JurisdictionSpecificEndorsementCodeDescription;

impl Element for JurisdictionSpecificEndorsementCodeDescription {
    fn data() -> ElementValidationData {
        ElementValidationData { id: "DCQ", alpha: &true, numeric: &true, special: &true, fixed_length: &false, length: &50, document_type: "DL" }

    }
}

pub struct JurisdictionSpecificRestrictionCodeDescription;

impl Element for JurisdictionSpecificRestrictionCodeDescription {
    fn data() -> ElementValidationData {
        ElementValidationData { id: "DCR", alpha: &true, numeric: &true, special: &true, fixed_length: &false, length: &50, document_type: "DL" }
    }
}

pub struct DHSComplianceType;

impl Element for DHSComplianceType {
    fn data() -> ElementValidationData {
        ElementValidationData { id: "DDA", alpha: &true, numeric: &false, special: &false, fixed_length: &true, length: &1, document_type: "Both" }
    }
}

pub struct DHSCardRevisionDate;

impl Element for DHSCardRevisionDate {
    fn data() -> ElementValidationData {
        ElementValidationData { id: "DDB", alpha: &false, numeric: &true, special: &false, fixed_length: &true, length: &8, document_type: "Both" }
    }
}

pub struct HAZMATEndorcementExpirationDate;

impl Element for HAZMATEndorcementExpirationDate {
    fn data() -> ElementValidationData {
        ElementValidationData { id: "DDC", alpha: &false, numeric: &true, special: &false, fixed_length: &true, length: &8, document_type: "DL" }
    }
}

pub struct LimitedDurationDocumentIndicator;

impl Element for LimitedDurationDocumentIndicator {
    fn data() -> ElementValidationData {
        ElementValidationData { id: "DDD", alpha: &false, numeric: &true, special: &false, fixed_length: &true, length: &1, document_type: "Both" }
    }
}

pub struct WeightPounds;

impl Element for WeightPounds {
    fn data() -> ElementValidationData {
        ElementValidationData { id: "DAW", alpha: &false, numeric: &true, special: &false, fixed_length: &true, length: &3, document_type: "Both" }
    }
}
pub struct WeightKilograms;

impl Element for WeightKilograms {
    fn data() -> ElementValidationData {
        ElementValidationData { id: "DAX", alpha: &false, numeric: &true, special: &false, fixed_length: &true, length: &3, document_type: "Both" }
    }
}
pub struct Under18Until;

impl Element for Under18Until {
    fn data() -> ElementValidationData {
        ElementValidationData { id: "DDH", alpha: &false, numeric: &true, special: &false, fixed_length: &true, length: &8, document_type: "Both" }
    }
}


pub struct Under19Until;

impl Element for Under19Until {
    fn data() -> ElementValidationData {
        ElementValidationData { id: "DDI", alpha: &false, numeric: &true, special: &false, fixed_length: &true, length: &8, document_type: "Both" }
    }
}

pub struct Under21Until;

impl Element for Under21Until {
    fn data() -> ElementValidationData {
        ElementValidationData { id: "DDJ", alpha: &false, numeric: &true, special: &false, fixed_length: &true, length: &8, document_type: "Both" }
    }
}

pub struct OrganDonorIndicator;

impl Element for OrganDonorIndicator {
    fn data() -> ElementValidationData {
        ElementValidationData { id: "DDK", alpha: &false, numeric: &true, special: &false, fixed_length: &true, length: &1, document_type: "Both" }
    }
}

pub struct VeteranIndicator;

impl Element for VeteranIndicator {
    fn data() -> ElementValidationData {
        ElementValidationData { id: "DDL", alpha: &false, numeric: &true, special: &false, fixed_length: &true, length: &1, document_type: "Both" }
    }
}

lazy_static!(
    static ref ELEMENTS: Vec<ElementValidationData> = {
        let mut elements = Vec::new();
        elements.push(JurisdictionSpecificVehicleClass::data());
        elements.push(JurisdictionSpecificRestrictionCodes::data());
        elements.push(JurisdictionSpecificEndorsementCodes::data());
        elements.push(DocumentExpirationDate::data());
        elements.push(CustomerFamilyName::data());
        elements.push(CustomerFirstName::data());
        elements.push(CustomerMiddleNames::data());
        elements.push(DocumentIssueDate::data());
        elements.push(DateOfBirth::data());
        elements.push(PhysicalDescriptionSex::data());
        elements.push(PhysicalDescriptionEyeColor::data());
        elements.push(PhysicalDescriptionHeight::data());
        elements.push(AddressStreet1::data());
        elements.push(AddressCity::data());
        elements.push(AddressJurisdictionCode::data());
        elements.push(AddressPostalCode::data());
        elements.push(CustomerIdNumber::data());
        elements.push(DocumentDescriminator::data());
        elements.push(CountryIdentification::data());
        elements.push(FamilyNameTruncation::data());
        elements.push(FirstNameTruncation::data());
        elements.push(MiddleNameTruncation::data());
        elements.push(AddressStreet2::data());
        elements.push(HairColor::data());
        elements.push(PlaceOfBirth::data());
        elements.push(AuditInformation::data());
        elements.push(InventoryControlNumber::data());
        elements.push(AliasFamilyName::data());
        elements.push(AliasGivenName::data());
        elements.push(AliasSuffixName::data());
        elements.push(NameSuffix::data());
        elements.push(PhysicalDescriptionWeightRange::data());
        elements.push(RaceEthnicity::data());
        elements.push(StandardVehicleClassification::data());
        elements.push(StandardEndorsementCode::data());
        elements.push(StandardRestrictionCode::data());
        elements.push(JurisdictionSpecificVehicleClassificationDescription::data());
        elements.push(JurisdictionSpecificEndorsementCodeDescription::data());
        elements.push(JurisdictionSpecificRestrictionCodeDescription::data());
        elements.push(DHSComplianceType::data());
        elements.push(DHSCardRevisionDate::data());
        elements.push(HAZMATEndorcementExpirationDate::data());
        elements.push(LimitedDurationDocumentIndicator::data());
        elements.push(WeightPounds::data());
        elements.push(WeightKilograms::data());
        elements.push(Under18Until::data());
        elements.push(Under19Until::data());
        elements.push(Under21Until::data());
        elements.push(OrganDonorIndicator::data());
        elements.push(VeteranIndicator::data());
        elements
    };
);



pub fn by_id(id: String) -> Result<&'static ElementValidationData, Error> {

    for element in &*ELEMENTS {
        if element.id == id { 
            return Ok(element);
        }
    };
    
    Err(Error::ElementNotFound { element_name: id.to_string() })
}

#[cfg(test)]
mod tests {
    use crate::elements::by_id;

    #[test]
    fn test_by_id() {
        println!("{:#?}", by_id("DCA".to_string()).unwrap());
    }

    #[test]
    fn peepeepoopoo() {
        let poo: Vec<&str> = "test\u{00}another\u{00}\u{00}\u{00}\u{00}".split("\u{00}").collect();
        println!("{:#?}", poo)
    }
}