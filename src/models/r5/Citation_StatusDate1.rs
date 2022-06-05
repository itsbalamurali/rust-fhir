#![allow(unused_imports, non_camel_case_types)]

use crate::models::r5::CodeableConcept::CodeableConcept;
use crate::models::r5::Element::Element;
use crate::models::r5::Extension::Extension;
use crate::models::r5::Period::Period;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// The Citation Resource enables reference to any knowledge artifact for purposes of
/// identification and attribution. The Citation Resource supports existing reference
/// structures and developing publication practices such as versioning, expressing
/// complex contributorship roles, and referencing computable resources.

#[derive(Debug)]
pub struct Citation_StatusDate1<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Citation_StatusDate1<'_> {
    pub fn new(value: &Value) -> Citation_StatusDate1 {
        Citation_StatusDate1 {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for actual
    pub fn _actual(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_actual") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Classification of the status.
    pub fn activity(&self) -> CodeableConcept {
        CodeableConcept {
            value: Cow::Borrowed(&self.value["activity"]),
        }
    }

    /// Either occurred or expected.
    pub fn actual(&self) -> Option<bool> {
        if let Some(val) = self.value.get("actual") {
            return Some(val.as_bool().unwrap());
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

    /// When the status started and/or ended.
    pub fn period(&self) -> Period {
        Period {
            value: Cow::Borrowed(&self.value["period"]),
        }
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._actual() {
            if !_val.validate() {
                return false;
            }
        }
        if !self.activity().validate() {
            return false;
        }
        if let Some(_val) = self.actual() {}
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
        if !self.period().validate() {
            return false;
        }
        return true;
    }
}

#[derive(Debug)]
pub struct Citation_StatusDate1Builder {
    pub(crate) value: Value,
}

impl Citation_StatusDate1Builder {
    pub fn build(&self) -> Citation_StatusDate1 {
        Citation_StatusDate1 {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Citation_StatusDate1) -> Citation_StatusDate1Builder {
        Citation_StatusDate1Builder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(activity: CodeableConcept, period: Period) -> Citation_StatusDate1Builder {
        let mut __value: Value = json!({});
        __value["activity"] = json!(activity.value);
        __value["period"] = json!(period.value);
        return Citation_StatusDate1Builder { value: __value };
    }

    pub fn _actual<'a>(&'a mut self, val: Element) -> &'a mut Citation_StatusDate1Builder {
        self.value["_actual"] = json!(val.value);
        return self;
    }

    pub fn actual<'a>(&'a mut self, val: bool) -> &'a mut Citation_StatusDate1Builder {
        self.value["actual"] = json!(val);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut Citation_StatusDate1Builder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut Citation_StatusDate1Builder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut Citation_StatusDate1Builder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }
}
