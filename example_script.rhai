/*
Functions:

width() -> get width as number
height() -> get height as number

get_pixel(x: number, y: number) -> get the rgba pixel as array

pixel(r: number, g: number, b: number, a: number) -> create a rgba pixel
pixel_raw(values: array) -> transform number array to pixel

Types:

Rgba {
    r: number,
    g: number,
    b: number,
    a: number
}

*/

const WIDTH = width().to_float() - 1.0;
const HEIGHT = height().to_float() - 1.0;

fn min(x,y) {
    if (x < y) {
        x
    } else {
        y
    }
}

fn max(x,y) {
    if (x < y) {
        y
    } else {
        x
    }
}

fn constrain(value, min, max) {
    max(min(value, max), min)
}

// Return the pixel for the given x,y coordinates
fn width_height_gradient(x, y) {
    let r = constrain(((x.to_float() / global::WIDTH) * 255.0).to_int(), 0, 255);
    let g = constrain(((y.to_float() / global::HEIGHT) * 255.0).to_int(), 0, 255);

    pixel(r, g, 0, 255)
}

fn grayscale(x, y) {
    const MULTIPLIER_RED   = 0.299;
    const MULTIPLIER_GREEN = 0.587;
    const MULTIPLIER_BLUE  = 0.114;

    let pixel = pixel_raw(get_pixel(x, y));

    let r = pixel.r.to_float() * MULTIPLIER_RED;
    let g = pixel.g.to_float() * MULTIPLIER_GREEN;
    let b = pixel.b.to_float() * MULTIPLIER_BLUE;

    let grayscale_value = (r + g + b).to_int();

    pixel(grayscale_value, grayscale_value, grayscale_value, pixel.a)
}

fn box_blur(x, y) {
    const SIZE = 3;
    const OFFSET = (SIZE.to_float() / 2.0).floor().to_int();

    let count = 0.0;

    let r = 0.0;
    let g = 0.0;
    let b = 0.0;
    let a = 0.0;

    for offset_x in 0..SIZE {
        for offset_y in 0..SIZE {
            let c = get_pixel(x + offset_x - OFFSET, y + offset_y - OFFSET);

            if (type_of(c) == "array") {
                r += c[0].to_float();
                g += c[1].to_float();
                b += c[2].to_float();
                a += c[3].to_float();

                count += 1.0;
            }
        }
    }

    pixel(
        (r / count).to_int(),
        (g / count).to_int(),
        (b / count).to_int(),
        (a / count).to_int()
    )
}
