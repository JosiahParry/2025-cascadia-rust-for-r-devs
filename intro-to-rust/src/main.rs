fn hello_world() {
    println!("Hello, world!");
}

// Classic fizz buzz to introduce if else statements
fn fizz_buzz(i: i32) {
    if (i % 3 == 0) && (i % 5 == 0) {
        println!("FizzBuzz")
    } else if i % 3 == 0 {
        println!("Fizz")
    } else if i % 5 == 0 {
        println!("Buzz")
    }
}

// introduce scalar operators
fn is_even(x: i32) -> bool {
    // like tidyverse style guide
    // returns are implicit in rust
    // UNLESS exiting early
    x % 2 == 0
}

fn is_odd(x: i32) -> bool {
    !is_even(x)
}

// move on to "real" functions
// introduce vectors
fn sum(x: Vec<f64>) -> f64 {
    let mut total = 0.0;
    for xi in x {
        total += xi;
    }
    total
}

// introduce the concept of a "slice"
fn mean(x: &[f64]) -> f64 {
    // show primitive type casting here
    let n = x.len() as f64;
    let mut total = 0.0;

    for xi in x {
        total += xi;
    }

    total / n
}

// introduce the concept of struct
// strong typing—closest analog in R is S7
#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

// We introduce enums after creating our first two
// distance methods
enum Measure {
    Euclidean,
    Haversine,
}

impl Point {
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    fn euclidean_distance(&self, destination: &Self) -> f64 {
        ((self.x - destination.x).powi(2) + (self.y - destination.y).powi(2)).sqrt()
    }

    fn haversine_distance(&self, destination: &Self) -> f64 {
        let radius = 6_371_008.7714;
        let theta1 = self.y.to_radians();
        let theta2 = destination.y.to_radians();
        let delta_theta = (destination.y - self.y).to_radians();
        let delta_lambda = (destination.x - self.x).to_radians();
        let a = (delta_theta / 2f64).sin().powi(2)
            + theta1.cos() * theta2.cos() * (delta_lambda / 2f64).sin().powi(2);
        2f64 * a.sqrt().asin() * radius
    }

    // Demonstrates using pattern matching an enum
    fn distance(&self, destination: &Self, measure: &Measure) -> f64 {
        match measure {
            Measure::Euclidean => self.euclidean_distance(destination),
            Measure::Haversine => self.haversine_distance(destination),
        }
    }

    // Demonstrates using .iter().map()
    fn distances(&self, destinations: &[Point], measure: &Measure) -> Vec<f64> {
        destinations
            .iter()
            .map(|pi| self.distance(pi, measure))
            .collect()
    }
}

fn centroid(points: &[Point]) -> Point {
    let mut center_x = 0.0;
    let mut center_y = 0.0;
    let n = points.len();

    for point in points {
        let Point { x, y } = point;
        center_x += x;
        center_y += y;
    }

    Point {
        x: center_x / n as f64,
        y: center_y / n as f64,
    }
}

fn distance_pairwise(origin: &[Point], destination: &[Point], measure: &Measure) -> Vec<f64> {
    origin
        .iter()
        .zip(destination)
        .map(|(oi, di)| oi.distance(di, measure))
        .collect::<Vec<f64>>()
}

fn main() {
    println!("Hello, world!");
}
