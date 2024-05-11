use edges::Edges;
use raqote::*;
use std::path::Path;

fn main() {
    draw_png("car.png");
    draw_png("lines.png");
    draw_png("terrain.png");
}

fn draw_png(img_path: &str) {
    let image = &image::open(Path::new(&format!("assets/{}", img_path))).unwrap();
    let edges = Edges::from(image);
    let scale = 8;
    let (width, height) = (image.width() as i32 * scale, image.height() as i32 * scale);

    // draw the edges to a png
    let mut dt = DrawTarget::new(width, height);
    let mut pb = PathBuilder::new();

    let mut edges_iter = edges.single_image_edge_raw().into_iter();
    let first_edge = edges_iter.next().unwrap();
    pb.move_to(first_edge.x * scale as f32, first_edge.y * scale as f32);
    for edge in edges_iter {
        pb.line_to(edge.x * scale as f32, edge.y * scale as f32);
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

    dt.write_png(format!("edges-{}", img_path)).unwrap();
    _ = open::that(format!("edges-{}", img_path));
}
