# Flowfield Wallpaper Generator

Welcome to the **Flowfield Wallpaper Generator**! This project generates mesmerizing flowfield patterns and captures frames at regular intervals, saving them as PNG images.

## Features

- **Flowfield Simulation**: Runs a flowfield simulation to create beautiful, dynamic patterns.
- **Frame Capture**: Captures frames every 500 iterations.
- **PNG Output**: Saves captured frames as PNG images in a specified directory.

## Installation

To get started, clone the repository and navigate to the project directory:

```sh
git clone https://github.com/yourusername/flowfield-wallpaper-generator.git
cd flowfield-wallpaper-generator
```

## Usage

Run the generator using the following command:

```sh
cargo run --release
```

This will start the flowfield simulation. Frames will be captured every 500 iterations and saved in the `output` directory, which will be created if it doesn't exist.

## Output

Captured frames are saved in PNG format in the `flowfield-wallpaper-generator` directory. Each frame is named sequentially based on the capture order.

## Dependencies

Ensure you have the following dependencies installed:

- Rust (latest stable version)
- Cargo (Rust package manager)

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests.

## License

This project is licensed under the MIT License. See the [LICENSE](../LICENSE) file for details.

## Contact

For any questions or suggestions, please open an issue.

Enjoy creating beautiful flowfield patterns!
