use std::{fs::File, io::BufWriter, path::Path};

use super::utils::vector::Color;

pub struct Image {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<Color>,
}

impl Image {
    pub fn new(width: u32, height: u32, background_color: Color) -> Self {
        let pixels = std::iter::repeat_with(|| background_color.clone())
        .take((width * height) as usize)
        .collect::<Vec<_>>();

        Self {
            width,
            height,
            pixels,
        }
    }

    pub fn set_pixels(&mut self, pixels: &[Color]){
        self.pixels = Vec::from(pixels);
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, color: Color) {
        let el = (self.height - 1 - y) * self.width + x;
        self.pixels[el as usize] = color;
    }

    pub fn write_image(&self, path: &str) -> bool {
        println!("writing Image with {} pixels", self.width * self.height);
        if (self.width * self.height) as usize == self.pixels.len() {
            let path = Path::new(path);
            let file = File::create(path).unwrap();
            let ref mut w = BufWriter::new(file);

            let mut encoder = png::Encoder::new(w, self.width, self.height);
            encoder.set_color(png::ColorType::Rgba);
            encoder.set_depth(png::BitDepth::Eight);
            encoder.set_source_gamma(png::ScaledFloat::from_scaled(45455)); // 1.0 / 2.2, scaled by 100000
            encoder.set_source_gamma(png::ScaledFloat::new(1.0 / 2.2)); // 1.0 / 2.2, unscaled, but rounded
            let source_chromaticities = png::SourceChromaticities::new(
                // Using unscaled instantiation here
                (0.31270, 0.32900),
                (0.64000, 0.33000),
                (0.30000, 0.60000),
                (0.15000, 0.06000),
            );
            encoder.set_source_chromaticities(source_chromaticities);
            let mut writer = encoder.write_header().unwrap();

            let data:Vec<u8> = self.pixels.iter().map(|vec| [vec[0]*255.,vec[1]*255.,vec[2]*255.,255.]).flatten().map(|x| x.clamp(0., 255.) as u8).collect();

            writer.write_image_data(&data).unwrap();

            return true;
        } else {
            println!("pixels dosn't match the count");
            return false;
        }
    }
}
