# AAMVA PDF417 Data Reader, written in rust, for Drivers Licenses and Identification Cards
Rust crate for reading ASNI files from a PDF417 barcode.
**Breaking changes may happen without notice until a better design implementation has been created, this is not a stable release.**

## Usage
```
[dependencies]
aamva-rs = { git = "https://github.com/ibotva/aamva-rs", branch = "main" }
```

## Basic Example
> This example uses the `rxing` crate to decode the picture of the pdf147 barcode, and then passes the contents to the ASNI struct.
> Bare in mind, I had to crop the pdf417 code to size and it has to be a good picture of the code.

```rust
use asni_rs::documents::{DriversLicense, Reader};
use rxing::BarcodeFormat::PDF_417;

fn main() {
    match rxing::helpers::detect_in_file("./path_to_img.jpeg", Some(PDF_417)) {
        Ok(res) => {
            match DriversLicense::new(res.to_string()) {
                Ok((header, drivers_license)) => {
                    println!("{:#?}", header);
                    println!("{:#?}", drivers_license);
                },
                Err(err) => println!("Drivers License Error: {}", err),
            }
            
        },
        Err(err) => println!("PDF417 Barcode Error: {}", err),
    };
}

}
```

## Disclosure
Use of this software is subject to [USC §2721: Prohibition on release and use of certain personal information from State motor vehicle records](https://uscode.house.gov/view.xhtml?path=/prelim@title18/part1/chapter123&edition=prelim). Please see section b, permissible uses before using this software. This software was made for research purposes. The owner of the software claims no responsibility, in any form, for the usage of the software.

My interpretation of this bill is that before you can obtain the information from someones government document, you must obtain the written consent of the document holder, or eletronic consent with electronic signature. See [USC §2721: Definitions](https://uscode.house.gov/view.xhtml?hl=false&edition=prelim&path=%2Fprelim%40title18%2Fpart1%2Fchapter123&req=granuleid%3AUSC-prelim-title18-section2725&num=0&saved=L3ByZWxpbUB0aXRsZTE4L3BhcnQxL2NoYXB0ZXIxMjM%3D%7CZ3JhbnVsZWlkOlVTQy1wcmVsaW0tdGl0bGUxOC1jaGFwdGVyMTIz%7C%7C%7C0%7Cfalse%7Cprelim) for how they define consent.

You are also subject to state laws, do your research before using this software.

## Contribution & Issues
If you would like to contribute please make a pull request, if you would like for me to help with your problem, please submit an issue.

## AAMVA Standard Docs and Notes
- [AAMVA DL/ID Card Design Standard PDF](https://www.aamva.org/getmedia/99ac7057-0f4d-4461-b0a2-3a5532e1b35c/AAMVA-2020-DLID-Card-Design-Standard.pdf) Pages 46 through 61 define the PDF417 design spec, including an element table starting at 50.

- Data from AAMVA spec cards is encoded in ASCII/iso-8859-1, also known as Latin1. However, because the barcode decoder decodes it to a rust string slice, the encoding for this reader will be UTF-8.

- [Open Source AAMVA JavaScript Client](https://github.com/winfinit/aamvajs/blob/master/index.js), this has loads of useful information about specific versions.
