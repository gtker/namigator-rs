use crate::error::NamigatorError;
use std::ffi::CString;
use std::path::Path;

pub fn path_to_cstr(p: &Path) -> Result<CString, NamigatorError> {
    let e = match p.to_str() {
        None => return Err(NamigatorError::PathCStringConversion),
        Some(e) => e,
    };

    Ok(CString::new(e)?)
}

#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
macro_rules! specific_pathfind {
    ($map:ty, $ty_name:ident, $zone_ty:ty, $area_ty:ty) => {
        pub use $area_ty;
        pub use $map;

        #[derive(Debug)]
        pub struct $ty_name {
            map: $crate::raw::PathfindMap,
        }

        // SAFETY: Namigator should allow this.
        unsafe impl Send for $ty_name {}

        impl $ty_name {
            pub fn new(
                data_path: impl AsRef<std::path::Path>,
                map: $map,
            ) -> Result<Self, $crate::error::NamigatorError> {
                fn inner(
                    data_path: &std::path::Path,
                    map_name: &str,
                ) -> Result<$ty_name, $crate::error::NamigatorError> {
                    Ok($ty_name {
                        map: $crate::raw::PathfindMap::new(data_path, map_name)?,
                    })
                }

                inner(data_path.as_ref(), map.directory_name())
            }

            pub fn build_gameobjects(
                data_path: impl AsRef<std::path::Path>,
                output_path: impl AsRef<std::path::Path>,
                threads: u32,
            ) -> Result<(), $crate::NamigatorError> {
                fn inner(
                    data_path: &std::path::Path,
                    output_path: &std::path::Path,
                    threads: u32,
                ) -> Result<(), $crate::NamigatorError> {
                    if !$crate::raw::bvh_files_exist(output_path)? {
                        $crate::build::build_bvh(data_path, output_path, threads)?;
                    }

                    Ok(())
                }

                inner(data_path.as_ref(), output_path.as_ref(), threads)
            }

            pub fn build_gameobjects_and_map(
                data_path: impl AsRef<std::path::Path>,
                output_path: impl AsRef<std::path::Path>,
                map: $map,
                threads: u32,
            ) -> Result<$ty_name, $crate::NamigatorError> {
                fn inner(
                    data_path: &std::path::Path,
                    output_path: &std::path::Path,
                    map: $map,
                    threads: u32,
                ) -> Result<$ty_name, $crate::NamigatorError> {
                    $ty_name::build_gameobjects(data_path, output_path, threads)?;

                    if !$crate::raw::map_files_exist(output_path, map.directory_name())? {
                        $crate::build::build_map(
                            data_path,
                            output_path,
                            map.directory_name(),
                            "",
                            threads,
                        )?;
                    }

                    $ty_name::new(output_path, map)
                }

                inner(data_path.as_ref(), output_path.as_ref(), map, threads)
            }

            pub fn load_all_adts(&mut self) -> Result<u32, $crate::error::NamigatorError> {
                self.map.load_all_adts()
            }

            pub fn load_adt(
                &mut self,
                x: i32,
                y: i32,
            ) -> Result<(f32, f32), $crate::error::NamigatorError> {
                self.map.load_adt(x, y)
            }

            pub fn load_adt_at(
                &mut self,
                x: f32,
                y: f32,
            ) -> Result<(f32, f32), $crate::error::NamigatorError> {
                self.map.load_adt_at(x, y)
            }

            pub fn unload_adt(&self, x: i32, y: i32) -> Result<(), $crate::error::NamigatorError> {
                self.map.unload_adt(x, y)
            }

            pub fn adt_loaded(
                &self,
                x: i32,
                y: i32,
            ) -> Result<bool, $crate::error::NamigatorError> {
                self.map.adt_loaded(x, y)
            }

            pub fn get_zone_and_area(
                &self,
                x: f32,
                y: f32,
                z: f32,
            ) -> Result<($zone_ty, $area_ty), $crate::error::NamigatorError> {
                let (zone, area) = self.map.get_zone_and_area(x, y, z)?;

                Ok((zone.try_into().unwrap(), area.try_into().unwrap()))
            }

            pub fn find_path(
                &mut self,
                start: $crate::Vector3d,
                stop: $crate::Vector3d,
            ) -> Result<&[$crate::Vector3d], $crate::error::NamigatorError> {
                self.map.find_path(start, stop)
            }

            pub fn find_heights(
                &mut self,
                x: f32,
                y: f32,
            ) -> Result<&[f32], $crate::error::NamigatorError> {
                self.map.find_heights(x, y)
            }

            pub fn line_of_sight(
                &self,
                from: $crate::Vector3d,
                to: $crate::Vector3d,
            ) -> Result<bool, $crate::error::NamigatorError> {
                self.map.line_of_sight(from, to)
            }

            pub fn find_height(
                &self,
                from: $crate::Vector3d,
                to: $crate::Vector2d,
            ) -> Result<f32, $crate::error::NamigatorError> {
                self.map.find_height(from, to)
            }

            pub fn find_random_point_around_circle(
                &self,
                start: $crate::Vector3d,
                radius: f32,
            ) -> Result<$crate::Vector3d, $crate::error::NamigatorError> {
                self.map.find_random_point_around_circle(start, radius)
            }
        }
    };
}
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub(crate) use specific_pathfind;
