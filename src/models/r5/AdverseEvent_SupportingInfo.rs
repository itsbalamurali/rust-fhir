#![allow(unused_imports, non_camel_case_types)]

use crate::models::r5::CodeableConcept::CodeableConcept;
use crate::models::r5::Extension::Extension;
use crate::models::r5::Reference::Reference;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// An event (i.e. any change to current patient status) that may be related to
/// unintended effects on a patient or research subject.  The unintended effects may
/// require additional monitoring, treatment or hospitalization or may result in
/// death.  The AdverseEvent resource also extends to potential or avoided events
/// that could have had such effects.

#[derive(Debug)]
pub struct AdverseEvent_SupportingInfo<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl AdverseEvent_SupportingInfo<'_> {
    pub fn new(value: &Value) -> AdverseEvent_SupportingInfo {
        AdverseEvent_SupportingInfo {
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

    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
        }
        return None;
    }

    /// Relevant past history for the subject. In a clinical care context, an example
    /// being a patient had an adverse event following a pencillin administration and
    /// the patient had a previously documented penicillin allergy. In a clinical trials
    /// context, an example is a bunion or rash that was present prior to the study.
    /// Additionally, the supporting item can be a document that is relevant to this
    /// instance of the adverse event that is not part of the subject's medical history.
    /// For example, a clinical note, staff list, or material safety data sheet (MSDS).
    /// Supporting information is not a contributing factor, preventive action, or
    /// mitigating action.
    pub fn item_codeable_concept(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("itemCodeableConcept") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Relevant past history for the subject. In a clinical care context, an example
    /// being a patient had an adverse event following a pencillin administration and
    /// the patient had a previously documented penicillin allergy. In a clinical trials
    /// context, an example is a bunion or rash that was present prior to the study.
    /// Additionally, the supporting item can be a document that is relevant to this
    /// instance of the adverse event that is not part of the subject's medical history.
    /// For example, a clinical note, staff list, or material safety data sheet (MSDS).
    /// Supporting information is not a contributing factor, preventive action, or
    /// mitigating action.
    pub fn item_reference(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("itemReference") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
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

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.item_codeable_concept() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.item_reference() {
            if !_val.validate() {
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
pub struct AdverseEvent_SupportingInfoBuilder {
    pub(crate) value: Value,
}

impl AdverseEvent_SupportingInfoBuilder {
    pub fn build(&self) -> AdverseEvent_SupportingInfo {
        AdverseEvent_SupportingInfo {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: AdverseEvent_SupportingInfo) -> AdverseEvent_SupportingInfoBuilder {
        AdverseEvent_SupportingInfoBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> AdverseEvent_SupportingInfoBuilder {
        let mut __value: Value = json!({});
        return AdverseEvent_SupportingInfoBuilder { value: __value };
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut AdverseEvent_SupportingInfoBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut AdverseEvent_SupportingInfoBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn item_codeable_concept<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut AdverseEvent_SupportingInfoBuilder {
        self.value["itemCodeableConcept"] = json!(val.value);
        return self;
    }

    pub fn item_reference<'a>(
        &'a mut self,
        val: Reference,
    ) -> &'a mut AdverseEvent_SupportingInfoBuilder {
        self.value["itemReference"] = json!(val.value);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut AdverseEvent_SupportingInfoBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }
}
