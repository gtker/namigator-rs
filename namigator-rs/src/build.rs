use crate::error::{error_code_to_error, NamigatorError};
use crate::util::path_to_cstr;
use namigator_sys::{mapbuild_build_bvh, mapbuild_build_map, SUCCESS};
use std::ffi::{c_uint, CString};
use std::path::Path;

/// Build the game objects as a Bounded Volume Hierachy (BVH) and returns the amount of objects built.
///
/// `data_path` must point to the `Data` directory of the client that contains the `MPQ` files..
///
/// `output_path` points to where the generated files will be placed. If the directory does not exist it will be created.
///
/// `threads` is the amount of threads that will be used.
pub fn build_bvh(
    data_path: impl AsRef<Path>,
    output_path: impl AsRef<Path>,
    threads: u32,
) -> Result<u32, NamigatorError> {
    fn inner(data_path: &Path, output_path: &Path, threads: u32) -> Result<u32, NamigatorError> {
        let data_path = path_to_cstr(data_path)?;

        let output_path = path_to_cstr(output_path)?;

        let mut amount_of_bvhs_built: c_uint = 0;

        let result = unsafe {
            mapbuild_build_bvh(
                data_path.as_ptr(),
                output_path.as_ptr(),
                threads,
                &mut amount_of_bvhs_built as *const c_uint,
            )
        };

        if result == SUCCESS {
            Ok(amount_of_bvhs_built)
        } else {
            Err(error_code_to_error(result))
        }
    }

    inner(data_path.as_ref(), output_path.as_ref(), threads)
}

/// Build the map files.
///
/// `data_path` must point to the `Data` directory of the client that contains the `MPQ` files..
///
/// `output_path` points to where the generated files will be placed. If the directory does not exist it will be created.
///
/// `map_name` TODO
///
/// `gameobject_csv` TODO
///
/// `threads` is the amount of threads that will be used.
pub fn build_map(
    data_path: impl AsRef<Path>,
    output_path: impl AsRef<Path>,
    map_name: &str,
    gameobject_csv: impl AsRef<Path>,
    threads: u32,
) -> Result<(), NamigatorError> {
    fn inner(
        data_path: &Path,
        output_path: &Path,
        map_name: &str,
        gameobject_csv: &Path,
        threads: u32,
    ) -> Result<(), NamigatorError> {
        let data_path = path_to_cstr(data_path)?;
        let output_path = path_to_cstr(output_path)?;
        let gameobject_csv = path_to_cstr(gameobject_csv)?;
        let map_name = CString::new(map_name)?;

        let result = unsafe {
            mapbuild_build_map(
                data_path.as_ptr(),
                output_path.as_ptr(),
                map_name.as_ptr(),
                gameobject_csv.as_ptr(),
                threads,
            )
        };

        if result == SUCCESS {
            return Ok(());
        }

        Err(error_code_to_error(result))
    }

    inner(
        data_path.as_ref(),
        output_path.as_ref(),
        map_name,
        gameobject_csv.as_ref(),
        threads,
    )
}
