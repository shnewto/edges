use crate::Edges;
use bevy_image::{prelude::Image, CompressedImageFormats, ImageSampler, ImageType};
use bevy_render::render_asset::RenderAssetUsages;
use std::path::Path;

#[test]
fn same_image_same_edges() {
    let dynamic_image = image::open(Path::new("assets/car.png")).unwrap();
    let dynamic_edges = Edges::from(dynamic_image);

    let bevy_image = Image::from_buffer(
        include_bytes!("../assets/car.png"), // buffer
        ImageType::Extension("png"),
        CompressedImageFormats::default(),
        true,
        ImageSampler::default(),
        RenderAssetUsages::default(),
    )
    .unwrap();
    let bevy_edges = Edges::from(bevy_image);

    assert_eq!(
        dynamic_edges.single_image_edge_raw(),
        bevy_edges.single_image_edge_raw()
    );
    assert_eq!(
        dynamic_edges.single_image_edge_translated(),
        bevy_edges.single_image_edge_translated()
    );
}

#[test]
fn same_images_same_edges() {
    let dynamic_image = image::open(Path::new("assets/boulders.png")).unwrap();
    let dynamic_edges = Edges::from(dynamic_image);

    let bevy_image = Image::from_buffer(
        include_bytes!("../assets/boulders.png"), // buffer
        ImageType::Extension("png"),
        CompressedImageFormats::default(),
        true,
        ImageSampler::default(),
        RenderAssetUsages::default(),
    )
    .unwrap();
    let bevy_edges = Edges::from(bevy_image);

    assert_eq!(
        dynamic_edges.multi_image_edge_raw(),
        bevy_edges.multi_image_edge_raw()
    );
    assert_eq!(
        dynamic_edges.multi_image_edge_translated(),
        bevy_edges.multi_image_edge_translated()
    );
}
