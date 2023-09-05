//! Rust bindings for the [namigator](https://github.com/namreeb/namigator) World of Warcraft pathfinding library.
//!
//! ```rust
//! # use namigator::{build_bvh, build_map, PathfindMap};
//! # fn t() -> Result<(), Box<dyn std::error::Error>> {
//! let data_path = "/WoW3.3.5/Data";
//! let output_path = "/output";
//! let threads = 8;
//!
//! build_bvh(output_path, data_path, 8)?;
//! build_map(data_path, output_path, "Azeroth", "", threads)?;
//!
//! let mut azeroth = PathfindMap::new(output_path, "Azeroth")?;
//! azeroth.load_all_adts()?;
//! let (zone, area) = azeroth.get_zone_and_area_raw(-8949.95, -132.493, 83.5312)?;
//! assert_eq!(zone, 12);
//! assert_eq!(area, 9);
//!
//! # Ok(())
//! # }
//! ```
//!
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![cfg_attr(docsrs, feature(doc_cfg))]

extern crate namigator_sys;

mod build;
pub use build::*;

mod pathfind;
pub use pathfind::*;

mod error;
pub use error::*;

mod util;

#[cfg(feature = "vanilla")]
util::specific_pathfind!(
    wow_world_base::vanilla::Map,
    VanillaMap,
    wow_world_base::vanilla::Area,
    wow_world_base::vanilla::Area
);

#[cfg(feature = "tbc")]
util::specific_pathfind!(
    wow_world_base::tbc::Map,
    TbcMap,
    wow_world_base::tbc::Area,
    wow_world_base::tbc::Area
);

#[cfg(feature = "wrath")]
util::specific_pathfind!(
    wow_world_base::wrath::Map,
    WrathMap,
    wow_world_base::wrath::Area,
    wow_world_base::wrath::Area
);

#[cfg(test)]
mod test;
