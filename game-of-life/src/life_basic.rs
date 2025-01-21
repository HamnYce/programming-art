use crate::{Monoid, NextValue, RandomInit, Render};
use nannou::color::Rgba;

pub type Basic = i32;

impl NextValue<Self> for Basic {
    fn next_value(&self, neighbors: Self) -> Self {
        if ((*self == 1) && (neighbors == 2 || neighbors == 3)) || (*self == 0 && neighbors == 3) {
            1
        } else {
            0
        }
    }
}

impl RandomInit<Self> for Basic {
    fn random() -> Self {
        nannou::rand::random::<Self>() % 2
    }
}

impl Render<Self> for Basic {
    fn color(&self) -> Rgba<u8> {
        if *self == 1 {
            Rgba::new(255, 255, 255, 255)
        } else {
            Rgba::new(0, 0, 0, 255)
        }
    }
}

impl Monoid<Self> for Basic {
    fn append(self, other: Self) -> Self {
        return self + other;
    }
}
