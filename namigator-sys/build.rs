use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=vendor");

    let out_dir = PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
    let c_lib_dir = out_dir.join("c_libs");
    let pathfind_dir = out_dir.join("pathfind");
    let namigator_dir = out_dir.join("namigator");

    // C files can not be compiled with the C++ compiler
    // which means we'll have to compile them separately.
    let mut c_build = cc::Build::new();
    c_build.files(c_files()).out_dir(c_lib_dir);

    // Both pathfind and MapBuilder in namigator have
    // a file called Map.cpp. Since `cc` just compiles
    // directly into the OUT_DIR these will overwrite each other
    // and lead to linker errors.
    // It is therefore necessary to compile them in different invocations.
    let mut pathfind = cc::Build::new();
    pathfind
        .files(pathfind_files())
        .includes(cpp_includes())
        .out_dir(pathfind_dir);

    let mut namigator = cc::Build::new();
    namigator
        .files(cpp_files())
        .includes(cpp_includes())
        .out_dir(namigator_dir);

    for def in defines() {
        c_build.define(def, None);
        pathfind.define(def, None);
        namigator.define(def, None);
    }

    for c in [&mut c_build, &mut pathfind, &mut namigator] {
        c.flag_if_supported("-w");
        c.warnings(false);
        c.cargo_metadata(true);
    }

    for c in [&mut pathfind, &mut namigator] {
        c.flag_if_supported("-std=c++17");
        c.flag_if_supported("/std:c++17");
        c.cpp(true);
    }

    c_build.compile("c_build");
    pathfind.compile("pathfind");
    namigator.compile("namigator");

    let test_dir = vendor_dir().join("test");
    let out_dir = PathBuf::from(std::env::var_os("OUT_DIR").unwrap());

    const TEST_FILE_NAME: &str = "test_map.mpq";
    let test_file = test_dir.join(TEST_FILE_NAME);
    let out_test_file = out_dir.join(TEST_FILE_NAME);

    if test_file.exists() {
        std::fs::copy(test_file, out_test_file).unwrap();
    }
}

fn vendor_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("vendor")
}

fn stormlib_src_dir() -> PathBuf {
    vendor_dir().join("stormlib").join("src")
}

fn cpp_includes() -> Vec<PathBuf> {
    vec![
        vendor_dir(),
        vendor_dir().join("MapBuilder"),
        vendor_dir().join("MapViewer"),
        vendor_dir().join("parser"),
        vendor_dir().join("pathfind"),
        vendor_dir().join("utility"),
        vendor_dir().join("stormlib/src"),
        vendor_dir().join("stormlib"),
        vendor_dir().join("recastnavigation/Detour/Include"),
        vendor_dir().join("recastnavigation/DetourCrowd/Include"),
        vendor_dir().join("recastnavigation/DetourTileCache/Include"),
        vendor_dir().join("recastnavigation/Recast/Include"),
    ]
}

fn defines() -> Vec<&'static str> {
    vec![
        "DT_POLYREF64",          // Make Detour use 64 bit values. Set by namigator
        "STORMLIB_NO_AUTO_LINK", // Make stormlib not do some weird linking. Set by namigator.
        "_7ZIP_ST",              // Make lzma be single threaded only. Set by stormlib
        "BZ_STRICT_ANSI",        // Make bzip2 not use _fdopen. Set by stormlib.
    ]
}

fn pathfind_files() -> Vec<PathBuf> {
    vec![
        vendor_dir().join("pathfind/BVH.cpp"),
        vendor_dir().join("pathfind/Map.cpp"),
        vendor_dir().join("pathfind/TemporaryObstacle.cpp"),
        vendor_dir().join("pathfind/Tile.cpp"),
        vendor_dir().join("pathfind/pathfind_c_bindings.cpp"),
    ]
}

fn cpp_files() -> Vec<PathBuf> {
    vec![
        vendor_dir().join("MapBuilder/BVHConstructor.cpp"),
        vendor_dir().join("MapBuilder/GameObjectBVHBuilder.cpp"),
        vendor_dir().join("MapBuilder/FileExist.cpp"),
        vendor_dir().join("MapBuilder/MapBuilder_c_bindings.cpp"),
        vendor_dir().join("MapBuilder/MeshBuilder.cpp"),
        vendor_dir().join("MapBuilder/RecastContext.cpp"),
        vendor_dir().join("MapBuilder/Worker.cpp"),
        vendor_dir().join("parser/Adt/Adt.cpp"),
        vendor_dir().join("parser/Adt/AdtChunk.cpp"),
        vendor_dir().join("parser/Adt/Chunks/MCNK.cpp"),
        vendor_dir().join("parser/Adt/Chunks/MDDF.cpp"),
        vendor_dir().join("parser/Adt/Chunks/MH2O.cpp"),
        vendor_dir().join("parser/Adt/Chunks/MHDR.cpp"),
        vendor_dir().join("parser/Adt/Chunks/MMDX.cpp"),
        vendor_dir().join("parser/Adt/Chunks/MODF.cpp"),
        vendor_dir().join("parser/Adt/Chunks/MWMO.cpp"),
        vendor_dir().join("parser/Adt/Chunks/Subchunks/MCLQ.cpp"),
        vendor_dir().join("parser/Adt/Chunks/Subchunks/MCVT.cpp"),
        vendor_dir().join("parser/DBC.cpp"),
        vendor_dir().join("parser/Doodad/Doodad.cpp"),
        vendor_dir().join("parser/Doodad/DoodadInstance.cpp"),
        vendor_dir().join("parser/Map/Map.cpp"),
        vendor_dir().join("parser/MpqManager.cpp"),
        vendor_dir().join("parser/Wmo/GroupFile/Chunks/MLIQ.cpp"),
        vendor_dir().join("parser/Wmo/GroupFile/Chunks/MOPY.cpp"),
        vendor_dir().join("parser/Wmo/GroupFile/Chunks/MOVI.cpp"),
        vendor_dir().join("parser/Wmo/GroupFile/Chunks/MOVT.cpp"),
        vendor_dir().join("parser/Wmo/GroupFile/WmoGroupChunk.cpp"),
        vendor_dir().join("parser/Wmo/GroupFile/WmoGroupFile.cpp"),
        vendor_dir().join("parser/Wmo/RootFile/Chunks/MODD.cpp"),
        vendor_dir().join("parser/Wmo/RootFile/Chunks/MODN.cpp"),
        vendor_dir().join("parser/Wmo/RootFile/Chunks/MODS.cpp"),
        vendor_dir().join("parser/Wmo/Wmo.cpp"),
        vendor_dir().join("parser/Wmo/WmoDoodad.cpp"),
        vendor_dir().join("parser/Wmo/WmoInstance.cpp"),
        vendor_dir().join("recastnavigation/Detour/Source/DetourAlloc.cpp"),
        vendor_dir().join("recastnavigation/Detour/Source/DetourAssert.cpp"),
        vendor_dir().join("recastnavigation/Detour/Source/DetourCommon.cpp"),
        vendor_dir().join("recastnavigation/Detour/Source/DetourNavMesh.cpp"),
        vendor_dir().join("recastnavigation/Detour/Source/DetourNavMeshBuilder.cpp"),
        vendor_dir().join("recastnavigation/Detour/Source/DetourNavMeshQuery.cpp"),
        vendor_dir().join("recastnavigation/Detour/Source/DetourNode.cpp"),
        vendor_dir().join("recastnavigation/DetourCrowd/Source/DetourCrowd.cpp"),
        vendor_dir().join("recastnavigation/DetourCrowd/Source/DetourLocalBoundary.cpp"),
        vendor_dir().join("recastnavigation/DetourCrowd/Source/DetourObstacleAvoidance.cpp"),
        vendor_dir().join("recastnavigation/DetourCrowd/Source/DetourPathCorridor.cpp"),
        vendor_dir().join("recastnavigation/DetourCrowd/Source/DetourPathQueue.cpp"),
        vendor_dir().join("recastnavigation/DetourCrowd/Source/DetourProximityGrid.cpp"),
        vendor_dir().join("recastnavigation/DetourTileCache/Source/DetourTileCache.cpp"),
        vendor_dir().join("recastnavigation/DetourTileCache/Source/DetourTileCacheBuilder.cpp"),
        vendor_dir().join("recastnavigation/Recast/Source/Recast.cpp"),
        vendor_dir().join("recastnavigation/Recast/Source/RecastAlloc.cpp"),
        vendor_dir().join("recastnavigation/Recast/Source/RecastArea.cpp"),
        vendor_dir().join("recastnavigation/Recast/Source/RecastAssert.cpp"),
        vendor_dir().join("recastnavigation/Recast/Source/RecastContour.cpp"),
        vendor_dir().join("recastnavigation/Recast/Source/RecastFilter.cpp"),
        vendor_dir().join("recastnavigation/Recast/Source/RecastLayers.cpp"),
        vendor_dir().join("recastnavigation/Recast/Source/RecastMesh.cpp"),
        vendor_dir().join("recastnavigation/Recast/Source/RecastMeshDetail.cpp"),
        vendor_dir().join("recastnavigation/Recast/Source/RecastRasterization.cpp"),
        vendor_dir().join("recastnavigation/Recast/Source/RecastRegion.cpp"),
        vendor_dir().join("utility/AABBTree.cpp"),
        vendor_dir().join("utility/BinaryStream.cpp"),
        vendor_dir().join("utility/BoundingBox.cpp"),
        vendor_dir().join("utility/MathHelper.cpp"),
        vendor_dir().join("utility/Matrix.cpp"),
        vendor_dir().join("utility/Quaternion.cpp"),
        vendor_dir().join("utility/Ray.cpp"),
        vendor_dir().join("utility/String.cpp"),
        vendor_dir().join("utility/Vector.cpp"),
        vendor_dir().join("stormlib/src/FileStream.cpp"),
        vendor_dir().join("stormlib/src/SBaseCommon.cpp"),
        vendor_dir().join("stormlib/src/SBaseDumpData.cpp"),
        vendor_dir().join("stormlib/src/SBaseFileTable.cpp"),
        vendor_dir().join("stormlib/src/SBaseSubTypes.cpp"),
        vendor_dir().join("stormlib/src/SCompression.cpp"),
        vendor_dir().join("stormlib/src/SFileAddFile.cpp"),
        vendor_dir().join("stormlib/src/SFileAttributes.cpp"),
        vendor_dir().join("stormlib/src/SFileCompactArchive.cpp"),
        vendor_dir().join("stormlib/src/SFileCreateArchive.cpp"),
        vendor_dir().join("stormlib/src/SFileExtractFile.cpp"),
        vendor_dir().join("stormlib/src/SFileFindFile.cpp"),
        vendor_dir().join("stormlib/src/SFileGetFileInfo.cpp"),
        vendor_dir().join("stormlib/src/SFileListFile.cpp"),
        vendor_dir().join("stormlib/src/SFileOpenArchive.cpp"),
        vendor_dir().join("stormlib/src/SFileOpenFileEx.cpp"),
        vendor_dir().join("stormlib/src/SFilePatchArchives.cpp"),
        vendor_dir().join("stormlib/src/SFileReadFile.cpp"),
        vendor_dir().join("stormlib/src/SFileVerify.cpp"),
        vendor_dir().join("stormlib/src/adpcm/adpcm.cpp"),
        vendor_dir().join("stormlib/src/huffman/huff.cpp"),
        vendor_dir().join("stormlib/src/sparse/sparse.cpp"),
    ]
}

fn c_files() -> Vec<PathBuf> {
    vec![
        stormlib_src_dir().join("zlib/adler32.c"),
        stormlib_src_dir().join("zlib/compress_zlib.c"),
        stormlib_src_dir().join("zlib/crc32.c"),
        stormlib_src_dir().join("zlib/deflate.c"),
        stormlib_src_dir().join("zlib/inffast.c"),
        stormlib_src_dir().join("zlib/inflate.c"),
        stormlib_src_dir().join("zlib/inftrees.c"),
        stormlib_src_dir().join("zlib/trees.c"),
        stormlib_src_dir().join("zlib/zutil.c"),
        stormlib_src_dir().join("bzip2/blocksort.c"),
        stormlib_src_dir().join("bzip2/bzlib.c"),
        stormlib_src_dir().join("bzip2/compress.c"),
        stormlib_src_dir().join("bzip2/crctable.c"),
        stormlib_src_dir().join("bzip2/decompress.c"),
        stormlib_src_dir().join("bzip2/huffman.c"),
        stormlib_src_dir().join("bzip2/randtable.c"),
        stormlib_src_dir().join("jenkins/lookup3.c"),
        stormlib_src_dir().join("libtomcrypt/src/hashes/hash_memory.c"),
        stormlib_src_dir().join("libtomcrypt/src/hashes/md5.c"),
        stormlib_src_dir().join("libtomcrypt/src/hashes/sha1.c"),
        stormlib_src_dir().join("libtomcrypt/src/math/ltm_desc.c"),
        stormlib_src_dir().join("libtomcrypt/src/math/multi.c"),
        stormlib_src_dir().join("libtomcrypt/src/math/rand_prime.c"),
        stormlib_src_dir().join("libtomcrypt/src/misc/base64_decode.c"),
        stormlib_src_dir().join("libtomcrypt/src/misc/crypt_argchk.c"),
        stormlib_src_dir().join("libtomcrypt/src/misc/crypt_find_hash.c"),
        stormlib_src_dir().join("libtomcrypt/src/misc/crypt_find_prng.c"),
        stormlib_src_dir().join("libtomcrypt/src/misc/crypt_hash_descriptor.c"),
        stormlib_src_dir().join("libtomcrypt/src/misc/crypt_hash_is_valid.c"),
        stormlib_src_dir().join("libtomcrypt/src/misc/crypt_libc.c"),
        stormlib_src_dir().join("libtomcrypt/src/misc/crypt_ltc_mp_descriptor.c"),
        stormlib_src_dir().join("libtomcrypt/src/misc/crypt_prng_descriptor.c"),
        stormlib_src_dir().join("libtomcrypt/src/misc/crypt_prng_is_valid.c"),
        stormlib_src_dir().join("libtomcrypt/src/misc/crypt_register_hash.c"),
        stormlib_src_dir().join("libtomcrypt/src/misc/crypt_register_prng.c"),
        stormlib_src_dir().join("libtomcrypt/src/misc/zeromem.c"),
        stormlib_src_dir().join("libtomcrypt/src/pk/asn1/der_decode_bit_string.c"),
        stormlib_src_dir().join("libtomcrypt/src/pk/asn1/der_decode_boolean.c"),
        stormlib_src_dir().join("libtomcrypt/src/pk/asn1/der_decode_choice.c"),
        stormlib_src_dir().join("libtomcrypt/src/pk/asn1/der_decode_ia5_string.c"),
        stormlib_src_dir().join("libtomcrypt/src/pk/asn1/der_decode_integer.c"),
        stormlib_src_dir().join("libtomcrypt/src/pk/asn1/der_decode_object_identifier.c"),
        stormlib_src_dir().join("libtomcrypt/src/pk/asn1/der_decode_octet_string.c"),
        stormlib_src_dir().join("libtomcrypt/src/pk/asn1/der_decode_printable_string.c"),
        stormlib_src_dir().join("libtomcrypt/src/pk/asn1/der_decode_sequence_ex.c"),
        stormlib_src_dir().join("libtomcrypt/src/pk/asn1/der_decode_sequence_flexi.c"),
        stormlib_src_dir().join("libtomcrypt/src/pk/asn1/der_decode_sequence_multi.c"),
        stormlib_src_dir().join("libtomcrypt/src/pk/asn1/der_decode_short_integer.c"),
        stormlib_src_dir().join("libtomcrypt/src/pk/asn1/der_decode_utctime.c"),
        stormlib_src_dir().join("libtomcrypt/src/pk/asn1/der_decode_utf8_string.c"),
        stormlib_src_dir().join("libtomcrypt/src/pk/asn1/der_encode_bit_string.c"),
        stormlib_src_dir().join("libtomcrypt/src/pk/asn1/der_encode_boolean.c"),
        stormlib_src_dir().join("libtomcrypt/src/pk/asn1/der_encode_ia5_string.c"),
        stormlib_src_dir().join("libtomcrypt/src/pk/asn1/der_encode_integer.c"),
        stormlib_src_dir().join("libtomcrypt/src/pk/asn1/der_encode_object_identifier.c"),
        stormlib_src_dir().join("libtomcrypt/src/pk/asn1/der_encode_octet_string.c"),
        stormlib_src_dir().join("libtomcrypt/src/pk/asn1/der_encode_printable_string.c"),
        stormlib_src_dir().join("libtomcrypt/src/pk/asn1/der_encode_sequence_ex.c"),
        stormlib_src_dir().join("libtomcrypt/src/pk/asn1/der_encode_sequence_multi.c"),
        stormlib_src_dir().join("libtomcrypt/src/pk/asn1/der_encode_set.c"),
        stormlib_src_dir().join("libtomcrypt/src/pk/asn1/der_encode_setof.c"),
        stormlib_src_dir().join("libtomcrypt/src/pk/asn1/der_encode_short_integer.c"),
        stormlib_src_dir().join("libtomcrypt/src/pk/asn1/der_encode_utctime.c"),
        stormlib_src_dir().join("libtomcrypt/src/pk/asn1/der_encode_utf8_string.c"),
        stormlib_src_dir().join("libtomcrypt/src/pk/asn1/der_length_bit_string.c"),
        stormlib_src_dir().join("libtomcrypt/src/pk/asn1/der_length_boolean.c"),
        stormlib_src_dir().join("libtomcrypt/src/pk/asn1/der_length_ia5_string.c"),
        stormlib_src_dir().join("libtomcrypt/src/pk/asn1/der_length_integer.c"),
        stormlib_src_dir().join("libtomcrypt/src/pk/asn1/der_length_object_identifier.c"),
        stormlib_src_dir().join("libtomcrypt/src/pk/asn1/der_length_octet_string.c"),
        stormlib_src_dir().join("libtomcrypt/src/pk/asn1/der_length_printable_string.c"),
        stormlib_src_dir().join("libtomcrypt/src/pk/asn1/der_length_sequence.c"),
        stormlib_src_dir().join("libtomcrypt/src/pk/asn1/der_length_short_integer.c"),
        stormlib_src_dir().join("libtomcrypt/src/pk/asn1/der_length_utctime.c"),
        stormlib_src_dir().join("libtomcrypt/src/pk/asn1/der_length_utf8_string.c"),
        stormlib_src_dir().join("libtomcrypt/src/pk/asn1/der_sequence_free.c"),
        stormlib_src_dir().join("libtomcrypt/src/pk/ecc/ltc_ecc_map.c"),
        stormlib_src_dir().join("libtomcrypt/src/pk/ecc/ltc_ecc_mul2add.c"),
        stormlib_src_dir().join("libtomcrypt/src/pk/ecc/ltc_ecc_mulmod.c"),
        stormlib_src_dir().join("libtomcrypt/src/pk/ecc/ltc_ecc_points.c"),
        stormlib_src_dir().join("libtomcrypt/src/pk/ecc/ltc_ecc_projective_add_point.c"),
        stormlib_src_dir().join("libtomcrypt/src/pk/ecc/ltc_ecc_projective_dbl_point.c"),
        stormlib_src_dir().join("libtomcrypt/src/pk/pkcs1/pkcs_1_mgf1.c"),
        stormlib_src_dir().join("libtomcrypt/src/pk/pkcs1/pkcs_1_oaep_decode.c"),
        stormlib_src_dir().join("libtomcrypt/src/pk/pkcs1/pkcs_1_pss_decode.c"),
        stormlib_src_dir().join("libtomcrypt/src/pk/pkcs1/pkcs_1_pss_encode.c"),
        stormlib_src_dir().join("libtomcrypt/src/pk/pkcs1/pkcs_1_v1_5_decode.c"),
        stormlib_src_dir().join("libtomcrypt/src/pk/pkcs1/pkcs_1_v1_5_encode.c"),
        stormlib_src_dir().join("libtomcrypt/src/pk/rsa/rsa_exptmod.c"),
        stormlib_src_dir().join("libtomcrypt/src/pk/rsa/rsa_free.c"),
        stormlib_src_dir().join("libtomcrypt/src/pk/rsa/rsa_import.c"),
        stormlib_src_dir().join("libtomcrypt/src/pk/rsa/rsa_make_key.c"),
        stormlib_src_dir().join("libtomcrypt/src/pk/rsa/rsa_sign_hash.c"),
        stormlib_src_dir().join("libtomcrypt/src/pk/rsa/rsa_verify_hash.c"),
        stormlib_src_dir().join("libtomcrypt/src/pk/rsa/rsa_verify_simple.c"),
        stormlib_src_dir().join("libtommath/bn_fast_mp_invmod.c"),
        stormlib_src_dir().join("libtommath/bn_fast_mp_montgomery_reduce.c"),
        stormlib_src_dir().join("libtommath/bn_fast_s_mp_mul_digs.c"),
        stormlib_src_dir().join("libtommath/bn_fast_s_mp_mul_high_digs.c"),
        stormlib_src_dir().join("libtommath/bn_fast_s_mp_sqr.c"),
        stormlib_src_dir().join("libtommath/bn_mp_2expt.c"),
        stormlib_src_dir().join("libtommath/bn_mp_abs.c"),
        stormlib_src_dir().join("libtommath/bn_mp_add.c"),
        stormlib_src_dir().join("libtommath/bn_mp_add_d.c"),
        stormlib_src_dir().join("libtommath/bn_mp_addmod.c"),
        stormlib_src_dir().join("libtommath/bn_mp_and.c"),
        stormlib_src_dir().join("libtommath/bn_mp_clamp.c"),
        stormlib_src_dir().join("libtommath/bn_mp_clear.c"),
        stormlib_src_dir().join("libtommath/bn_mp_clear_multi.c"),
        stormlib_src_dir().join("libtommath/bn_mp_cmp.c"),
        stormlib_src_dir().join("libtommath/bn_mp_cmp_d.c"),
        stormlib_src_dir().join("libtommath/bn_mp_cmp_mag.c"),
        stormlib_src_dir().join("libtommath/bn_mp_cnt_lsb.c"),
        stormlib_src_dir().join("libtommath/bn_mp_copy.c"),
        stormlib_src_dir().join("libtommath/bn_mp_count_bits.c"),
        stormlib_src_dir().join("libtommath/bn_mp_div.c"),
        stormlib_src_dir().join("libtommath/bn_mp_div_2.c"),
        stormlib_src_dir().join("libtommath/bn_mp_div_2d.c"),
        stormlib_src_dir().join("libtommath/bn_mp_div_3.c"),
        stormlib_src_dir().join("libtommath/bn_mp_div_d.c"),
        stormlib_src_dir().join("libtommath/bn_mp_dr_is_modulus.c"),
        stormlib_src_dir().join("libtommath/bn_mp_dr_reduce.c"),
        stormlib_src_dir().join("libtommath/bn_mp_dr_setup.c"),
        stormlib_src_dir().join("libtommath/bn_mp_exch.c"),
        stormlib_src_dir().join("libtommath/bn_mp_expt_d.c"),
        stormlib_src_dir().join("libtommath/bn_mp_exptmod.c"),
        stormlib_src_dir().join("libtommath/bn_mp_exptmod_fast.c"),
        stormlib_src_dir().join("libtommath/bn_mp_exteuclid.c"),
        stormlib_src_dir().join("libtommath/bn_mp_fread.c"),
        stormlib_src_dir().join("libtommath/bn_mp_fwrite.c"),
        stormlib_src_dir().join("libtommath/bn_mp_gcd.c"),
        stormlib_src_dir().join("libtommath/bn_mp_get_int.c"),
        stormlib_src_dir().join("libtommath/bn_mp_grow.c"),
        stormlib_src_dir().join("libtommath/bn_mp_init.c"),
        stormlib_src_dir().join("libtommath/bn_mp_init_copy.c"),
        stormlib_src_dir().join("libtommath/bn_mp_init_multi.c"),
        stormlib_src_dir().join("libtommath/bn_mp_init_set.c"),
        stormlib_src_dir().join("libtommath/bn_mp_init_set_int.c"),
        stormlib_src_dir().join("libtommath/bn_mp_init_size.c"),
        stormlib_src_dir().join("libtommath/bn_mp_invmod.c"),
        stormlib_src_dir().join("libtommath/bn_mp_invmod_slow.c"),
        stormlib_src_dir().join("libtommath/bn_mp_is_square.c"),
        stormlib_src_dir().join("libtommath/bn_mp_jacobi.c"),
        stormlib_src_dir().join("libtommath/bn_mp_karatsuba_mul.c"),
        stormlib_src_dir().join("libtommath/bn_mp_karatsuba_sqr.c"),
        stormlib_src_dir().join("libtommath/bn_mp_lcm.c"),
        stormlib_src_dir().join("libtommath/bn_mp_lshd.c"),
        stormlib_src_dir().join("libtommath/bn_mp_mod.c"),
        stormlib_src_dir().join("libtommath/bn_mp_mod_2d.c"),
        stormlib_src_dir().join("libtommath/bn_mp_mod_d.c"),
        stormlib_src_dir().join("libtommath/bn_mp_montgomery_calc_normalization.c"),
        stormlib_src_dir().join("libtommath/bn_mp_montgomery_reduce.c"),
        stormlib_src_dir().join("libtommath/bn_mp_montgomery_setup.c"),
        stormlib_src_dir().join("libtommath/bn_mp_mul.c"),
        stormlib_src_dir().join("libtommath/bn_mp_mul_2.c"),
        stormlib_src_dir().join("libtommath/bn_mp_mul_2d.c"),
        stormlib_src_dir().join("libtommath/bn_mp_mul_d.c"),
        stormlib_src_dir().join("libtommath/bn_mp_mulmod.c"),
        stormlib_src_dir().join("libtommath/bn_mp_n_root.c"),
        stormlib_src_dir().join("libtommath/bn_mp_neg.c"),
        stormlib_src_dir().join("libtommath/bn_mp_or.c"),
        stormlib_src_dir().join("libtommath/bn_mp_prime_fermat.c"),
        stormlib_src_dir().join("libtommath/bn_mp_prime_is_divisible.c"),
        stormlib_src_dir().join("libtommath/bn_mp_prime_is_prime.c"),
        stormlib_src_dir().join("libtommath/bn_mp_prime_miller_rabin.c"),
        stormlib_src_dir().join("libtommath/bn_mp_prime_next_prime.c"),
        stormlib_src_dir().join("libtommath/bn_mp_prime_rabin_miller_trials.c"),
        stormlib_src_dir().join("libtommath/bn_mp_prime_random_ex.c"),
        stormlib_src_dir().join("libtommath/bn_mp_radix_size.c"),
        stormlib_src_dir().join("libtommath/bn_mp_radix_smap.c"),
        stormlib_src_dir().join("libtommath/bn_mp_rand.c"),
        stormlib_src_dir().join("libtommath/bn_mp_read_radix.c"),
        stormlib_src_dir().join("libtommath/bn_mp_read_signed_bin.c"),
        stormlib_src_dir().join("libtommath/bn_mp_read_unsigned_bin.c"),
        stormlib_src_dir().join("libtommath/bn_mp_reduce.c"),
        stormlib_src_dir().join("libtommath/bn_mp_reduce_2k.c"),
        stormlib_src_dir().join("libtommath/bn_mp_reduce_2k_l.c"),
        stormlib_src_dir().join("libtommath/bn_mp_reduce_2k_setup.c"),
        stormlib_src_dir().join("libtommath/bn_mp_reduce_2k_setup_l.c"),
        stormlib_src_dir().join("libtommath/bn_mp_reduce_is_2k.c"),
        stormlib_src_dir().join("libtommath/bn_mp_reduce_is_2k_l.c"),
        stormlib_src_dir().join("libtommath/bn_mp_reduce_setup.c"),
        stormlib_src_dir().join("libtommath/bn_mp_rshd.c"),
        stormlib_src_dir().join("libtommath/bn_mp_set.c"),
        stormlib_src_dir().join("libtommath/bn_mp_set_int.c"),
        stormlib_src_dir().join("libtommath/bn_mp_shrink.c"),
        stormlib_src_dir().join("libtommath/bn_mp_signed_bin_size.c"),
        stormlib_src_dir().join("libtommath/bn_mp_sqr.c"),
        stormlib_src_dir().join("libtommath/bn_mp_sqrmod.c"),
        stormlib_src_dir().join("libtommath/bn_mp_sqrt.c"),
        stormlib_src_dir().join("libtommath/bn_mp_sub.c"),
        stormlib_src_dir().join("libtommath/bn_mp_sub_d.c"),
        stormlib_src_dir().join("libtommath/bn_mp_submod.c"),
        stormlib_src_dir().join("libtommath/bn_mp_to_signed_bin.c"),
        stormlib_src_dir().join("libtommath/bn_mp_to_signed_bin_n.c"),
        stormlib_src_dir().join("libtommath/bn_mp_to_unsigned_bin.c"),
        stormlib_src_dir().join("libtommath/bn_mp_to_unsigned_bin_n.c"),
        stormlib_src_dir().join("libtommath/bn_mp_toom_mul.c"),
        stormlib_src_dir().join("libtommath/bn_mp_toom_sqr.c"),
        stormlib_src_dir().join("libtommath/bn_mp_toradix.c"),
        stormlib_src_dir().join("libtommath/bn_mp_toradix_n.c"),
        stormlib_src_dir().join("libtommath/bn_mp_unsigned_bin_size.c"),
        stormlib_src_dir().join("libtommath/bn_mp_xor.c"),
        stormlib_src_dir().join("libtommath/bn_mp_zero.c"),
        stormlib_src_dir().join("libtommath/bn_prime_tab.c"),
        stormlib_src_dir().join("libtommath/bn_reverse.c"),
        stormlib_src_dir().join("libtommath/bn_s_mp_add.c"),
        stormlib_src_dir().join("libtommath/bn_s_mp_exptmod.c"),
        stormlib_src_dir().join("libtommath/bn_s_mp_mul_digs.c"),
        stormlib_src_dir().join("libtommath/bn_s_mp_mul_high_digs.c"),
        stormlib_src_dir().join("libtommath/bn_s_mp_sqr.c"),
        stormlib_src_dir().join("libtommath/bn_s_mp_sub.c"),
        stormlib_src_dir().join("libtommath/bncore.c"),
        stormlib_src_dir().join("lzma/C/LzFind.c"),
        stormlib_src_dir().join("lzma/C/LzmaDec.c"),
        stormlib_src_dir().join("lzma/C/LzmaEnc.c"),
        stormlib_src_dir().join("pklib/explode.c"),
        stormlib_src_dir().join("pklib/implode.c"),
    ]
}
