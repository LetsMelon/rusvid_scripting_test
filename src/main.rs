use anyhow::Result;
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
            plane.put_pixel(x, y, [255; 4])?;
        }
    }

    let effect = CustomEffect::new("calc", SCRIPT);

    let output_plane = effect.apply(plane)?;
    output_plane.save_as_png("out.png")?;

    Ok(())
}
