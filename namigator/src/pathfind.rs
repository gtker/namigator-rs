use crate::error::{error_code_to_error, NamigatorError};
use crate::util::path_to_cstr;
use namigator_sys::{
    pathfind_find_height, pathfind_find_heights, pathfind_find_path,
    pathfind_find_random_point_around_circle, pathfind_free_map, pathfind_get_zone_and_area,
    pathfind_line_of_sight, pathfind_load_adt_at, pathfind_load_all_adts, pathfind_new_map, Vertex,
    BUFFER_TOO_SMALL, SUCCESS,
};
use std::ffi::{c_float, c_uint, CString};
use std::path::Path;

pub use wow_world_base::shared::vector2d_vanilla_tbc_wrath::Vector2d;
pub use wow_world_base::shared::vector3d_vanilla_tbc_wrath::Vector3d;

#[derive(Debug)]
pub struct PathfindMap {
    map: *const namigator_sys::Map,
    // Vector3d does not have repr(c) so we can't be sure that it's correctly set up
    // The benefits of having interop with a wow_world_base type far outweighs the extra
    // ~124 bytes of storage for the vec.
    // This might be replaceable with a const array if we get hard limitations from namigator
    inner_path: Vec<Vertex>,
    path: Vec<Vector3d>,
    height: Vec<f32>,
}

const INITIAL_VEC_SIZE: usize = 10;

impl PathfindMap {
    pub fn new(data_path: impl AsRef<Path>, map_name: &str) -> Result<Self, NamigatorError> {
        fn inner(data_path: &Path, map_name: &str) -> Result<PathfindMap, NamigatorError> {
            let data_path = path_to_cstr(data_path)?;
            let map_name = CString::new(map_name)?;

            let mut result: u8 = 0;
            // SAFETY: CStrings are guaranteed to be valid pointers
            let map = unsafe {
                pathfind_new_map(
                    data_path.as_ptr(),
                    map_name.as_ptr(),
                    &mut result as *const u8,
                )
            };

            if result != SUCCESS {
                return Err(error_code_to_error(result));
            }

            if map.is_null() {
                return Err(NamigatorError::MapIsNullPointer);
            }

            Ok(PathfindMap {
                map,
                inner_path: vec![Vertex::default(); INITIAL_VEC_SIZE],
                path: vec![Vector3d::default(); INITIAL_VEC_SIZE],
                height: vec![f32::default(); INITIAL_VEC_SIZE],
            })
        }
        inner(data_path.as_ref(), map_name)
    }

    pub fn load_all_adts(&mut self) -> Result<u32, NamigatorError> {
        let mut adts_loaded: c_uint = 0;

        // SAFETY: map is guaranteed to be initialized in a member function
        let result = unsafe { pathfind_load_all_adts(self.map, &mut adts_loaded as *const c_uint) };

        if result != SUCCESS {
            return Err(error_code_to_error(result));
        }

        Ok(adts_loaded)
    }

    pub fn load_adt_at(&mut self, x: f32, y: f32) -> Result<(f32, f32), NamigatorError> {
        let mut out_adt_x: f32 = 0.0;
        let mut out_adt_y: f32 = 0.0;

        let result = unsafe {
            pathfind_load_adt_at(
                self.map,
                x,
                y,
                &mut out_adt_x as *const f32,
                &mut out_adt_y as *const f32,
            )
        };

        if result == SUCCESS {
            Ok((out_adt_x, out_adt_y))
        } else {
            Err(error_code_to_error(result))
        }
    }

    pub fn get_zone_and_area_raw(
        &self,
        x: f32,
        y: f32,
        z: f32,
    ) -> Result<(u32, u32), NamigatorError> {
        let mut out_zone: c_uint = 0;
        let mut out_area: c_uint = 0;

        // SAFETY: map is guaranteed to be valid in member functions
        let result = unsafe {
            pathfind_get_zone_and_area(
                self.map,
                x,
                y,
                z,
                &mut out_zone as *const c_uint,
                &mut out_area as *const c_uint,
            )
        };

        if result != SUCCESS {
            return Err(error_code_to_error(result));
        }

        Ok((out_zone, out_area))
    }

    pub fn find_path(
        &mut self,
        start: Vector3d,
        stop: Vector3d,
    ) -> Result<&[Vector3d], NamigatorError> {
        let mut amount_of_vertices: c_uint = 0;

        let result = unsafe {
            pathfind_find_path(
                self.map,
                start.x,
                start.y,
                start.z,
                stop.x,
                stop.y,
                stop.z,
                self.inner_path.as_mut_ptr(),
                self.inner_path.len() as c_uint,
                &mut amount_of_vertices as *const c_uint,
            )
        };

        if result == SUCCESS {
            self.transfer_paths();
            return Ok(&self.path[..usize::try_from(amount_of_vertices).unwrap()]);
        } else if result == BUFFER_TOO_SMALL {
            self.resize_paths(amount_of_vertices);

            let result = unsafe {
                pathfind_find_path(
                    self.map,
                    start.x,
                    start.y,
                    start.z,
                    stop.x,
                    stop.y,
                    stop.z,
                    self.inner_path.as_mut_ptr(),
                    self.inner_path.len() as c_uint,
                    &mut amount_of_vertices as *const c_uint,
                )
            };

            if result == SUCCESS {
                self.transfer_paths();
                return Ok(&self.path[..usize::try_from(amount_of_vertices).unwrap()]);
            } else {
                panic!("buffer was too small even after making it larger")
            }
        }

        Err(error_code_to_error(result))
    }

    pub fn find_heights(&mut self, x: f32, y: f32) -> Result<&[f32], NamigatorError> {
        let mut amount_of_heights: u32 = 0;

        let result = unsafe {
            pathfind_find_heights(
                self.map,
                x,
                y,
                self.height.as_mut_ptr(),
                self.height.len() as c_uint,
                &mut amount_of_heights as *const c_uint,
            )
        };

        if result == SUCCESS {
            return Ok(&self.height[..usize::try_from(amount_of_heights).unwrap()]);
        } else if result == BUFFER_TOO_SMALL {
            self.height
                .resize(usize::try_from(amount_of_heights).unwrap(), f32::default());

            let result = unsafe {
                pathfind_find_heights(
                    self.map,
                    x,
                    y,
                    self.height.as_mut_ptr(),
                    self.height.len() as c_uint,
                    &mut amount_of_heights as *const c_uint,
                )
            };

            if result == SUCCESS {
                return Ok(&self.height[..usize::try_from(amount_of_heights).unwrap()]);
            }
        }

        Err(error_code_to_error(result))
    }

    pub fn line_of_sight(&self, from: Vector3d, to: Vector3d) -> Result<bool, NamigatorError> {
        let mut los: u8 = 0;
        // SAFETY: self.map is always valid in member functions.
        let doodads: u8 = 0;
        let result = unsafe {
            pathfind_line_of_sight(
                self.map,
                from.x,
                from.y,
                from.z,
                to.x,
                to.y,
                to.z,
                &mut los as *const u8,
                doodads,
            )
        };

        if result == SUCCESS {
            Ok(match los {
                1 => true,
                0 => false,
                los => {
                    panic!(
                        "invalid value received from line_of_sight: '{}', from: '{:?}', to: '{:?}'",
                        los, from, to
                    )
                }
            })
        } else {
            Err(error_code_to_error(result))
        }
    }

    pub fn find_height(&self, start: Vector3d, stop: Vector2d) -> Result<f32, NamigatorError> {
        let mut out_stop_z: c_float = 0.0;

        let result = unsafe {
            pathfind_find_height(
                self.map,
                start.x,
                start.y,
                start.z,
                stop.x,
                stop.y,
                &mut out_stop_z as *const c_float,
            )
        };

        if result == SUCCESS {
            Ok(out_stop_z)
        } else {
            Err(error_code_to_error(result))
        }
    }

    pub fn find_random_point_around_circle(
        &self,
        start: Vector3d,
        radius: f32,
    ) -> Result<Vector3d, NamigatorError> {
        let mut out_x: c_float = 0.0;
        let mut out_y: c_float = 0.0;
        let mut out_z: c_float = 0.0;

        let result = unsafe {
            pathfind_find_random_point_around_circle(
                self.map,
                start.x,
                start.y,
                start.z,
                radius,
                &mut out_x as *const c_float,
                &mut out_y as *const c_float,
                &mut out_z as *const c_float,
            )
        };

        if result == SUCCESS {
            Ok(Vector3d {
                x: out_x,
                y: out_y,
                z: out_z,
            })
        } else {
            Err(error_code_to_error(result))
        }
    }

    fn resize_paths(&mut self, size: u32) {
        let size = usize::try_from(size).unwrap();
        self.inner_path.resize(size, Vertex::default());
        self.path.resize(size, Vector3d::default());
    }

    fn transfer_paths(&mut self) {
        for (i, v) in self.inner_path.iter().enumerate() {
            self.path[i].x = v.x;
            self.path[i].y = v.y;
            self.path[i].z = v.z;
        }
    }
}

impl Drop for PathfindMap {
    fn drop(&mut self) {
        unsafe { pathfind_free_map(self.map) }
    }
}
