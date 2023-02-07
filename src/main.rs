use anyhow::Result;
use rand::prelude::*;
use rusvid_core::plane::Plane;
use rusvid_effect::EffectLogic;

use crate::effect::*;

mod effect;
mod pixel;

static SCRIPT: &'static str = include_str!("../example_script.rhai");

fn main() -> Result<()> {
    let width = 2_u32.pow(8);
    let height = 2_u32.pow(8);

    let mut plane = Plane::new(width, height)?;
    for x in 0..width {
        for y in 0..height {
            plane.put_pixel(x, y, [random(), random(), random(), 255])?;
        }
    }
    plane.clone().save_as_png("og.png")?;

    let effect = CustomEffect::new("box_blur", SCRIPT);
    plane = effect.apply(plane)?;
    plane.save_as_png("out.png")?;

    Ok(())
}
