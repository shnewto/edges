# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.7.0](https://github.com/shnewto/edges/compare/0.6.0...0.7.0) - 2025-01-05

### Added

- public `anchor` module ([f2c4560](https://github.com/shnewto/edges/commit/f3c4560ca1bd5209155d632e36e0be374b915947)).
  - Enumeration `Anchor`:
    - `Center` for translate points by center of rect.
    - `Vertical` | `Horizontal` center to translate points by center of line.
    - `Absolute` center for translate points by center of her bounding box.
- `utils` module marked public ([f2c4560](https://github.com/shnewto/edges/commit/f3c4560ca1bd5209155d632e36e0be374b915947)).
- `utils::center_of` function for get center of polygon ([03165ca](https://github.com/shnewto/edges/commit/03165cabc3a9b384703c47e3d1855d357c6380e7)).
- `utils::bounding_box` function for get bounding box of polygon ([f2c4560](https://github.com/shnewto/edges/commit/f3c4560ca1bd5209155d632e36e0be374b915947)).

### Changed

- Upgrade dependencies: [`binary_image`] 0.6 ([159761a](https://github.com/shnewto/edges/commit/159761a1be6fb494d3d2295f4885e535d65cccc2)).
- `utils::translate` function moved to `anchor::Anchor` implementation ([f2c4560](https://github.com/shnewto/edges/commit/f3c4560ca1bd5209155d632e36e0be374b915947)).
- `utils::translate_objects` function renamed to `translate_polygons`
  and moved to `anchor::Anchor` ([f2c4560](https://github.com/shnewto/edges/commit/f3c4560ca1bd5209155d632e36e0be374b915947)).

## [0.6.0](https://github.com/shnewto/edges/compare/0.5.1...0.6.0) - 2024-12-14

### Added

- Feature `parallel`, which depends on [`rayon`]
  ([6dab192](https://github.com/shnewto/edges/commit/6dab192ee37572bcd6d35fa14421d401f574f47b)).
- External public crate [`binary_image`]
  ([076d53b](https://github.com/shnewto/edges/commit/076d53b636d31583d41f34b4fdc55267478de085)).
- Iterator `iter::Edges` for `Edges`
  ([10397be](https://github.com/shnewto/edges/commit/10397beb6557fb239be9f254c31fa227389d47c2)).

### Changed

- [`rayon`] marked as optional dependencies
  ([6dab192](https://github.com/shnewto/edges/commit/6dab192ee37572bcd6d35fa14421d401f574f47b)).
- Functions factored out.
  - `translate`
  - `translate_objects`
- Now `UVec2`, `Vec2` is inner
  ([ba68b6c](https://github.com/shnewto/edges/commit/ba68b6c1fc8fb49c04697c6e20694f1deaea4fcc)).
- Function names have been shortened
  ([0266d7e](https://github.com/shnewto/edges/commit/0266d7e0c6a0c9b99ab5eac69e20eccfb9b25276)):
  - `single_image_edge_translated` to `single_translated`.
  - `single_image_edge_raw` to `single_raw`.
  - `multi_image_edge_translated` to `multi_translated`.
  - `multi_image_edge_raw` to `multi_raw`.
- Now `Edges` is unit
  ([10397be](https://github.com/shnewto/edges/commit/10397beb6557fb239be9f254c31fa227389d47c2)).
- Implementation of `From` for `bevy_image::prelude::Image`
  replaced by `TryFrom`
  ([10397be](https://github.com/shnewto/edges/commit/10397beb6557fb239be9f254c31fa227389d47c2)).

### Fixed

- Work with channels of pixels
  ([10397be](https://github.com/shnewto/edges/commit/10397beb6557fb239be9f254c31fa227389d47c2)).

### Removed

- Functions `image_edges`, `new`
  ([10397be](https://github.com/shnewto/edges/commit/10397beb6557fb239be9f254c31fa227389d47c2)).

## [0.5.1](https://github.com/shnewto/edges/compare/0.5.0...0.5.1) - 2024-12-04

### Added

- `CHANGELOG.md`, `REALESE.md` Files
  ([27c0309](https://github.com/shnewto/edges/commit/27c0309de7cf4cf1d9bb3939eebd2b073a3bb81e))
  ([88444d1](https://github.com/shnewto/edges/commit/88444d1ef60a61c1fcfb8c30cba919353de8010b)).

### Changed

- Refactored inner logic without changes in API
  ([ffadade](https://github.com/shnewto/edges/commit/ffadade2004535a2fb0930412f4b95586b0f8383)).
- Updated docs ([dbdbca7](https://github.com/shnewto/edges/commit/dbdbca79871fcb6928344860eb5586617e0beacd)).

## [0.5.0](https://github.com/shnewto/edges/compare/0.4.0...0.5.0) - 2024-12-04

### Added

- Implementation of `Clone`, `Into<Vec<Vec<UVec2>>` traits for `Edges`
  ([662f42c](https://github.com/shnewto/edges/commit/662f42c7e1d478a66b62555801bf6f85ad6f36d4))
  ([0fdc732](https://github.com/shnewto/edges/commit/0fdc7329ddf46bb4e4e60e16348c788c8de1b7e7)).
- `translate_objects` method to `Edges` for replace flag `translate` ([1bb608c](https://github.com/shnewto/edges/commit/1bb608c39711da1e45cde17d1bb988076672b80d)).

### Changed

- Upgrade dependencies: Bevy 0.15 ([ac1a5a5](https://github.com/shnewto/edges/commit/ac1a5a5b7ed056723d4727bbb3a2bd11def3c70f)).
  - Dependency on [`bevy_render`](https://crates.io/crates/bevy_render)
    replaced with [`bevy_image`](https://crates.io/crates/bevy_image).
- Function `multi_image_edges_raw` renamed to `multi_image_edge_raw` ([1bb608c](https://github.com/shnewto/edges/commit/1bb608c39711da1e45cde17d1bb988076672b80d)).
- Now functions `image_edges`, `multi_image_edges_raw`, `single_image_edge_raw`
  returns `Vec<Vec<UVec2>>` ([1bb608c](https://github.com/shnewto/edges/commit/1bb608c39711da1e45cde17d1bb988076672b80d)).

### Removed

- `translate` flag for `image_edges` function ([1bb608c](https://github.com/shnewto/edges/commit/1bb608c39711da1e45cde17d1bb988076672b80d)).

## [0.4.0](https://github.com/shnewto/edges/compare/0.3.4...0.4.0) - 2024-11-12

### Added

- Reworked edges search algorithm for work with diagonal lines too
  ([60cb046](https://github.com/shnewto/edges/commit/60cb046930b899926877e62dd5700dfc37ec32b8))
  ([c4ca604](https://github.com/shnewto/edges/commit/c4ca604e3cde1a40dffc8d92b2dd378b951335f9))
  ([06012b7](https://github.com/shnewto/edges/commit/06012b753aeb91da70cfbb75a47b3d4023482cb4))
  ([ac0c3dd](https://github.com/shnewto/edges/commit/ac0c3ddcde59c036fa55c59e3a9880d77e348ae2))
  ([8d38555](https://github.com/shnewto/edges/commit/8d38555bfa9252a8fe70c799fc68653780641232)).
- Feature `glam-latest` for those who use [`glam`](https://crates.io/crates/glam)
  ([e7ab40a](https://github.com/shnewto/edges/commit/e7ab40a25e933bce24380bc090dea503b0bc93d4)).
- Crate [`rayon`](https://crates.io/crates/rayon) to dependencies for parallelism
  ([df77244](https://github.com/shnewto/edges/commit/df77244fc05604334285ce426b7186030a61ee7b)).
- Method `new` to take `Edges` from any data
  ([3f5052f](https://github.com/shnewto/edges/commit/3f5052fbe720eee8011e26e617b737f4577a28d7)).

### Changed

- Now `Edges` is structure ([3f5052f](https://github.com/shnewto/edges/commit/3f5052fbe720eee8011e26e617b737f4577a28d7)).
- Method `translate_vec` renamed to `translate`
  ([5573751](https://github.com/shnewto/edges/commit/55737517a246b207e87c8abf99d6fbe3d3786e0a)).
- Dependency on [`bevy`](https://crates.io/crates/bevy) replaced with
  [`bevy_math`] and [`bevy_render`]
  ([e7ab40a](https://github.com/shnewto/edges/commit/e7ab40a25e933bce24380bc090dea503b0bc93d4)).

### Fixed

- Collecting of inner edges (#3)
  ([60cb046]) ([c4ca604]) ([06012b7]) ([ac0c3dd]) ([8d38555]).

### Removed

- Crates from dependencies
  ([334361c](https://github.com/shnewto/edges/commit/334361c7c1acca3e3e548b679046c5117f087de2))
  ([e7ab40a](https://github.com/shnewto/edges/commit/e7ab40a25e933bce24380bc090dea503b0bc93d4)):
  - [`hashbrown`]
  - [`mashmap`]
  - [`ordered-float`]
  - [`thiserror`]
- Method `march_edges` replaced by `new`
  ([3f5052f](https://github.com/shnewto/edges/commit/3f5052fbe720eee8011e26e617b737f4577a28d7)).

## [0.3.4](https://github.com/shnewto/edges/compare/0.3.3...0.3.4) - 2024-08-13

### Changed

- Upgrade dependencies: [`glam`] 0.27.0 ([1912e24](https://github.com/shnewto/edges/commit/1912e24647e885c9340c7667f0f8967bca670456)).

## [0.3.3](https://github.com/shnewto/edges/compare/0.3.2...0.3.3) - 2024-07-08

### Changed

- Upgrade dependencies:
  [`image`] 0.25,
  [`bevy`] 0.14
  ([92acaa1](https://github.com/shnewto/edges/commit/92acaa1a3be42b085bf2fe9c4e258662254edcf5)).

## [0.3.2](https://github.com/shnewto/edges/compare/0.3.1...0.3.2) - 2024-05-13

### Added

- Crates to dependencies
  ([c508c2a](https://github.com/shnewto/edges/commit/c508c2a6816593efbeaf807e5af1e06c9f165376)):
  - [`hashbrown`]
  - [`mashmap`]
  - [`ordered-float`]

### Changed

- Reworked the algorithm for sorting points in drawing order ([c508c2a](https://github.com/shnewto/edges/commit/c508c2a6816593efbeaf807e5af1e06c9f165376)).

### Fixed

- Incorrect drawing order for complex images (#1)
  ([c508c2a](https://github.com/shnewto/edges/commit/c508c2a6816593efbeaf807e5af1e06c9f165376)).

## [0.3.1](https://github.com/shnewto/edges/compare/0.3.0...0.3.1) - 2024-03-05

### Fixed

- Implementation of trait `Debug` for `Edges` ([022e575](https://github.com/shnewto/edges/commit/022e57560681a4e92bbbd3d96505a1548e31923d)).
- Tests module `cfg` ([022e575](https://github.com/shnewto/edges/commit/022e57560681a4e92bbbd3d96505a1548e31923d)).

## [0.3.0](https://github.com/shnewto/edges/compare/0.2.0...0.3.0) - 2024-03-04

### Changed

- Methods `march_edges`, `translate_vec` are provided as associated functions
  ([faee752](https://github.com/shnewto/edges/commit/faee752f042fcd54f90ce13e74516691be7dbc0c)).

## [0.2.0](https://github.com/shnewto/edges/compare/0.1.0...0.2.0) - 2024-03-04

### Added

- Implementation of
  `Debug`, `From<bevy::prelude::Image>`, `From<image::DynamicImage>` etc.
  traits for `Edges` ([006b40c](https://github.com/shnewto/edges/commit/006b40c7ff9557dac4166b04aa8e2fee7ce1bedc))

### Changed

- Now `Edges` is enumeration ([a945451](https://github.com/shnewto/edges/commit/a945451a4649cb61fee9175fae478fb310060304)).
- Methods `image_to_edges` renamed to `image_edges`
  ([83d3680](https://github.com/shnewto/edges/commit/83d3680243df382faf0b5cf605b499e204ce4249)).

### Removed

- Implementation of
  `TryFrom<bevy::prelude::Image>`, `TryFrom<image::DynamicImage>` etc.
  traits for `Edges` ([006b40c](https://github.com/shnewto/edges/commit/006b40c7ff9557dac4166b04aa8e2fee7ce1bedc)).

[`bevy`]: https://crates.io/crates/bevy
[`bevy_math`]: https://crates.io/crates/bevy_math
[`bevy_render`]: https://crates.io/crates/bevy_render
[`image`]: https://crates.io/crates/image
[`rayon`]: https://crates.io/crates/rayon
[`binary_image`]: https://crates.io/crates/binary_image
[`thiserror`]: https://crates.io/crates/thiserror
[`hashbrown`]: https://crates.io/crates/hashbrown
[`mashmap`]: https://crates.io/crates/mashmap
[`ordered-float`]: https://crates.io/crates/ordered-float
