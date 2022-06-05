#![allow(unused_imports, non_camel_case_types)]

use crate::models::r5::CodeableConcept::CodeableConcept;
use crate::models::r5::Extension::Extension;
use crate::models::r5::Quantity::Quantity;
use crate::models::r5::Range::Range;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Information about a medication that is used to support knowledge.

#[derive(Debug)]
pub struct MedicationKnowledge_PatientCharacteristic<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl MedicationKnowledge_PatientCharacteristic<'_> {
    pub fn new(value: &Value) -> MedicationKnowledge_PatientCharacteristic {
        MedicationKnowledge_PatientCharacteristic {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
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

    /// Unique id for the element within a resource (for internal references). This may be
    /// any string value that does not contain spaces.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
        }
        return None;
    }

    /// May be used to represent additional information that is not part of the basic
    /// definition of the element and that modifies the understanding of the element
    /// in which it is contained and/or the understanding of the containing element's
    /// descendants. Usually modifier elements provide negation or qualification. To make
    /// the use of extensions safe and manageable, there is a strict set of governance
    /// applied to the definition and use of extensions. Though any implementer can define
    /// an extension, there is a set of requirements that SHALL be met as part of the
    /// definition of the extension. Applications processing a resource are required to
    /// check for modifier extensions.    Modifier extensions SHALL NOT change the meaning
    /// of any elements on Resource or DomainResource (including cannot change the meaning
    /// of modifierExtension itself).
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

    /// The categorization of the specific characteristic that is relevant to the
    /// administration guideline (e.g. height, weight, gender).
    pub fn fhir_type(&self) -> CodeableConcept {
        CodeableConcept {
            value: Cow::Borrowed(&self.value["type"]),
        }
    }

    /// The specific characteristic (e.g. height, weight, gender, etc.).
    pub fn value_codeable_concept(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("valueCodeableConcept") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The specific characteristic (e.g. height, weight, gender, etc.).
    pub fn value_quantity(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("valueQuantity") {
            return Some(Quantity {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The specific characteristic (e.g. height, weight, gender, etc.).
    pub fn value_range(&self) -> Option<Range> {
        if let Some(val) = self.value.get("valueRange") {
            return Some(Range {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
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
        if !self.fhir_type().validate() {
            return false;
        }
        if let Some(_val) = self.value_codeable_concept() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.value_quantity() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.value_range() {
            if !_val.validate() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct MedicationKnowledge_PatientCharacteristicBuilder {
    pub(crate) value: Value,
}

impl MedicationKnowledge_PatientCharacteristicBuilder {
    pub fn build(&self) -> MedicationKnowledge_PatientCharacteristic {
        MedicationKnowledge_PatientCharacteristic {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(
        existing: MedicationKnowledge_PatientCharacteristic,
    ) -> MedicationKnowledge_PatientCharacteristicBuilder {
        MedicationKnowledge_PatientCharacteristicBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(fhir_type: CodeableConcept) -> MedicationKnowledge_PatientCharacteristicBuilder {
        let mut __value: Value = json!({});
        __value["type"] = json!(fhir_type.value);
        return MedicationKnowledge_PatientCharacteristicBuilder { value: __value };
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut MedicationKnowledge_PatientCharacteristicBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut MedicationKnowledge_PatientCharacteristicBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut MedicationKnowledge_PatientCharacteristicBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn value_codeable_concept<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut MedicationKnowledge_PatientCharacteristicBuilder {
        self.value["valueCodeableConcept"] = json!(val.value);
        return self;
    }

    pub fn value_quantity<'a>(
        &'a mut self,
        val: Quantity,
    ) -> &'a mut MedicationKnowledge_PatientCharacteristicBuilder {
        self.value["valueQuantity"] = json!(val.value);
        return self;
    }

    pub fn value_range<'a>(
        &'a mut self,
        val: Range,
    ) -> &'a mut MedicationKnowledge_PatientCharacteristicBuilder {
        self.value["valueRange"] = json!(val.value);
        return self;
    }
}
