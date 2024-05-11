use bevy::{prelude::Image, render::texture::ImageType};
use edges::Edges;
use raqote::*;
// in an actual bevy app, you wouldn't need all this building an Image from scratch logic,
// it'd be something closer to this:
// `let image = image_assets.get(handle).unwrap();`
//  let e = Edges::from(image);
fn main() {
    // read png as bytes and manually construct a bevy Image

    let boulders = Image::from_buffer(
        include_bytes!("../assets/boulders.png"),
        ImageType::Extension("png"),
        Default::default(),
        true,
        Default::default(),
        Default::default(),
    )
    .unwrap();

    let more_lines = Image::from_buffer(
        include_bytes!("../assets/more-lines.png"),
        ImageType::Extension("png"),
        Default::default(),
        true,
        Default::default(),
        Default::default(),
    )
    .unwrap();

    draw_png(boulders, "boulders.png");
    draw_png(more_lines, "more-lines.png");
}

fn draw_png(image: Image, img_path: &str) {
    // get the image's edges
    let edges = Edges::from(image.clone());
    let scale = 8;
    let (width, height) = (image.width() as i32 * scale, image.height() as i32 * scale);

    // draw the edges to a png
    let mut dt = DrawTarget::new(width, height);

    let objects_iter = edges.multi_image_edges_raw().into_iter();

    for object in objects_iter {
        let mut pb = PathBuilder::new();
        let mut edges_iter = object.into_iter();

        if let Some(first_edge) = edges_iter.next() {
            pb.move_to(first_edge.x * scale as f32, first_edge.y * scale as f32);
            for edge in edges_iter {
                pb.line_to(edge.x * scale as f32, edge.y * scale as f32);
            }
        }

        let path = pb.finish();
        dt.stroke(
            &path,
            &Source::Solid(SolidSource {
                r: 0xff,
                g: 0xff,
                b: 0xff,
                a: 0xff,
            }),
            &StrokeStyle {
                width: 1.,
                ..StrokeStyle::default()
            },
            &DrawOptions::new(),
        );
    }

    dt.write_png(format!("edges-{}", img_path)).unwrap();
    _ = open::that(format!("edges-{}", img_path));
}
