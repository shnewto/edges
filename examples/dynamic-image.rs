use edges::Edges;
use raqote::{DrawOptions, DrawTarget, PathBuilder, SolidSource, Source, StrokeStyle};
use std::path::Path;

fn main() {
    draw_png("car.png");
    draw_png("lines.png");
    draw_png("terrain.png");
}

fn draw_png(img_path: &str) {
    let image = image::open(Path::new(&format!("assets/{img_path}"))).unwrap();
    // get the image's edges
    let edges = Edges::from(&image);

    let scale = 8;
    let (width, height) = (
        i32::try_from(image.width() * scale).expect("Image to wide."),
        i32::try_from(image.height() * scale).expect("Image to tall."),
    );

    // draw the edges to a png
    let mut dt = DrawTarget::new(width, height);
    let mut pb = PathBuilder::new();

    let mut edges_iter = edges.single_image_edge_raw().unwrap().into_iter();
    let first_edge = edges_iter.next().unwrap();
    pb.move_to((first_edge.x * scale) as f32, (first_edge.y * scale) as f32);
    for edge in edges_iter {
        pb.line_to((edge.x * scale) as f32, (edge.y * scale) as f32);
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
            width: scale as f32,
            ..StrokeStyle::default()
        },
        &DrawOptions::new(),
    );

    dt.write_png(format!("edges-{img_path}")).unwrap();
    _ = open::that(format!("edges-{img_path}"));
}
