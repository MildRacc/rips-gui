use image::{GenericImageView, DynamicImage};
use rayon::{iter::ParallelIterator, slice::ParallelSliceMut};
use std::marker::{Send, Sync};
use std::simd;
use std::simd::num::SimdUint;


const LANES8: usize = 8;


pub fn sort_image_rgba8_simd<F>(img: &mut DynamicImage, sort_idx: usize, condition: F) -> Result<(), String>
where F: Fn(simd::f32x8) -> simd::Mask<i32, 8> + Sync + Send 
{
    
    assert!(sort_idx < 4);
    let (width, _) = img.dimensions();
    let width = width as usize;
    let rgba = img.as_mut_rgba8().ok_or("Image is not RGBA")?;
    let shift = (sort_idx * 8) as u32;

    let par_it = rgba.par_chunks_mut(4 * width as usize);
    par_it.for_each_init(
        || { vec![0u64; width.div_ceil(64)] },
        move |bits, row|
        {
            bits.fill(0);
            let px: &mut [[u8; 4]] = bytemuck::cast_slice_mut(row);

            for (i, chunk) in px.chunks(LANES8).enumerate()
            {
                let mut buf = [[0u8; 4]; LANES8];    
                buf[..chunk.len()].copy_from_slice(chunk);

                let raw = simd::u32x8::from_array(bytemuck::cast(buf));

                let chan = (raw >> simd::u32x8::splat(shift)) & simd::u32x8::splat(0xFF);
                let vals = chan.cast::<f32>() * simd::f32x8::splat(1.0 / 255.0);

                let mut m = condition(vals).to_bitmask();
                if chunk.len() < LANES8
                {
                    m &= (1u64) << chunk.len() - 1;
                }
                let bit = i * LANES8;
                bits[bit / 64] |= m << (bit % 64);
            }

            let mut x = 0;
            while x < width
            {
                let start = find_bit(bits, x, true, width);
                if start > width { break; }
                let end = find_bit(bits, start, false, width);
                px[start..end].sort_by_key(|p| p[sort_idx]);
                x = end;
            }
            
        }
    );

    Ok(())
}


fn find_bit(bits: &[u64], from: usize, set: bool, width: usize) -> usize
{
    let (mut w, mut off) = (from / 64, from % 64);

    while w < bits.len()
    {
        let word = if set {bits[w]} else {!bits[w]} & (!0u64 << off);
        if word != 0
        {
            return (w * 64 + word.trailing_zeros() as usize).min(width);
        }
        w+=1;
        off = 0;
    }

    width
}
