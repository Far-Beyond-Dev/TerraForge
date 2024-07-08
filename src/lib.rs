mod fibonacci_sphere;
mod delaunay_triangulation;

use fibonacci_sphere::generate_fibonacci_sphere;
use delaunay_triangulation::{create_spherical_voronoi, print_voronoi_edges};

/// The main function of the program.
///
/// This function orchestrates the generation of a Fibonacci sphere, creation of a spherical 
/// Voronoi diagram, and writing of the Voronoi edges to a file.
///
/// # Returns
///
/// A `std::io::Result<()>`, which is `Ok(())` if all operations were successful, 
/// or an `Err` containing the I/O error if there was a problem during execution.
pub fn main() -> std::io::Result<()> {
    let num_samples = 1000; // Increase the number of points for better coverage
    let jitter = 0.1; // Adjust this value to control the randomness (0.0 to 1.0)
    let points = generate_fibonacci_sphere(num_samples, jitter)?;
    
    let triangulation = create_spherical_voronoi(points);

    print_voronoi_edges(&triangulation)?;

    println!("Voronoi edges have been written to voronoi_edges.txt");

    Ok(())
}