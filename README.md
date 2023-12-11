# Rookie vs ChatGPT 3.5
on the left corner:
I, 10+ year experienced Java developper with small C/C++ knowledge, I never studied/coded in Rust :construction_worker:

on the right corner:
Chat GPT 3.5, the future for coding ?

Let's compete by implementing solution of Advent of Code 2023 in Rust. 

### Note
I volunteerly let not compilable code as is. if someone wants to fix, feel free!

Otherwise, navigate to "day" you want to test and run from there


# Advancement
| Day |                        Rookie                         |  Chat GPT                                |
|-----|:----------------------------------------------------:|:----------------------------------------:|
| 1   |                     :star::star:                     | coming soon                              |
| 2   |                     :star::star:                     | not compilable, see md file for details  |
| 3   |                                                      | run, wrong result, see md file for details  |


# For Developpers
:warning: This repo is made as a rust workspace. To add new day, update also the root Cargo.toml file

usefull commands to know:
```
cargo init
cargo new <module>
cargo new <test_name> --lib
cargo test
```

to compile run: 
```
rustc day1.rs
```

to run: 
```
./day1.rs
```

# Benchmarking
Rust mostly offer possibility to bench even if it seems quite complex to set it up.

as per december 2023, it requires nightly
> rustup toolchain install nightly-x86_64-pc-windows-msvc

Go to day you want to bench
> cd day2

Edit source to uncomment lines
```
#![feature(test)]
.
.
.
extern crate test;
use test::Bencher;
.
.
#[bench]
```

And run bench, for instance
> cargo +nightly bench

outputs something like:
```
test tests::test_tuto ... ignored
test tests::test_part1 ... bench:     402,635 ns/iter (+/- 33,181)
```
