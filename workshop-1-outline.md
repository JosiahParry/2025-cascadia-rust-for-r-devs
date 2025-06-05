# Intro to Rust for R Developers




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
-
