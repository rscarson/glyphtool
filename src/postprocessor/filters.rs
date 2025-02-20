use image::imageops::fast_blur;
use image::RgbImage;
use rayon::{
    iter::{IndexedParallelIterator, ParallelIterator},
    slice::{ParallelSlice, ParallelSliceMut},
};

use super::utilities::{darken, highpass_filter, perlin_noise, random_noise, sepia, unsharp_mask};

pub fn sketch(image: &mut RgbImage, strength: f32) {
    // Step 1: Apply blur, and a sepia filter
    println!("Step 1... Applying subtle blur and sepia");
    let mut blurred = fast_blur(image, strength);
    sepia(&mut blurred, strength);

    // Step 2: Unsharp masking
    println!("Step 2... Applying unsharp masking");
    unsharp_mask(image, &blurred, strength);

    // Step 3: Apply perlin noise, to simulate paper texture
    println!("Step 3... Applying perlin noise");
    perlin_noise(image, strength);

    // Step 4: Apply a final blur to smooth out the noise
    println!("Step 4... Applying final blur");
    *image = fast_blur(image, 0.25 * strength);
}

pub fn space(image: &mut RgbImage, strength: f32) {
    // Step 1: Introduce random noise to simulate stars
    random_noise(image, false, 10, 0.3 * strength);
    random_noise(image, false, 50, 0.15 * strength);
    random_noise(image, false, 100, 0.01 * strength);

    // Step 2: Apply subtle blur and sepia
    let mut blurred = fast_blur(image, 1.5 * strength); // Adjust the radius as needed
    sepia(&mut blurred, strength);

    // Step 3: Highpass filter to enhance edges
    highpass_filter(image, &blurred, strength);

    // Step 4: Unsharp masking to highlight glyphs
    unsharp_mask(image, &blurred, strength);
}

pub fn granite(image: &mut RgbImage, strength: f32) {
    // Step 0: Create a mask of the original image
    println!("Step 0... Creating mask of original image");
    let mask = fast_blur(image, 0.5 * strength);
    darken(image, 0.3);

    // Step 1: Apply a filter to enhance edges
    println!("Step 1... Applying edge filter");
    let blurred = fast_blur(image, 2.0 * strength);
    unsharp_mask(image, &blurred, strength);

    // Step 2: Apply random noise to simulate granite texture
    println!("Step 2... Applying random noise");
    random_noise(image, true, 25, 0.01 * strength);
    random_noise(image, true, 50, 0.005 * strength);
    random_noise(image, true, 75, 0.001 * strength);

    // Step 3: Apply perlin noise to add a subtle grain
    println!("Step 3... Applying perlin noise");
    perlin_noise(image, 1.5 * strength);

    // Step 4, Add the inverted original image to the final image
    println!("Step 4... Adding inverted original image");
    image
        .par_chunks_mut(3)
        .zip(mask.par_chunks(3))
        .for_each(|(px, ox)| {
            let origin_lum = (ox[0] as f32 + ox[1] as f32 + ox[2] as f32) / 3.0 / 255.0;
            let lum = (1.0 - origin_lum) * 0.5 * strength * 255.0;

            let r = (px[0] as f32 + lum).clamp(0.0, 255.0);
            let g = (px[1] as f32 + lum).clamp(0.0, 255.0);
            let b = (px[2] as f32 + lum).clamp(0.0, 255.0);

            px[0] = r as u8;
            px[1] = g as u8;
            px[2] = b as u8;
        });
}
