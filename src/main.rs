use resvg::tiny_skia::Pixmap;
use resvg::tiny_skia::Transform;
use tiny_skia::Color;
use std::fs::File;
use std::io::{self, Write};
use usvg::{Options, Tree};

fn main() {
    println!("svg path enter:");
    let mut path = String::new();
    io::stdin().read_line(&mut path).expect("not read path");
    let path = path.trim();

    let mut options = Options::default();
    options.resources_dir = Some(std::path::PathBuf::from(path).parent().unwrap().to_path_buf());
    let options_ref = options.to_ref();

    let svg_data = std::fs::read(path).expect("not read file svg");
    let tree = Tree::from_data(&svg_data, &options_ref).expect("not parse svg file");

    let pixmap_size = 80; 
    let mut pixmap = Pixmap::new(pixmap_size, pixmap_size).expect("not create pixmap");
    let transform = Transform::default();

    let gray = Color::from_rgba(42.0 / 255.0, 50.0 / 255.0, 63.0 / 255.0, 1.0).unwrap();
    pixmap.fill(gray);
    
    resvg::render(
        &tree,
        usvg::FitTo::Size(pixmap_size, pixmap_size),
        transform,
        pixmap.as_mut(),
    )
    .expect("not render SVG");

    let mut rgb565_data = Vec::new();
    for pixel in pixmap.pixels() {
        let r = pixel.red() >> 3;
        let g = pixel.green() >> 2;
        let b = pixel.blue() >> 3;
        let rgb565 = (r as u16) << 11 | (g as u16) << 5 | (b as u16);
        rgb565_data.push(rgb565);
    }

    let mut byte_data = Vec::new();
    for &rgb in &rgb565_data {
        let high_byte = (rgb >> 8) as u8;
        let low_byte = (rgb & 0xFF) as u8;
        byte_data.push(high_byte);
        byte_data.push(low_byte);
    }

    let file_path = "output/file.txt";
    let mut file = File::create(file_path).expect("not create file");
    writeln!(file, "pub mod test1 {{").expect("err : not write to file");
    writeln!(file, "#[rustfmt::skip]").expect("err : not write to file");
    writeln!(file, "pub const DATA: &[u8] = &[").expect("err : not write to file");
    for chunk in byte_data.chunks(2) {
        let byte1 = chunk[0];
        let byte2 = chunk[1];
        writeln!(file, "    0b{:08b}, 0b{:08b},", byte1, byte2).expect("err : not write to file");
    }
    writeln!(file, "];").expect("err : not write to file");
    writeln!(file, "}}").expect("err : not write to file");
    println!("Data successfully! in {}", file_path);
}