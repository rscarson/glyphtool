use image::RgbImage;
use noise::{NoiseFn, Perlin};
use rayon::{
    iter::{IndexedParallelIterator, ParallelIterator},
    slice::{ParallelSlice, ParallelSliceMut},
};

pub fn sepia(image: &mut RgbImage, strength: f32) {
    let mut factors = [
        0.393, 0.769, 0.189, 0.349, 0.686, 0.168, 0.272, 0.534, 0.131,
    ];
    factors.iter_mut().for_each(|f| *f *= strength);

    image.par_chunks_mut(3).for_each(|px| {
        let (r, g, b) = (f32::from(px[0]), f32::from(px[1]), f32::from(px[2]));

        let r2 = r * factors[0] + r * factors[1] + r * factors[2];
        let g2 = g * factors[3] + g * factors[4] + g * factors[5];
        let b2 = b * factors[6] + b * factors[7] + b * factors[8];

        px[0] = r2.min(255.0) as u8;
        px[1] = g2.min(255.0) as u8;
        px[2] = b2.min(255.0) as u8;
    });
}

pub fn perlin_noise(image: &mut RgbImage, strength: f32) {
    let perlin = Perlin::new(42);
    let strength = 200.0 * f64::from(strength);

    let (width, height) = (f64::from(image.width()), f64::from(image.height()));

    image.enumerate_pixels_mut().for_each(|(x, y, px)| {
        let nx = (f64::from(x) / width) * strength;
        let ny = (f64::from(y) / height) * strength;
        let value = perlin.get([nx, ny]);
        let normalized = (((value + 1.0) / 2.0) * 255.0).clamp(0.0, 255.0);

        // Reduce range to avoid overpowering
        let noise_factor = (normalized / 255.0) * 0.05;

        // Uniform darkening
        let brightness = (u16::from(px[0]) + u16::from(px[1]) + u16::from(px[2])) / 3;
        let darkening = (f64::from(brightness) * noise_factor) as u8;

        px[0] = px[0].saturating_sub(darkening);
        px[1] = px[1].saturating_sub(darkening);
        px[2] = px[2].saturating_sub(darkening);
    });
}

pub fn unsharp_mask(image: &mut RgbImage, blurred: &RgbImage, strength: f32) {
    image
        .par_chunks_exact_mut(3)
        .zip(blurred.par_chunks_exact(3))
        .for_each(|(px, bpx)| {
            let (r, g, b) = (i16::from(px[0]), i16::from(px[1]), i16::from(px[2]));
            let (br, bg, bb) = (i16::from(bpx[0]), i16::from(bpx[1]), i16::from(bpx[2]));

            let (mr, mg, mb) = ((r - br).abs(), (g - bg).abs(), (b - bb).abs());

            // Scale the difference to enhance the effect
            let (mr, mg, mb) = (
                f32::from(mr) * 1.5 * strength,
                f32::from(mg) * 1.5 * strength,
                f32::from(mb) * 1.5 * strength,
            );

            let (r, g, b) = (f32::from(r) - mr, f32::from(g) - mg, f32::from(b) - mb);

            px[0] = r.clamp(0.0, 255.0) as u8;
            px[1] = g.clamp(0.0, 255.0) as u8;
            px[2] = b.clamp(0.0, 255.0) as u8;
        });
}

pub fn highpass_filter(image: &mut RgbImage, blurred: &RgbImage, strength: f32) {
    image
        .par_chunks_mut(3)
        .zip(blurred.par_chunks(3))
        .for_each(|(px, bpx)| {
            let (r, g, b) = (i16::from(px[0]), i16::from(px[1]), i16::from(px[2]));
            let (br, bg, bb) = (i16::from(bpx[0]), i16::from(bpx[1]), i16::from(bpx[2]));

            let rdif = (r - br).abs();
            let gdif = (g - bg).abs();
            let bdif = (b - bb).abs();

            px[0] = (f32::from(rdif) * strength).clamp(0.0, 255.0) as u8;
            px[1] = (f32::from(gdif) * strength).clamp(0.0, 255.0) as u8;
            px[2] = (f32::from(bdif) * strength).clamp(0.0, 255.0) as u8;
        });
}

pub fn random_noise(
    image: &mut RgbImage,
    single_channel_noise: bool,
    range: i16,
    probability: f32,
) {
    image.par_chunks_mut(3).for_each(|px| {
        if rand::random::<f32>() > probability {
            return;
        }

        let (r, g, b) = if single_channel_noise {
            let noise = rand::random::<i16>().clamp(-range, range);
            (noise, noise, noise)
        } else {
            (
                rand::random::<i16>().clamp(-range, range),
                rand::random::<i16>().clamp(-range, range),
                rand::random::<i16>().clamp(-range, range),
            )
        };

        let r = i16::from(px[0]) + r;
        let g = i16::from(px[1]) + g;
        let b = i16::from(px[2]) + b;

        px[0] = r.clamp(0, 255) as u8;
        px[1] = g.clamp(0, 255) as u8;
        px[2] = b.clamp(0, 255) as u8;
    });
}

pub fn darken(image: &mut RgbImage, factor: f32) {
    image.par_chunks_mut(3).for_each(|px| {
        let (r, g, b) = (f32::from(px[0]), f32::from(px[1]), f32::from(px[2]));

        px[0] = (r * factor).clamp(0.0, 255.0) as u8;
        px[1] = (g * factor).clamp(0.0, 255.0) as u8;
        px[2] = (b * factor).clamp(0.0, 255.0) as u8;
    });
}
