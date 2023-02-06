use itertools::*;
use rhai::{Array, Dynamic, Engine, EvalAltResult, Func, Scope, INT};
use rusvid_core::plane::Plane;

use pixel::Pixel;

mod pixel;

static SCRIPT: &'static str = include_str!("../example_script.rhai");

fn main() -> Result<(), Box<EvalAltResult>> {
    // Create scripting engine
    let mut engine = Engine::new();
    engine.set_optimization_level(rhai::OptimizationLevel::Full);
    engine.build_type::<Pixel>();

    let width = 2_u32.pow(8);
    let height = 2_u32.pow(8);

    let mut plane = Plane::new(width, height).unwrap();
    for x in 0..width {
        for y in 0..height {
            plane.put_pixel(x, y, [255; 4]).unwrap();
        }
    }

    engine.register_fn("width", move || -> u32 { width.clone() });
    engine.register_fn("height", move || -> u32 { height.clone() });
    engine.register_fn("get_pixel", move |x: i64, y: i64| -> Dynamic {
        let p = plane.pixel(x as u32, y as u32);

        match p {
            Some(value) => Dynamic::from_array(vec![
                Dynamic::from(value[0] as INT),
                Dynamic::from(value[1] as INT),
                Dynamic::from(value[2] as INT),
                Dynamic::from(value[3] as INT),
            ]),
            None => Dynamic::UNIT,
        }
    });

    let calc_func = Func::<(u32, u32), Pixel>::create_from_script(engine, SCRIPT, "calc")?;

    let mut plane = Plane::new(width, height).unwrap();
    let data = plane.as_data_mut();

    data.iter_mut()
        .zip((0..width).cartesian_product(0..height))
        .for_each(|(p, (x, y))| *p = calc_func(x, y).unwrap().to_raw());

    plane.clone().save_as_png("out.png").unwrap();

    assert!(plane.pixel(0, 0).is_some());

    Ok(())
}
