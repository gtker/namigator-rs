#[cfg(test)]
mod test;

use core::ffi::{c_char, c_float, c_int, c_uchar, c_uint};

#[repr(C)]
#[derive(Debug)]
pub struct Map {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Default)]
pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub const SUCCESS: u8 = 0;
pub const UNRECOGNIZED_EXTENSION: u8 = 1;
pub const NO_MOGP_CHUNK: u8 = 2;
pub const NO_MOPY_CHUNK: u8 = 3;
pub const NO_MOVI_CHUNK: u8 = 4;
pub const NO_MOVT_CHUNK: u8 = 5;
pub const NO_MCNK_CHUNK: u8 = 6;

pub const WDT_OPEN_FAILED: u8 = 7;
pub const MPHD_NOT_FOUND: u8 = 8;
pub const MAIN_NOT_FOUND: u8 = 9;
pub const MDNM_NOT_FOUND: u8 = 10;
pub const MONM_NOT_FOUND: u8 = 11;
pub const MWMO_NOT_FOUND: u8 = 12;
pub const MODF_NOT_FOUND: u8 = 13;

pub const MVER_NOT_FOUND: u8 = 14;
pub const MOMO_NOT_FOUND: u8 = 15;
pub const MOHD_NOT_FOUND: u8 = 16;
pub const MODS_NOT_FOUND: u8 = 17;
pub const MODN_NOT_FOUND: u8 = 18;
pub const MODD_NOT_FOUND: u8 = 19;

pub const MHDR_NOT_FOUND: u8 = 20;

pub const UNRECOGNIZED_DBC_FILE: u8 = 21;
pub const FAILED_TO_OPEN_DBC: u8 = 22;

pub const FAILED_TO_OPEN_ADT: u8 = 23;
pub const ADT_VERSION_IS_INCORRECT: u8 = 24;
pub const MVER_DOES_NOT_BEGIN_ADT_FILE: u8 = 25;

pub const EMPTY_WMO_DOODAD_INSTANTIATED: u8 = 26;

pub const FAILED_TO_OPEN_FILE_FOR_BINARY_STREAM: u8 = 27;

pub const BAD_FORMAT_OF_GAMEOBJECT_FILE: u8 = 28;
pub const FAILED_TO_OPEN_GAMEOBJECT_FILE: u8 = 29;
pub const UNRECOGNIZED_MODEL_EXTENSION: u8 = 30;
pub const GAME_OBJECT_REFERENCES_NON_EXISTENT_MODEL_ID: u8 = 31;

pub const ADT_SERIALIZATION_FAILED_TO_OPEN_OUTPUT_FILE: u8 = 32;
pub const WMO_SERIALIZATION_FAILED_TO_OPEN_OUTPUT_FILE: u8 = 33;

pub const REQUESTED_BVH_NOT_FOUND: u8 = 34;
pub const BVH_INDEX_FILE_NOT_FOUND: u8 = 35;

pub const INCORRECT_FILE_SIGNATURE: u8 = 36;
pub const INCORRECT_FILE_VERSION: u8 = 37;
pub const NOT_WMO_NAV_FILE: u8 = 38;
pub const NOT_WMO_TILE_COORDINATES: u8 = 39;
pub const NOT_ADT_NAV_FILE: u8 = 40;
pub const INVALID_MAP_FILE: u8 = 41;
pub const INCORRECT_WMO_COORDINATES: u8 = 42;
pub const DTNAVMESHQUERY_INIT_FAILED: u8 = 43;
pub const UNKNOWN_WMO_INSTANCE_REQUESTED: u8 = 44;
pub const UNKNOWN_DOODAD_INSTANCE_REQUESTED: u8 = 45;
pub const COULD_NOT_DESERIALIZE_DOODAD: u8 = 46;
pub const COULD_NOT_DESERIALIZE_WMO: u8 = 47;
pub const INCORRECT_ADT_COORDINATES: u8 = 48;
pub const TILE_NOT_FOUND_FOR_REQUESTED: u8 = 49;
pub const COULD_NOT_FIND_WMO: u8 = 50;

pub const DOODAD_PATH_NOT_FOUND: u8 = 51;
pub const UNSUPPORTED_DOOAD_FORMAT: u8 = 52;
pub const MAP_ID_NOT_FOUND: u8 = 53;
pub const WMO_NOT_FOUND: u8 = 54;

pub const BINARYSTREAM_COMPRESS_FAILED: u8 = 55;
pub const MZ_INFLATEINIT_FAILED: u8 = 56;
pub const MZ_INFLATE_FAILED: u8 = 57;

pub const INVALID_ROW_COLUMN_FROM_DBC: u8 = 58;

pub const GAMEOBJECT_WITH_SPECIFIED_GUID_ALREADY_EXISTS: u8 = 59;

pub const TEMPORARY_WMO_OBSTACLES_ARE_NOT_SUPPORTED: u8 = 60;
pub const NO_DOODAD_SET_SPECIFIED_FOR_WMO_GAME_OBJECT: u8 = 61;

pub const BAD_MATRIX_ROW: u8 = 62;
pub const INVALID_MATRIX_MULTIPLICATION: u8 = 63;
pub const ONLY_4X4_MATRIX_IS_SUPPORTED: u8 = 64;
pub const MATRIX_NOT_INVERTIBLE: u8 = 65;

pub const UNEXPECTED_VERSION_MAGIC_IN_ALPHA_MODEL: u8 = 66;
pub const UNEXPECTED_VERSION_SIZE_IN_ALPHA_MODEL: u8 = 67;
pub const UNSUPPORTED_ALPHA_MODEL_VERSION: u8 = 68;
pub const UNEXPECTED_VERTEX_MAGIC_IN_ALPHA_MODEL: u8 = 69;
pub const UNEXPECTED_TRIANGLE_MAGIC_IN_ALPHA_MODEL: u8 = 70;
pub const INVALID_DOODAD_FILE: u8 = 71;

pub const COULD_NOT_OPEN_MPQ: u8 = 72;
pub const GETROOTAREAID_INVALID_ID: u8 = 73;
pub const NO_DATA_FILES_FOUND: u8 = 74;
pub const MPQ_MANAGER_NOT_INIATIALIZED: u8 = 75;
pub const ERROR_IN_SFILEOPENFILEX: u8 = 76;
pub const ERROR_IN_SFILEREADFILE: u8 = 77;
pub const MULTIPLE_CANDIDATES_IN_ALPHA_MPQ: u8 = 78;
pub const TOO_MANY_FILES_IN_ALPHA_MPQ: u8 = 79;
pub const NO_MPQ_CANDIDATE: u8 = 80;

pub const RECAST_FAILURE: u8 = 81;

pub const BUFFER_TOO_SMALL: u8 = 82;
pub const UNKNOWN_PATH: u8 = 83;
pub const UNKNOWN_HEIGHT: u8 = 84;
pub const UNKNOWN_ZONE_AND_AREA: u8 = 85;
pub const FAILED_TO_LOAD_ADT: u8 = 86;
pub const MAP_DOES_NOT_HAVE_ADT: u8 = 87;

pub const UNABLE_TO_FIND_RANDOM_POINT_IN_CIRCLE: u8 = 88;

pub const FAILED_TO_FIND_POINT_BETWEEN_VECTORS: u8 = 89;

pub const UNKNOWN_EXCEPTION: u8 = 0xFF;

#[link(name = "namigator")]
extern "C" {
    pub fn pathfind_new_map(
        data_path: *const c_char,
        map_name: *const c_char,
        result: *const c_uchar,
    ) -> *const Map;

    pub fn pathfind_free_map(map: *const Map);

    pub fn pathfind_load_all_adts(map: *const Map, amount_of_adts_loaded: *const c_uint)
        -> c_uchar;

    pub fn pathfind_load_adt(
        map: *const Map,
        x: c_int,
        y: c_int,
        out_adt_x: *const c_float,
        out_adt_y: *const c_float,
    ) -> c_uchar;

    pub fn pathfind_load_adt_at(
        map: *const Map,
        x: c_float,
        y: c_float,
        out_adt_x: *const c_float,
        out_adt_y: *const c_float,
    ) -> c_uchar;

    pub fn pathfind_unload_adt(map: *const Map, x: c_int, y: c_int) -> c_uchar;

    pub fn pathfind_is_adt_loaded(
        map: *const Map,
        x: c_int,
        y: c_int,
        out_loaded: *const u8,
    ) -> c_uchar;

    pub fn pathfind_has_adts(map: *const Map, has_adts: *const bool) -> c_uchar;

    pub fn pathfind_get_zone_and_area(
        map: *const Map,
        x: c_float,
        y: c_float,
        z: c_float,
        out_zone: *const c_uint,
        out_area: *const c_uint,
    ) -> c_uchar;

    pub fn pathfind_find_point_in_between_vectors(
        map: *const Map,
        distance: f32,
        x1: f32,
        y1: f32,
        z1: f32,
        x2: f32,
        y2: f32,
        z2: f32,
        out_vertex: *mut Vertex,
    ) -> c_uchar;

    pub fn pathfind_find_path(
        map: *const Map,
        start_x: c_float,
        start_y: c_float,
        start_z: c_float,
        stop_x: c_float,
        stop_y: c_float,
        stop_z: c_float,
        buffer: *const Vertex,
        buffer_length: c_uint,
        amount_of_vertices: *const c_uint,
    ) -> c_uchar;

    pub fn pathfind_find_heights(
        map: *const Map,
        x: c_float,
        y: c_float,
        buffer: *const c_float,
        buffer_length: c_uint,
        amount_of_heights: *const c_uint,
    ) -> c_uchar;

    pub fn pathfind_find_height(
        map: *const Map,
        start_x: c_float,
        start_y: c_float,
        start_z: c_float,
        stop_x: c_float,
        stop_y: c_float,
        out_stop_z: *const c_float,
    ) -> c_uchar;

    pub fn pathfind_line_of_sight(
        map: *const Map,
        start_x: c_float,
        start_y: c_float,
        start_z: c_float,
        stop_x: c_float,
        stop_y: c_float,
        stop_z: c_float,
        line_of_sight: *const u8,
        doodads: u8,
    ) -> c_uchar;

    pub fn pathfind_find_random_point_around_circle(
        map: *const Map,
        x: c_float,
        y: c_float,
        z: c_float,
        radius: c_float,
        out_random_x: *const c_float,
        out_random_y: *const c_float,
        out_random_z: *const c_float,
    ) -> c_uchar;

    pub fn mapbuild_build_bvh(
        data_path: *const c_char,
        output_path: *const c_char,
        threads: c_uint,
        amount_of_bvhs_built: *const c_uint,
    ) -> c_uchar;

    pub fn mapbuild_build_map(
        data_path: *const c_char,
        output_path: *const c_char,
        map_name: *const c_char,
        gameobject_csv: *const c_char,
        threads: c_uint,
    ) -> c_uchar;

    pub fn mapbuild_bvh_files_exist(output_path: *const c_char, exists: *const u8) -> c_uchar;

    pub fn mapbuild_map_files_exist(
        output_path: *const c_char,
        map_name: *const c_char,
        exists: *const u8,
    ) -> c_uchar;
}
