# DistroHoop

DistroHoop is a CLI tool written in Rust that helps you determine your next Linux distribution. It uses the mighty force of the RNG to determine where to hop on next!

<div align="center">
    <img src="/img/intro.png" alt="Intro Image" width="600">
</div>

## Installation

To install DistroHoop, you need to have Rust and Cargo installed on your system. You can install Rust and Cargo by following the instructions on the [official Rust website](https://www.rust-lang.org/tools/install).

Clone the repository and build the project:

```sh
git clone https://github.com/yourusername/distrohoop.git
cd distrohoop
cargo build --release
```

## Usage

Run the DistroHoop CLI tool:

```sh
cargo run --release
```

## Example Output

<div align="center">
    <img src="/img/example.gif" alt="Intro Image" width="600">
</div>

## Contributing

Contributions are welcome! Please open an issue or submit a pull request if you have any improvements or new features to suggest.

## License

This project is licensed under the MIT License. See the 

LICENSE

 file for details.

## Acknowledgements

- [colored](https://github.com/mackwic/colored) for terminal text coloring
- [crossterm](https://github.com/crossterm-rs/crossterm) for cross-platform terminal manipulation
- [rand](https://github.com/rust-random/rand) for random number generation

