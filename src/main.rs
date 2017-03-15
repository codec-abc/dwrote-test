extern crate dwrote;

use std::io::prelude::*;
use std::fs::File;

#[derive(Debug)]
enum FontCheckResult {
    IOError,
    Failure,
    Success
}

fn check_font(path : &str) -> FontCheckResult
{
    let mut file = File::open(path);
    match file.as_mut() 
    {
        Err(_) =>
            { 
                FontCheckResult::IOError
            },
        Ok(ref mut f) => 
        {
            let mut buffer : Vec<u8> = vec![];
            let result = f.read_to_end(&mut buffer);
            match result 
            {
                Err(_) => 
                {
                    FontCheckResult::IOError
                },
                Ok(_) => 
                {
                    let new_font = dwrote::FontFile::new_from_data(&buffer);
                    match new_font {
                        Some (_) => 
                            {
                                FontCheckResult::Success
                            },
                        None => 
                            {
                                FontCheckResult::Failure
                            }
                    }
                }
            }
        }
    }
}

fn main() 
{
    {
        let success_font_path = "data/fontawesome-webfont-fixed.woff";
        let result = check_font(success_font_path);
        println!("{} : {:?}", success_font_path, result);
    }

    {
        let failure_font_path = "data/fontawesome-webfont-orig.woff";
        let result = check_font(failure_font_path);
        println!("{} : {:?}", failure_font_path, result);
    }
}
