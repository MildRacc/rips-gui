use std::{path::PathBuf, sync::{LazyLock, Mutex}};

use gtk4::glib;
use image::{DynamicImage, GenericImageView};
use rips_algorithms as algorithms;
use algorithms::color_utils::SortBy;

static WORKING_IMAGE: Mutex<LazyLock<DynamicImage>> = Mutex::new(std::sync::LazyLock::new(|| image::open("/home/sashad/Pictures/sorted_fucker.png").unwrap()));


#[derive(Clone, Copy, Debug)]
pub struct SortingConfig
{
    pub sort_selection: SortBy,
    pub upper: f32,
    pub lower: f32
}


pub fn change_working_image(path: PathBuf) -> Result<(), String>
{
    let new_image = match image::open(&path) 
    {
        Ok(img) => img,
        Err(_) => 
        {
            println!("Failed to open image at path: {:?}", path.to_str());
            return Err(String::from(""));
        }
    };

    if let Ok(mut working) = WORKING_IMAGE.lock()
    {
        **working = new_image;
    }
    else 
    {
        return Err(String::from("Working image mutex guard locked"));    
    }

    Ok(())
}


pub fn export(path: PathBuf) -> Result<(), String>
{

    if let Ok(working) = WORKING_IMAGE.lock()
    {
        let cloned = working.clone();
        println!("Cloned working image"); 
        drop(working);

        if cloned.save(path).is_err()
        {
            return Err(String::from("Failed to save image to specified path"));
        }
    }
    else 
    {
        return Err(String::from("Working Image mutex guard locked"));
    }

    Ok(())
}


pub fn sort<F>(condition: F, sort_by: SortBy) 
where 
    F: Fn(f32) -> bool + std::marker::Send + std::marker::Sync
{
    let mut working = WORKING_IMAGE.lock().unwrap();
    algorithms::sort_image(&mut working, sort_by, condition);
}



pub fn picture_from_working_image() -> gtk4::Picture
{
    let working = WORKING_IMAGE.lock().unwrap();

    let img = working.clone().to_rgba8();
    let (width, height) = working.dimensions();
    
    let bytes = glib::Bytes::from_owned(img.into_raw());

    let tex = gtk4::gdk::MemoryTexture::new(width as i32, height as i32, gtk4::gdk::MemoryFormat::R8g8b8a8, &bytes, (width*4) as usize);

    gtk4::Picture::for_paintable(&tex)
}



pub fn _pixbuf_from_working_image() -> gtk4::gdk_pixbuf::Pixbuf
{
    let working = WORKING_IMAGE.lock().unwrap();

    let raw_bytes = &working.clone().into_bytes();
    let image_bytes = gtk4::glib::Bytes::from(raw_bytes);

    let (width, height) = working.dimensions();

    gtk4::gdk_pixbuf::Pixbuf::from_bytes(&image_bytes, gtk4::gdk_pixbuf::Colorspace::Rgb, true, 8, width as i32, height as i32, (width * 4) as i32)
}



pub fn texture_from_working_image() -> gtk4::gdk::MemoryTexture
{
    let working = WORKING_IMAGE.lock().unwrap();

    let img = working.clone().to_rgba8();
    let (width, height) = img.dimensions();
    let bytes = glib::Bytes::from_owned(img.into_raw());
    gtk4::gdk::MemoryTexture::new(width as i32, height as i32, gtk4::gdk::MemoryFormat::R8g8b8a8, &bytes, (width * 4) as usize)
}
