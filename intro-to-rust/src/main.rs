#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

enum Measure {
    Euclidean,
    Haversine,
}

impl Point {
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    fn euclidean_distance(&self, destination: &Self) -> f64 {
        todo!()
    }

    fn haversine_distance(&self, destination: &Self) -> f64 {
        // taken from geo source code
        // let theta1 = origin.y().to_radians();
        // let theta2 = destination.y().to_radians();
        // let delta_theta = (destination.y() - origin.y()).to_radians();
        // let delta_lambda = (destination.x() - origin.x()).to_radians();
        // let a = (delta_theta / two).sin().powi(2)
        //     + theta1.cos() * theta2.cos() * (delta_lambda / two).sin().powi(2);
        // let c = two * a.sqrt().asin();
        // F::from(self.radius).unwrap() * c
        todo!()
    }

    fn distance(&self, destination: &Self, measure: &Measure) -> f64 {
        match measure {
            Measure::Euclidean => self.euclidean_distance(destination),
            Measure::Haversine => self.haversine_distance(destination),
        }
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
