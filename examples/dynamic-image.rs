use edges::Edges;
use std::path::Path;

fn main() {
    let image = image::open(Path::new("assets/car.png"));
    let edges = Edges::from(image.unwrap());
    println!("{:#?}", edges.single_image_edge_translated());
}
