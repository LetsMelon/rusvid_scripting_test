use std::fmt::Debug;

use anyhow::Result;
use itertools::*;
use rhai::{Dynamic, Engine, Func, OptimizationLevel, INT};
use rusvid_core::plane::Plane;
use rusvid_effect::{EffectLogic, Element, ID};

use crate::pixel::Pixel;

pub struct CustomEffect {
    entry_point: String,
    script: &'static str,
}

impl Debug for CustomEffect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CustomEffect").finish()
    }
}

impl Element for CustomEffect {
    fn id(&self) -> Option<&ID> {
        None
    }
}

impl CustomEffect {
    pub fn new(entry_point: impl Into<String>, script: &'static str) -> Self {
        CustomEffect {
            entry_point: entry_point.into(),
            script,
        }
    }
}

impl EffectLogic for CustomEffect {
    fn apply(&self, original: Plane) -> Result<Plane> {
        let width = original.width();
        let height = original.height();

        let mut engine = Engine::new();
        engine.set_optimization_level(OptimizationLevel::Full);
        engine.build_type::<crate::pixel::Pixel>();

        engine.register_fn("width", move || -> u32 { width.clone() });
        engine.register_fn("height", move || -> u32 { height.clone() });
        engine.register_fn("get_pixel", move |x: u32, y: u32| -> Dynamic {
            let p = original.pixel(x, y);

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

        let function =
            Func::<(u32, u32), Pixel>::create_from_script(engine, self.script, &self.entry_point)?;

        let data = (0..width)
            .cartesian_product(0..height)
            .map(|(x, y)| {
                let p = function(x, y).unwrap();
                p.to_raw()
            })
            .collect_vec();

        Ok(Plane::from_data_unchecked(width, height, data))
    }
}
