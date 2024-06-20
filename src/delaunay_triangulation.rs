use spade::{DelaunayTriangulation, Triangulation, Point2, InsertionError};

pub fn perform_triangulation(points: Vec<(f64, f64, f64)>) {//-> Result<DelaunayTriangulation<Point2<f64>>, InsertionError> {
    let mut triangulation: DelaunayTriangulation<_> = DelaunayTriangulation::new();

    for point in points {
        triangulation.insert(Point2::new(point.0, point.1));//?;
    }

    for face in triangulation.vertices() {
        // face is a FaceHandle
        // edges is an array containing 3 directed edge handles
        //   let edges = face.adjacent_edges();
        //   for edge in &edges {
        //     let from = edge.from();
        //    let to = edge.to();
        //    from and to are vertex handles
        //    println!("found an edge: {:?} -> {:?}", from, to);
        //    println!("Points.Add(Fvector2D({:?}, {:?}));", from.position().x, to.position().y);
        //   }

         // vertices is an array containing 3 vertex handles
        //let vertices = face.vertices();
        //for vertex in &vertices {
          //println!("Points.Add({:?}", vertex.position());
        //}
      }

        
      

    //println!("Triangulation: {:?}", triangulation);
    
    //Ok(triangulation)
}
