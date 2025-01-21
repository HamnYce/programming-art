type CellType = Basic;

mod cell;
mod life;
mod life_basic;
mod monoid;
mod next_value;
mod random_init;
mod render;

use cell::Cell;
use life::*;
use life_basic::Basic;
use monoid::Monoid;
use nannou::prelude::*;
use next_value::NextValue;
use random_init::RandomInit;
use render::Render;
use std::{self, mem::swap, ops::Add};

struct Model<T>
where
    T: Add + NextValue<T>,
{
    _window: WindowId,
    life: Life<T>,
}

// need a function to decide whether a cell should be kept alive or not

const SCREEN_SIZE: i32 = 500;
const ROWS: i32 = 250;
const COLS: i32 = 250;
const CELL_SIZE: i32 = SCREEN_SIZE / ROWS;

fn model<T>(app: &App) -> Model<T>
where
    T: Add + RandomInit<T> + NextValue<T>,
{
    // Create a new window! Store the ID so we can refer to it later.
    let _window = app
        .new_window()
        .size(SCREEN_SIZE as u32, SCREEN_SIZE as u32)
        .title("Game of Life")
        .build()
        .unwrap();

    let life: Life<T> = Life {
        board: (0..ROWS * COLS)
            .map(|_| Cell {
                value: <T as RandomInit<T>>::random(),
            })
            .collect(),
        next_board: (0..ROWS * COLS)
            .map(|_| Cell {
                value: <T as RandomInit<T>>::random(),
            })
            .collect(),
        rows: ROWS,
        cols: COLS,
    };

    Model { _window, life }
}

fn update<T>(_app: &App, model: &mut Model<T>, _update: Update)
where
    T: Add + NextValue<T> + Monoid<T> + Copy + Default,
{
    for x in 0..model.life.cols {
        for y in 0..model.life.rows {
            let mut neighbors: T = Default::default();
            for dx in -1..=1 {
                for dy in -1..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    let nx = x + dx;
                    let ny = y + dy;
                    if nx < 0 || ny < 0 || nx >= model.life.cols || ny >= model.life.rows {
                        continue;
                    }

                    let other = &model.life.board[(nx + ny * model.life.cols).to_usize().unwrap()];
                    neighbors = neighbors.append(other.value);
                }
            }
            model.life.next_board[(x + y * model.life.cols).to_usize().unwrap()].value =
                model.life.board[(x + y * model.life.cols).to_usize().unwrap()]
                    .value
                    .next_value(neighbors)
        }
    }
    swap(&mut model.life.board, &mut model.life.next_board);
}

fn view<T>(app: &App, model: &Model<T>, frame: Frame)
where
    T: Add + NextValue<T> + Render<T> + Clone,
{
    let draw = app
        .draw()
        .scale_y(-1.0)
        .x_y(-(SCREEN_SIZE as f32) / 2., -(SCREEN_SIZE as f32) / 2.);
    draw.background().color(BLACK);

    for x in 0..model.life.cols {
        for y in 0..model.life.rows {
            let cell = &model.life.board[(x + y * model.life.cols) as usize];
            draw.rect()
                .x_y((x * CELL_SIZE) as f32, (y * CELL_SIZE) as f32)
                .w_h(CELL_SIZE as f32, CELL_SIZE as f32)
                .color(cell.color());
        }
    }

    draw.to_frame(app, &frame).unwrap();
}

fn main() {
    nannou::app(model::<CellType>)
        .update(update::<CellType>)
        .view(view)
        .run();
}
