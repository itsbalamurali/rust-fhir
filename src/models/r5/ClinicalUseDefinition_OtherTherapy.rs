#![allow(unused_imports, non_camel_case_types)]

use crate::models::r5::CodeableConcept::CodeableConcept;
use crate::models::r5::CodeableReference::CodeableReference;
use crate::models::r5::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A single issue - either an indication, contraindication, interaction or an
/// undesirable effect for a medicinal product, medication, device or procedure.

#[derive(Debug)]
pub struct ClinicalUseDefinition_OtherTherapy<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl ClinicalUseDefinition_OtherTherapy<'_> {
    pub fn new(value: &Value) -> ClinicalUseDefinition_OtherTherapy {
        ClinicalUseDefinition_OtherTherapy {
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

    /// The type of relationship between the medicinal product indication or
    /// contraindication and another therapy.
    pub fn relationship_type(&self) -> CodeableConcept {
        CodeableConcept {
            value: Cow::Borrowed(&self.value["relationshipType"]),
        }
    }

    /// Reference to a specific medication (active substance, medicinal product or class
    /// of products) as part of an indication or contraindication.
    pub fn therapy(&self) -> CodeableReference {
        CodeableReference {
            value: Cow::Borrowed(&self.value["therapy"]),
        }
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
        if !self.relationship_type().validate() {
            return false;
        }
        if !self.therapy().validate() {
            return false;
        }
        return true;
    }
}

#[derive(Debug)]
pub struct ClinicalUseDefinition_OtherTherapyBuilder {
    pub(crate) value: Value,
}

impl ClinicalUseDefinition_OtherTherapyBuilder {
    pub fn build(&self) -> ClinicalUseDefinition_OtherTherapy {
        ClinicalUseDefinition_OtherTherapy {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(
        existing: ClinicalUseDefinition_OtherTherapy,
    ) -> ClinicalUseDefinition_OtherTherapyBuilder {
        ClinicalUseDefinition_OtherTherapyBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(
        relationship_type: CodeableConcept,
        therapy: CodeableReference,
    ) -> ClinicalUseDefinition_OtherTherapyBuilder {
        let mut __value: Value = json!({});
        __value["relationshipType"] = json!(relationship_type.value);
        __value["therapy"] = json!(therapy.value);
        return ClinicalUseDefinition_OtherTherapyBuilder { value: __value };
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ClinicalUseDefinition_OtherTherapyBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut ClinicalUseDefinition_OtherTherapyBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ClinicalUseDefinition_OtherTherapyBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }
}
