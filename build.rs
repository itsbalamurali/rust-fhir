use convert_case::{Case, Casing};
use prost::Message;
use prost_build::Module;
use prost_types::FileDescriptorSet;
use quote::{format_ident, quote};
use std::{
    env,
    ffi::OsString,
    fs::{self, File, OpenOptions},
    io::{Result, Write},
    path::PathBuf,
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
    let descriptor_file = outdir.as_os_str().to_str().unwrap().to_string() + "descriptors.bin";
    fs::create_dir_all(&outdir).unwrap();

    build(&outdir, "pbtime");
    build(&outdir, "pbstruct");
    build(&outdir, "pbany");

    let mut config = prost_build::Config::new();
    config
        .compile_well_known_types()
        .type_attribute(
            ".",
            "#[derive(Serialize,Deserialize)] #[serde(rename_all = \"camelCase\")]",
        )
        .extern_path(".google.protobuf.Any", "crate::pbany::Any")
        .extern_path(".google.protobuf.Timestamp", "crate::pbtime::Timestamp")
        .extern_path(".google.protobuf.Value", "crate::pbstruct::Value")
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
                let file_name = fp.file_name().to_str().unwrap().to_owned();
                match file_path.extension() {
                    Some(ex) => {
                        if ex == "rs" {
                            let mut file_content =
                                fs::read_to_string(file_path.to_owned()).unwrap();
                            file_content = file_content.replace("::prost::", "prost::");
                            if file_name.to_owned().contains("r4") {
                                file_content = file_content
                                    .replace("super::super::super::core::", "super::super::core::")
                                    .replace("super::super::core::", "crate::r4::core::")
                                    .replace("super::crate::r4", "crate::r4")
                                    .replace("super::core", "crate::r4::core")
                                    .replace("super::uscore", "crate::r4::uscore")
                                    .replace("super::crate", "crate");
                            }
                            if file_name.contains("stu3") {
                                file_content = file_content
                                    .replace(
                                        "super::super::super::super::proto::",
                                        "crate::stu3::core::",
                                    )
                                    .replace("super::super::super::proto::", "crate::stu3::core::")
                                    .replace("super::super::proto::", "crate::stu3::core::")
                                    .replace("super::proto::", "crate::stu3::core::")
                                    .replace("super::crate", "crate");
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

    add_serde(PathBuf::from(outdir), descriptor);
    Ok(())
}

fn build(outdir: &OsString, proto: &str) {
    let dir = PathBuf::from(outdir);
    let out = dir.join(proto);
    fs::create_dir_all(&out).unwrap();
    let source = format!("src/proto/{}.proto", proto);
    let descriptor_file = out.join("descriptors.bin");
    let mut prost_build = prost_build::Config::new();
    prost_build
        .compile_well_known_types()
        .type_attribute("google.protobuf.Struct","#[derive(serde_derive::Serialize, serde_derive::Deserialize)] #[serde(default, rename_all=\"camelCase\")]")
        .type_attribute("google.protobuf.ListValue","#[derive(serde_derive::Serialize, serde_derive::Deserialize)] #[serde(default, rename_all=\"camelCase\")]")
        .type_attribute("google.protobuf.Duration","#[derive(serde_derive::Serialize, serde_derive::Deserialize)] #[serde(default, rename_all=\"camelCase\")]")
        .file_descriptor_set_path(&descriptor_file)
        .out_dir(&out)
        .compile_protos(
            &[
                source
            ],
            &["src/proto/".to_string()],
        )
        .unwrap();

    let descriptor_bytes = std::fs::read(descriptor_file).unwrap();
    let descriptor = FileDescriptorSet::decode(&descriptor_bytes[..]).unwrap();
    add_serde(out, descriptor);
}

fn add_serde(out: PathBuf, descriptor: FileDescriptorSet) {
    for fd in &descriptor.file {
        let package_name = match fd.package {
            Some(ref pkg) => pkg,
            None => continue,
        };

        let rust_path = out
            .join(Module::from_protobuf_package_name(package_name).to_file_name_or(package_name));

        // In some cases the generated file would be in empty. These files are no longer created by Prost, so
        // we'll create here. Otherwise we append.
        let mut rust_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(rust_path)
            .unwrap();

        for msg in &fd.message_type {
            let message_name = match msg.name {
                Some(ref name) => name,
                None => continue,
            };

            let type_url = format!("type.googleapis.com/{}.{}", package_name, message_name);

            gen_trait_impl(&mut rust_file, package_name, message_name, &type_url);
        }
    }
}

fn gen_trait_impl(rust_file: &mut File, package_name: &str, message_name: &str, type_url: &str) {
    let type_name = message_name.to_case(Case::Pascal);
    let type_name = format_ident!("{}", type_name);

    let dummy_const = format_ident!(
        "IMPL_MESSAGE_SERDE_FOR_{}",
        message_name.to_case(Case::UpperSnake)
    );

    let tokens = quote! {
        #[allow(dead_code)]
        const #dummy_const: () = {
            use crate::typetag;
            #[typetag::serde(name=#type_url)]
            impl crate::MessageSerde for #type_name {
                fn package_name(&self) -> &'static str {
                    #package_name
                }
                fn message_name(&self) -> &'static str {
                    #message_name
                }
                fn type_url(&self) -> &'static str {
                    #type_url
                }
                fn new_instance(&self, data: Vec<u8>) -> Result<Box<dyn crate::MessageSerde>, ::prost::DecodeError> {
                    let mut target = Self::default();
                    prost::Message::merge(&mut target, data.as_slice())?;
                    let erased: Box<dyn crate::MessageSerde> = Box::new(target);
                    Ok(erased)
                }
                fn encoded(&self) -> Vec<u8> {
                    let mut buf = Vec::new();
                    buf.reserve(prost::Message::encoded_len(self));
                    prost::Message::encode(self, &mut buf).expect("Failed to encode message");
                    buf
                }
                fn try_encoded(&self) -> Result<Vec<u8>, ::prost::EncodeError> {
                    let mut buf = Vec::new();
                    buf.reserve(::prost::Message::encoded_len(self));
                    prost::Message::encode(self, &mut buf)?;
                    Ok(buf)
                }
            }
        };
    };

    writeln!(rust_file).unwrap();
    writeln!(rust_file, "{}", &tokens).unwrap();
}
