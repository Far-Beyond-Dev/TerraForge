use uuid::Uuid;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::thread;
use std::time::Duration;

// Struct representing a Galaxy
#[derive(Debug)]
struct Galaxy {
    guid: Uuid,
    position: (f64, f64, f64),
    velocity: (f64, f64, f64),
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
    // Generate a UUID from a random seed or predefined seed logic
    // For example:
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

// Example function to handle rendering nearby objects
fn render_nearby_objects(player_position: (f64, f64, f64), object_positions: &[(Uuid, (f64, f64, f64))], render_distance: f64) -> Vec<Uuid> {
    object_positions
        .iter()
        .filter(|(_, pos)| {
            let distance_sq = (pos.0 - player_position.0).powi(2) +
                              (pos.1 - player_position.1).powi(2) +
                              (pos.2 - player_position.2).powi(2);
            distance_sq <= render_distance.powi(2)
        })
        .map(|(guid, _)| *guid)
        .collect()
}

// Example function to simulate object movement based on initial conditions
fn simulate_object_movement(initial_position: (f64, f64, f64), velocity: (f64, f64, f64), time_delta: f64) -> (f64, f64, f64) {
    let new_position = (
        initial_position.0 + velocity.0 * time_delta,
        initial_position.1 + velocity.1 * time_delta,
        initial_position.2 + velocity.2 * time_delta,
    );
    new_position
}

fn simulate() {
    // Generate the universe seed
    let universe_seed = generate_guid_from_seed(123);

    // Example galaxies
    let mut galaxies = vec![
        Galaxy {
            guid: generate_galaxy_guid(universe_seed, (10.0, 20.0, 30.0)),
            position: (10.0, 20.0, 30.0),
            velocity: (0.1, 0.2, 0.3),
        },
        Galaxy {
            guid: generate_galaxy_guid(universe_seed, (50.0, 60.0, 70.0)),
            position: (50.0, 60.0, 70.0),
            velocity: (0.2, 0.3, 0.1),
        },
    ];

    // Simulation loop
    loop {
        // Print galaxy positions
        for galaxy in &galaxies {
            println!("Galaxy {:?} position: {:?}", galaxy.guid, galaxy.position);
        }

        // Sleep for 1 second
        thread::sleep(Duration::from_secs(1));

        // Update galaxy positions
        for galaxy in &mut galaxies {
            galaxy.position = simulate_object_movement(galaxy.position, galaxy.velocity, 1.0); // time_delta is 1 second
        }

        println!("-----------------");
    }
}