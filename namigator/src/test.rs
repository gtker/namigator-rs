use crate::build::{build_bvh, build_map};
use crate::error::NamigatorError;
use crate::pathfind::PathfindMap;
use crate::{bvh_files_exist, map_files_exist, Vector2d, Vector3d};
use std::path::Path;

const MAP_NAME: &str = "development";

#[test]
fn test_both() {
    let output_directory = "/tmp/namirstest";
    let data_directory = "/tmp/test";

    let output = Path::new(output_directory);
    if output.exists() {
        std::fs::remove_dir_all(output_directory).unwrap();
    }

    test_build(&output_directory, &data_directory);
}

fn test_build(output_directory: &str, data_directory: &str) {
    let threads = 8;

    assert!(!bvh_files_exist(output_directory).unwrap());
    assert!(!map_files_exist(output_directory, MAP_NAME).unwrap());

    match build_bvh(data_directory, output_directory, threads) {
        Ok(_) => {}
        Err(e) => match e {
            NamigatorError::FailedToOpenDbc => {} // Default test file does not include DBC
            e => panic!("{}", e),
        },
    }
    build_map(data_directory, output_directory, MAP_NAME, "", threads).unwrap();
    test_pathfind(output_directory);

    assert!(bvh_files_exist(output_directory).unwrap());
    assert!(map_files_exist(output_directory, MAP_NAME).unwrap());
}

fn test_pathfind(output_directory: &str) {
    let mut map = PathfindMap::new(output_directory, MAP_NAME).unwrap();

    const X: f32 = 16271.025391;
    const Y: f32 = 16845.421875;

    map.load_adt_at(X, Y).unwrap();

    let heights = map.find_heights(X, Y).unwrap();
    assert_eq!(heights.len(), 2);
    assert_eq!(heights[0], 46.30131);
    assert_eq!(heights[1], 35.611702);

    map.load_all_adts().unwrap();

    const START_X: f32 = 16303.294922;
    const START_Y: f32 = 16789.242188;
    const START_Z: f32 = 45.219631;
    const END_X: f32 = 16200.13648;
    const END_Y: f32 = 16834.345703;
    const END_Z: f32 = 37.028622;

    let path = map
        .find_path(
            Vector3d {
                x: START_X,
                y: START_Y,
                z: START_Z,
            },
            Vector3d {
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

    let should_fail = map
        .line_of_sight(
            Vector3d {
                x: 16268.3809,
                y: 16812.7148,
                z: 36.1483,
            },
            Vector3d {
                x: 16266.5781,
                y: 16782.623,
                z: 38.5035019,
            },
        )
        .unwrap();
    assert!(!should_fail);

    let should_pass = map
        .line_of_sight(
            Vector3d {
                x: 16873.2168,
                y: 16926.9551,
                z: 15.9072571,
            },
            Vector3d {
                x: 16987.4277,
                y: 16950.0742,
                z: 69.4590912,
            },
        )
        .unwrap();
    assert!(should_pass);

    let should_pass_doodad = map
        .line_of_sight(
            Vector3d {
                x: 16275.6895,
                y: 16853.9023,
                z: 37.8341751,
            },
            Vector3d {
                x: 16987.4277,
                y: 16950.0742,
                z: 69.4590912,
            },
        )
        .unwrap();
    assert!(should_pass_doodad);

    let start = Vector3d {
        x: 16232.7373,
        y: 16828.2734,
        z: 37.1330833,
    };
    let stop = Vector2d {
        x: 16208.6,
        y: 16830.7,
    };

    let z = map.find_height(start, stop).unwrap();
    assert_eq!(z, 36.86227);

    const POINT: Vector3d = Vector3d {
        x: 16303.294922,
        y: 16789.242188,
        z: 45.219631,
    };
    const POINT_DISTANCE: f32 = 10.0;
    let out = map
        .find_random_point_around_circle(POINT, POINT_DISTANCE)
        .unwrap();

    let x = (out.x - POINT.x).powi(2);
    let y = (out.y - POINT.y).powi(2);
    let z = (out.z - POINT.z).powi(2);

    let distance = (x + y + z).sqrt();

    assert!(distance < POINT_DISTANCE);
}
