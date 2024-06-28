use uuid::Uuid;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::thread;
use rand::{SeedableRng, Rng};
use rand::rngs::StdRng;
use std::f64::consts::PI;
use std::time::{Duration, Instant};

// Struct representing a Galaxy
#[derive(Debug)]
struct Galaxy {
    guid: Uuid,
    position: (f64, f64, f64),
    velocity: (f64, f64, f64),
    a: f64, // Semi-major axis
    b: f64, // Semi-minor axis
    T: f64, // Orbital period
    inclination: f64, // Inclination angle
    ascending_node: f64, // Longitude of ascending node
    time_offset: f64, // Initial time offset for orbit calculation
}

// Function to generate a deterministic GUID (UUID) from a seed value
fn generate_guid_from_seed(seed: u64) -> Uuid {
    let mut hasher = DefaultHasher::new();
    seed.hash(&mut hasher);
    let guid_bytes = hasher.finish().to_ne_bytes();
    
    // Ensure guid_bytes is extended to 16 bytes if needed
    let mut full_bytes = [0; 16];
    full_bytes[..guid_bytes.len()].copy_from_slice(&guid_bytes);
    
    Uuid::from_bytes(full_bytes)
}

// Generate a top-level universe seed (GUID)
fn generate_universe_seed() -> Uuid {
    Uuid::new_v4()
}

// Generate a GUID for a galaxy based on its coordinates and universe seed
fn generate_galaxy_guid(universe_seed: Uuid, coords: (f64, f64, f64)) -> Uuid {
    let mut hasher = DefaultHasher::new();
    universe_seed.hash(&mut hasher);
    coords.0.to_bits().hash(&mut hasher);
    coords.1.to_bits().hash(&mut hasher);
    coords.2.to_bits().hash(&mut hasher);
    let guid_bytes = hasher.finish().to_ne_bytes();
    
    // Ensure guid_bytes is extended to 16 bytes if needed
    let mut full_bytes = [0; 16];
    full_bytes[..guid_bytes.len()].copy_from_slice(&guid_bytes);
    
    Uuid::from_bytes(full_bytes)
}

// Function to generate galaxy parameters from GUID
fn generate_galaxy_parameters(guid: Uuid) -> (f64, f64, f64, f64, f64, f64) {
    let seed: [u8; 16] = *guid.as_bytes();
    let mut seed_32: [u8; 32] = [0; 32];
    seed_32[..16].copy_from_slice(&seed);
    let mut rng: StdRng = SeedableRng::from_seed(seed_32);
    
    let a = rng.gen_range(10.0..50.0); // Semi-major axis
    let b = rng.gen_range(5.0..25.0); // Semi-minor axis
    let T = rng.gen_range(100.0..500.0); // Orbital period
    let inclination = rng.gen_range(0.0..PI); // Inclination angle
    let ascending_node = rng.gen_range(0.0..(2.0 * PI)); // Longitude of ascending node
    let time_offset = rng.gen_range(0.0..T); // Initial time offset for orbit calculation
    
    (a, b, T, inclination, ascending_node, time_offset)
}

// Function to update position based on elliptical orbit
fn update_position(galaxy: &mut Galaxy, time: f64) {
    let theta: f64 = 2.0 * PI * (time + galaxy.time_offset) / galaxy.T;
    let x: f64 = galaxy.a * theta.cos();
    let y: f64 = galaxy.b * theta.sin();
    
    // Rotate by inclination and ascending node
    let cos_i: f64 = galaxy.inclination.cos();
    let sin_i: f64 = galaxy.inclination.sin();
    let cos_O: f64 = galaxy.ascending_node.cos();
    let sin_O: f64 = galaxy.ascending_node.sin();

    let x_rot: f64 = x * cos_O - y * cos_i * sin_O;
    let y_rot: f64 = x * sin_O + y * cos_i * cos_O;
    let z_rot: f64 = y * sin_i;

    galaxy.position = (x_rot, y_rot, z_rot);
}

// Function to generate galaxies using the universe seed
fn generate_galaxies(universe_seed: Uuid) -> Vec<Galaxy> {
    let seed: [u8; 16] = *universe_seed.as_bytes();
    let mut seed_32: [u8; 32] = [0; 32];
    seed_32[..16].copy_from_slice(&seed);
    let mut rng: StdRng = SeedableRng::from_seed(seed_32);

    // Generate the number of galaxies
    let num_galaxies: i32 = rng.gen_range(1000000..5000000); // For example, generate between 300 and 500 thousand galaxies

    // Generate galaxies
    (0..num_galaxies).map(|_| {
        // Generate initial position
        let position = (
            rng.gen_range(-100.0..100.0),
            rng.gen_range(-100.0..100.0),
            rng.gen_range(-100.0..100.0),
        );

        // Generate GUID based on position and universe seed
        let guid = generate_galaxy_guid(universe_seed, position);

        // Generate orbital parameters
        let (a, b, T, inclination, ascending_node, time_offset) = generate_galaxy_parameters(guid);

        Galaxy {
            guid,
            position,
            velocity: (0.0, 0.0, 0.0), // Initially, velocity is not used
            a,
            b,
            T,
            inclination,
            ascending_node,
            time_offset,
        }
    }).collect()
}

pub fn simulate() {
    // Generate the universe seed
    let start = Instant::now();
    let universe_seed = generate_guid_from_seed(123);
    let duration = start.elapsed();
    println!("Generating universe seed took: {:?}", duration);

    // Generate galaxies using the universe seed
    let start = Instant::now();
    let mut galaxies = generate_galaxies(universe_seed);
    let duration = start.elapsed();
    println!("Generating galaxies took: {:?}", duration);

    // Simulation loop
    let mut time = 0.0;
    loop {
        // Sleep for 1 second
        let start = Instant::now();
        thread::sleep(Duration::from_secs(1));
        let duration = start.elapsed();
        println!("Sleeping for 1 second took: {:?}", duration);

        // Update galaxy positions
        time += 1.0;
        let start = Instant::now();
        for galaxy in &mut galaxies {
            update_position(galaxy, time);
        }
        let duration = start.elapsed();
        println!("Updating galaxy positions took: {:?}", duration);
        println!("Updated {} objects", galaxies.len().to_string());
        println!("-----------------");
    }
}
