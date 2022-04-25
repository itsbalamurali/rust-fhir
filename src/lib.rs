#[macro_use]
extern crate serde_derive;
extern crate prost_wkt_types;
extern crate serde;

pub mod r4 {
    pub mod core {
        include!(concat!(env!("OUT_DIR"), "/google.fhir.r4.core.rs"));
    }
    include!(concat!(env!("OUT_DIR"), "/google.fhir.r4.fhirproto.rs"));
    include!(concat!(env!("OUT_DIR"), "/google.fhir.r4.ml.rs"));
    //include!(concat!(env!("OUT_DIR"), "/google.fhir.r4.qicore.rs"));
    //include!(concat!(env!("OUT_DIR"), "/google.fhir.r4.uscore.rs"));
}

pub mod stu3 {
    pub mod core {
        include!(concat!(env!("OUT_DIR"), "/google.fhir.stu3.proto.rs"));
    }
    // include!(concat!(env!("OUT_DIR"), "/google.fhir.stu3.fhirproto.rs"));
    // include!(concat!(env!("OUT_DIR"), "/google.fhir.stu3.uscore.rs"));
}
