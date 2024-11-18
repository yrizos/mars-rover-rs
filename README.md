# The Mars Rover Challenge in Rust

This project implements the Mars Rover Challenge in Rust. The goal is to navigate a rover on a plateau based on user input for the plateau dimensions, the rover's initial position and direction, and a series of instructions.

I've documented my progress in a series of articles:

1. [The Mars Rover Challenge in Rust: Setting the Scene](https://dev.to/yrizos/the-mars-rover-challenge-in-rust-setting-the-scene-49l8)
2. [The Mars Rover Challenge in Rust: Let's Get Moving!](https://dev.to/yrizos/the-mars-rover-challenge-in-rust-lets-get-moving-3a32)
3. [The Mars Rover Challenge in Rust: Houston, Do You Copy?](https://dev.to/yrizos/the-mars-rover-challenge-in-rust-houston-do-you-copy-334o)

The text of the challenge can be found in [CHALLENGE.md](CHALLENGE.md).

## Setup

### Prerequisites

Rust and Cargo need to be installed on your system. You can install Rust and Cargo by following the instructions at [rust-lang.org](https://www.rust-lang.org/tools/install).

If you are on a Mac you may also [install Rust via Homebrew](https://dev.to/yrizos/installing-rust-on-macos-with-homebrew-51fk).

### Running the program

1. **Clone the Repository**

    ```sh
    git clone git@github.com:yrizos/mars-rover-rs.git
    cd mars-rover-rs
    ```

2. **Build the program**

    ```sh
    cargo build
    ```

3. **Run the program**

    ```
    cargo run
    ```

When prompted, provide the input for the plateau dimensions and multiple rovers. For example:

```plaintext
5 5
1 2 N
LMLMLMLMM
3 3 E
MMRMMRMRRM
```

After providing the input, you need to signal the end of input so that the program knows you are done. On Unix-like systems (Linux, macOS), you can do this by pressing `Ctrl+D`.

The program will output the final state of each rover. For the provided input, the expected output is:

```plaintext
1 3 N
5 1 E
```

### Running the tests

Unit tests can be found in the source files, while integration tests are located in the `tests` directory.

Both types of tests can be run with:

```sh
cargo test
```
