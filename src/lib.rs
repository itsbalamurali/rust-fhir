use serde::{Deserialize, Serialize};
use chrono::prelude::*;
use prost_wkt_types::*;

pub mod r4 {
    pub mod core {
        include!(concat!(env!("OUT_DIR"), "/google.fhir.r4.core.rs"));
    }

    include!(concat!(env!("OUT_DIR"), "/google.fhir.r4.fhirproto.rs"));

    pub mod ml {
        include!(concat!(env!("OUT_DIR"), "/google.fhir.r4.ml.rs"));
    }
    pub mod uscore {
        include!(concat!(env!("OUT_DIR"), "/google.fhir.r4.uscore.rs"));
    }
    pub mod qicore {
        include!(concat!(env!("OUT_DIR"), "/google.fhir.r4.qicore.rs"));
    }
}

pub mod stu3 {
    pub mod core {
        include!(concat!(env!("OUT_DIR"), "/google.fhir.stu3.proto.rs"));
    }

    include!(concat!(env!("OUT_DIR"), "/google.fhir.stu3.fhirproto.rs"));

    pub mod uscore {
        include!(concat!(env!("OUT_DIR"), "/google.fhir.stu3.uscore.rs"));
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test_patient_resource() {
        let patient = crate::r4::core::Patient {
            ..Default::default()
        };
        let patient_json = serde_json::to_string(&patient);
        println!("{}", patient_json);
    }
}
