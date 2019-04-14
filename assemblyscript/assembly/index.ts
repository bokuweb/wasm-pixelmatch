function get(addr: u32, offset: u32): u8 {
  return load<u8>(addr + offset);
}

function set(addr: u32, offset: u32, value: u8): void {
  store<u8>(addr + offset, value);
}

export function pixelmatch(
  img1: u32,
  img2: u32,
  width: u32,
  height: u32
): u32 {
  let maxDelta = (35215 as f32) * 0.1 * 0.1;
  let diff = 0;
  for (let y: u32 = 0; y < height; y++) {
    for (let x: u32 = 0; x < width; x++) {
      let pos = (y * width + x) * 4;
      let delta = colorDelta(img1, img2, pos, pos, false);
      if (delta > maxDelta) {
        diff++;
        drawPixel(img1 + img2, pos, 255, 0, 0);
      } else {
        let val = grayPixel(pos, 0.1) as u32;
        drawPixel(img1 + img2, pos, val, val, val);
      }
    }
  }
  return diff;
}

function colorDelta(img1: u32, img2: u32, k: u32, m: u32, yOnly: bool): f32 {
  let r1 = get(k + 0, 0) as f32;
  let g1 = get(k + 1, 0) as f32;
  let b1 = get(k + 2, 0) as f32;
  let a1 = get(k + 3, 0) as f32;

  let r2 = get(m + 0, img1) as f32;
  let g2 = get(m + 1, img1) as f32;
  let b2 = get(m + 2, img1) as f32;
  let a2 = get(m + 3, img1) as f32;

  if (a1 === a2 && r1 === r2 && g1 === g2 && b1 === b2) return 0.0;

  if (a1 < 255) {
    a1 = (a1 as f32) / 255;
    r1 = blend(r1, a1);
    g1 = blend(g1, a1);
    b1 = blend(b1, a1);
  }

  if (a2 < 255) {
    a2 = (a2 as f32) / 255;
    r2 = blend(r2, a2);
    g2 = blend(g2, a2);
    b2 = blend(b2, a2);
  }

  let y = rgb2y(r1 as f32, g1, b1) - rgb2y(r2 as f32, g2, b2);

  if (yOnly) return y; // brightness difference only

  let i = rgb2i(r1 as f32, g1, b1) - rgb2i(r2 as f32, g2, b2);
  let q = rgb2q(r1 as f32, g1, b1) - rgb2q(r2 as f32, g2, b2);

  return 0.5053 * y * y + 0.299 * i * i + 0.1957 * q * q;
}

function blend(c: f32, a: f32): f32 {
  return 255.0 + (c - 255.0) * a;
}

function rgb2y(r: f32, g: f32, b: f32): f32 {
  return r * 0.29889531 + g * 0.58662247 + b * 0.11448223;
}

function rgb2i(r: f32, g: f32, b: f32): f32 {
  return r * 0.59597799 - g * 0.2741761 - b * 0.32180189;
}

function rgb2q(r: f32, g: f32, b: f32): f32 {
  return r * 0.21147017 - g * 0.52261711 + b * 0.31114694;
}

function drawPixel(offset: u32, pos: u32, r: u32, g: u32, b: u32): void {
  set(pos + 0, offset, r as u8);
  set(pos + 1, offset, g as u8);
  set(pos + 2, offset, b as u8);
  set(pos + 3, offset, 255);
}

function grayPixel(i: u32, alpha: f32): f32 {
  let r = get(i + 0, 0) as f32;
  let g = get(i + 1, 0) as f32;
  let b = get(i + 2, 0) as f32;
  let a = get(i + 3, 0) as f32;
  return blend(rgb2y(r, g, b), (alpha * a) / 255.0);
}
