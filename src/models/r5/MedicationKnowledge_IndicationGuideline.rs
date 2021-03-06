#![allow(unused_imports, non_camel_case_types)]

use crate::models::r5::CodeableReference::CodeableReference;
use crate::models::r5::Extension::Extension;
use crate::models::r5::MedicationKnowledge_DosingGuideline::MedicationKnowledge_DosingGuideline;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Information about a medication that is used to support knowledge.

#[derive(Debug)]
pub struct MedicationKnowledge_IndicationGuideline<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl MedicationKnowledge_IndicationGuideline<'_> {
    pub fn new(value: &Value) -> MedicationKnowledge_IndicationGuideline {
        MedicationKnowledge_IndicationGuideline {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// The guidelines for the dosage of the medication for the indication.
    pub fn dosing_guideline(&self) -> Option<Vec<MedicationKnowledge_DosingGuideline>> {
        if let Some(Value::Array(val)) = self.value.get("dosingGuideline") {
            return Some(
                val.into_iter()
                    .map(|e| MedicationKnowledge_DosingGuideline {
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

    /// Indication or reason for use of the medication that applies to the specific
    /// administration guideline.
    pub fn indication(&self) -> Option<Vec<CodeableReference>> {
        if let Some(Value::Array(val)) = self.value.get("indication") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableReference {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
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

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.dosing_guideline() {
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
        if let Some(_val) = self.indication() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct MedicationKnowledge_IndicationGuidelineBuilder {
    pub(crate) value: Value,
}

impl MedicationKnowledge_IndicationGuidelineBuilder {
    pub fn build(&self) -> MedicationKnowledge_IndicationGuideline {
        MedicationKnowledge_IndicationGuideline {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(
        existing: MedicationKnowledge_IndicationGuideline,
    ) -> MedicationKnowledge_IndicationGuidelineBuilder {
        MedicationKnowledge_IndicationGuidelineBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> MedicationKnowledge_IndicationGuidelineBuilder {
        let mut __value: Value = json!({});
        return MedicationKnowledge_IndicationGuidelineBuilder { value: __value };
    }

    pub fn dosing_guideline<'a>(
        &'a mut self,
        val: Vec<MedicationKnowledge_DosingGuideline>,
    ) -> &'a mut MedicationKnowledge_IndicationGuidelineBuilder {
        self.value["dosingGuideline"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut MedicationKnowledge_IndicationGuidelineBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut MedicationKnowledge_IndicationGuidelineBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn indication<'a>(
        &'a mut self,
        val: Vec<CodeableReference>,
    ) -> &'a mut MedicationKnowledge_IndicationGuidelineBuilder {
        self.value["indication"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut MedicationKnowledge_IndicationGuidelineBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }
}
