use std::sync::{LazyLock, Mutex};

use gtk4::glib;
use image::{DynamicImage, GenericImageView, Pixel};

static WORKING_IMAGE: Mutex<LazyLock<DynamicImage>> = Mutex::new(std::sync::LazyLock::new(|| image::open("/home/sashad/Pictures/art/StoryTellingCollage.png").unwrap()));


#[derive(Clone, Copy, Debug)]
pub struct SortingConfig
{
    pub sort_selection: SortBy,
    pub upper: f32,
    pub lower: f32
}


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SortBy
{
    Red,
    Green,
    Blue,
    Hue,
    Chroma,
    Saturation,
    Lightness,
    Luminance,
    Value,
    Alpha
}


pub fn sort<F>(condition: F) 
where 
    F: Fn((f32, f32, f32)) -> bool
{
    let mut working = WORKING_IMAGE.lock().unwrap();
    pixel_sorting::sort_image(working.as_mut_rgba8().unwrap(), condition);
}


pub fn check_pixel(pixel: <DynamicImage as GenericImageView>::Pixel, conf: &SortingConfig) -> bool
{

    let sortby = conf.sort_selection;
    let min = conf.lower;
    let max = conf.upper;

    match sortby
    {
        SortBy::Red | SortBy::Green | SortBy::Blue => {
        },
        SortBy::Hue | SortBy::Saturation | SortBy::Lightness => {
        },
        SortBy::Value => {
        },
        SortBy::Chroma => {
        },
        SortBy::Luminance => {
        },
        SortBy::Alpha => {
        }
        _ => {}
    }
    
    true
}

pub fn picture_from_working_image() -> gtk4::Picture
{
    let working = unsafe { WORKING_IMAGE.lock().unwrap() };

    let img = working.clone().to_rgba8();
    let (width, height) = working.dimensions();
    
    let bytes = glib::Bytes::from_owned(img.into_raw());

    let tex = gtk4::gdk::MemoryTexture::new(width as i32, height as i32, gtk4::gdk::MemoryFormat::R8g8b8a8, &bytes, (width*4) as usize);

    gtk4::Picture::for_paintable(&tex)
}

pub fn pixbuf_from_working_image() -> gtk4::gdk_pixbuf::Pixbuf
{
    let working = unsafe { WORKING_IMAGE.lock().unwrap() };

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
