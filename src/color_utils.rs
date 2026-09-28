
pub fn rgb2hsl(r: f32, g: f32, b: f32) -> (f32, f32, f32)
{
    // Make r, g, and b fractions of 1
    let r = r / 255.0;
    let g = g / 255.0;
    let b = b / 255.0;

    // Find greatest and smallest channel values
    let cmin = r.min(g).min(b);
    let cmax = r.max(g).max(b);
    let delta = cmax - cmin;
    let mut h;
    let mut s;
    let mut l;

  // Calculate hue
  // No difference
  if delta == 0.0 { h = 0.0 }
  // Red is max
  else if cmax == r { h = ((g - b) / delta) % 6.0 }
  // Green is max
  else if cmax == g { h = (b - r) / delta + 2.0 }
  // Blue is max
  else { h = (r - g) / delta + 4.0 }

  h = (h * 60.0).round();
    
  // Make negative hues positive behind 360°
  if h < 0.0 { h += 360.0 };

  // Calculate lightness
  l = (cmax + cmin) / 2.0;

  // Calculate saturation
  s = if delta == 0.0 { 0.0 } else { delta / (1.0 - (2.0 * l - 1.0).abs()) };
    
  // Multiply l and s by 100
  s = s * 100.0;
  l = l * 100.0;

  (h, s, l)
}
