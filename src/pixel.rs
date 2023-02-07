use std::fmt::Display;

use rhai::{Array, CustomType, EvalAltResult, Position, TypeBuilder, INT};

// TODO replace with `Pixel` from `rusvid_core`
#[derive(Debug, Clone, PartialEq)]
pub struct Pixel {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Display for Pixel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}, {}, {}]", self.r, self.g, self.b, self.a)
    }
}

impl Pixel {
    pub fn new(r: INT, g: INT, b: INT, a: INT) -> Self {
        Self {
            r: r as u8,
            g: g as u8,
            b: b as u8,
            a: a as u8,
        }
    }

    pub fn from_raw(values: Array) -> Self {
        let r = values[0].clone_cast::<INT>().max(0).min(255);
        let g = values[1].clone_cast::<INT>().max(0).min(255);
        let b = values[2].clone_cast::<INT>().max(0).min(255);
        let a = values[3].clone_cast::<INT>().max(0).min(255);

        Self::new(r, g, b, a)
    }

    pub fn to_raw(&self) -> [u8; 4] {
        [self.r, self.g, self.b, self.a]
    }

    pub fn get_r(&mut self) -> INT {
        self.r as INT
    }
    pub fn set_r(&mut self, value: INT) {
        self.r = value as u8
    }

    pub fn get_g(&mut self) -> INT {
        self.g as INT
    }
    pub fn set_g(&mut self, value: INT) {
        self.g = value as u8
    }

    pub fn get_b(&mut self) -> INT {
        self.b as INT
    }
    pub fn set_b(&mut self, value: INT) {
        self.b = value as u8
    }

    pub fn get_a(&mut self) -> INT {
        self.a as INT
    }
    pub fn set_a(&mut self, value: INT) {
        self.a = value as u8
    }

    pub fn get_component(&mut self, idx: i64) -> Result<u8, Box<EvalAltResult>> {
        match idx {
            0 => Ok(self.r),
            1 => Ok(self.g),
            2 => Ok(self.b),
            3 => Ok(self.a),
            _ => Err(EvalAltResult::ErrorIndexNotFound(idx.into(), Position::NONE).into()),
        }
    }
}

impl CustomType for Pixel {
    fn build(mut builder: TypeBuilder<Self>) {
        builder
            .with_name("Pixel")
            .with_fn("pixel", Self::new)
            .with_fn("pixel_raw", Self::from_raw)
            .with_fn("to_string", |x: &mut Pixel| -> String {
                format!("{x}").to_string()
            })
            .with_fn("to_debug", |x: &mut Pixel| -> String {
                format!("{x:?}").to_string()
            })
            .with_get_set("r", Self::get_r, Self::set_r)
            .with_get_set("g", Self::get_g, Self::set_g)
            .with_get_set("b", Self::get_b, Self::set_b)
            .with_get_set("a", Self::get_a, Self::set_a)
            .with_indexer_get(Self::get_component);
    }
}
