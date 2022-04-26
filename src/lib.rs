#[macro_use]
extern crate serde_derive;
extern crate mopa;
extern crate serde;

mod pbtime;
pub use crate::pbtime::*;

mod pbstruct;
pub use crate::pbstruct::*;

mod pbany;
pub use crate::pbany::*;

use mopa::mopafy;
pub use typetag;

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
    // include!(concat!(env!("OUT_DIR"), "/google.fhir.stu3.fhirproto.rs"));
    // include!(concat!(env!("OUT_DIR"), "/google.fhir.stu3.uscore.rs"));
}



/// Trait to support serialization and deserialization of `prost` messages.
#[typetag::serde(tag = "@type")]
pub trait MessageSerde: prost::Message + mopa::Any {
    /// message name as in proto file
    fn message_name(&self) -> &'static str;
    /// package name as in proto file
    fn package_name(&self) -> &'static str;
    /// the message proto type url e.g. type.googleapis.com/my.package.MyMessage
    fn type_url(&self) -> &'static str;
    /// Creates a new instance of this message using the protobuf encoded data
    fn new_instance(&self, data: Vec<u8>) -> Result<Box<dyn MessageSerde>, prost::DecodeError>;
    /// Returns the encoded protobuf message as bytes
    #[deprecated(since = "0.3.0", note = "please use `try_encoded` instead")]
    fn encoded(&self) -> Vec<u8>;
    /// Returns the encoded protobuf message as bytes
    fn try_encoded(&self) -> Result<Vec<u8>, prost::EncodeError>;
}

mopafy!(MessageSerde);

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
