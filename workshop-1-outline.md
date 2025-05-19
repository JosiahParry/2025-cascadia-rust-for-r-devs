# Intro to Rust for R Developers


## Your first crate

Let's create your first crate
```shell
cargo new intro-to-rust
```

## Crate anatomy

Two types of crates: binary, library. This first workshop we will work only with a binary crate.

Binary crates are standalone applications like command line tools, or things that run once—simiar to a script that you run with `Rscript main.R`

- `src/` where your source code goes
- `src/main.rs` the primary file for your binary
- Cargo.toml: like a `DESCRIPTION` file
- Cargo.lock: like an `renv.lock` file

## Hello world

- The hello world is always prepopulated.
- The `main()` function is required.
- the contents of the `main()` function are what is executed by your binary crate
- to run your binary, from the command line we write `cargo run`
- this will compile your code and then run it

```rust
fn main() {
    println!("Hello, world!");
}
```

## `println!()` macro

- macros are different than functions
- you can tell them apart by the presence of the `!`
- println! is a handy helper to help us print things out to the terminal, otherwise we wont be able to see output
- in R printing is implicit, rust it is explicit—and kind of confusing to be honest
- uses format strings `"{}"` is a placeholder for something that can be _displayed_
- not everything can be displayed, something only have a **debug** implementation we can use `{:?}`
  - recommend keeping this in mind for later.

## FizzBuzz

- lets do this inside of main() fn
- cover variable assignment
- start with a simple and classic programming problem
- fizz buzz
- will introduce operators `==` and `%`
- cover `i32` type
- leave placeholder code for them
- create a function outside of `main()`
- fn keyword structure
  - arguments are typed

### Solution


```rust
fn fizz_buzz(i: i32) {
    if (i % 3 == 0) && (i % 5 == 0) {
        println!("FizzBuzz")
    } else if i % 3 == 0 {
        println!("Fizz")
    } else if i % 5 == 0 {
        println!("Buzz")
    }
}

fn main() {
    // FizzBuzz
    fizz_buzz(15);
    // Fizz
    fizz_buzz(9);
    // Buzz
    fizz_buzz(5);
    // Nothing
    fizz_buzz(0);
}
```

## `is_even()` & `is_odd()`

- meme about is_odd in javascript
- we will create our first function with a return type
- discuss bools
- first create `is_even()` then use that toe create an `is_odd()`

```rust
fn is_even(x: i32) -> bool {
    x % 2 == 0
}

fn is_odd(x: i32) -> bool {
    !is_even(x)
}

fn main() {
    let x = 3;
    println!("{x} is even: {}", is_even(x));
    println!("{x} is odd: {}", is_odd(x));
}
```

## Introduce collections

- discuss arrays first
- arrays are fixed size
- vectors are not fixed sized
- arrays are better for performance but much more difficult to use
- creating vectors: `vec![]`
- debug format for println!();
- R is unique in that most things are vectorized that is not often the case
- in order to use vectors we need to iterate
- print the vector using debug format

```rust
fn main() {
    let x = vec![3, 6, 9, 12];
    println!("Vector is: {:?}", x);
}
```


## Introduce for loops

- create for loops to iterate through vector of integers
- print the value of the integer for each iteration
- our next task is to create `sum()` function that can add all elements of these up

## Mutability

- everything that we create in rust is "immutable" by default. it cannot change
- we haven't gone over assigning values. we do that with the `let` keyword and termination with `;`
- to have something that can change value—e.g. accumulate a value ;)—we need to define it as mutable

```rust
// immutable
let pi = 3.1415;
let mut total = 0;
```

----------

## Miscellany

- what is Rust
- compiled vs interpreted lanuage
- strongly typed
- rustup vs rustc vs cargo
- hello world
  - `cargo run`
- primitive types
  - scalars
- logical operators
  - use `&&` and `||` there is no vectorized version
  - we do everything on the primitive types
- vectors
- iteration with for loops
- mutable variables
- iterators
- .iter()
- .map()

## Function Ideas:

- is_odd
  - meme opportunity here https://www.npmjs.com/package/is-odd
- parallel minimum
- average
- centroid
- euclidean distance
  - use this to demonstrate vectorization
