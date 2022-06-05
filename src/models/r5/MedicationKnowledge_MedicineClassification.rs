#![allow(unused_imports, non_camel_case_types)]

use crate::models::r5::CodeableConcept::CodeableConcept;
use crate::models::r5::Element::Element;
use crate::models::r5::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Information about a medication that is used to support knowledge.

#[derive(Debug)]
pub struct MedicationKnowledge_MedicineClassification<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl MedicationKnowledge_MedicineClassification<'_> {
    pub fn new(value: &Value) -> MedicationKnowledge_MedicineClassification {
        MedicationKnowledge_MedicineClassification {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for sourceString
    pub fn _source_string(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_sourceString") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for sourceUri
    pub fn _source_uri(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_sourceUri") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Specific category assigned to the medication (e.g. anti-infective, anti-
    /// hypertensive, antibiotic, etc.).
    pub fn classification(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("classification") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept {
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

    /// Either a textual source of the classification or a reference to an online source.
    pub fn source_string(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("sourceString") {
            return Some(string);
        }
        return None;
    }

    /// Either a textual source of the classification or a reference to an online source.
    pub fn source_uri(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("sourceUri") {
            return Some(string);
        }
        return None;
    }

    /// The type of category for the medication (for example, therapeutic classification,
    /// therapeutic sub-classification).
    pub fn fhir_type(&self) -> CodeableConcept {
        CodeableConcept {
            value: Cow::Borrowed(&self.value["type"]),
        }
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._source_string() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._source_uri() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.classification() {
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
        if let Some(_val) = self.source_string() {}
        if let Some(_val) = self.source_uri() {}
        if !self.fhir_type().validate() {
            return false;
        }
        return true;
    }
}

#[derive(Debug)]
pub struct MedicationKnowledge_MedicineClassificationBuilder {
    pub(crate) value: Value,
}

impl MedicationKnowledge_MedicineClassificationBuilder {
    pub fn build(&self) -> MedicationKnowledge_MedicineClassification {
        MedicationKnowledge_MedicineClassification {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(
        existing: MedicationKnowledge_MedicineClassification,
    ) -> MedicationKnowledge_MedicineClassificationBuilder {
        MedicationKnowledge_MedicineClassificationBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(fhir_type: CodeableConcept) -> MedicationKnowledge_MedicineClassificationBuilder {
        let mut __value: Value = json!({});
        __value["type"] = json!(fhir_type.value);
        return MedicationKnowledge_MedicineClassificationBuilder { value: __value };
    }

    pub fn _source_string<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut MedicationKnowledge_MedicineClassificationBuilder {
        self.value["_sourceString"] = json!(val.value);
        return self;
    }

    pub fn _source_uri<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut MedicationKnowledge_MedicineClassificationBuilder {
        self.value["_sourceUri"] = json!(val.value);
        return self;
    }

    pub fn classification<'a>(
        &'a mut self,
        val: Vec<CodeableConcept>,
    ) -> &'a mut MedicationKnowledge_MedicineClassificationBuilder {
        self.value["classification"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut MedicationKnowledge_MedicineClassificationBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut MedicationKnowledge_MedicineClassificationBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut MedicationKnowledge_MedicineClassificationBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn source_string<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut MedicationKnowledge_MedicineClassificationBuilder {
        self.value["sourceString"] = json!(val);
        return self;
    }

    pub fn source_uri<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut MedicationKnowledge_MedicineClassificationBuilder {
        self.value["sourceUri"] = json!(val);
        return self;
    }
}
