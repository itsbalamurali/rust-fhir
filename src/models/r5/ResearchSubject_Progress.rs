#![allow(unused_imports, non_camel_case_types)]

use crate::models::r5::CodeableConcept::CodeableConcept;
use crate::models::r5::Element::Element;
use crate::models::r5::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A physical entity which is the primary unit of operational and/or administrative
/// interest in a study.

#[derive(Debug)]
pub struct ResearchSubject_Progress<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl ResearchSubject_Progress<'_> {
    pub fn new(value: &Value) -> ResearchSubject_Progress {
        ResearchSubject_Progress {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for endDate
    pub fn _end_date(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_endDate") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for startDate
    pub fn _start_date(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_startDate") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The date when the state ended.
    pub fn end_date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("endDate") {
            return Some(string);
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

    /// The milestones the subject has passed through.
    pub fn milestone(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("milestone") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
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

    /// The reason for the state change.  If coded it should follow the formal subject
    /// state model.
    pub fn reason(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("reason") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The date when the new status started.
    pub fn start_date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("startDate") {
            return Some(string);
        }
        return None;
    }

    /// The current state of the subject.
    pub fn subject_state(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("subjectState") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Identifies the aspect of the subject's journey that the state refers to.
    pub fn fhir_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("type") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._end_date() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._start_date() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.end_date() {}
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.milestone() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.reason() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.start_date() {}
        if let Some(_val) = self.subject_state() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.fhir_type() {
            if !_val.validate() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct ResearchSubject_ProgressBuilder {
    pub(crate) value: Value,
}

impl ResearchSubject_ProgressBuilder {
    pub fn build(&self) -> ResearchSubject_Progress {
        ResearchSubject_Progress {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: ResearchSubject_Progress) -> ResearchSubject_ProgressBuilder {
        ResearchSubject_ProgressBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> ResearchSubject_ProgressBuilder {
        let mut __value: Value = json!({});
        return ResearchSubject_ProgressBuilder { value: __value };
    }

    pub fn _end_date<'a>(&'a mut self, val: Element) -> &'a mut ResearchSubject_ProgressBuilder {
        self.value["_endDate"] = json!(val.value);
        return self;
    }

    pub fn _start_date<'a>(&'a mut self, val: Element) -> &'a mut ResearchSubject_ProgressBuilder {
        self.value["_startDate"] = json!(val.value);
        return self;
    }

    pub fn end_date<'a>(&'a mut self, val: &str) -> &'a mut ResearchSubject_ProgressBuilder {
        self.value["endDate"] = json!(val);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ResearchSubject_ProgressBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut ResearchSubject_ProgressBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn milestone<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut ResearchSubject_ProgressBuilder {
        self.value["milestone"] = json!(val.value);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ResearchSubject_ProgressBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn reason<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut ResearchSubject_ProgressBuilder {
        self.value["reason"] = json!(val.value);
        return self;
    }

    pub fn start_date<'a>(&'a mut self, val: &str) -> &'a mut ResearchSubject_ProgressBuilder {
        self.value["startDate"] = json!(val);
        return self;
    }

    pub fn subject_state<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut ResearchSubject_ProgressBuilder {
        self.value["subjectState"] = json!(val.value);
        return self;
    }

    pub fn fhir_type<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut ResearchSubject_ProgressBuilder {
        self.value["type"] = json!(val.value);
        return self;
    }
}
