# Sudoku-solver
[![](https://img.shields.io/badge/Category-Applications%20in%20Rust-E5A505?style=flat-square)]() [![](https://img.shields.io/badge/Language-Rust-E5A505?style=flat-square)]() [![](https://img.shields.io/badge/Version-0.1.1-E5A505?style=flat-square&color=green)]()

Sudoku Solver in Rust

## Usage

- Using `cargo run`

```shell
> cargo run
    .
    .
    .
Enter input mode: [1]File, [2]Stdin:
2
Enter initial Sudoku puzzle 9x9 matrix delimited by spaces and newlines:
8 0 0 0 0 0 0 0 0
0 0 3 6 0 0 0 0 0
0 7 0 0 9 0 2 0 0
0 5 0 0 0 7 0 0 0
0 0 0 0 4 5 7 0 0
0 0 0 1 0 0 0 3 0
0 0 1 0 0 0 0 6 8
0 0 8 5 0 0 0 1 0
0 9 0 0 0 0 4 0 0

<Answer outputted>
```

- Using Makefile `make` or `make compile`

```shell
> make
    .
    .
    .
Enter input mode: [1]File, [2]Stdin:
1
Enter file location:
src/sudoku

<Answer outputted>
```

## Makefile

- `make` or `make compile` to run the binary executable

- `make build` to only build the binary executable in ./target/ directory

## Dependency

- No dependencies   ¯\\_(ツ)_/¯

## References

- Wikipedia
  
  - Optimized Sudoku solving algorithms

- Backtracing algorithm

  - [Python Solution](https://youtu.be/eqUwSA0xI-s)
