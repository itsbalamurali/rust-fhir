pub mod models;
pub mod client;
use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests {
    #[test]
    fn test_patient_resource() {
        let patient = crate::models::r4::Patient {
            ..Default::default()
        };
        let patient_json = serde_json::to_string(&patient).unwrap();
        println!("{:?}", patient_json);
    }
}
