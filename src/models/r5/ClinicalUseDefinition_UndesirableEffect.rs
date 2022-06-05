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
pub struct ClinicalUseDefinition_UndesirableEffect<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl ClinicalUseDefinition_UndesirableEffect<'_> {
    pub fn new(value: &Value) -> ClinicalUseDefinition_UndesirableEffect {
        ClinicalUseDefinition_UndesirableEffect {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// High level classification of the effect.
    pub fn classification(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("classification") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
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

    /// How often the effect is seen.
    pub fn frequency_of_occurrence(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("frequencyOfOccurrence") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
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

    /// The situation in which the undesirable effect may manifest.
    pub fn symptom_condition_effect(&self) -> Option<CodeableReference> {
        if let Some(val) = self.value.get("symptomConditionEffect") {
            return Some(CodeableReference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.classification() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.frequency_of_occurrence() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.symptom_condition_effect() {
            if !_val.validate() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct ClinicalUseDefinition_UndesirableEffectBuilder {
    pub(crate) value: Value,
}

impl ClinicalUseDefinition_UndesirableEffectBuilder {
    pub fn build(&self) -> ClinicalUseDefinition_UndesirableEffect {
        ClinicalUseDefinition_UndesirableEffect {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(
        existing: ClinicalUseDefinition_UndesirableEffect,
    ) -> ClinicalUseDefinition_UndesirableEffectBuilder {
        ClinicalUseDefinition_UndesirableEffectBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> ClinicalUseDefinition_UndesirableEffectBuilder {
        let mut __value: Value = json!({});
        return ClinicalUseDefinition_UndesirableEffectBuilder { value: __value };
    }

    pub fn classification<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut ClinicalUseDefinition_UndesirableEffectBuilder {
        self.value["classification"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ClinicalUseDefinition_UndesirableEffectBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn frequency_of_occurrence<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut ClinicalUseDefinition_UndesirableEffectBuilder {
        self.value["frequencyOfOccurrence"] = json!(val.value);
        return self;
    }

    pub fn id<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut ClinicalUseDefinition_UndesirableEffectBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ClinicalUseDefinition_UndesirableEffectBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn symptom_condition_effect<'a>(
        &'a mut self,
        val: CodeableReference,
    ) -> &'a mut ClinicalUseDefinition_UndesirableEffectBuilder {
        self.value["symptomConditionEffect"] = json!(val.value);
        return self;
    }
}
