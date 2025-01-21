# Game of Life

Welcome to the Game of Life! This project is an implementation of Conway's Game of Life using Rust and the Nannou creative coding framework.

## Overview

Conway's Game of Life is a cellular automaton devised by the British mathematician John Horton Conway in 1970. It is a zero-player game, meaning that its evolution is determined by its initial state, requiring no further input. One interacts with the Game of Life by creating an initial configuration and observing how it evolves.

## Features

- **Type Agnostic Board**: The board is designed to be agnostic of the cell types. This means you can define your own cell types and behaviors by implementing a few traits.
- **Customizable Cell Types**: By defining the required traits, you can create your own versions of cell behaviors and interactions.

## Type System

The Game of Life implementation in this project is highly flexible due to its type system. The board does not depend on a specific cell type. Instead, it relies on traits that define the behavior of the cells. The following traits are essential for defining a cell type:

1. **Monoid**: This trait is used to combine two cells.
2. **NextValue**: This trait determines the next state of a cell based on its neighbors.
3. **RandomInit**: This trait initializes a cell with a random value.
4. **Render**: This trait defines how a cell is rendered on the screen.

### Example: Basic Cell Type

The `life_basic.rs` file provides an example of a basic cell type that implements these traits:

```rust
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
    fn color(&self) -> nannou::color::Rgba<u8> {
        if *self == 1 {
            nannou::color::Rgba::new(255, 255, 255, 255)
        } else {
            nannou::color::Rgba::new(0, 0, 0, 255)
        }
    }
}

impl Monoid<Self> for Basic {
    fn append(self, other: Self) -> Self {
        self + other
    }
}
```

## Creating Your Own Cell Types

To create your own cell types, you need to implement the four traits mentioned above. Here is a brief overview of what each trait should do:

- **Monoid**: Define how two cells combine.
- **NextValue**: Define the logic for the next state of a cell based on its neighbors.
- **RandomInit**: Provide a method to initialize a cell with a random value.
- **Render**: Define how the cell should be rendered on the screen.

Once you have implemented these traits, you can use your custom cell type with the Game of Life board.

## Running the Game

To run the game, use the following command:

```sh
cargo run
```

This will start the Game of Life with the default cell type defined in `life_basic.rs`.

## Conclusion

This implementation of Conway's Game of Life is designed to be flexible and extensible. By defining a few traits, you can create your own cell types and explore different behaviors and interactions. Have fun experimenting with different configurations and see how they evolve over time!
