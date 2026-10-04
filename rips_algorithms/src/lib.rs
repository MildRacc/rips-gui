#![feature(portable_simd)]

#[cfg(feature = "image")]
use image::{DynamicImage, GenericImageView};
#[cfg(feature = "image")]
use rayon::{iter::ParallelIterator, slice::ParallelSliceMut};
use crate::color_utils::SortBy;
#[cfg(feature = "simd")]
use std::simd;

pub mod color_utils;
pub mod sort;


fn get_sort_idx(sort: SortBy) -> usize
{


    match sort
    {
        SortBy::Red => 0,
        SortBy::Green => 1,
        SortBy::Blue => 2,
        SortBy::Hue => 0,
        SortBy::Saturation => 1,
        SortBy::Lightness => 2,
        SortBy::Value => 2,
        SortBy::Alpha => 3,
        _ => 0
    }
}



#[cfg(feature = "image")]
pub fn sort_image<F>(img: &mut DynamicImage, sort: SortBy, condition: F) -> Result<(), String>
where F: Fn(f32) -> bool + std::marker::Sync + std::marker::Send
{
    let sort_idx = get_sort_idx(sort);

    match sort
    {
        SortBy::Red | SortBy::Green | SortBy::Blue => sort::image::sort_image_rgba8(img, sort_idx, condition),

        _ => Err("Unknown sort type".to_owned())
    }
}



#[cfg(all(feature = "image", feature = "simd"))]
pub fn sort_image_simd<F>(img: &mut DynamicImage, sort: SortBy, condition: F) -> Result<(), String>
where F: Fn(simd::f32x8) -> simd::Mask<i32, 8> + std::marker::Sync + std::marker::Send
{
    let sort_idx = get_sort_idx(sort);

    match sort
    {
        SortBy::Red | SortBy::Green | SortBy::Blue => sort::simd::sort_image_rgba8_simd(img, sort_idx, condition),

        _ => Err("Unknown sort type".to_owned())
    }
}




