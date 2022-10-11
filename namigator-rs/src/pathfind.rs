use crate::error::{error_code_to_error, NamigatorError};
use namigator_sys::{
    pathfind_find_heights, pathfind_find_path, pathfind_free_map, pathfind_get_zone_and_area,
    pathfind_load_adt_at, pathfind_load_all_adts, pathfind_new_map, Vertex, BUFFER_TOO_SMALL,
    SUCCESS,
};
use std::ffi::{c_uint, CString};

#[derive(Debug)]
pub struct PathfindMap {
    map: *const namigator_sys::Map,
    path: Vec<Vertex>,
    height: Vec<f32>,
}

const INITIAL_VEC_SIZE: usize = 10;

impl PathfindMap {
    pub fn new(data_path: &str, map_name: &str) -> Result<Self, NamigatorError> {
        let data_path = CString::new(data_path)?;
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

        Ok(Self {
            map,
            path: vec![Vertex::default(); INITIAL_VEC_SIZE],
            height: vec![f32::default(); INITIAL_VEC_SIZE],
        })
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

    pub fn find_path(&mut self, start: Vertex, stop: Vertex) -> Result<&[Vertex], NamigatorError> {
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
                self.path.as_ptr(),
                self.path.len() as c_uint,
                &mut amount_of_vertices as *const c_uint,
            )
        };

        if result == SUCCESS {
            return Ok(&self.path[..usize::try_from(amount_of_vertices).unwrap()]);
        } else if result == BUFFER_TOO_SMALL {
            self.path.resize(
                usize::try_from(amount_of_vertices).unwrap(),
                Vertex::default(),
            );

            let result = unsafe {
                pathfind_find_path(
                    self.map,
                    start.x,
                    start.y,
                    start.z,
                    stop.x,
                    stop.y,
                    stop.z,
                    self.path.as_ptr(),
                    self.path.len() as c_uint,
                    &mut amount_of_vertices as *const c_uint,
                )
            };

            if result == SUCCESS {
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
                self.height.as_ptr(),
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
                    self.height.as_ptr(),
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
}

impl Drop for PathfindMap {
    fn drop(&mut self) {
        unsafe { pathfind_free_map(self.map) }
    }
}
