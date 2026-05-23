//! Postprocessor module for image processing
use image::{
    DynamicImage, GrayImage, ImageEncoder, RgbImage, codecs::png::PngEncoder, imageops::filter3x3,
};
use rayon::iter::ParallelIterator;
use std::path::Path;

use crate::renderer::bitmap::Bitmap;

mod filters;
mod utilities;

/// Output image type
#[derive(Clone)]
pub enum OutputImage {
    /// Single channel grayscale image
    Grayscale(image::GrayImage),

    /// Three channel RGB image
    Rgb(image::RgbImage),
}
impl OutputImage {
    /// Load an image from a file
    ///
    /// # Errors
    /// Will return an error if the image cannot be loaded
    pub fn load(path: impl AsRef<Path>) -> Result<Self, image::ImageError> {
        let img = image::open(path)?;
        if img.color().has_color() {
            let mut image = RgbImage::new(img.width(), img.height());
            for (x, y, pixel) in img.to_rgb8().enumerate_pixels() {
                image.put_pixel(x, y, *pixel);
            }

            Ok(Self::Rgb(image))
        } else {
            Ok(Self::Grayscale(img.to_luma8()))
        }
    }

    /// Convert the image to a byte vector (RGB)
    #[must_use]
    pub fn into_bytes(mut self) -> Vec<u8> {
        self.convert_to_rgb();
        match self {
            Self::Grayscale(_) => unreachable!(),
            Self::Rgb(img) => img.into_raw(),
        }
    }

    /// Convert the image to a png byte vector
    ///
    /// # Errors
    /// Will return an error if the image cannot be encoded
    pub fn into_png(mut self) -> Result<Vec<u8>, image::ImageError> {
        self.convert_to_rgb();
        match self {
            Self::Grayscale(_) => unreachable!(),
            Self::Rgb(img) => {
                let mut buf = Vec::new();
                PngEncoder::new(&mut buf).write_image(
                    img.as_ref(),
                    img.width(),
                    img.height(),
                    image::ExtendedColorType::Rgb8,
                )?;
                Ok(buf)
            }
        }
    }

    /// Convert the image to a webp byte vector
    ///
    /// compression is a value between 0.0 and 100.0
    #[must_use]
    pub fn into_webp(mut self, compression: f32) -> Option<Vec<u8>> {
        self.convert_to_rgb();
        match self {
            Self::Grayscale(_) => unreachable!(),
            Self::Rgb(img) => {
                let img = DynamicImage::from(img);
                let encoder = webp::Encoder::from_image(&img).ok()?;
                let img = encoder.encode(compression);
                let buf = img.to_vec();
                Some(buf)
            }
        }
    }

    /// Create a new grayscale image from a bitmap
    #[must_use]
    pub fn new_grayscale(bitmap: &Bitmap) -> Self {
        let (width, height) = bitmap.size();
        let bitmap = bitmap.inner();

        let mut img = image::GrayImage::new(width as u32, height as u32);
        img.par_enumerate_pixels_mut().for_each(|(x, y, pixel)| {
            *pixel = image::Luma([bitmap[y as usize][x as usize]]);
        });

        Self::Grayscale(img)
    }

    /// Export the image to a file
    ///
    /// # Errors
    /// Will return an error if the image cannot be saved
    pub fn export(&self, path: impl AsRef<Path>) -> Result<(), image::ImageError> {
        match self {
            Self::Grayscale(img) => img.save(path),
            Self::Rgb(img) => img.save(path),
        }
    }

    /// Convert the image to RGB
    pub fn convert_to_rgb(&mut self) {
        match self {
            Self::Grayscale(img) => {
                let mut new_img = image::RgbImage::new(img.width(), img.height());
                for (x, y, pixel) in img.enumerate_pixels() {
                    let gray = pixel.0[0];
                    new_img.put_pixel(x, y, image::Rgb([gray, gray, gray]));
                }
                *self = Self::Rgb(new_img);
            }
            Self::Rgb(_) => {}
        }
    }

    /// Scale the image by a factor
    pub fn scale(&mut self, factor: usize) {
        match self {
            Self::Grayscale(bitmap) => {
                let (width, height) = (bitmap.width() as usize, bitmap.height() as usize);
                let mut new_image =
                    GrayImage::new((width * factor) as u32, (height * factor) as u32);

                new_image
                    .par_enumerate_pixels_mut()
                    .for_each(|(x, y, pixel)| {
                        let old_x = x / factor as u32;
                        let old_y = y / factor as u32;
                        *pixel = *bitmap.get_pixel(old_x, old_y);
                    });

                *self = Self::Grayscale(new_image);
            }
            Self::Rgb(bitmap) => {
                let (width, height) = (bitmap.width() as usize, bitmap.height() as usize);
                let mut new_image =
                    RgbImage::new((width * factor) as u32, (height * factor) as u32);

                new_image
                    .par_enumerate_pixels_mut()
                    .for_each(|(x, y, pixel)| {
                        let old_x = x / factor as u32;
                        let old_y = y / factor as u32;
                        *pixel = *bitmap.get_pixel(old_x, old_y);
                    });

                *self = Self::Rgb(new_image);
            }
        }
    }

    /// 1px radius blur for antialiasing
    pub fn antialias(&mut self) {
        const KERNEL: &[f32] = &[0.0, -1.0, 0.0, -1.0, 5.0, -1.0, 0.0, -1.0, 0.0];
        match self {
            Self::Grayscale(img) => {
                *img = filter3x3(img, KERNEL);
            }
            Self::Rgb(img) => {
                *img = filter3x3(img, KERNEL);
            }
        }
    }

    /// Apply a filter to the image (sketchbook look)
    pub fn filter_sketch(&mut self, strength: f32, verbose: bool) {
        self.apply_filter(filters::sketch, strength, verbose);
    }

    /// Apply a filter to the image (space look)
    pub fn filter_space(&mut self, strength: f32, verbose: bool) {
        self.apply_filter(filters::space, strength, verbose);
    }

    /// Apply a filter to the image (granite look)
    pub fn filter_granite(&mut self, strength: f32, verbose: bool) {
        self.apply_filter(filters::granite, strength, verbose);
    }

    /// Apply a custom filter to the image
    pub fn apply_filter<F: Fn(&mut RgbImage, f32, bool)>(
        &mut self,
        f: F,
        strength: f32,
        verbose: bool,
    ) {
        self.convert_to_rgb();
        match self {
            Self::Rgb(img) => {
                f(img, strength, verbose);
            }
            Self::Grayscale(_) => unreachable!(),
        }
    }
}
