# Rust SDK for HL7 FHIR

[![crates.io](https://buildstats.info/crate/fhir)](https://crates.io/crates/fhir) [![build](https://github.com/itsbalamurali/rust-fhir/actions/workflows/generate.yml/badge.svg)](https://github.com/itsbalamurali/rust-fhir/actions/workflows/generate.yml)

This library is auto generated from [HL7 FHIR Definitions](https://build.fhir.org/definitions.json.zip)

## Getting Started

```rust
use fhir::models::r4::Patient::PatientGender;

fn main() {
    let mut patient_builder = crate::models::r4::Patient::PatientBuilder::new();
    patient_builder.active(true);
    patient_builder.gender(PatientGender::Male);
    let patient = patient_builder.build();
    println!("{:?}", patient.to_json().as_str());
}
```

## License

[Apache License 2.0](./LICENSE)

## Credits

[Oliver Clark Rickard](https://github.com/ocrickard)

[Etienne Prothon](https://github.com/etienneprothon)

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be licensed as above, without any additional terms or conditions.
