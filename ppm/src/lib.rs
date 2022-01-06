use std::{path::Path, time::SystemTime};

use vec3::Color;

/// RGB color
#[derive(Debug, Clone)]
pub struct RGB(pub u8, pub u8, pub u8);

pub struct PPM {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<Vec<RGB>>,
}

impl PPM {
    pub fn new(width: u32, height: u32) -> Self {
        let mut pixels = vec![];
        for _i in 0..height {
            pixels.push(vec![RGB(0, 0, 0); width as usize]);
        }
        PPM {
            width,
            height,
            pixels,
        }
    }

    // no bounding check for performance
    pub fn set(&mut self, row: usize, column: usize, color: Color) {
        self.pixels[row][column] = RGB(
            (color.0 * 255.99) as u8,
            (color.1 * 255.99) as u8,
            (color.2 * 255.99) as u8,
        );
    }

    pub fn save<F>(&self, fp: F) -> std::io::Result<()>
    where
        F: AsRef<Path>,
    {
        let t = SystemTime::now();
        println!("saving to {:?}", fp.as_ref());
        let mut content = String::new();
        content.push_str("P3\n");
        content.push_str(&format!("{} {}\n", self.width, self.height));
        content.push_str("255\n");
        for row in &self.pixels {
            for c in row {
                content.push_str(&format!("{} {} {} ", c.0, c.1, c.2));
            }
            content.push('\n');
        }

        std::fs::write(fp, content)?;
        println!("time: {:?}", t.elapsed());
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use vec3::v3;

    use crate::PPM;

    #[test]
    fn test_generate_img() {
        let width = 256;
        let height = 256;
        let mut image = PPM::new(width, height);
        let b = 0.25;
        for j in (0..height).rev() {
            for i in 0..width {
                let r = i as f64 / (width as f64 - 1.);
                let g = j as f64 / (height as f64 - 1.);
                image.set((height - j - 1) as usize, i as usize, v3!(r, g, b));
            }
        }
        assert!(image.save("test.ppm").is_ok());
    }
}
