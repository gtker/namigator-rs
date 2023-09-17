use crate::build::{build_bvh, build_map};
use crate::error::NamigatorError;
use crate::pathfind::PathfindMap;
use crate::raw::{bvh_files_exist, map_files_exist};
use crate::{Vector2d, Vector3d};
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

    const ADT_X: f32 = 16271.025391;
    const ADT_Y: f32 = 16845.421875;

    assert!(!map.adt_loaded(0, 1).unwrap());
    map.load_adt(0, 1).unwrap();
    assert!(map.adt_loaded(0, 1).unwrap());
    map.unload_adt(0, 1).unwrap();
    assert!(!map.adt_loaded(0, 1).unwrap());

    map.load_adt_at(ADT_X, ADT_Y).unwrap();

    const ADT_HEIGHTS: [f32; 2] = [46.30131, 35.611702];

    let heights = map.find_heights(ADT_X, ADT_Y).unwrap();
    assert_eq!(heights.len(), ADT_HEIGHTS.len());
    assert_eq!(heights[0], ADT_HEIGHTS[0]);
    assert_eq!(heights[1], ADT_HEIGHTS[1]);

    map.load_all_adts().unwrap();

    const PATH_FIND_FROM: Vector3d = Vector3d {
        x: 16303.294922,
        y: 16789.242188,
        z: 45.219631,
    };

    const PATH_FIND_TO: Vector3d = Vector3d {
        x: 16200.13648,
        y: 16834.345703,
        z: 37.028622,
    };

    let path = map.find_path(PATH_FIND_FROM, PATH_FIND_TO).unwrap();

    const PATH_FIND_MAX_STEPS: usize = 5;

    assert!(path.len() >= PATH_FIND_MAX_STEPS);

    let (zone, area) = map
        .get_zone_and_area_raw(ADT_X, ADT_Y, ADT_HEIGHTS[0])
        .unwrap();

    const ZONE_AND_AREA: u32 = 22;

    assert_eq!(zone, ZONE_AND_AREA);
    assert_eq!(area, ZONE_AND_AREA);

    const LINE_OF_SIGHT_SHOULD_FAIL_FROM: Vector3d = Vector3d {
        x: 16268.3809,
        y: 16812.7148,
        z: 36.1483,
    };

    const LINE_OF_SIGHT_SHOULD_FAIL_TO: Vector3d = Vector3d {
        x: 16266.5781,
        y: 16782.623,
        z: 38.5035019,
    };

    let should_fail = map
        .line_of_sight(LINE_OF_SIGHT_SHOULD_FAIL_FROM, LINE_OF_SIGHT_SHOULD_FAIL_TO)
        .unwrap();
    assert!(!should_fail);

    const LINE_OF_SIGHT_SHOULD_PASS_FROM: Vector3d = Vector3d {
        x: 16873.2168,
        y: 16926.9551,
        z: 15.9072571,
    };

    const LINE_OF_SIGHT_SHOULD_PASS_TO: Vector3d = Vector3d {
        x: 16987.4277,
        y: 16950.0742,
        z: 69.4590912,
    };

    let should_pass = map
        .line_of_sight(LINE_OF_SIGHT_SHOULD_PASS_FROM, LINE_OF_SIGHT_SHOULD_PASS_TO)
        .unwrap();
    assert!(should_pass);

    const LINE_OF_SIGHT_SHOULD_PASS_DOODAD_FROM: Vector3d = Vector3d {
        x: 16275.6895,
        y: 16853.9023,
        z: 37.8341751,
    };

    const LINE_OF_SIGHT_SHOULD_PASS_DOODAD_TO: Vector3d = Vector3d {
        x: 16987.4277,
        y: 16950.0742,
        z: 69.4590912,
    };

    let should_pass_doodad = map
        .line_of_sight(
            LINE_OF_SIGHT_SHOULD_PASS_DOODAD_FROM,
            LINE_OF_SIGHT_SHOULD_PASS_DOODAD_TO,
        )
        .unwrap();
    assert!(should_pass_doodad);

    const FIND_HEIGHT_START: Vector3d = Vector3d {
        x: 16232.7373,
        y: 16828.2734,
        z: 37.1330833,
    };
    const FIND_HEIGTH_STOP: Vector2d = Vector2d {
        x: 16208.6,
        y: 16830.7,
    };
    const FIND_HEIGHT_RESULT: f32 = 36.86227;

    let z = map
        .find_height(FIND_HEIGHT_START, FIND_HEIGTH_STOP)
        .unwrap();
    assert_eq!(z, FIND_HEIGHT_RESULT);

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
