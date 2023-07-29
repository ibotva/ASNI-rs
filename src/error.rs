use snafu::prelude::*;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Non compliance error, @ sign required as first character."))]
    NonCompliance,

    #[snafu(display("Formatting error, violation:\n{violation}"))]
    Formmating { violation: String },

    #[snafu(display("File type should be 'ANSI ', please check your headers."))]
    InvalidFileType,

    #[snafu(display("Error with threading lock in static element structs"))]
    LockError,

    #[snafu(display("Element with name '{element_name}' not found"))]
    ElementNotFound { element_name: String },

    #[snafu(display("Element has invalid document_type: {document_type}, document_type must be one of 'Both', 'ID', 'DL"))]
    DocumentTypeInvalid { document_type: String },

    #[snafu(display("Match error: {var_1} does not match {var_2}"))]
    MatchError { var_1: String, var_2: String },

    #[snafu(display("Invalid length of string"))]
    InvalidStringLength
}
