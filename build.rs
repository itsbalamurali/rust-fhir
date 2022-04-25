use prost_wkt_build::*;
use std::{env, fs, io::Result, path::PathBuf, process};
fn main() -> Result<()> {
    let outdir = match env::var_os("OUT_DIR") {
        Some(outdir) => outdir,
        None => {
            eprintln!("OUT_DIR environment variable not defined.");
            process::exit(1)
        }
    };
    let descriptor_file = outdir.as_os_str().to_str().unwrap().to_string() + "descriptors.bin";
    fs::create_dir_all(&outdir).unwrap();

    let mut config = prost_build::Config::new();
    config
        .type_attribute(
            ".",
            "#[derive(Serialize,Deserialize)] #[serde(rename_all = \"camelCase\")]",
        )
        .extern_path(".google.protobuf.Any", "::prost_wkt_types::Any")
        .extern_path(".google.protobuf.Timestamp", "::prost_wkt_types::Timestamp")
        .extern_path(".google.protobuf.Value", "::prost_wkt_types::Value")
        .file_descriptor_set_path(&descriptor_file);

    config.compile_protos(
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
    for file_entry in generated_files {
        match file_entry {
            Ok(fp) => {
                let file_path = fp.path();
                let file_name = fp.file_name();
                match file_path.extension() {
                    Some(ex) => {
                        if ex == "rs" {
                            let mut file_content =
                                fs::read_to_string(file_path.to_owned()).unwrap();
                            file_content = file_content.replace("::prost::", "prost::");
                            if file_name.to_str().unwrap().contains("r4") {
                                file_content = file_content
                                    .replace("super::super::super::core::", "super::super::core::")
                                    .replace("super::super::core::", "crate::r4::core::")
                                    .replace("super::crate::r4", "crate::r4")
                                    .replace("super::core", "crate::r4::core");
                            }
                            // Write file
                            fs::write(file_path.to_owned(), file_content)?;
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

    let descriptor_bytes = std::fs::read(descriptor_file).unwrap();
    let descriptor = FileDescriptorSet::decode(&descriptor_bytes[..]).unwrap();

    prost_wkt_build::add_serde(PathBuf::from(outdir), descriptor);
    Ok(())
}
