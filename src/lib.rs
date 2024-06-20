mod fibonacci_sphere;

use fibonacci_sphere::generate_fibonacci_sphere;
//use delaunay_triangulation::perform_triangulation;


pub fn main() {
    let num_samples = 256;
    let min_latitude = -90.0;
    let max_latitude = 90.0;
    let min_longitude = -180.0;
    let max_longitude = 180.0;
    let seed = 1.0;
    

    let result = generate_fibonacci_sphere(num_samples, min_latitude, max_latitude, min_longitude, max_longitude, seed);

    print!("Fibonacci Sphere: {:?}", result);
}
