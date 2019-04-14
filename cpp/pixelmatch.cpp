
#include <cstdint>
// #include <cstddef>
// #include <algorithm>
// #include <emscripten.h>

// blend semi-transparent color with white
uint8_t blend(int8_t c, float a)
{
    return uint8_t(255 + float(int32_t(c) - 255) * a);
}

float rgb2y(uint8_t r, uint8_t g, uint8_t b) { return r * 0.29889531 + g * 0.58662247 + b * 0.11448223; }
float rgb2i(uint8_t r, uint8_t g, uint8_t b) { return r * 0.59597799 - g * 0.27417610 - b * 0.32180189; }
float rgb2q(uint8_t r, uint8_t g, uint8_t b) { return r * 0.21147017 - g * 0.52261711 + b * 0.31114694; }

// calculate color difference according to the paper "Measuring perceived color difference
// using YIQ NTSC transmission color space in mobile applications" by Y. Kotsarenko and F. Ramos

float colorDelta(const uint8_t *img1, const uint8_t *img2, uint32_t k, uint32_t m, bool yOnly)
{

    uint8_t r1 = img1[k + 0];
    uint8_t g1 = img1[k + 1];
    uint8_t b1 = img1[k + 2];
    uint8_t a1 = img1[k + 3];

    uint8_t r2 = img2[m + 0];
    uint8_t g2 = img2[m + 1];
    uint8_t b2 = img2[m + 2];
    uint8_t a2 = img2[m + 3];

    if (a1 == a2 && r1 == r2 && g1 == g2 && b1 == b2)
        return 0;

    if (a1 < 255)
    {
        a1 = float(a1) / 255;
        r1 = blend(r1, a1);
        g1 = blend(g1, a1);
        b1 = blend(b1, a1);
    }

    if (a2 < 255)
    {
        a2 = float(a1) / 255;
        r2 = blend(r2, a2);
        g2 = blend(g2, a2);
        b2 = blend(b2, a2);
    }

    float y = rgb2y(r1, g1, b1) - rgb2y(r2, g2, b2);

    if (yOnly)
        return y; // brightness difference only

    float i = rgb2i(r1, g1, b1) - rgb2i(r2, g2, b2);
    float q = rgb2q(r1, g1, b1) - rgb2q(r2, g2, b2);

    return 0.5053 * y * y + 0.299 * i * i + 0.1957 * q * q;
}

void drawPixel(uint8_t *output, uint32_t pos, uint8_t r, uint8_t g, uint8_t b)
{
    output[pos + 0] = r;
    output[pos + 1] = g;
    output[pos + 2] = b;
    output[pos + 3] = 255;
}

// float grayPixel(const uint8_t *img, uint32_t i)
// {
//     float a = float(img[i + 3]) / 255;
//     uint8_t r = blend(img[i + 0], a);
//     uint8_t g = blend(img[i + 1], a);
//     uint8_t b = blend(img[i + 2], a);
//     return rgb2y(r, g, b);
// }

uint8_t grayPixel(const uint8_t *img, uint32_t i, float alpha)
{
    uint8_t r = img[i + 0];
    uint8_t g = img[i + 1];
    uint8_t b = img[i + 2];
    return blend(rgb2y(r, g, b), (alpha * float(img[i + 3])) / 255);
}

extern "C" uint32_t pixelmatch(const uint8_t *img1,
                               const uint8_t *img2,
                               uint32_t width,
                               uint32_t height,
                               uint8_t *output)
{
    float maxDelta = 35215 * 0.1 * 0.1;
    uint32_t diff = 0;

    for (uint32_t y = 0; y < height; y++)
    {
        for (uint32_t x = 0; x < width; x++)
        {
            uint32_t pos = (y * width + x) * 4;
            float delta = colorDelta(img1, img2, pos, pos, false);
            if (delta > maxDelta)
            {
                drawPixel(output, pos, 255, 0, 0);
                diff++;
            }
            else if (output)
            {
                uint8_t val = grayPixel(img1, pos, 0.1);
                drawPixel(output, pos, val, val, val);
            }
        }
    }
    return diff;
}


