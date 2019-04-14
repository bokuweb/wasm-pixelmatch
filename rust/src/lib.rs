#![no_std]

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern fn pixelmatch(img1: *mut u8, img2: *mut u8, width: u32, height: u32, output: *mut u8) -> u32 {
    let buf1: &mut [u8] =
        unsafe { core::slice::from_raw_parts_mut(img1, (width * height * 4) as usize) };
    let buf2: &mut [u8] =
        unsafe { core::slice::from_raw_parts_mut(img2, (width * height * 4) as usize) };
    let out: &mut [u8] =
        unsafe { core::slice::from_raw_parts_mut(output, (width * height * 4) as usize) };
    let max_delta = 35215.0 * 0.1 * 0.1;
    let mut diff_count = 0;
    for y in 0..height {
        for x in 0..width {
            let pos = ((y * width + x) * 4) as u32;
            let delta = color_delta(buf1, buf2, pos, pos, false);
            if delta > max_delta {
                draw_pixel(out, pos, 255, 0, 0, 255);
                diff_count += 1;
            } else {
                let y = gray_pixel(buf1, pos, 0.1);
                draw_pixel(out, pos, y, y, y, 255);
            }
        }
    }
    diff_count
}

fn draw_pixel(output: &mut [u8], pos: u32, r: u8, g: u8, b: u8, a: u8) {
    output[pos as usize + 0] = r;
    output[pos as usize + 1] = g;
    output[pos as usize + 2] = b;
    output[pos as usize + 3] = a;
}

fn gray_pixel(img: &[u8], i: u32, alpha: f32) -> u8 {
    let r = img[i as usize + 0];
    let g = img[i as usize + 1];
    let b = img[i as usize + 2];
    blend(rgb2y(r, g, b) as u8, ((alpha * img[i as usize + 3] as f32) / 255.0))
}

fn color_delta(img1: &[u8], img2: &[u8], pos1: u32, pos2: u32, only_brightness: bool) -> f32 {
    let mut r1 = img1[pos1 as usize + 0];
    let mut g1 = img1[pos1 as usize + 1];
    let mut b1 = img1[pos1 as usize + 2];
    let a1 = img1[pos1 as usize + 3];

    let mut r2 = img2[pos2 as usize + 0];
    let mut g2 = img2[pos2 as usize + 1];
    let mut b2 = img2[pos2 as usize + 2];
    let a2 = img2[pos2 as usize + 3];

    if a1 == a2 && r1 == r2 && g1 == g2 && b1 == b2 {
        return 0.0;
    }

    if a1 < 255
    {
        let a1 = (a1 as f32) / 255.0;
        r1 = blend(r1, a1);
        g1 = blend(g1, a1);
        b1 = blend(b1, a1);
    }

    if a2 < 255
    {
        let a2 = (a2 as f32) / 255.0;
        r2 = blend(r2, a2);
        g2 = blend(g2, a2);
        b2 = blend(b2, a2);
    }

    let y = rgb2y(r1, g1, b1) - rgb2y(r2, g2, b2);

    if only_brightness {
        return y;
    }

    let i = rgb2i(r1, g1, b1) - rgb2i(r2, g2, b2);
    let q = rgb2q(r1, g1, b1) - rgb2q(r2, g2, b2);

    0.5053 * y * y + 0.299 * i * i + 0.1957 * q * q
}

fn blend(c: u8, a: f32) -> u8 {
    (255.0 + ((c as i32 - 255) as f32) * a) as u8
}

fn rgb2y(r: u8, g: u8, b: u8) -> f32 {
    r as f32 * 0.29889531 + g as f32 * 0.58662247 + b as f32 * 0.11448223
}
fn rgb2i(r: u8, g: u8, b: u8) -> f32 {
    r as f32 * 0.59597799 - g as f32 * 0.27417610 - b as f32 * 0.32180189
}
fn rgb2q(r: u8, g: u8, b: u8) -> f32 {
    r as f32 * 0.21147017 - g as f32 * 0.52261711 + b as f32 * 0.31114694
}
