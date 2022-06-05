#![allow(unused_imports, non_camel_case_types)]

use crate::models::r4b::CodeableConcept::CodeableConcept;
use crate::models::r4b::Element::Element;
use crate::models::r4b::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// The Citation Resource enables reference to any knowledge artifact for purposes
/// of identification and attribution. The Citation Resource supports existing
/// reference structures and developing publication practices such as versioning,
/// expressing complex contributorship roles, and referencing computable resources.

#[derive(Debug)]
pub struct Citation_Summary1<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Citation_Summary1<'_> {
    pub fn new(value: &Value) -> Citation_Summary1 {
        Citation_Summary1 {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for value
    pub fn _value(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_value") {
            return Some(Element {
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

    /// Used to code the producer or rule for creating the display string.
    pub fn source(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("source") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The format for the display string.
    pub fn style(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("style") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Used most commonly to express an author list or a contributorship statement.
    pub fn fhir_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("type") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The display string for the author list, contributor list, or contributorship
    /// statement.
    pub fn value(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("value") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._value() {
            if !_val.validate() {
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
        if let Some(_val) = self.source() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.style() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.fhir_type() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.value() {}
        return true;
    }
}

#[derive(Debug)]
pub struct Citation_Summary1Builder {
    pub(crate) value: Value,
}

impl Citation_Summary1Builder {
    pub fn build(&self) -> Citation_Summary1 {
        Citation_Summary1 {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Citation_Summary1) -> Citation_Summary1Builder {
        Citation_Summary1Builder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> Citation_Summary1Builder {
        let mut __value: Value = json!({});
        return Citation_Summary1Builder { value: __value };
    }

    pub fn _value<'a>(&'a mut self, val: Element) -> &'a mut Citation_Summary1Builder {
        self.value["_value"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut Citation_Summary1Builder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut Citation_Summary1Builder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut Citation_Summary1Builder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn source<'a>(&'a mut self, val: CodeableConcept) -> &'a mut Citation_Summary1Builder {
        self.value["source"] = json!(val.value);
        return self;
    }

    pub fn style<'a>(&'a mut self, val: CodeableConcept) -> &'a mut Citation_Summary1Builder {
        self.value["style"] = json!(val.value);
        return self;
    }

    pub fn fhir_type<'a>(&'a mut self, val: CodeableConcept) -> &'a mut Citation_Summary1Builder {
        self.value["type"] = json!(val.value);
        return self;
    }

    pub fn value<'a>(&'a mut self, val: &str) -> &'a mut Citation_Summary1Builder {
        self.value["value"] = json!(val);
        return self;
    }
}
