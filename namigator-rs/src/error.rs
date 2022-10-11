use std::ffi::NulError;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum NamigatorError {
    CStringConversion(NulError),
    UnrecognizedExtension,
    NoMogpChunk,
    NoMopyChunk,
    NoMoviChunk,
    NoMovtChunk,
    NoMcnkChunk,
    WdtOpenFailed,
    MphdNotFound,
    MainNotFound,
    MdnmNotFound,
    MonmNotFound,
    MwmoNotFound,
    ModfNotFound,
    MverNotFound,
    MomoNotFound,
    MohdNotFound,
    ModsNotFound,
    ModnNotFound,
    ModdNotFound,
    MhdrNotFound,
    UnrecognizedDbcFile,
    FailedToOpenDbc,
    FailedToOpenAdt,
    AdtVersionIsIncorrect,
    MverDoesNotBeginAdtFile,
    EmptyWmoDoodadInstantiated,
    FailedToOpenFileForBinaryStream,
    BadFormatOfGameobjectFile,
    FailedToOpenGameobjectFile,
    UnrecognizedModelExtension,
    GameObjectReferencesNonExistentModelId,
    AdtSerializationFailedToOpenOutputFile,
    WmoSerializationFailedToOpenOutputFile,
    RequestedBvhNotFound,
    BvhIndexFileNotFound,
    IncorrectFileSignature,
    IncorrectFileVersion,
    NotWmoNavFile,
    NotWmoTileCoordinates,
    NotAdtNavFile,
    InvalidMapFile,
    IncorrectWmoCoordinates,
    DtNavMeshQueryInitFailed,
    UnknownWmoInstanceRequested,
    UnknownDoodadInstanceRequested,
    CouldNotDeserializeDoodad,
    CouldNotDeserializeWmo,
    IncorrectAdtCoordinates,
    TileNotFoundForRequested,
    CouldNotFindWmo,
    DoodadPathNotFound,
    UnsupportedDooadFormat,
    MapIdNotFound,
    WmoNotFound,
    BinaryStreamCompressFailed,
    MzInflateinitFailed,
    MzInflateFailed,
    InvalidRowColumnFromDbc,
    GameobjectWithSpecifiedGuidAlreadyExists,
    TemporaryWmoObstaclesAreNotSupported,
    NoDoodadSetSpecifiedForWmoGameObject,
    BadMatrixRow,
    InvalidMatrixMultiplication,
    Only4x4MatrixIsSupported,
    MatrixNotInvertible,
    UnexpectedVersionMagicInAlphaModel,
    UnexpectedVersionSizeInAlphaModel,
    UnsupportedAlphaModelVersion,
    UnexpectedVertexMagicInAlphaModel,
    UnexpectedTriangleMagicInAlphaModel,
    InvalidDoodadFile,
    CouldNotOpenMpq,
    GetRootAreaIdInvalidId,
    NoDataFilesFound,
    MpqManagerNotIniatialized,
    ErrorInSFileOpenFileEx,
    ErrorInSFileReadFile,
    MultipleCandidatesInAlphaMpq,
    TooManyFilesInAlphaMpq,
    NoMpqCandidate,
    RecastFailure,
    BufferTooSmall,
    UnknownPath,
    UnknownHeight,
    UnknownZoneAndArea,
    FailedToLoadAdt,
    MapDoesNotHaveAdt,
    UnknownException,

    MapIsNullPointer,
}

pub(crate) fn error_code_to_error(v: u8) -> NamigatorError {
    match v {
        namigator_sys::UNRECOGNIZED_EXTENSION => NamigatorError::UnrecognizedExtension,
        namigator_sys::NO_MOGP_CHUNK => NamigatorError::NoMogpChunk,
        namigator_sys::NO_MOPY_CHUNK => NamigatorError::NoMopyChunk,
        namigator_sys::NO_MOVI_CHUNK => NamigatorError::NoMoviChunk,
        namigator_sys::NO_MOVT_CHUNK => NamigatorError::NoMovtChunk,
        namigator_sys::NO_MCNK_CHUNK => NamigatorError::NoMcnkChunk,
        namigator_sys::WDT_OPEN_FAILED => NamigatorError::WdtOpenFailed,
        namigator_sys::MPHD_NOT_FOUND => NamigatorError::MphdNotFound,
        namigator_sys::MAIN_NOT_FOUND => NamigatorError::MainNotFound,
        namigator_sys::MDNM_NOT_FOUND => NamigatorError::MdnmNotFound,
        namigator_sys::MONM_NOT_FOUND => NamigatorError::MonmNotFound,
        namigator_sys::MWMO_NOT_FOUND => NamigatorError::MwmoNotFound,
        namigator_sys::MODF_NOT_FOUND => NamigatorError::ModfNotFound,
        namigator_sys::MVER_NOT_FOUND => NamigatorError::MverNotFound,
        namigator_sys::MOMO_NOT_FOUND => NamigatorError::MomoNotFound,
        namigator_sys::MOHD_NOT_FOUND => NamigatorError::MohdNotFound,
        namigator_sys::MODS_NOT_FOUND => NamigatorError::ModsNotFound,
        namigator_sys::MODN_NOT_FOUND => NamigatorError::ModnNotFound,
        namigator_sys::MODD_NOT_FOUND => NamigatorError::ModdNotFound,
        namigator_sys::MHDR_NOT_FOUND => NamigatorError::MhdrNotFound,
        namigator_sys::UNRECOGNIZED_DBC_FILE => NamigatorError::UnrecognizedDbcFile,
        namigator_sys::FAILED_TO_OPEN_DBC => NamigatorError::FailedToOpenDbc,
        namigator_sys::FAILED_TO_OPEN_ADT => NamigatorError::FailedToOpenAdt,
        namigator_sys::ADT_VERSION_IS_INCORRECT => NamigatorError::AdtVersionIsIncorrect,
        namigator_sys::MVER_DOES_NOT_BEGIN_ADT_FILE => NamigatorError::MverDoesNotBeginAdtFile,
        namigator_sys::EMPTY_WMO_DOODAD_INSTANTIATED => NamigatorError::EmptyWmoDoodadInstantiated,
        namigator_sys::FAILED_TO_OPEN_FILE_FOR_BINARY_STREAM => {
            NamigatorError::FailedToOpenFileForBinaryStream
        }
        namigator_sys::BAD_FORMAT_OF_GAMEOBJECT_FILE => NamigatorError::BadFormatOfGameobjectFile,
        namigator_sys::FAILED_TO_OPEN_GAMEOBJECT_FILE => NamigatorError::FailedToOpenGameobjectFile,
        namigator_sys::UNRECOGNIZED_MODEL_EXTENSION => NamigatorError::UnrecognizedModelExtension,
        namigator_sys::GAME_OBJECT_REFERENCES_NON_EXISTENT_MODEL_ID => {
            NamigatorError::GameObjectReferencesNonExistentModelId
        }
        namigator_sys::ADT_SERIALIZATION_FAILED_TO_OPEN_OUTPUT_FILE => {
            NamigatorError::AdtSerializationFailedToOpenOutputFile
        }
        namigator_sys::WMO_SERIALIZATION_FAILED_TO_OPEN_OUTPUT_FILE => {
            NamigatorError::WmoSerializationFailedToOpenOutputFile
        }
        namigator_sys::REQUESTED_BVH_NOT_FOUND => NamigatorError::RequestedBvhNotFound,
        namigator_sys::BVH_INDEX_FILE_NOT_FOUND => NamigatorError::BvhIndexFileNotFound,
        namigator_sys::INCORRECT_FILE_SIGNATURE => NamigatorError::IncorrectFileSignature,
        namigator_sys::INCORRECT_FILE_VERSION => NamigatorError::IncorrectFileVersion,
        namigator_sys::NOT_WMO_NAV_FILE => NamigatorError::NotWmoNavFile,
        namigator_sys::NOT_WMO_TILE_COORDINATES => NamigatorError::NotWmoTileCoordinates,
        namigator_sys::NOT_ADT_NAV_FILE => NamigatorError::NotAdtNavFile,
        namigator_sys::INVALID_MAP_FILE => NamigatorError::InvalidMapFile,
        namigator_sys::INCORRECT_WMO_COORDINATES => NamigatorError::IncorrectWmoCoordinates,
        namigator_sys::DTNAVMESHQUERY_INIT_FAILED => NamigatorError::DtNavMeshQueryInitFailed,
        namigator_sys::UNKNOWN_WMO_INSTANCE_REQUESTED => {
            NamigatorError::UnknownWmoInstanceRequested
        }
        namigator_sys::UNKNOWN_DOODAD_INSTANCE_REQUESTED => {
            NamigatorError::UnknownDoodadInstanceRequested
        }
        namigator_sys::COULD_NOT_DESERIALIZE_DOODAD => NamigatorError::CouldNotDeserializeDoodad,
        namigator_sys::COULD_NOT_DESERIALIZE_WMO => NamigatorError::CouldNotDeserializeWmo,
        namigator_sys::INCORRECT_ADT_COORDINATES => NamigatorError::IncorrectAdtCoordinates,
        namigator_sys::TILE_NOT_FOUND_FOR_REQUESTED => NamigatorError::TileNotFoundForRequested,
        namigator_sys::COULD_NOT_FIND_WMO => NamigatorError::CouldNotFindWmo,
        namigator_sys::DOODAD_PATH_NOT_FOUND => NamigatorError::DoodadPathNotFound,
        namigator_sys::UNSUPPORTED_DOOAD_FORMAT => NamigatorError::UnsupportedDooadFormat,
        namigator_sys::MAP_ID_NOT_FOUND => NamigatorError::MapIdNotFound,
        namigator_sys::WMO_NOT_FOUND => NamigatorError::WmoNotFound,
        namigator_sys::BINARYSTREAM_COMPRESS_FAILED => NamigatorError::BinaryStreamCompressFailed,
        namigator_sys::MZ_INFLATEINIT_FAILED => NamigatorError::MzInflateinitFailed,
        namigator_sys::MZ_INFLATE_FAILED => NamigatorError::MzInflateFailed,
        namigator_sys::INVALID_ROW_COLUMN_FROM_DBC => NamigatorError::InvalidRowColumnFromDbc,
        namigator_sys::GAMEOBJECT_WITH_SPECIFIED_GUID_ALREADY_EXISTS => {
            NamigatorError::GameobjectWithSpecifiedGuidAlreadyExists
        }
        namigator_sys::TEMPORARY_WMO_OBSTACLES_ARE_NOT_SUPPORTED => {
            NamigatorError::TemporaryWmoObstaclesAreNotSupported
        }
        namigator_sys::NO_DOODAD_SET_SPECIFIED_FOR_WMO_GAME_OBJECT => {
            NamigatorError::NoDoodadSetSpecifiedForWmoGameObject
        }
        namigator_sys::BAD_MATRIX_ROW => NamigatorError::BadMatrixRow,
        namigator_sys::INVALID_MATRIX_MULTIPLICATION => NamigatorError::InvalidMatrixMultiplication,
        namigator_sys::ONLY_4X4_MATRIX_IS_SUPPORTED => NamigatorError::Only4x4MatrixIsSupported,
        namigator_sys::MATRIX_NOT_INVERTIBLE => NamigatorError::MatrixNotInvertible,
        namigator_sys::UNEXPECTED_VERSION_MAGIC_IN_ALPHA_MODEL => {
            NamigatorError::UnexpectedVersionMagicInAlphaModel
        }
        namigator_sys::UNEXPECTED_VERSION_SIZE_IN_ALPHA_MODEL => {
            NamigatorError::UnexpectedVersionSizeInAlphaModel
        }
        namigator_sys::UNSUPPORTED_ALPHA_MODEL_VERSION => {
            NamigatorError::UnsupportedAlphaModelVersion
        }
        namigator_sys::UNEXPECTED_VERTEX_MAGIC_IN_ALPHA_MODEL => {
            NamigatorError::UnexpectedVertexMagicInAlphaModel
        }
        namigator_sys::UNEXPECTED_TRIANGLE_MAGIC_IN_ALPHA_MODEL => {
            NamigatorError::UnexpectedTriangleMagicInAlphaModel
        }
        namigator_sys::INVALID_DOODAD_FILE => NamigatorError::InvalidDoodadFile,
        namigator_sys::COULD_NOT_OPEN_MPQ => NamigatorError::CouldNotOpenMpq,
        namigator_sys::GETROOTAREAID_INVALID_ID => NamigatorError::GetRootAreaIdInvalidId,
        namigator_sys::NO_DATA_FILES_FOUND => NamigatorError::NoDataFilesFound,
        namigator_sys::MPQ_MANAGER_NOT_INIATIALIZED => NamigatorError::MpqManagerNotIniatialized,
        namigator_sys::ERROR_IN_SFILEOPENFILEX => NamigatorError::ErrorInSFileOpenFileEx,
        namigator_sys::ERROR_IN_SFILEREADFILE => NamigatorError::ErrorInSFileReadFile,
        namigator_sys::MULTIPLE_CANDIDATES_IN_ALPHA_MPQ => {
            NamigatorError::MultipleCandidatesInAlphaMpq
        }
        namigator_sys::TOO_MANY_FILES_IN_ALPHA_MPQ => NamigatorError::TooManyFilesInAlphaMpq,
        namigator_sys::NO_MPQ_CANDIDATE => NamigatorError::NoMpqCandidate,
        namigator_sys::RECAST_FAILURE => NamigatorError::RecastFailure,
        namigator_sys::BUFFER_TOO_SMALL => NamigatorError::BufferTooSmall,
        namigator_sys::UNKNOWN_PATH => NamigatorError::UnknownPath,
        namigator_sys::UNKNOWN_HEIGHT => NamigatorError::UnknownHeight,
        namigator_sys::UNKNOWN_ZONE_AND_AREA => NamigatorError::UnknownZoneAndArea,
        namigator_sys::FAILED_TO_LOAD_ADT => NamigatorError::FailedToLoadAdt,
        namigator_sys::MAP_DOES_NOT_HAVE_ADT => NamigatorError::MapDoesNotHaveAdt,
        namigator_sys::UNKNOWN_EXCEPTION => NamigatorError::UnknownException,
        _ => NamigatorError::UnknownException,
    }
}

impl Display for NamigatorError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            NamigatorError::CStringConversion(e) => e.fmt(f),
            NamigatorError::UnknownException => f.write_str("Unknown Exception"),
            NamigatorError::UnrecognizedExtension => f.write_str("Unrecognized extension"),
            NamigatorError::NoMogpChunk => f.write_str("No MOGP chunk"),
            NamigatorError::NoMopyChunk => f.write_str("No MOPY chunk"),
            NamigatorError::NoMoviChunk => f.write_str("No MOVI chunk"),
            NamigatorError::NoMovtChunk => f.write_str("No MOVT chunk"),
            NamigatorError::NoMcnkChunk => f.write_str("No MCNK chunk"),
            NamigatorError::WdtOpenFailed => f.write_str("WDT open failed"),
            NamigatorError::MphdNotFound => f.write_str("MPHD not found"),
            NamigatorError::MainNotFound => f.write_str("MAIN not found"),
            NamigatorError::MdnmNotFound => f.write_str("MDNM not found"),
            NamigatorError::MonmNotFound => f.write_str("MONM not found"),
            NamigatorError::MwmoNotFound => f.write_str("MWMO not found"),
            NamigatorError::ModfNotFound => f.write_str("MODF not found"),
            NamigatorError::MverNotFound => f.write_str("MVER not found"),
            NamigatorError::MomoNotFound => f.write_str("MOMO not found"),
            NamigatorError::MohdNotFound => f.write_str("MOHD not found"),
            NamigatorError::ModsNotFound => f.write_str("MODS not found"),
            NamigatorError::ModnNotFound => f.write_str("MODN not found"),
            NamigatorError::ModdNotFound => f.write_str("MODD not found"),
            NamigatorError::MhdrNotFound => f.write_str("MHDR not found"),
            NamigatorError::UnrecognizedDbcFile => f.write_str("Unrecognized DBC file"),
            NamigatorError::FailedToOpenDbc => f.write_str("Failed to open DBC"),
            NamigatorError::FailedToOpenAdt => f.write_str("Failed to open ADT"),
            NamigatorError::AdtVersionIsIncorrect => f.write_str("ADT version is incorrect"),
            NamigatorError::MverDoesNotBeginAdtFile => f.write_str("MVER does not begin ADT file"),
            NamigatorError::EmptyWmoDoodadInstantiated => {
                f.write_str("Empty WMO doodads instantiated")
            }
            NamigatorError::FailedToOpenFileForBinaryStream => {
                f.write_str("Failed to open file for BinaryStream")
            }
            NamigatorError::BadFormatOfGameobjectFile => {
                f.write_str("Bad format of gameobject file")
            }
            NamigatorError::FailedToOpenGameobjectFile => {
                f.write_str("Failed to open gameobject file")
            }
            NamigatorError::UnrecognizedModelExtension => {
                f.write_str("Unrecognized model extension")
            }
            NamigatorError::GameObjectReferencesNonExistentModelId => {
                f.write_str("Gameobject references non-existent model id")
            }
            NamigatorError::AdtSerializationFailedToOpenOutputFile => {
                f.write_str("ADT serialization failed to open output file")
            }
            NamigatorError::WmoSerializationFailedToOpenOutputFile => {
                f.write_str("WMO serialization failed to open output file")
            }
            NamigatorError::RequestedBvhNotFound => f.write_str("Requested BVH not found"),
            NamigatorError::BvhIndexFileNotFound => f.write_str("BVH index file not found"),
            NamigatorError::IncorrectFileSignature => f.write_str("Incorrect file signature"),
            NamigatorError::IncorrectFileVersion => f.write_str("Incorrect file version"),
            NamigatorError::NotWmoNavFile => f.write_str("Not WMO nav file"),
            NamigatorError::NotWmoTileCoordinates => f.write_str("Not WMO tile coordinates"),
            NamigatorError::NotAdtNavFile => f.write_str("Not ADT nav file"),
            NamigatorError::InvalidMapFile => f.write_str("Invalid map file"),
            NamigatorError::IncorrectWmoCoordinates => f.write_str("Incorrect WMO coordinates"),
            NamigatorError::DtNavMeshQueryInitFailed => f.write_str("dtNavMeshQuery::init failed"),
            NamigatorError::UnknownWmoInstanceRequested => {
                f.write_str("Unknown WMO instance requested")
            }
            NamigatorError::UnknownDoodadInstanceRequested => {
                f.write_str("Unknown doodad instance requested")
            }
            NamigatorError::CouldNotDeserializeDoodad => {
                f.write_str("Could not deserialize doodad")
            }
            NamigatorError::CouldNotDeserializeWmo => f.write_str("Could not deserialize WMO"),
            NamigatorError::IncorrectAdtCoordinates => f.write_str("Incorrect ADT coordinates"),
            NamigatorError::TileNotFoundForRequested => f.write_str("Tile not found for requested"),
            NamigatorError::CouldNotFindWmo => f.write_str("Could not find WMO"),
            NamigatorError::DoodadPathNotFound => f.write_str("Dooad path not found"),
            NamigatorError::UnsupportedDooadFormat => f.write_str("Unsupported doodad format"),
            NamigatorError::MapIdNotFound => f.write_str("Map id not found"),
            NamigatorError::WmoNotFound => f.write_str("WMO not found"),
            NamigatorError::BinaryStreamCompressFailed => {
                f.write_str("BinaryStream::Compress failed")
            }
            NamigatorError::MzInflateinitFailed => f.write_str("mz_inflateInit failed"),
            NamigatorError::MzInflateFailed => f.write_str("mz_inflate failed"),
            NamigatorError::InvalidRowColumnFromDbc => f.write_str("Invalid row, column from DBC"),
            NamigatorError::GameobjectWithSpecifiedGuidAlreadyExists => {
                f.write_str("Gameobject with specified GUID already exists")
            }
            NamigatorError::TemporaryWmoObstaclesAreNotSupported => {
                f.write_str("Temporary WMO obstacles are not supported")
            }
            NamigatorError::NoDoodadSetSpecifiedForWmoGameObject => {
                f.write_str("No doodads specified for WMO gameobject")
            }
            NamigatorError::BadMatrixRow => f.write_str("Bad matrix row"),
            NamigatorError::InvalidMatrixMultiplication => {
                f.write_str("Invalid matrix multiplication")
            }
            NamigatorError::Only4x4MatrixIsSupported => f.write_str("Only 4x4 matrix is supported"),
            NamigatorError::MatrixNotInvertible => f.write_str("Matrix not invertible"),
            NamigatorError::UnexpectedVersionMagicInAlphaModel => {
                f.write_str("Unexpected version magic in alpha model")
            }
            NamigatorError::UnexpectedVersionSizeInAlphaModel => {
                f.write_str("Unexpected version size in alpha model")
            }
            NamigatorError::UnsupportedAlphaModelVersion => {
                f.write_str("Unsupported alpha model version")
            }
            NamigatorError::UnexpectedVertexMagicInAlphaModel => {
                f.write_str("Unexpected vertex magic in alpha model")
            }
            NamigatorError::UnexpectedTriangleMagicInAlphaModel => {
                f.write_str("Unexpected triangle magic in alpha model")
            }
            NamigatorError::InvalidDoodadFile => f.write_str("Invalid doodad file"),
            NamigatorError::CouldNotOpenMpq => f.write_str("Could not open MPQ"),
            NamigatorError::GetRootAreaIdInvalidId => f.write_str("GetRootAreaId invalid id"),
            NamigatorError::NoDataFilesFound => f.write_str("No data files found"),
            NamigatorError::MpqManagerNotIniatialized => {
                f.write_str("MpqManager is not initialized")
            }
            NamigatorError::ErrorInSFileOpenFileEx => f.write_str("Error in SFileOpenFileEx"),
            NamigatorError::ErrorInSFileReadFile => f.write_str("Error in SFileReadFile"),
            NamigatorError::MultipleCandidatesInAlphaMpq => {
                f.write_str("Multiple candidates in alpha MPQ")
            }
            NamigatorError::TooManyFilesInAlphaMpq => f.write_str("Too many files in alpha MPQ"),
            NamigatorError::NoMpqCandidate => f.write_str("No MPQ candidate"),
            NamigatorError::RecastFailure => f.write_str("Recast failure"),
            NamigatorError::BufferTooSmall => f.write_str("Buffer too small"),
            NamigatorError::UnknownPath => f.write_str("Unknown path"),
            NamigatorError::UnknownHeight => f.write_str("Unknown height"),
            NamigatorError::UnknownZoneAndArea => f.write_str("Unknown zone and area"),
            NamigatorError::FailedToLoadAdt => f.write_str("Failed to load ADT"),
            NamigatorError::MapDoesNotHaveAdt => f.write_str("Map does not have ADT"),
            NamigatorError::MapIsNullPointer => f.write_str("Map is null pointer"),
        }
    }
}

impl std::error::Error for NamigatorError {}

impl From<NulError> for NamigatorError {
    fn from(e: NulError) -> Self {
        Self::CStringConversion(e)
    }
}
