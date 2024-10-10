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
    let data_directory = option_env!("NAMIGATOR_RS_TEST_DATA").unwrap_or("/tmp/test");

    let output = Path::new(output_directory);
    if output.exists() {
        std::fs::remove_dir_all(output_directory).unwrap();
    }

    test_build(&output_directory, &data_directory);
    test_pathfind(output_directory);

    #[cfg(feature = "vanilla")]
    test_vanilla(output_directory);

    #[cfg(feature = "tbc")]
    test_tbc(output_directory);

    #[cfg(feature = "wrath")]
    test_wrath(output_directory);
}

fn test_build(output_directory: &str, data_directory: &str) {
    let threads = std::thread::available_parallelism().unwrap().get() as u32;

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

    assert!(bvh_files_exist(output_directory).unwrap());
    assert!(map_files_exist(output_directory, MAP_NAME).unwrap());
}

const ADT_COORD_X: i32 = 0;
const ADT_COORD_Y: i32 = 0;

const ADT_X: f32 = 16271.025391;
const ADT_Y: f32 = 16845.421875;

const ADT_HEIGHTS: [f32; 2] = [46.30131, 35.611702];

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

const PATH_FIND_MAX_STEPS: usize = 5;

const ZONE_AND_AREA: u32 = 22;

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

const POINT: Vector3d = Vector3d {
    x: 16303.294922,
    y: 16789.242188,
    z: 45.219631,
};

const POINT_DISTANCE: f32 = 15.0;

macro_rules! test_map {
    ($ty_name:ident, $zone_and_area:expr) => {
        assert!(!$ty_name.adt_loaded(ADT_COORD_X, ADT_COORD_Y).unwrap());
        $ty_name.load_adt(ADT_COORD_X, ADT_COORD_Y).unwrap();
        assert!($ty_name.adt_loaded(ADT_COORD_X, ADT_COORD_Y).unwrap());
        $ty_name.unload_adt(ADT_COORD_X, ADT_COORD_Y).unwrap();
        assert!(!$ty_name.adt_loaded(ADT_COORD_X, ADT_COORD_Y).unwrap());

        $ty_name.load_adt_at(ADT_X, ADT_Y).unwrap();

        let heights = $ty_name.find_heights(ADT_X, ADT_Y).unwrap();
        assert_eq!(heights.len(), ADT_HEIGHTS.len());
        assert_eq!(heights[0], ADT_HEIGHTS[0]);
        assert_eq!(heights[1], ADT_HEIGHTS[1]);

        $ty_name.load_all_adts().unwrap();

        let path = $ty_name.find_path(PATH_FIND_FROM, PATH_FIND_TO).unwrap();

        assert!(path.len() >= PATH_FIND_MAX_STEPS);

        let (zone, area) = $ty_name
            .get_zone_and_area(ADT_X, ADT_Y, ADT_HEIGHTS[0])
            .unwrap();

        assert_eq!(zone, $zone_and_area);
        assert_eq!(area, $zone_and_area);

        let should_fail = $ty_name
            .line_of_sight(LINE_OF_SIGHT_SHOULD_FAIL_FROM, LINE_OF_SIGHT_SHOULD_FAIL_TO)
            .unwrap();
        assert!(!should_fail);

        let should_pass = $ty_name
            .line_of_sight(LINE_OF_SIGHT_SHOULD_PASS_FROM, LINE_OF_SIGHT_SHOULD_PASS_TO)
            .unwrap();
        assert!(should_pass);

        let should_pass_doodad = $ty_name
            .line_of_sight(
                LINE_OF_SIGHT_SHOULD_PASS_DOODAD_FROM,
                LINE_OF_SIGHT_SHOULD_PASS_DOODAD_TO,
            )
            .unwrap();
        assert!(should_pass_doodad);

        let z = $ty_name
            .find_height(FIND_HEIGHT_START, FIND_HEIGTH_STOP)
            .unwrap();
        assert_eq!(z, FIND_HEIGHT_RESULT);

        let out = $ty_name
            .find_random_point_around_circle(POINT, POINT_DISTANCE)
            .unwrap();

        let distance = distance(out.x, out.y, out.z, POINT.x, POINT.y, POINT.z);

        assert!(distance < POINT_DISTANCE);
    };
}

fn test_pathfind(output_directory: &str) {
    let mut map = PathfindMap::new(output_directory, MAP_NAME).unwrap();

    test_map!(map, ZONE_AND_AREA);
}

#[cfg(feature = "vanilla")]
fn test_vanilla(output_directory: &str) {
    let mut map =
        crate::vanilla::VanillaMap::new(output_directory, crate::vanilla::Map::DevelopmentLand)
            .unwrap();

    test_map!(map, crate::vanilla::Area::try_from(ZONE_AND_AREA).unwrap());
}

#[cfg(feature = "tbc")]
fn test_tbc(output_directory: &str) {
    let mut map =
        crate::tbc::TbcMap::new(output_directory, crate::tbc::Map::DevelopmentLand).unwrap();

    test_map!(map, crate::tbc::Area::try_from(ZONE_AND_AREA).unwrap());
}

#[cfg(feature = "wrath")]
fn test_wrath(output_directory: &str) {
    let mut map =
        crate::wrath::WrathMap::new(output_directory, crate::wrath::Map::DevelopmentLand).unwrap();

    test_map!(map, crate::wrath::Area::try_from(ZONE_AND_AREA).unwrap());
}

fn distance(from_x: f32, from_y: f32, from_z: f32, to_x: f32, to_y: f32, to_z: f32) -> f32 {
    let x = (from_x - to_x).powi(2);
    let y = (from_y - to_y).powi(2);
    let z = (from_z - to_z).powi(2);

    (x + y + z).sqrt()
}
