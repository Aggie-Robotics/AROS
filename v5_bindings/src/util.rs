use rgb::RGB8;

pub fn rgb_to_i32(rgb: RGB8) -> i32 {
    (rgb.r as i32) << 16 + (rgb.g as i32) << 8 + (rgb.b as i32)
}
