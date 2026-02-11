use std::io::Write;

use crate::vec3::Vec3;

// Type alias
pub type Color = Vec3;

pub fn write_color(out: &mut impl Write, pixel_color: Color) {
    // Write the translated [0, 255] value of each color component.
    let r = (255.999 * pixel_color.x()) as u8;
    let g = (255.999 * pixel_color.y()) as u8;
    let b = (255.999 * pixel_color.z()) as u8;
    writeln!(out, "{} {} {}", r, g, b).expect("Writing color");
}
