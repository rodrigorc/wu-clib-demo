use std::path::PathBuf;
use std::env;

fn main() {
    let mut build = cc::Build::new();
    let sources = [
        "jaricom.c", "jcapimin.c", "jcapistd.c", "jcarith.c", "jccoefct.c", "jccolor.c",
        "jcdctmgr.c", "jchuff.c", "jcinit.c", "jcmainct.c", "jcmarker.c", "jcmaster.c",
        "jcomapi.c", "jcparam.c", "jcprepct.c", "jcsample.c", "jctrans.c", "jdapimin.c",
        "jdapistd.c", "jdarith.c", "jdatadst.c", "jdatasrc.c", "jdcoefct.c", "jdcolor.c",
        "jddctmgr.c", "jdhuff.c", "jdinput.c", "jdmainct.c", "jdmarker.c", "jdmaster.c",
        "jdmerge.c", "jdpostct.c", "jdsample.c", "jdtrans.c", "jerror.c", "jfdctflt.c",
        "jfdctfst.c", "jfdctint.c", "jidctflt.c", "jidctfst.c", "jidctint.c", "jquant1.c",
        "jquant2.c", "jutils.c", "jmemmgr.c",
        // There are several jmem*.c implementations to choose from. This is the simpler:
        // "NOBackingStore", meaning no temporary files or shared memory, just malloc/free.
        "jmemnobs.c",
    ];
    for src in sources {
        let name = format!("jpeg-9f/{src}");
        println!("cargo:rerun-if-changed={name}");
        build.file(&name);
    }
    build.compile("jpeg");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindgen::Builder::default()
        .header("jpeg-9f/cdjpeg.h")
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
