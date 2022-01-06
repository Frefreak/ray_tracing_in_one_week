use ppm::{PPM, RGB};

fn main() {
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
    image.save("test.ppm").unwrap();
}
