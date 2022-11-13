use super::utils::vector::Color;

pub struct Image {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<Color>,
}

impl Image {
    pub fn set_pixel(&mut self, x: u32, y:u32, color:Color){
        let el = y * self.width + x;
        self.pixels[el as usize] = color;
    }
}