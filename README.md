# Basic Example for Command Line Program using Rust - Blackjack

This is an experiment in creating a command line game, Blackjack, using the Rust programming language.

## Installation

Clone this repository to your local machine and install dependencies using [Cargo](https://doc.rust-lang.org/cargo/)

## Getting started

Once you have your environment setup, you can either play the game on your shell using the following command

```sh
cargo run
```

Or build an executable to be played on any architecture using the following command

```sh
cargo build --release
```

And then you will have an executble under your `targer/` directory as such

```sh
$ target/release/blackjack-rs
>
> Cards in your hand:
> * 5 of Spades
> * 1 of Hearts
> Player's hand value: 6
> 
> Would you like to hit? [y/n]
```

## References
1. [The Rust Book](https://doc.rust-lang.org/stable/book/)
2. [Rustjack](https://github.com/felky/Rustjack) by `github/felky`
3. [Rust by Example - Randomness](https://rust-by-example-ext.com/rand.html)
4. [Rick Astley](https://www.youtube.com/watch?v=AC3Ejf7vPEY)

## License

This project is licensed under the terms of the MIT license.