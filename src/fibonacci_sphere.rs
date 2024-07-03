use std::f64::consts::PI;

pub fn generate_fibonacci_sphere(
    num_samples: usize,
    min_latitude: f64,
    max_latitude: f64,
    min_longitude: f64,
    max_longitude: f64,
    seed: f64,
    radius: f64
) -> Vec<(f64, f64, f64)> {
    let golden_ratio = (1.0 + 5.0_f64.sqrt()) / 2.0;
    let angle_increment = PI * 2.0 * golden_ratio;

    (0..num_samples).map(|i| {
        let t = (i as f64 + seed) / num_samples as f64;
        
        // Map t to latitude range
        let latitude = (max_latitude - min_latitude) * t + min_latitude;
        
        // Generate longitude
        let longitude = ((angle_increment * i as f64) % (2.0 * PI)) / PI * 180.0;
        let longitude = longitude * (max_longitude - min_longitude) / 360.0 + min_longitude;

        // Convert to radians for spherical coordinates
        let lat_rad = latitude.to_radians();
        let lon_rad = longitude.to_radians();

        // Convert to Cartesian coordinates
        let x = radius * lat_rad.cos() * lon_rad.cos();
        let y = radius * lat_rad.cos() * lon_rad.sin();
        let z = radius * lat_rad.sin();

        (x, y, z)
    }).collect()
}