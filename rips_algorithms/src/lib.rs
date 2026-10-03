#[cfg(feature = "image")]
use image::{DynamicImage, GenericImageView};
#[cfg(feature = "image")]
use rayon::{iter::ParallelIterator, slice::ParallelSliceMut};
use crate::color_utils::SortBy;

pub mod color_utils;



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
        SortBy::Red | SortBy::Green | SortBy::Blue => sort_image_rgba8(img, sort_idx, condition),

        _ => Err("Unknown sort type".to_owned())
    }
}




#[cfg(feature = "image")]
pub fn sort_image_rgba8<F>(img: &mut DynamicImage, sort_idx: usize, condition: F) -> Result<(), String>
where F: Fn(f32) -> bool + std::marker::Sync + std::marker::Send
{
    
    let (width, _) = img.dimensions();
    let width = width as usize;
    
    if let Some(rgba) = img.as_mut_rgba8()
    {
        let par_it = rgba.par_chunks_mut(4 * width as usize);
        par_it.for_each( move |row|
        {
                
            let mut is_grouping = false;
            let mut group_start = 0usize;
            let mut sort_group: Vec<[u8; 4]> = Vec::new();
        

            for x in 0..width
            {

                let value = row[x * 4 + sort_idx] as f32 / 255.0;
                let matches = condition(value);

                if matches && !is_grouping
                {
                    is_grouping = true;
                    group_start = x;
                    sort_group.clear();
                }

                if is_grouping && matches
                {
                    sort_group.push([
                        row[x * 4], // R
                        row[x * 4 + 1], // G
                        row[x * 4 + 2], // B
                        row[x * 4 + 3] // A
                    ]);
                }

                let at_row_end = x == width-1;
                if is_grouping && (!matches || at_row_end)
                {
                    is_grouping = false;
                    sort_group.sort_by_key(|p| p[sort_idx]);

                    for (i, pix) in sort_group.iter().enumerate()
                    {
                        let target = (group_start + i) * 4;
                        row[target..target+4].copy_from_slice(pix);
                    }

                }
            }
        });
    }
    else
    {
        return Err("Image is not RGBA8".to_owned());
    }

    Ok(())
}
