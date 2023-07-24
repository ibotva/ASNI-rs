use snafu::prelude::*;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Non compliance error, @ sign required as first character."))]
    NonCompliance,

    #[snafu(display("Formatting error, violation:\n{violation}"))]
    Formmating { violation: String },

    #[snafu(display("File type should be 'ANSI ', please check your headers."))]
    InvalidFileType
}
