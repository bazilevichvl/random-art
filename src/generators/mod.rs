use crate::expression::Expression;

use image::{GenericImage, Luma, Pixel, Rgb};

pub trait PixelGenerator {
    type Color: Pixel;

    fn generate_color(&self, x: f32, y: f32) -> Self::Color;
}

pub struct GrayscaleGenerator {
    intensity: Expression,
}

impl GrayscaleGenerator {
    pub fn new(depth: usize) -> Self {
        GrayscaleGenerator {
            intensity: Expression::new(depth),
        }
    }
}

impl PixelGenerator for GrayscaleGenerator {
    type Color = Luma<u8>;

    fn generate_color(&self, x: f32, y: f32) -> Self::Color {
        // Note: expression always returns a value from -1 to 1, hence we should re-scale it here
        let i = (self.intensity.eval(x, y) + 1.) / 2.;
        Luma([(i * 255.0) as u8])
    }
}

pub struct RgbGenerator {
    r: Expression,
    g: Expression,
    b: Expression,
}

impl RgbGenerator {
    pub fn new(depth: usize) -> Self {
        RgbGenerator {
            r: Expression::new(depth),
            g: Expression::new(depth),
            b: Expression::new(depth),
        }
    }
}

impl PixelGenerator for RgbGenerator {
    type Color = Rgb<u8>;

    fn generate_color(&self, x: f32, y: f32) -> Self::Color {
        // Note: Expression always returns a number from -1 to 1, hence we should re-scale it here
        let r = ((self.r.eval(x, y) + 1.) / 2. * 255.0) as u8;
        let g = ((self.g.eval(x, y) + 1.) / 2. * 255.0) as u8;
        let b = ((self.b.eval(x, y) + 1.) / 2. * 255.0) as u8;
        Rgb([r, g, b])
    }
}

pub fn generate_image<I: GenericImage, Gen: PixelGenerator<Color = I::Pixel>>(
    image: &mut I,
    generator: Gen,
) {
    let w = image.width() as i32;
    let h = image.height() as i32;
    (0..h).for_each(|j| {
        for i in 0..w {
            let x = (i - w / 2) as f32 / w as f32;
            let y = (j - h / 2) as f32 / h as f32;
            //dbg!(x, y);
            let intensity = generator.generate_color(x, y);
            image.put_pixel(i as u32, j as u32, intensity);
        }
    })
}
