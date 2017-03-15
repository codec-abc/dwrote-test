extern crate dwrote;
extern crate fontsan;
extern crate image;

use std::io::prelude::*;
use std::fs::File;

#[derive(Debug)]
enum FontCheckResult {
    IOError,
    FailureDirectWrite,
    FailureFontSanitizer,
    Success,
}


fn print_stuff(face : &dwrote::FontFace, out_path: &str) -> () {
    let a_index = face.get_glyph_indices(&['A' as u32])[0];

    //let metrics = face.get_metrics();

    let gm = face.get_design_glyph_metrics(&[a_index], false)[0];

    let device_pixel_ratio = 1.0f32;
    let em_size = 10.0f32;

    let design_units_per_pixel = face.metrics().designUnitsPerEm as f32 / 16. as f32;
    let scaled_design_units_to_pixels = (em_size * device_pixel_ratio) / design_units_per_pixel;

    let width = (gm.advanceWidth as i32 - (gm.leftSideBearing + gm.rightSideBearing)) as f32 * scaled_design_units_to_pixels;
    let height = (gm.advanceHeight as i32 - (gm.topSideBearing + gm.bottomSideBearing)) as f32 * scaled_design_units_to_pixels;
    let x = (-gm.leftSideBearing) as f32 * scaled_design_units_to_pixels;
    let y = (gm.verticalOriginY - gm.topSideBearing) as f32 * scaled_design_units_to_pixels;

    // FIXME I'm pretty sure we need to do a proper RoundOut type
    // operation on this rect to properly handle any aliasing
    //let left_i = x.floor() as i32;
    //let top_i = (height - y).floor() as i32;
    let width_u = width.ceil() as u32;
    let height_u = height.ceil() as u32;

    //println!("GlyphDimensions: {} {} {} {}", left_i, top_i, width_u, height_u);

    let gdi_interop = dwrote::GdiInterop::create();
    let rt = gdi_interop.create_bitmap_render_target(width_u, height_u);
    let rp = dwrote::RenderingParams::create_for_primary_monitor();
    rt.set_pixels_per_dip(device_pixel_ratio);
    rt.draw_glyph_run(x as f32, y as f32,
                      dwrote::DWRITE_MEASURING_MODE_NATURAL,
                      &face,
                      em_size,
                      &[a_index],
                      &[0f32],
                      &[dwrote::GlyphOffset { advanceOffset: 0., ascenderOffset: 0. }],
                      &rp,
                      &(255.0f32, 0.0f32, 0.0f32));

    let bytes : Vec<u8> = rt.get_opaque_values_as_mask();
    let buffer = File::create(out_path).unwrap();

    let mut my_bytes : Vec<u8> = vec![];
    let mut i = 0;
    for x in 0..(bytes.len()) {
        if i <= 2 {
            let index : usize = x as usize;
            my_bytes.push(bytes[index]);
            print!("{} ", bytes[index]);
        }
        i = i + 1;
        if i == 4 {
            print!("{} ", bytes[x as usize]);
            i = 0;
            //println!("");
        }
    }

    let encoder : image::png::PNGEncoder<File> = image::png::PNGEncoder::new(buffer);
    //encoder.encode(&my_bytes, width_u, height_u, image::ColorType::RGB(8)).unwrap();
    encoder.encode(&bytes, width_u, height_u, image::ColorType::RGBA(8)).unwrap();
}

fn check_font(in_path: &str, out_path: &str) -> FontCheckResult {
    let mut file = File::open(in_path);
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
                                Some(font_file) => {
                                  let face = font_file.create_face(0, dwrote::DWRITE_FONT_SIMULATIONS_NONE);
                                  //let glyphs_count = face.get_glyph_count();
                                  print_stuff(&face, out_path);
                                  FontCheckResult::Success
                                } 
                                None => FontCheckResult::FailureDirectWrite,
                            }
                        }
                    }
                }
            }
        }
    }
}

fn open_file_and_print_result(in_path: &str, out_path: &str) -> () {
    let result = check_font(in_path, out_path);
    println!("{} : {:?}", in_path, result);
}

fn main() {
    let file_paths = vec![("data/fontawesome-webfont-fixed.woff", "fixed.png"),
                          ("data/fontawesome-webfont-orig.woff", "orig.png"),
                          ("data/fontawesome-webfont.woff", "fontawesome.png"),
                          ("data/garbage.woff", "garbage.png"),
                          ("data/roboto.woff2", "roboto.png"),];

    for &(in_path, out_path) in &file_paths {
        open_file_and_print_result(in_path, out_path);
    }
}
