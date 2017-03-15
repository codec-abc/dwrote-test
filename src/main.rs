extern crate dwrote;
extern crate fontsan;

use std::io::prelude::*;
use std::fs::File;

#[derive(Debug)]
enum FontCheckResult {
    IOError,
    FailureDirectWrite,
    FailureFontSanitizer,
    Success,
}

fn check_font(path: &str) -> FontCheckResult {
    let mut file = File::open(path);
    match file.as_mut() {
        Err(_) => FontCheckResult::IOError,
        Ok(ref mut f) => {
            let mut buffer: Vec<u8> = vec![];
            let result = f.read_to_end(&mut buffer);
            match result {
                Err(_) => FontCheckResult::IOError,
                Ok(_) => {
                    let bytes = fontsan::process(&buffer);
                    match bytes {
                        Err(_) => FontCheckResult::FailureFontSanitizer,
                        Ok(sanitized_buffer) => {
                            let new_font = dwrote::FontFile::new_from_data(&sanitized_buffer);
                            match new_font {
                                Some(_) => FontCheckResult::Success,
                                None => FontCheckResult::FailureDirectWrite,
                            }
                        }
                    }
                }
            }
        }
    }
}

fn open_file_and_print_result(path: &str) -> () {
    let result = check_font(path);
    println!("{} : {:?}", path, result);
}

fn main() {
    let file_paths = vec!["data/fontawesome-webfont-fixed.woff",
                          "data/fontawesome-webfont-orig.woff",
                          "data/garbage.woff"];

    for path in &file_paths {
        open_file_and_print_result(path);
    }
}
