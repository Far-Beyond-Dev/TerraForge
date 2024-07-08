use std::f64::consts::PI;
use rand::Rng;
use std::io::{self, Write};
use std::time::Instant;
use std::fs::File;

/// Normalizes a 3D point to lie on the surface of a unit sphere.
///
/// This function takes a point in 3D space and projects it onto the surface of a
/// unit sphere (a sphere with radius 1) centered at the origin.
///
/// # Arguments
///
/// * `point` - A tuple `(x, y, z)` representing the coordinates of the point to be normalized.
///
/// # Returns
///
/// A tuple `(x, y, z)` representing the normalized point on the surface of the unit sphere.
fn normalize_to_sphere(point: (f64, f64, f64)) -> (f64, f64, f64) {
    let magnitude = (point.0 * point.0 + point.1 * point.1 + point.2 * point.2).sqrt();
    (point.0 / magnitude, point.1 / magnitude, point.2 / magnitude)
}

/// Generates a point on a Fibonacci sphere.
///
/// This function creates a point on a Fibonacci sphere, which provides a nearly uniform distribution 
/// of points on a sphere's surface. It uses the golden ratio to determine the spacing between points.
///
/// # Arguments
///
/// * `numsamples` - The total number of points to be generated on the sphere.
/// * `samplev` - The index of the current point being generated (0 to numsamples-1).
/// * `seed` - A seed value for pseudo-random number generation to add variety to the point distribution.
/// * `min_lat` - The minimum latitude in degrees.
/// * `max_lat` - The maximum latitude in degrees.
/// * `min_lon` - The minimum longitude in degrees.
/// * `max_lon` - The maximum longitude in degrees.
///
/// # Returns
///
/// A tuple `(x, y, z)` representing the Cartesian coordinates of the point on the sphere, 
/// scaled by 1000 to match Unreal Engine's coordinate system.
fn fibonacci_point(index: usize, num_points: usize, jitter: f64) -> (f64, f64, f64) {
    let phi = PI * (3.0 - (5.0_f64).sqrt());
    let z = 1.0 - (index as f64 + 0.5) / (num_points as f64) * 2.0;
    let radius = (1.0 - z * z).sqrt();
    let theta = phi * index as f64;

    let x = radius * theta.cos();
    let y = radius * theta.sin();

    // Apply jitter
    let mut rng = rand::thread_rng();
    let jitter_x = rng.gen::<f64>() - 0.5;
    let jitter_y = rng.gen::<f64>() - 0.5;
    let jitter_z = rng.gen::<f64>() - 0.5;

    (
        x + jitter * jitter_x * 0.1,
        y + jitter * jitter_y * 0.1,
        z + jitter * jitter_z * 0.1
    )
}

/// Generates a Fibonacci sphere with the specified number of points.
///
/// This function creates a set of points distributed on a sphere using the Fibonacci sphere algorithm.
/// It utilizes multi-threading to speed up the point generation process.
///
/// # Arguments
///
/// * `num_samples` - The number of points to generate on the sphere.
/// * `min_latitude` - The minimum latitude in degrees.
/// * `max_latitude` - The maximum latitude in degrees.
/// * `min_longitude` - The minimum longitude in degrees.
/// * `max_longitude` - The maximum longitude in degrees.
/// * `seed` - A seed value for pseudo-random number generation.
///
/// # Returns
///
/// A `Result` containing a vector of `(f64, f64, f64)` tuples representing the x, y, and z 
/// coordinates of the generated points, or an `io::Error` if there was an issue writing to the output file.
///
/// # Note
///
/// This function also writes the generated points and execution time to a file named "output.txt".
pub fn generate_fibonacci_sphere(num_samples: usize, jitter: f64) -> io::Result<Vec<(f64, f64, f64)>> {
    let start_time = Instant::now();

    let points: Vec<(f64, f64, f64)> = (0..num_samples)
        .into_iter()
        .map(|i| {
            let (x, y, z) = fibonacci_point(i, num_samples, jitter);
            let (nx, ny, nz) = normalize_to_sphere((x, y, z));
            (nx * 1000.0, ny * 1000.0, nz * 1000.0)
        })
        .collect();

    let duration = start_time.elapsed();

    let mut file = File::create("raw_points.txt")?;
    writeln!(file, "Generated points:")?;
    writeln!(file, "Time elapsed: {:?}", duration)?;
    for point in &points {
        writeln!(file, "{:?}", point)?;
    }

    Ok(points)
}