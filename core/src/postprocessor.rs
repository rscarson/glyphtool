//! Postprocessor module for image processing
use image::{GrayImage, RgbImage};
use rayon::{
    iter::{IndexedParallelIterator, IntoParallelRefMutIterator, ParallelIterator},
    slice::ParallelSliceMut,
};
use std::path::Path;

mod filters;
mod utilities;

/// Trait for converting a raw bitmap array to a usable image format
pub trait ImageExt {
    /// Create a grayscale image from the bitmap
    fn to_grayscale(&self) -> OutputImage;

    /// Scale the image by a factor
    fn scale(&mut self, factor: usize);
}
impl ImageExt for Vec<Vec<u8>> {
    fn to_grayscale(&self) -> OutputImage {
        OutputImage::new_grayscale(self)
    }

    fn scale(&mut self, factor: usize) {
        if factor <= 1 {
            return;
        }

        for row in self.iter() {
            for px in row {
                let c = if *px < 128 { "█" } else { " " };
                print!("{c}");
            }

            println!();
        }

        let width = self.first().map_or(0, Vec::len);
        let height = self.len();
        let mut new_image = vec![vec![0; width * factor]; height * factor];

        new_image
            .par_iter_mut()
            .enumerate()
            .for_each(|(new_y, row)| {
                let old_y = new_y / factor;
                if old_y < height {
                    for (new_x, item) in row.iter_mut().enumerate().take(width * factor) {
                        let old_x = new_x / factor;
                        *item = self[old_y][old_x];
                    }
                }
            });

        *self = new_image;
    }
}
impl ImageExt for OutputImage {
    fn to_grayscale(&self) -> OutputImage {
        match self {
            Self::Grayscale(_) => self.clone(),
            Self::Rgb(img) => {
                let mut new_img = image::GrayImage::new(img.width(), img.height());
                new_img.par_chunks_mut(3).for_each(|px| {
                    let r = f32::from(px[0]);
                    let g = f32::from(px[1]);
                    let b = f32::from(px[2]);

                    let gray = (r + g + b) / 3.0;
                    px[0] = gray as u8;
                    px[1] = gray as u8;
                    px[2] = gray as u8;
                });

                OutputImage::Grayscale(new_img)
            }
        }
    }

    fn scale(&mut self, factor: usize) {
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
            }
        }
    }
}

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

    /// Create a new grayscale image from a bitmap
    #[must_use]
    pub fn new_grayscale(bitmap: &[Vec<u8>]) -> Self {
        let height = bitmap.len();
        let width = bitmap.first().map_or(0, Vec::len);

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

    /// Apply a filter to the image (sketchbook look)
    pub fn filter_sketch(&mut self, strength: f32) {
        self.apply_filter(filters::sketch, strength);
    }

    /// Apply a filter to the image (space look)
    pub fn filter_space(&mut self, strength: f32) {
        self.apply_filter(filters::space, strength);
    }

    /// Apply a filter to the image (granite look)
    pub fn filter_granite(&mut self, strength: f32) {
        self.apply_filter(filters::granite, strength);
    }

    /// Apply a custom filter to the image
    pub fn apply_filter<F: Fn(&mut RgbImage, f32)>(&mut self, f: F, strength: f32) {
        self.convert_to_rgb();
        match self {
            Self::Rgb(img) => {
                f(img, strength);
            }
            Self::Grayscale(_) => unreachable!(),
        }
    }
}
