use edges::Edges;
use std::path::Path;

fn main() {
    let image = image::open(Path::new("assets/car.png"));

    // get the image's edges
    let edges = Edges::from(image.unwrap());
    println!("{:#?}", edges);
}
