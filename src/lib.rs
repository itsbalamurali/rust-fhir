pub mod models;

#[cfg(test)]
mod tests {
    #[test]
    fn test_patient_resource() {
        let patient = crate::models::r4::Patient::Patient {
            ..Default::default()
        };
        let patient_json = serde_json::to_string(&patient).unwrap();
        println!("{:?}", patient_json);
    }
}
