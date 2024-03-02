#![doc = include_str!("../README.md")]

mod edges;
mod error;

#[cfg(feature = "bevy")]
mod bevy;

pub use edges::Edges;
pub use glam::Vec2;
