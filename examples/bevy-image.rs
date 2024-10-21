use bevy_render::{
    prelude::Image,
    render_asset::RenderAssetUsages,
    texture::{CompressedImageFormats, ImageSampler, ImageType},
};
use edges::Edges;
use raqote::{DrawOptions, DrawTarget, PathBuilder, SolidSource, Source, StrokeStyle};
// in an actual bevy app, you wouldn't need all this building an Image from scratch logic,
// it'd be something closer to this:
// `let image = image_assets.get(handle).unwrap();`
//  let e = Edges::from(image);
fn main() {
    // read png as bytes and manually construct a bevy Image

    let boulders = Image::from_buffer(
        include_bytes!("../assets/boulders.png"),
        ImageType::Extension("png"),
        CompressedImageFormats::default(),
        true,
        ImageSampler::default(),
        RenderAssetUsages::default(),
    )
    .unwrap();

    let more_lines = Image::from_buffer(
        include_bytes!("../assets/more-lines.png"),
        ImageType::Extension("png"),
        CompressedImageFormats::default(),
        true,
        ImageSampler::default(),
        RenderAssetUsages::default(),
    )
    .unwrap();

    draw_png(boulders, "boulders");
    draw_png(more_lines, "more-lines");
}

fn draw_png(image: Image, img_path: &str) {
    let scale = 8;
    let (width, height) = (
        i32::try_from(image.width()).expect("Image to wide.") * scale,
        i32::try_from(image.height()).expect("Image to tall.") * scale,
    );
    // get the image's edges
    let edges = Edges::from(image);

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

    dt.write_png(format!("edges-{img_path}.png")).unwrap();
    _ = open::that(format!("edges-{img_path}.png"));
}
