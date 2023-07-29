# AAMVA PDF417 DATA Reader for Drivers Licenses and Identification Cards written in rust
Rust crate for reading ASNI files from a pdf147 barcode.
**Breaking changes may happen without notice until a better design implementation has been created, this is not a stable release.**

## Usage
```
[dependencies]
my-secret-crate = { git = "https://github.com/ibotva/ASNI-rs", branch = "main" }
```

## Basic Example
> This example uses the `rxing` crate to decode the picture of the pdf147 barcode, and then passes the contents to the ASNI struct.

```rust
let file_name = "./img.jpeg";

let results = rxing::helpers::detect_multiple_in_file(file_name).expect("decode error");
    
for result in results {
    let reader = ASNI::from(result.getText().to_string()).unwrap();
    println!("{:#?}", reader);
}
```

## Disclosure
Use of this software is subject to [USC §2721: Prohibition on release and use of certain personal information from State motor vehicle records](https://uscode.house.gov/view.xhtml?path=/prelim@title18/part1/chapter123&edition=prelim). Please see section b, permissible uses before using this software. This software was made for research purposes. The owner of the software claims no responsibility, in any form, for the usage of the software.

My interpretation of this bill is that before you can obtain the information from someones government document, you must obtain the written consent of the document holder, or eletronic consent with electronic signature. See [USC §2721: Definitions](https://uscode.house.gov/view.xhtml?hl=false&edition=prelim&path=%2Fprelim%40title18%2Fpart1%2Fchapter123&req=granuleid%3AUSC-prelim-title18-section2725&num=0&saved=L3ByZWxpbUB0aXRsZTE4L3BhcnQxL2NoYXB0ZXIxMjM%3D%7CZ3JhbnVsZWlkOlVTQy1wcmVsaW0tdGl0bGUxOC1jaGFwdGVyMTIz%7C%7C%7C0%7Cfalse%7Cprelim) for how they define consent.

## Contribution & Issues
If you would like to contribute please make a pull request, if you would like for me to help with your problem, please submit an issue.

## AAMVA Standard Docs and Notes
- [AAMVA DL/ID Card Design Standard PDF](https://www.aamva.org/getmedia/99ac7057-0f4d-4461-b0a2-3a5532e1b35c/AAMVA-2020-DLID-Card-Design-Standard.pdf) Pages 46 through 61 define the PDF417 design spec, including an element table starting at 50.

- Data from AAMVA spec cards is encoded in ASCII/iso-8859-1, also known as Latin1. However, because the barcode decoder decodes it to a rust string slice, the encoding for this reader will be UTF-8.

- [Open Source AAMVA JavaScript Client](https://github.com/winfinit/aamvajs/blob/master/index.js), this has loads of useful information about specific versions.