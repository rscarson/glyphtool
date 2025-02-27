//! Utility functions for the renderer module

/// Inserts the source image into the destination image at the given position
///
/// # Panics
/// Panics if the source image does not fit in the destination image at the given position
pub fn insert_into_bitmap(dst: &mut [Vec<u8>], src: &[Vec<u8>], pos: (u16, u16)) {
    // Sanity check - ensure the source fits in the destination at the given position
    let (src_w, src_h) = (src[0].len() as u16, src.len() as u16);
    let (dst_w, dst_h) = (dst[0].len() as u16, dst.len() as u16);
    assert!(
        !(pos.0 + src_w > dst_w || pos.1 + src_h > dst_h),
        "Image of size {src_w}x{src_h} at position {pos:?} does not fit in destination of size {dst_w}x{dst_h}"
    );

    for (y, row) in src.iter().enumerate() {
        for (x, pixel) in row.iter().enumerate() {
            let x = pos.0 as usize + x;
            let y = pos.1 as usize + y;

            dst[y][x] = *pixel;
        }
    }
}
