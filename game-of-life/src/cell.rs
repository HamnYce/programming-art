use crate::{monoid::Monoid, render::Render, RandomInit};

#[derive(Clone, Copy)]
pub struct Cell<T> {
    pub value: T,
}

impl<T> Monoid<Cell<T>> for Cell<T>
where
    T: Monoid<T>,
{
    fn append(self, other: Self) -> Self {
        Cell {
            value: self.value.append(other.value),
        }
    }
}

/// Random initialiser for values that are supported by nannou random()
impl<T> RandomInit<Cell<T>> for Cell<T>
where
    T: RandomInit<T>,
{
    fn random() -> Cell<T> {
        Cell {
            value: <T as RandomInit<T>>::random(),
        }
    }
}

/// color string for the cell
impl<T> Render<Cell<T>> for Cell<T>
where
    T: Render<T>,
{
    fn color(&self) -> nannou::prelude::Rgba<u8> {
        self.value.color()
    }
}
