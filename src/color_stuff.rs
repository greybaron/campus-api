fn hex_to_rgb(hex: &str) -> (f32, f32, f32) {
    let hex = hex.trim_start_matches('#');
    let r = u8::from_str_radix(&hex[0..2], 16).expect("Invalid hex color") as f32;
    let g = u8::from_str_radix(&hex[2..4], 16).expect("Invalid hex color") as f32;
    let b = u8::from_str_radix(&hex[4..6], 16).expect("Invalid hex color") as f32;
    (r, g, b)
}

pub fn hex_to_luminance(hex: &str) -> f32 {
    let (r, g, b) = hex_to_rgb(hex);

    0.299 * r + 0.587 * g + 0.114 * b
}
