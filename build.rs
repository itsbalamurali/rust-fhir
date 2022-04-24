use std::{
    env,
    ffi::OsStr,
    fs,
    io::Result,
    path::{Path, PathBuf},
    process,
};
fn main() -> Result<()> {
    let outdir = match env::var_os("OUT_DIR") {
        Some(outdir) => outdir,
        None => {
            eprintln!("OUT_DIR environment variable not defined.");
            process::exit(1)
        }
    };
    let source_dir = match env::var_os("CARGO_MANIFEST_DIR") {
        Some(srcdir) => {
            let mut s = srcdir.to_str().unwrap().to_string();
            s.push_str("/src");
            s
        }
        None => {
            eprintln!("CARGO_MANIFEST_DIR environment variable not defined.");
            process::exit(1)
        }
    };
    fs::create_dir_all(&outdir).unwrap();

    prost_build::compile_protos(
        &[
            "src/proto/google/fhir/proto/annotations.proto",
            "src/proto/google/fhir/proto/profile_config.proto",
            "src/proto/google/fhir/proto/protogenerator_annotations.proto",
            "src/proto/google/fhir/proto/r4/fhirproto.proto",
            "src/proto/google/fhir/proto/r4/core/resources/bundle_and_contained_resource.proto",
            "src/proto/google/fhir/proto/r4/fhirproto_extensions.proto",
            "src/proto/google/fhir/proto/r4/ml_extensions.proto",
            "src/proto/google/fhir/proto/r4/uscore.proto",
            "src/proto/google/fhir/proto/r4/uscore_codes.proto",
            "src/proto/google/fhir/proto/r4/qicore/qicore.proto",
            "src/proto/google/fhir/proto/stu3/fhirproto_extensions.proto",
            "src/proto/google/fhir/proto/stu3/uscore.proto",
            "src/proto/google/fhir/proto/stu3/uscore_codes.proto",
        ],
        &["src/"],
    )?;

    let generated_files = fs::read_dir(&outdir).unwrap();
    let target_path = Path::new(OsStr::new(&source_dir));
    for file_entry in generated_files {
        match file_entry {
            Ok(fp) => {
                let file_path = fp.path();
                let file_name = fp.file_name();
                match file_path.extension() {
                    Some(ex) => {
                        if ex == "rs" {
                            println!("Name: {:?}", file_path);
                            let mut target_file_name_vec: Vec<&str> =
                                file_name.to_str().unwrap().split(".").collect();
                            target_file_name_vec.remove(0);
                            target_file_name_vec.remove(0);
                            target_file_name_vec.pop(); // remove .rs
                            target_file_name_vec.pop(); // remove last folder

                            let mut target_file_name = file_name
                                .to_str()
                                .unwrap()
                                .replace("google.fhir.", "")
                                .replace(".proto", "")
                                .replace(".rs", "")
                                .replace(".", "/");

                            let target_file_path =
                                target_path.join(PathBuf::from(target_file_name_vec.join("/")));
                            fs::create_dir_all(&target_file_path).unwrap();

                            target_file_name.push_str(".rs");

                            if target_file_name == "proto.rs" {
                                target_file_name = "mod.rs".to_string()
                            }

                            if target_file_name == "stu3.rs" {
                                target_file_name = "stu3/core.rs".to_string()
                            }
                            // Copy over file
                            fs::copy(
                                file_path.to_owned(),
                                target_path.join(PathBuf::from(target_file_name)),
                            )?;
                            // Remove file
                            fs::remove_file(file_path)?;
                        }
                    }
                    None => {}
                }
            }
            Err(e) => {
                eprintln!("{}", e);
                process::exit(1)
            }
        };
    }
    Ok(())
}
