use crate::{build_bvh, build_map, Map};
use namigator_sys::Vertex;
use std::ffi::CString;

const MAP_NAME: &str = "development";

#[test]
fn test_both() {
    let temp_directory = "/tmp/namirstest";
    let data_directory = "/tmp/test";

    test_build(&temp_directory, &data_directory);
}

fn test_build(temp_directory: &str, data_directory: &str) {
    let map_name = CString::new(MAP_NAME).unwrap();
    let threads = 8;

    build_bvh(data_directory, temp_directory, threads).unwrap();
    build_map(data_directory, temp_directory, &map_name, "", threads).unwrap();
    test_pathfind(temp_directory);
}

fn test_pathfind(temp_directory: &str) {
    let mut map = Map::new(temp_directory, MAP_NAME).unwrap();

    const X: f32 = 16271.025391;
    const Y: f32 = 16845.421875;

    map.load_adt_at(X, Y).unwrap();

    let heights = map.find_heights(X, Y).unwrap();
    assert_eq!(heights.len(), 2);
    assert_eq!(heights[0], 35.611702);
    assert_eq!(heights[1], 46.301323);

    map.load_all_adts().unwrap();

    const START_X: f32 = 16303.294922;
    const START_Y: f32 = 16789.242188;
    const START_Z: f32 = 45.219631;
    const END_X: f32 = 16200.13648;
    const END_Y: f32 = 16834.345703;
    const END_Z: f32 = 37.028622;

    let path = map
        .find_path(
            Vertex {
                x: START_X,
                y: START_Y,
                z: START_Z,
            },
            Vertex {
                x: END_X,
                y: END_Y,
                z: END_Z,
            },
        )
        .unwrap();

    assert!(path.len() >= 5);

    let (zone, area) = map.get_zone_and_area_raw(X, Y, 46.301323).unwrap();

    assert_eq!(zone, 22);
    assert_eq!(area, 22);
}
