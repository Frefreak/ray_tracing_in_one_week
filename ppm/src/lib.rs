use std::{path::Path, io::Write, time::SystemTime};

#[derive(Clone)]
pub struct RGB(u8, u8, u8);

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
    pub fn set(&mut self, row: usize, column: usize, color: RGB) {
        self.pixels[row][column] = color;
    }

    pub fn save<F>(&self, fp: F) -> std::io::Result<()>
    where
        F: AsRef<Path>,
    {
        let t = SystemTime::now();
        println!("saving to {:?}", fp.as_ref());
        let mut f = std::fs::File::create(fp)?;
        f.write(b"P3\n")?;
        f.write(format!("{} {}\n", self.width, self.height).as_bytes())?;
        f.write(b"255\n")?;
        for row in &self.pixels {
            for c in row {
                f.write(format!("{} {} {} ", c.0, c.1, c.2).as_bytes())?;
            }
            f.write(b"\n")?;
        }
        println!("time: {:?}", t.elapsed());
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::{PPM, RGB};

    #[test]
    fn test_generate_img() {
        let width = 256;
        let height = 256;
        let mut image = PPM::new(width, height);
        let b = (255.999 * 0.25) as u8;
        for j in (0..height).rev() {
            for i in 0..width {
                let r = i as f32 / (width as f32 - 1.) * 255.999;
                let g = j as f32 / (height as f32 - 1.) * 255.999;
                image.set((height - j -1) as usize, i as usize, RGB(r as u8, g as u8, b));
            }
        }
        assert!(image.save("test.ppm").is_ok());
    }
}
