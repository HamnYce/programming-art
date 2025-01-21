use crate::cell::Cell;
use crate::NextValue;
use std::{self, ops::Add};

pub struct Life<T>
where
    T: Add + NextValue<T>,
{
    pub board: Vec<Cell<T>>,
    pub next_board: Vec<Cell<T>>,
    pub rows: i32,
    pub cols: i32,
}
