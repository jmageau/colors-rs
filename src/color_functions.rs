#[derive(Clone, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    lab: ColorLab
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Color {
            r: r,
            g: g,
            b: b,
            lab: rgb_to_lab(r, g, b)
        }
    }
}

#[derive(Clone, PartialEq)]
struct ColorLab {
    l: f64,
    a: f64,
    b: f64
}


pub fn color_distance(color1: &Color, color2: &Color) -> u32 {
    ((color2.r as i32 - color1.r as i32).pow(2) + (color2.g as i32 - color1.g as i32).pow(2) + (color2.b as i32 - color1.b as i32).pow(2)) as u32
    // (delta_e(color1, color2) * 10f64).floor() as u32
}

// http://www.ece.rochester.edu/~gsharma/ciede2000/ciede2000noteCRNA.pdf
pub fn delta_e(color1: &Color, color2: &Color) -> f64 {
    let k_l = 1f64;
    let k_c = 1f64;
    let k_h = 1f64;

    // 2
    let c1_ab = (color1.lab.a.powi(2) + color1.lab.b.powi(2)).sqrt();
    let c2_ab = (color2.lab.a.powi(2) + color2.lab.b.powi(2)).sqrt();

    // 3
    let c_ab = (c1_ab + c2_ab) / 2f64;

    // 4
    let g = 0.5f64 * (1f64 - (c_ab.powi(7) / (c_ab.powi(7) + 25f64.powi(7))).sqrt());

    // 5
    let a1_p = (1f64 + g) * color1.lab.a;
    let a2_p = (1f64 + g) * color2.lab.a;

    // 6
    let c1_p = (a1_p.powi(2) + color1.lab.b.powi(2)).sqrt();
    let c2_p = (a2_p.powi(2) + color2.lab.b.powi(2)).sqrt();

    // 7 TODO: check if needs to +360
    let h1_p = color1.lab.b.atan2(a1_p).to_degrees();
    let h2_p = color2.lab.b.atan2(a2_p).to_degrees();

    // 8
    let delta_l_p = color2.lab.l - color1.lab.l;

    // 9
    let delta_c_p = c2_p - c1_p;

    // 10
    let delta_h_p_temp = if c1_p * c2_p == 0f64 {
        0f64
    } else if (h2_p - h1_p).abs() <= 180f64 {
        h2_p - h1_p
    } else if h2_p - h1_p > 180f64 {
        h2_p - h1_p - 360f64
    } else if h2_p - h1_p < -180f64 {
        h2_p - h1_p + 360f64
    } else {
        panic!("");
    };

    // 11
    let delta_h_p = 2f64 * (c1_p * c2_p).sqrt() * (delta_h_p_temp.to_radians() / 2f64).sin();

    // 12
    let l_p = (color1.lab.l + color2.lab.l) / 2f64;

    // 13
    let c_p = (c1_p + c2_p) / 2f64;

    // 14
    let h_p = if c1_p * c2_p == 0f64 {
        h1_p + h2_p
    } else if (h1_p - h2_p).abs() <= 180f64 {
        (h1_p + h2_p) / 2f64
    } else if (h1_p - h2_p).abs() > 180f64 && h1_p + h2_p < 360f64 {
        (h1_p + h2_p + 360f64) / 2f64
    } else if (h1_p - h2_p).abs() > 180f64 && h1_p + h2_p >= 360f64 {
        (h1_p + h2_p - 360f64) / 2f64
    } else {
        panic!("");
    };

    // 15
    let t = 1f64 - 0.17f64 * (h_p - 30f64).to_radians().cos() + 0.24f64 * (2f64 * h_p).to_radians().cos() +
        0.32f64 * (3f64 * h_p + 6f64).to_radians().cos() - 0.2f64 * (4f64 * h_p - 63f64).to_radians().cos();

    // 16
    let delta_ro = 30f64 * (-((h_p - 275f64) / 25f64).powi(2)).exp();

    // 17
    let r_c = 2f64 * (c_p.powi(7) / (c_p.powi(7) + 25f64.powi(7))).sqrt();

    // 18
    let s_l = 1f64 + 0.015f64 * (l_p - 50f64).powi(2) / (20f64 + (l_p - 50f64).powi(2)).sqrt();

    // 19
    let s_c = 1f64 + 0.045f64 * c_p;

    // 20
    let s_h = 1f64 + 0.015f64 * c_p * t;

    // 21
    let r_t = -(2f64 * delta_ro).to_radians().sin() * r_c;

    // 22
    ((delta_l_p / (k_l * s_l)).powi(2) + (delta_c_p / (k_c * s_c)).powi(2) + (delta_h_p / (k_h * s_h)).powi(2) +
        r_t * (delta_c_p / (k_c * s_c)) * (delta_h_p / (k_h * s_h))).sqrt()
}

fn rgb_to_lab(r: u8, g: u8, b: u8) -> ColorLab {
    let mut r = r as f64 / 255f64;
    let mut g = g as f64 / 255f64;
    let mut b = b as f64 / 255f64;

    if r > 0.04045f64 {
        r = ((r + 0.055f64) / 1.055f64).powf(2.4f64)
    } else {
        r = r / 12.92f64;
    }
    if g > 0.04045f64 {
        g = ((g + 0.055f64) / 1.055f64).powf(2.4f64)
    } else {
        g = g / 12.92f64;
    }
    if b > 0.04045f64 {
        b = ((b + 0.055f64) / 1.055f64).powf(2.4f64)
    } else {
        b = b / 12.92f64;
    }
    r *= 100f64;
    g *= 100f64;
    b *= 100f64;

    // Observer. = 2°, Illuminant = D65
    let mut x = r * 0.4124f64 + g * 0.3576f64 + b * 0.1805f64 / 95.047f64;
    let mut y = r * 0.2126f64 + g * 0.7152f64 + b * 0.0722f64 / 100f64;
    let mut z = r * 0.0193f64 + g * 0.1192f64 + b * 0.9505f64 / 108.883f64;

    if x > 0.008856f64 {
        x = x.powf(1f64 / 3f64);
    } else {
        x = x * 7.787f64 + 16f64 / 116f64;
    }
    if y > 0.008856f64 {
        y = y.powf(1f64 / 3f64);
    } else {
        y = y * 7.787f64 + 16f64 / 116f64;
    }
    if z > 0.008856f64 {
        z = z.powf(1f64 / 3f64);
    } else {
        z = z * 7.787f64 + 16f64 / 116f64;
    }

    let l = y * 116f64 - 16f64;
    let a = (x - y) * 500f64;
    let b = (y - z) * 200f64;

    ColorLab { l: l, a: a, b: b }
}
