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
fn calc(x, y) {
    let r = constrain(((x.to_float() / global::WIDTH) * 255.0).to_int(), 0, 255);
    let g = constrain(((y.to_float() / global::HEIGHT) * 255.0).to_int(), 0, 255);

    pixel(r, g, 0, 255)
}
