use crate::build::{build_bvh, build_map};
use crate::error::NamigatorError;
use crate::pathfind::PathfindMap;
use crate::Vector3d;
use wow_world_base::vanilla::Vector2d;

const MAP_NAME: &str = "development";

#[test]
fn test_both() {
    let temp_directory = "/tmp/namirstest";
    let data_directory = "/tmp/test";

    test_build(&temp_directory, &data_directory);
}

fn test_build(temp_directory: &str, data_directory: &str) {
    let threads = 8;

    match build_bvh(data_directory, temp_directory, threads) {
        Ok(_) => {}
        Err(e) => match e {
            NamigatorError::FailedToOpenDbc => {} // Default test file does not include DBC
            _ => panic!(),
        },
    }
    build_map(data_directory, temp_directory, MAP_NAME, "", threads).unwrap();
    test_pathfind(temp_directory);
}

fn test_pathfind(temp_directory: &str) {
    let mut map = PathfindMap::new(temp_directory, MAP_NAME).unwrap();

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
}
