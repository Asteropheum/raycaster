use std::fs::File;
use std::io::{BufWriter, Write};

/// Store each RGBA u8 component in one u32 integer.
pub fn pack_color(r: u8, g: u8, b: u8, a: Option<u8>) -> u32 {
    (u32::from(a.unwrap_or(255)) << 24) + (u32::from(b) << 16) + (u32::from(g) << 8) + u32::from(r)
}

pub fn unpack_color(color: &u32, r: &mut u8, g: &mut u8, b: &mut u8, a: &mut u8) {
    *r = (color & 255) as u8;
    *g = ((color >> 8) & 255) as u8;
    *b = ((color >> 16) & 255) as u8;
    *a = ((color >> 24) & 255) as u8;
}

pub fn drop_ppm_image(filename: &str, image: &[u32], width: usize, height: usize) {
    assert_eq!(image.len(), (width * height) as usize);
    let file = File::create(filename).expect("can not open file");
    let mut writer = BufWriter::new(file);

    write!(writer, "P6\n{} {}\n255\n", width, height).expect("can not write to file");

    for color in image.iter().take(width * height) {
        let mut r = 0;
        let mut g = 0;
        let mut b = 0;
        let mut a = 0;
        unpack_color(color, &mut r, &mut g, &mut b, &mut a);
        writer.write_all(&[r, g, b]).expect("can not write to file");
    }
}