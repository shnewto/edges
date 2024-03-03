use bevy::render::texture::{Image, ImageType};
use edges::Edges;

fn main() {
    // in an actual bevy app, you wouldn't need all this building an Image from scratch logic,
    // it'd be something closer to this:
    // `let image = image_assets.get(handle).unwrap();`
    //  let e = Edges::from(image);

    // read png as bytes and manually construct a bevy Image
    let image = Image::from_buffer(
        include_bytes!("../assets/car.png"), // buffer
        ImageType::Extension("png"),
        Default::default(),
        true, //
        Default::default(),
        Default::default(),
    );

    // get the image's edges
    let edges = Edges::from(image.unwrap());
    println!("{:#?}", edges);
}
