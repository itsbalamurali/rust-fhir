# Rust SDK for HL7 FHIR

[![crates.io](https://buildstats.info/crate/fhir)](https://crates.io/crates/fhir) [![build](https://github.com/itsbalamurali/rust-fhir/actions/workflows/generate.yml/badge.svg)](https://github.com/itsbalamurali/rust-fhir/actions/workflows/generate.yml)

This libraries are auto generated from googles fhir protobufs: https://github.com/google/fhir

[HL7 FHIR Definitions](https://build.fhir.org/definitions.json.zip)

## Getting Started

```rust
use fhir;

fn main() {
    let patient = fhir::r4::core::Patient {
            ..Default::default()
        };
    let patient_json = serde_json::to_string(&patient);
    println!("{}", patient_json);
}
```

## License

[Apache License 2.0](./LICENSE)

## Credits

[Ferdinand de Antoni](https://github.com/fdeantoni/prost-wkt)

[Google](https://github.com/google/fhir)

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be licensed as above, without any additional terms or conditions.
