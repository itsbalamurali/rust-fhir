pub mod models;
pub mod client;

#[cfg(test)]
mod tests {
    use crate::models::r4::Patient::PatientGender;
    #[test]
    fn test_patient_resource() {
        let mut patient_builder = crate::models::r4::Patient::PatientBuilder::new();
        patient_builder.active(true);
        patient_builder.gender(PatientGender::Male);
        let patient = patient_builder.build();
        println!("{:?}", patient.to_json().as_str());
    }
}
