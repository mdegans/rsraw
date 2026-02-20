use std::{env, path::Path};

fn main() {
    let dir = env::var_os("OUT_DIR").unwrap();
    bindings(&dir);
    build(&dir);
}

fn build(out_dir: impl AsRef<Path>) {
    let mut libraw = cc::Build::new();
    let compiler = libraw.get_compiler();

    // On MSVC, LibRaw headers default to __declspec(dllimport) for the
    // public API.  We compile the sources directly (not importing a DLL),
    // so we must define LIBRAW_NODLL to suppress the dllimport attribute.
    if compiler.is_like_msvc() {
        libraw.define("LIBRAW_NODLL", None);
    }

    libraw.cpp(true);
    libraw.include("LibRaw/");

    libraw.file("LibRaw/src/decoders/canon_600.cpp");
    libraw.file("LibRaw/src/decoders/crx.cpp");
    libraw.file("LibRaw/src/decoders/decoders_dcraw.cpp");
    libraw.file("LibRaw/src/decoders/decoders_libraw.cpp");
    libraw.file("LibRaw/src/decoders/decoders_libraw_dcrdefs.cpp");
    libraw.file("LibRaw/src/decoders/dng.cpp");
    libraw.file("LibRaw/src/decoders/fp_dng.cpp");
    libraw.file("LibRaw/src/decoders/fuji_compressed.cpp");
    libraw.file("LibRaw/src/decoders/generic.cpp");
    libraw.file("LibRaw/src/decoders/kodak_decoders.cpp");
    libraw.file("LibRaw/src/decoders/load_mfbacks.cpp");
    libraw.file("LibRaw/src/decoders/smal.cpp");
    libraw.file("LibRaw/src/decoders/unpack.cpp");
    libraw.file("LibRaw/src/decoders/unpack_thumb.cpp");
    libraw.file("LibRaw/src/demosaic/aahd_demosaic.cpp");
    libraw.file("LibRaw/src/demosaic/ahd_demosaic.cpp");
    libraw.file("LibRaw/src/demosaic/dcb_demosaic.cpp");
    libraw.file("LibRaw/src/demosaic/dht_demosaic.cpp");
    libraw.file("LibRaw/src/demosaic/misc_demosaic.cpp");
    libraw.file("LibRaw/src/demosaic/xtrans_demosaic.cpp");
    libraw.file("LibRaw/src/integration/dngsdk_glue.cpp");
    libraw.file("LibRaw/src/integration/rawspeed_glue.cpp");
    libraw.file("LibRaw/src/metadata/adobepano.cpp");
    libraw.file("LibRaw/src/metadata/canon.cpp");
    libraw.file("LibRaw/src/metadata/ciff.cpp");
    libraw.file("LibRaw/src/metadata/cr3_parser.cpp");
    libraw.file("LibRaw/src/metadata/epson.cpp");
    libraw.file("LibRaw/src/metadata/exif_gps.cpp");
    libraw.file("LibRaw/src/metadata/fuji.cpp");
    libraw.file("LibRaw/src/metadata/hasselblad_model.cpp");
    libraw.file("LibRaw/src/metadata/identify.cpp");
    libraw.file("LibRaw/src/metadata/identify_tools.cpp");
    libraw.file("LibRaw/src/metadata/kodak.cpp");
    libraw.file("LibRaw/src/metadata/leica.cpp");
    libraw.file("LibRaw/src/metadata/makernotes.cpp");
    libraw.file("LibRaw/src/metadata/mediumformat.cpp");
    libraw.file("LibRaw/src/metadata/minolta.cpp");
    libraw.file("LibRaw/src/metadata/misc_parsers.cpp");
    libraw.file("LibRaw/src/metadata/nikon.cpp");
    libraw.file("LibRaw/src/metadata/normalize_model.cpp");
    libraw.file("LibRaw/src/metadata/olympus.cpp");
    libraw.file("LibRaw/src/metadata/p1.cpp");
    libraw.file("LibRaw/src/metadata/pentax.cpp");
    libraw.file("LibRaw/src/metadata/samsung.cpp");
    libraw.file("LibRaw/src/metadata/sony.cpp");
    libraw.file("LibRaw/src/metadata/tiff.cpp");
    libraw.file("LibRaw/src/postprocessing/aspect_ratio.cpp");
    libraw.file("LibRaw/src/postprocessing/dcraw_process.cpp");
    libraw.file("LibRaw/src/postprocessing/mem_image.cpp");
    libraw.file("LibRaw/src/postprocessing/postprocessing_aux.cpp");
    //libraw.file("LibRaw/src/postprocessing/postprocessing_ph.cpp");
    libraw.file("LibRaw/src/postprocessing/postprocessing_utils.cpp");
    libraw.file("LibRaw/src/postprocessing/postprocessing_utils_dcrdefs.cpp");
    libraw.file("LibRaw/src/preprocessing/ext_preprocess.cpp");
    //libraw.file("LibRaw/src/preprocessing/preprocessing_ph.cpp");
    libraw.file("LibRaw/src/preprocessing/raw2image.cpp");
    libraw.file("LibRaw/src/preprocessing/subtract_black.cpp");
    libraw.file("LibRaw/src/tables/cameralist.cpp");
    libraw.file("LibRaw/src/tables/colorconst.cpp");
    libraw.file("LibRaw/src/tables/colordata.cpp");
    libraw.file("LibRaw/src/tables/wblists.cpp");
    libraw.file("LibRaw/src/utils/curves.cpp");
    libraw.file("LibRaw/src/utils/decoder_info.cpp");
    libraw.file("LibRaw/src/utils/init_close_utils.cpp");
    libraw.file("LibRaw/src/utils/open.cpp");
    libraw.file("LibRaw/src/utils/phaseone_processing.cpp");
    libraw.file("LibRaw/src/utils/read_utils.cpp");
    libraw.file("LibRaw/src/utils/thumb_utils.cpp");
    libraw.file("LibRaw/src/utils/utils_dcraw.cpp");
    libraw.file("LibRaw/src/utils/utils_libraw.cpp");
    libraw.file("LibRaw/src/write/apply_profile.cpp");
    libraw.file("LibRaw/src/write/file_write.cpp");
    libraw.file("LibRaw/src/write/tiff_writer.cpp");
    //libraw.file("LibRaw/src/write/write_ph.cpp");
    libraw.file("LibRaw/src/x3f/x3f_parse_process.cpp");
    libraw.file("LibRaw/src/x3f/x3f_utils_patched.cpp");
    libraw.file("LibRaw/src/libraw_c_api.cpp");
    // libraw.file("LibRaw/src/libraw_cxx.cpp");
    libraw.file("LibRaw/src/libraw_datastream.cpp");

    libraw.warnings(false);
    libraw.extra_warnings(false);
    // do I really have to supress all of these?
    libraw.flag_if_supported("-Wno-format-truncation");
    libraw.flag_if_supported("-Wno-unused-result");
    libraw.flag_if_supported("-Wno-format-overflow");

    // thread safety (GCC/Clang only — MSVC uses /MT or /MD automatically)
    if !compiler.is_like_msvc() {
        libraw.flag("-pthread");
    }
    libraw.compile("raw");

    println!(
        "cargo:rustc-link-search=native={}",
        out_dir.as_ref().join("lib").display()
    );
    println!("cargo:rustc-link-lib=static=raw");
}

/// Generate FFI bindings from LibRaw headers using bindgen.
///
/// Requires libclang to be available at build time (dynamically loaded by
/// default, or statically linked with the `bindgen-static` feature).
///
/// Bindings are always regenerated for the target platform, ensuring correct
/// type sizes, alignment, and ABI for every supported architecture.
fn bindings(out_dir: impl AsRef<Path>) {
    let out_path = out_dir.as_ref().join("bindings.rs");
    if out_path.exists() {
        return;
    }
    let bindings = bindgen::Builder::default()
        .header("LibRaw/libraw/libraw.h")
        .use_core()
        .ctypes_prefix("libc")
        .generate_comments(true)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Only include LibRaw's public API — no platform types, no libc
        // functions, no long double math, no darwin/glibc internals.
        .allowlist_function("libraw_.*")
        .allowlist_type("libraw_.*")
        .allowlist_type("LibRaw_.*")
        .allowlist_var("libraw_.*|LIBRAW_.*")
        // ushort/uchar are defined in libraw_types.h and used pervasively
        // in LibRaw struct fields. They must be included explicitly since
        // they don't match the libraw_* prefix.
        .allowlist_type("ushort")
        .allowlist_type("uchar")
        // Use libc crate's platform-correct definitions for OS types that
        // leak in transitively (via internal_data_t and libraw_imgother_t).
        // This avoids darwin/glibc-specific type chains in pre-generated
        // bindings.
        .blocklist_type("FILE")
        .blocklist_type("time_t")
        // Block transitive platform types that bindgen emits even though
        // nothing in the LibRaw API directly uses them.
        .blocklist_type("__sFILE.*")
        .blocklist_type("__sbuf")
        .blocklist_type("__darwin_.*")
        .blocklist_type("__int64_t")
        .blocklist_type("fpos_t")
        .raw_line("use libc::{time_t, FILE};")
        .size_t_is_usize(true)
        .derive_eq(true)
        .no_partialeq("libraw_callbacks_t")
        .no_partialeq("LibRaw_abstract_datastream")
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_path)
        .expect("Couldn't write bindings!");
}
