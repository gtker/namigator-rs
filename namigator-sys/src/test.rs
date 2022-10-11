use crate::{
    mapbuild_build_bvh, mapbuild_build_map, pathfind_find_height, pathfind_find_heights,
    pathfind_find_path, pathfind_free_map, pathfind_get_zone_and_area, pathfind_line_of_sight,
    pathfind_load_adt_at, pathfind_load_all_adts, pathfind_new_map, Map, Vertex,
    FAILED_TO_OPEN_DBC, SUCCESS,
};
use alloc::ffi::CString;
use core::ffi::{c_float, c_uchar, c_uint};

const MAP_NAME: &str = "development";

fn test_build(temp_directory: &str, data_directory: &str) {
    let data_path = CString::new(data_directory).unwrap();
    let output_path = CString::new(temp_directory).unwrap();
    let map_name = CString::new(MAP_NAME).unwrap();
    let go_csv = CString::new("").unwrap();
    let threads = 8;
    let mut amount_of_bvhs_built: u32 = 0;

    // DBC is not inside MPQ
    let result = unsafe {
        mapbuild_build_bvh(
            data_path.as_ptr(),
            output_path.as_ptr(),
            8,
            &mut amount_of_bvhs_built as *const c_uint,
        )
    };
    assert_eq!(result, FAILED_TO_OPEN_DBC);

    let result = unsafe {
        mapbuild_build_map(
            data_path.as_ptr(),
            output_path.as_ptr(),
            map_name.as_ptr(),
            go_csv.as_ptr(),
            threads,
        )
    };
    assert_eq!(result, SUCCESS);
}

#[test]
fn test_both() {
    let temp_directory = concat!(env!("OUT_DIR"), "/test_tmp");
    let data_directory = env!("OUT_DIR");

    test_build(temp_directory, data_directory);
    test_pathfind(temp_directory);
}

fn test_pathfind(temp_directory: &str) {
    let data_path = CString::new(temp_directory).unwrap();
    let map_name = CString::new(MAP_NAME).unwrap();
    let mut result: u8 = 0;
    let map = unsafe {
        pathfind_new_map(
            data_path.as_ptr(),
            map_name.as_ptr(),
            &mut result as *const u8,
        )
    };
    assert!(!map.is_null());
    assert_eq!(result, SUCCESS);

    const X: f32 = 16271.025391;
    const Y: f32 = 16845.421875;

    let mut adt_x: f32 = 0.0;
    let mut adt_y: f32 = 0.0;
    let result = unsafe {
        pathfind_load_adt_at(
            map,
            X,
            Y,
            &mut adt_x as *const f32,
            &mut adt_y as *const f32,
        )
    };
    assert_eq!(result, SUCCESS);

    const SIZE: usize = 2;
    let mut buf: [f32; SIZE] = [0.0; SIZE];

    let mut amount_of_vertices: u32 = 0;

    let result = unsafe {
        pathfind_find_heights(
            map,
            X,
            Y,
            &mut buf as *const f32,
            SIZE as u32,
            &mut amount_of_vertices as *const u32,
        )
    };

    assert_eq!(result, SUCCESS);
    assert_eq!(buf[0], 46.30131);
    assert_eq!(buf[1], 35.611702);
    assert_eq!(amount_of_vertices, 2);

    let mut amount_of_adts_loaded: u32 = 0;
    let _a = unsafe { pathfind_load_all_adts(map, &mut amount_of_adts_loaded as *const u32) };

    const START_X: f32 = 16303.294922;
    const START_Y: f32 = 16789.242188;
    const START_Z: f32 = 45.219631;
    const END_X: f32 = 16200.13648;
    const END_Y: f32 = 16834.345703;
    const END_Z: f32 = 37.028622;

    const BUFFER_LENGTH: usize = 100;
    let buffer = [Vertex {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    }; BUFFER_LENGTH];
    let mut amount_of_vertices: u32 = 0;

    let result = unsafe {
        pathfind_find_path(
            map,
            START_X,
            START_Y,
            START_Z,
            END_X,
            END_Y,
            END_Z,
            buffer.as_ptr(),
            BUFFER_LENGTH as u32,
            &mut amount_of_vertices as *const u32,
        )
    };
    assert_eq!(result, SUCCESS);

    assert!(amount_of_vertices >= 5);

    let mut zone: u32 = 0;
    let mut area: u32 = 0;

    let result = unsafe {
        pathfind_get_zone_and_area(
            map,
            X,
            Y,
            46.301323,
            &mut zone as *const u32,
            &mut area as *const u32,
        )
    };

    assert_eq!(result, SUCCESS);
    assert_eq!(zone, 22);
    assert_eq!(area, 22);

    let should_fail = call_line_of_sight(
        map,
        Vertex {
            x: 16268.3809,
            y: 16812.7148,
            z: 36.1483,
        },
        Vertex {
            x: 16266.5781,
            y: 16782.623,
            z: 38.5035019,
        },
    );
    assert!(!should_fail);
    let should_pass = call_line_of_sight(
        map,
        Vertex {
            x: 16873.2168,
            y: 16926.9551,
            z: 15.9072571,
        },
        Vertex {
            x: 16987.4277,
            y: 16950.0742,
            z: 69.4590912,
        },
    );
    assert!(should_pass);

    let should_pass_doodad = call_line_of_sight(
        map,
        Vertex {
            x: 16275.6895,
            y: 16853.9023,
            z: 37.8341751,
        },
        Vertex {
            x: 16987.4277,
            y: 16950.0742,
            z: 69.4590912,
        },
    );
    assert!(should_pass_doodad);

    let start_x = 16232.7373;
    let start_y = 16828.2734;
    let start_z = 37.1330833;
    let stop_x = 16208.6;
    let stop_y = 16830.7;
    let mut out_stop_z: c_float = 0.0;

    let result = unsafe {
        pathfind_find_height(
            map,
            start_x,
            start_y,
            start_z,
            stop_x,
            stop_y,
            &mut out_stop_z as *const c_float,
        )
    };
    assert_eq!(result, SUCCESS);
    assert_eq!(out_stop_z, 36.86227);

    unsafe { pathfind_free_map(map) }
}

fn call_line_of_sight(map: *const Map, start: Vertex, end: Vertex) -> bool {
    let mut los: u8 = 0;
    let doodads: u8 = 0;
    unsafe {
        let result = pathfind_line_of_sight(
            map,
            start.x,
            start.y,
            start.z,
            end.x,
            end.y,
            end.z,
            &mut los as *const c_uchar,
            doodads,
        );

        assert_eq!(result, SUCCESS);

        if los == 1 {
            true
        } else if los == 0 {
            false
        } else {
            panic!("invalid line of sight value")
        }
    }
}
