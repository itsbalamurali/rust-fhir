use std::io::Result;
fn main() -> Result<()> {
    prost_build::compile_protos(
        &["fhir/proto/google/fhir/proto/annotations.proto"],
        &["fhir/proto/google/fhir/proto/"],
    )?;
    prost_build::compile_protos(
        &["fhir/proto/google/fhir/proto/profile_config.proto"],
        &["fhir/proto/google/fhir/proto/"],
    )?;
    Ok(())
}
