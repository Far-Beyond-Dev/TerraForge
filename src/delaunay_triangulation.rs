use spade::{DelaunayTriangulation, Point2, Triangulation};
use spade::handles::VoronoiVertex;
use std::io::{self, Write};
use std::fs::File;

/// Performs a stereographic projection of a 3D point onto a 2D plane.
///
/// This function projects a point from the surface of a sphere onto a plane 
/// using stereographic projection. It's adjusted for Unreal Engine coordinates where z is up.
///
/// # Arguments
///
/// * `x` - The x-coordinate of the 3D point.
/// * `y` - The y-coordinate of the 3D point.
/// * `z` - The z-coordinate of the 3D point.
///
/// # Returns
///
/// A `Point2<f64>` representing the projected 2D point.
fn stereographic_projection(x: f64, y: f64, z: f64) -> Point2<f64> {
    // Adjust for Unreal Engine coordinates (z is up)
    let scale = 1.0 / (1.0 + y);
    Point2::new(x * scale, -z * scale)
}

/// Performs an inverse stereographic projection of a 2D point back onto a 3D sphere.
///
/// This function takes a point on a 2D plane and projects it back onto the surface of a unit sphere.
/// It's adjusted for Unreal Engine coordinates where z is up.
///
/// # Arguments
///
/// * `point` - A `Point2<f64>` representing the 2D point to be projected.
///
/// # Returns
///
/// A tuple `(x, y, z)` representing the 3D point on the sphere's surface.
fn inverse_stereographic_projection(point: Point2<f64>) -> (f64, f64, f64) {
    let x = point.x;
    let z = -point.y;  // Adjust for Unreal Engine coordinates
    let x2z2 = x*x + z*z;
    let scale = 2.0 / (x2z2 + 1.0);
    let y = (x2z2 - 1.0) / (x2z2 + 1.0);
    (x * scale, y, z * scale)
}

/// Calculates the spherical circumcenter of a triangle on a sphere.
///
/// Given three points on a sphere, this function calculates the center of the circle 
/// that passes through all three points on the sphere's surface.
///
/// # Arguments
///
/// * `a` - A tuple `(x, y, z)` representing the first point of the triangle.
/// * `b` - A tuple `(x, y, z)` representing the second point of the triangle.
/// * `c` - A tuple `(x, y, z)` representing the third point of the triangle.
///
/// # Returns
///
/// A tuple `(x, y, z)` representing the coordinates of the spherical circumcenter.
fn calculate_spherical_circumcenter(a: (f64, f64, f64), b: (f64, f64, f64), c: (f64, f64, f64)) -> (f64, f64, f64) {
    // Cross product of (b-a) and (c-a)
    let normal = (
        (b.1 - a.1) * (c.2 - a.2) - (b.2 - a.2) * (c.1 - a.1),
        (b.2 - a.2) * (c.0 - a.0) - (b.0 - a.0) * (c.2 - a.2),
        (b.0 - a.0) * (c.1 - a.1) - (b.1 - a.1) * (c.0 - a.0)
    );
    
    // Normalize
    let length = (normal.0 * normal.0 + normal.1 * normal.1 + normal.2 * normal.2).sqrt();
    (normal.0 / length, normal.1 / length, normal.2 / length)
}

/// Creates a spherical Voronoi diagram using Delaunay triangulation.
///
/// This function takes a set of 3D points on a sphere, projects them onto a 2D plane using 
/// stereographic projection, creates a Delaunay triangulation, and then adds the south pole 
/// back into the mesh.
///
/// # Arguments
///
/// * `points` - A vector of `(f64, f64, f64)` tuples representing 3D points on a sphere.
///
/// # Returns
///
/// A `DelaunayTriangulation<Point2<f64>>` representing the Delaunay triangulation of the projected points.
pub fn create_spherical_voronoi(points: Vec<(f64, f64, f64)>) -> DelaunayTriangulation<Point2<f64>> {
    // Project points to 2D
    let projected_points: Vec<Point2<f64>> = points
        .iter()
        .map(|&(x, y, z)| stereographic_projection(x, y, z))
        .collect();

    // Create Delaunay triangulation
    let mut triangulation = DelaunayTriangulation::<Point2<f64>>::new();
    for point in projected_points {
        triangulation.insert(point).expect("Failed to insert point");
    }

    // Stitch south pole
    let south_pole = Point2::new(0.0, 0.0);
    triangulation.insert(south_pole).expect("Failed to insert south pole");

    triangulation
}

/// Prints the edges of the Voronoi diagram to a file.
///
/// This function calculates and writes the edges of the Voronoi diagram to a file named "voronoi_edges.txt".
/// The edges are represented as debug lines in a format suitable for visualization in Unreal Engine.
///
/// # Arguments
///
/// * `triangulation` - A reference to the `DelaunayTriangulation<Point2<f64>>` object.
///
/// # Returns
///
/// A `std::io::Result<()>`, which is `Ok(())` if the file was written successfully, 
/// or an `Err` containing the I/O error if there was a problem writing the file.
pub fn print_voronoi_edges(triangulation: &DelaunayTriangulation<Point2<f64>>) -> std::io::Result<()> {
    let mut file = File::create("voronoi_edges.txt")?;

    for voronoi_edge in triangulation.undirected_voronoi_edges() {
        let (from, to) = match voronoi_edge.vertices() {
            [VoronoiVertex::Inner(from_face), VoronoiVertex::Inner(to_face)] => {
                let from_vertices: Vec<_> = from_face.vertices().iter().map(|v| {
                    let p = v.position();
                    inverse_stereographic_projection(p)
                }).collect();
                
                let to_vertices: Vec<_> = to_face.vertices().iter().map(|v| {
                    let p = v.position();
                    inverse_stereographic_projection(p)
                }).collect();

                let from_3d = calculate_spherical_circumcenter(from_vertices[0], from_vertices[1], from_vertices[2]);
                let to_3d = calculate_spherical_circumcenter(to_vertices[0], to_vertices[1], to_vertices[2]);

                (from_3d, to_3d)
            },
            _ => continue, // Skip edges that go to infinity
        };

        // Adjust coordinates for Unreal Engine scale by 1000
        let from_unreal = (from.0 * 1000.0, from.2 * 1000.0, -from.1 * 1000.0);
        let to_unreal = (to.0 * 1000.0, to.2 * 1000.0, -to.1 * 1000.0);

        writeln!(file, "DrawDebugLine(GetWorld(), FVector({:.4}, {:.4}, {:.4}), FVector({:.4}, {:.4}, {:.4}), FColor::Blue, true, -1.0f, 0, 2.0f);",
            from_unreal.0, from_unreal.1, from_unreal.2,
            to_unreal.0, to_unreal.1, to_unreal.2
        )?;
    }

    Ok(())
}