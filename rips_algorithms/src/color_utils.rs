

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
    Alpha,
    Unknown,
}
impl From<u32> for SortBy 
{
    fn from(value: u32) -> Self {
        match value 
        {
            0 => Self::Red,
            1 => Self::Green,
            2 => Self::Blue,
            3 => Self::Hue,
            4 => Self::Chroma,
            5 => Self::Saturation,
            6 => Self::Lightness,
            7 => Self::Luminance,
            8 => Self::Value,
            9 => Self::Alpha,
            _ => Self::Unknown,
        } 
    }
}
impl From<&str> for SortBy 
{
    fn from(value: &str) -> Self {
        match value 
        {
            "Red" => Self::Red,
            "Green" => Self::Green,
            "Blue" => Self::Blue,
            "Hue" => Self::Hue,
            "Chroma" => Self::Chroma,
            "Saturation" => Self::Saturation,
            "Lightness" => Self::Lightness,
            "Luminance" => Self::Luminance,    
            "Value" => Self::Value,
            "Alpha" => Self::Alpha,    
            _ => Self::Unknown
        } 
    }
}
