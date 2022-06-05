#![allow(unused_imports, non_camel_case_types)]

use crate::models::r5::CodeableConcept::CodeableConcept;
use crate::models::r5::Extension::Extension;
use crate::models::r5::MedicationKnowledge_Dosage::MedicationKnowledge_Dosage;
use crate::models::r5::MedicationKnowledge_PatientCharacteristic::MedicationKnowledge_PatientCharacteristic;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Information about a medication that is used to support knowledge.

#[derive(Debug)]
pub struct MedicationKnowledge_DosingGuideline<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl MedicationKnowledge_DosingGuideline<'_> {
    pub fn new(value: &Value) -> MedicationKnowledge_DosingGuideline {
        MedicationKnowledge_DosingGuideline {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// The type of the treatment that the guideline applies to, for example, long term
    /// therapy, first line treatment, etc.
    pub fn administration_treatment(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("administrationTreatment") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Dosage for the medication for the specific guidelines.
    pub fn dosage(&self) -> Option<Vec<MedicationKnowledge_Dosage>> {
        if let Some(Value::Array(val)) = self.value.get("dosage") {
            return Some(
                val.into_iter()
                    .map(|e| MedicationKnowledge_Dosage {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// May be used to represent additional information that is not part of the basic
    /// definition of the element. To make the use of extensions safe and manageable,
    /// there is a strict set of governance  applied to the definition and use of
    /// extensions. Though any implementer can define an extension, there is a set of
    /// requirements that SHALL be met as part of the definition of the extension.
    pub fn extension(&self) -> Option<Vec<Extension>> {
        if let Some(Value::Array(val)) = self.value.get("extension") {
            return Some(
                val.into_iter()
                    .map(|e| Extension {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
        }
        return None;
    }

    /// May be used to represent additional information that is not part of the basic
    /// definition of the element and that modifies the understanding of the element in
    /// which it is contained and/or the understanding of the containing element's
    /// descendants. Usually modifier elements provide negation or qualification. To
    /// make the use of extensions safe and manageable, there is a strict set of
    /// governance applied to the definition and use of extensions. Though any
    /// implementer can define an extension, there is a set of requirements that SHALL
    /// be met as part of the definition of the extension. Applications processing a
    /// resource are required to check for modifier extensions.    Modifier extensions
    /// SHALL NOT change the meaning of any elements on Resource or DomainResource
    /// (including cannot change the meaning of modifierExtension itself).
    pub fn modifier_extension(&self) -> Option<Vec<Extension>> {
        if let Some(Value::Array(val)) = self.value.get("modifierExtension") {
            return Some(
                val.into_iter()
                    .map(|e| Extension {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Characteristics of the patient that are relevant to the administration
    /// guidelines (for example, height, weight, gender, etc.).
    pub fn patient_characteristic(&self) -> Option<Vec<MedicationKnowledge_PatientCharacteristic>> {
        if let Some(Value::Array(val)) = self.value.get("patientCharacteristic") {
            return Some(
                val.into_iter()
                    .map(|e| MedicationKnowledge_PatientCharacteristic {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The overall intention of the treatment, for example, prophylactic, supporative,
    /// curative, etc.
    pub fn treatment_intent(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("treatmentIntent") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.administration_treatment() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.dosage() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.patient_characteristic() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.treatment_intent() {
            if !_val.validate() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct MedicationKnowledge_DosingGuidelineBuilder {
    pub(crate) value: Value,
}

impl MedicationKnowledge_DosingGuidelineBuilder {
    pub fn build(&self) -> MedicationKnowledge_DosingGuideline {
        MedicationKnowledge_DosingGuideline {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(
        existing: MedicationKnowledge_DosingGuideline,
    ) -> MedicationKnowledge_DosingGuidelineBuilder {
        MedicationKnowledge_DosingGuidelineBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> MedicationKnowledge_DosingGuidelineBuilder {
        let mut __value: Value = json!({});
        return MedicationKnowledge_DosingGuidelineBuilder { value: __value };
    }

    pub fn administration_treatment<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut MedicationKnowledge_DosingGuidelineBuilder {
        self.value["administrationTreatment"] = json!(val.value);
        return self;
    }

    pub fn dosage<'a>(
        &'a mut self,
        val: Vec<MedicationKnowledge_Dosage>,
    ) -> &'a mut MedicationKnowledge_DosingGuidelineBuilder {
        self.value["dosage"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut MedicationKnowledge_DosingGuidelineBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut MedicationKnowledge_DosingGuidelineBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut MedicationKnowledge_DosingGuidelineBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn patient_characteristic<'a>(
        &'a mut self,
        val: Vec<MedicationKnowledge_PatientCharacteristic>,
    ) -> &'a mut MedicationKnowledge_DosingGuidelineBuilder {
        self.value["patientCharacteristic"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn treatment_intent<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut MedicationKnowledge_DosingGuidelineBuilder {
        self.value["treatmentIntent"] = json!(val.value);
        return self;
    }
}
