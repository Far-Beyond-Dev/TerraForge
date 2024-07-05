use rand::{SeedableRng, Rng};
use rand::rngs::StdRng;
use std::f64::consts::PI;

// Function to generate Fibonacci sphere points
pub fn fibonacci_sphere(samples: usize) -> Vec<[f64; 3]> {
    let mut points = Vec::with_capacity(samples);
    let phi = PI * (3.0 - (5.0f64).sqrt());

    for i in 0..samples {
        let y = 1.0 - (i as f64 / (samples as f64 - 1.0)) * 2.0;
        let radius = (1.0 - y * y).sqrt();
        let theta = phi * i as f64;

        let x = radius * theta.cos();
        let z = radius * theta.sin();

        points.push([x, y, z]);
    }

    points
}

// Weather conditions struct
#[derive(Debug, Clone)]
struct WeatherCondition {
    temperature: f64,
    humidity: f64,
    wind_speed: f64,
    wind_direction: f64, // In degrees, 0-360
    precipitation: f64,  // In mm/h
}

// Function to generate initial weather conditions
pub fn generate_weather_conditions(seed: u64, samples: usize) -> Vec<WeatherCondition> {
    let mut rng = StdRng::seed_from_u64(seed);
    let mut conditions = Vec::with_capacity(samples);

    for _ in 0..samples {
        let temperature = rng.gen_range(-30.0..50.0); // Example temperature range in Celsius
        let humidity = rng.gen_range(0.0..100.0); // Example humidity range in percentage
        let wind_speed = rng.gen_range(0.0..30.0); // Example wind speed in m/s
        let wind_direction = rng.gen_range(0.0..360.0); // Wind direction in degrees
        let precipitation = calculate_precipitation(temperature, humidity); // Precipitation in mm/h

        conditions.push(WeatherCondition {
            temperature,
            humidity,
            wind_speed,
            wind_direction,
            precipitation,
        });
    }

    conditions
}

// Function to calculate precipitation based on temperature and humidity
pub fn calculate_precipitation(temperature: f64, humidity: f64) -> f64 {
    if temperature > 0.0 && humidity > 50.0 {
        humidity / 100.0 * (temperature / 30.0) * 10.0 // Simplified precipitation calculation
    } else {
        0.0
    }
}

// Function to simulate weather evolution
pub fn simulate_weather(conditions: &mut Vec<WeatherCondition>, time_step: usize) {
    for condition in conditions.iter_mut() {
        condition.temperature += (time_step as f64 * 0.1) % 5.0 - 2.5; // Simplified temperature change
        condition.humidity += (time_step as f64 * 0.05) % 10.0 - 5.0; // Simplified humidity change
        condition.wind_speed += (time_step as f64 * 0.02) % 1.0 - 0.5; // Simplified wind speed change
        condition.wind_direction = (condition.wind_direction + (time_step as f64 * 5.0) % 360.0) % 360.0; // Wind direction change
        condition.precipitation = calculate_precipitation(condition.temperature, condition.humidity); // Update precipitation

        // Clamp values to realistic ranges
        condition.temperature = condition.temperature.clamp(-30.0, 50.0);
        condition.humidity = condition.humidity.clamp(0.0, 100.0);
        condition.wind_speed = condition.wind_speed.clamp(0.0, 30.0);
        condition.precipitation = condition.precipitation.clamp(0.0, 100.0);
    }
}

// Function to introduce global weather events
pub fn global_weather_event(conditions: &mut Vec<WeatherCondition>, event_type: &str) {
    match event_type {
        "storm" => {
            for condition in conditions.iter_mut() {
                condition.wind_speed += 10.0; // Increase wind speed
                condition.wind_direction = (condition.wind_direction + 45.0) % 360.0; // Change wind direction
                condition.humidity += 20.0; // Increase humidity
                condition.precipitation = calculate_precipitation(condition.temperature, condition.humidity); // Update precipitation
            }
        }
        "heatwave" => {
            for condition in conditions.iter_mut() {
                condition.temperature += 10.0; // Increase temperature
                condition.humidity -= 10.0; // Decrease humidity
                condition.precipitation = calculate_precipitation(condition.temperature, condition.humidity); // Update precipitation
            }
        }
        _ => {}
    }

    // Clamp values to realistic ranges
    for condition in conditions.iter_mut() {
        condition.temperature = condition.temperature.clamp(-30.0, 50.0);
        condition.humidity = condition.humidity.clamp(0.0, 100.0);
        condition.wind_speed = condition.wind_speed.clamp(0.0, 30.0);
        condition.precipitation = condition.precipitation.clamp(0.0, 100.0);
    }
}

// fn main() {
//     // Example UUID and orbital data
//     let planet_uuid = 123456789u64;
//     let num_samples = 1000; // Number of points on the Fibonacci sphere
// 
//     // Generate Fibonacci sphere points
//     let points = fibonacci_sphere(num_samples);
// 
//     // Generate initial weather conditions
//     let mut weather_conditions = generate_weather_conditions(planet_uuid, num_samples);
// 
//     // Simulate weather for 10 time steps
//     for time_step in 0..10 {
//         simulate_weather(&mut weather_conditions, time_step);
//     }
// 
//     // Introduce a global storm event
//     global_weather_event(&mut weather_conditions, "storm");
// 
//     // Print final weather conditions
//     for (i, condition) in weather_conditions.iter().enumerate() {
//         println!("Point {}: {:?}", i, condition);
//     }
// }
