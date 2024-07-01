mod fibonacci_sphere;
mod delaunay_triangulation;
mod space;

use fibonacci_sphere::generate_fibonacci_sphere;
use delaunay_triangulation::{perform_triangulation, generate_voronoi, generate_voronoi_on_sphere};

pub fn main() {
    println!("Generating Sphere...");
    let num_samples = 1000;
    let min_latitude = -90.0;
    let max_latitude = 90.0;
    let min_longitude = -180.0;
    let max_longitude = 180.0;
    let seed = 1.0;
    let radius = 1000.0; // Adjust this to match your sphere radius

    let points = generate_fibonacci_sphere(num_samples, min_latitude, max_latitude, min_longitude, max_longitude, seed).unwrap();
    let triangulation = perform_triangulation(points);

    // Generate 2D Voronoi diagram
    generate_voronoi(&triangulation);

    // Generate Voronoi diagram projected onto a sphere
    generate_voronoi_on_sphere(&triangulation, radius);

    println!("Done");
    space::simulate();
}
