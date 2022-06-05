#![allow(unused_imports, non_camel_case_types)]

use crate::models::r5::CodeableConcept::CodeableConcept;
use crate::models::r5::Coding::Coding;
use crate::models::r5::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A record of an event relevant for purposes such as operations, privacy, security,
/// maintenance, and performance analysis.

#[derive(Debug)]
pub struct AuditEvent_Outcome<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl AuditEvent_Outcome<'_> {
    pub fn new(value: &Value) -> AuditEvent_Outcome {
        AuditEvent_Outcome {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Indicates whether the event succeeded or failed.
    pub fn code(&self) -> Coding {
        Coding {
            value: Cow::Borrowed(&self.value["code"]),
        }
    }

    /// Additional details about the error. This may be a text description of the error or
    /// a system code that identifies the error.
    pub fn detail(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("detail") {
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

    pub fn validate(&self) -> bool {
        if !self.code().validate() {
            return false;
        }
        if let Some(_val) = self.detail() {
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
        return true;
    }
}

#[derive(Debug)]
pub struct AuditEvent_OutcomeBuilder {
    pub(crate) value: Value,
}

impl AuditEvent_OutcomeBuilder {
    pub fn build(&self) -> AuditEvent_Outcome {
        AuditEvent_Outcome {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: AuditEvent_Outcome) -> AuditEvent_OutcomeBuilder {
        AuditEvent_OutcomeBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(code: Coding) -> AuditEvent_OutcomeBuilder {
        let mut __value: Value = json!({});
        __value["code"] = json!(code.value);
        return AuditEvent_OutcomeBuilder { value: __value };
    }

    pub fn detail<'a>(
        &'a mut self,
        val: Vec<CodeableConcept>,
    ) -> &'a mut AuditEvent_OutcomeBuilder {
        self.value["detail"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut AuditEvent_OutcomeBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut AuditEvent_OutcomeBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut AuditEvent_OutcomeBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }
}
