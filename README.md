# The Mars Rover Challenge in Rust

This project implements the Mars Rover Challenge in Rust. The goal is to navigate a rover on a plateau based on user input for the plateau dimensions, the rover's initial position and direction, and a series of instructions.

I've documented my progress in a series of articles:

1. [The Mars Rover Challenge in Rust: Setting the Scene](https://dev.to/yrizos/the-mars-rover-challenge-in-rust-setting-the-scene-49l8)
2. [The Mars Rover Challenge in Rust: Let's Get Moving!](https://dev.to/yrizos/the-mars-rover-challenge-in-rust-lets-get-moving-3a32)
3. [The Mars Rover Challenge in Rust: Houston, Do You Copy?](https://dev.to/yrizos/the-mars-rover-challenge-in-rust-houston-do-you-copy-334o)

## The Challenge

A squad of robotic rovers is to be landed by NASA on a plateau on Mars.

This plateau, which is curiously rectangular, must be navigated by the rovers so that their onboard cameras can get a complete view of the surrounding terrain to send back to Earth. A rover's position is represented by a combination of an `x` and `y` coordinates and a letter representing one of the four cardinal compass points. The plateau is divided up into a grid to simplify navigation. An example position might be `0, 0, N` which means the rover is in the bottom left corner and facing North.

In order to control a rover, NASA sends a simple string of letters. The possible letters are `L`, `R` and `M`. `L` and `R` make the rover spin 90 degrees left or right respectively, without moving from its current spot.

`M` means move forward one grid point, and maintain the same heading.

Assume that the square directly North from `(x, y)` is `(x, y+1)`.

### Input:

The first line of input is the upper-right coordinates of the plateau, the lower-left coordinates are assumed to be `0,0`.

The rest of the input is information pertaining to the rovers that have been deployed. Each rover has two lines of input. The first line gives the rover's position, and the second line is a series of instructions telling the rover how to explore the plateau.

The position is made up of two integers and a letter separated by spaces, corresponding to the `x` and `y` coordinates and the rover's orientation.

Each rover will be finished sequentially, which means that the second rover won't start to move until the first one has finished moving.

### Output:

The output for each rover should be its final coordinates and heading.

#### Test input:

    5 5
    1 2 N
    LMLMLMLMM
    3 3 E
    MMRMMRMRRM

#### Test output:

    1 3 N
    5 1 E

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
