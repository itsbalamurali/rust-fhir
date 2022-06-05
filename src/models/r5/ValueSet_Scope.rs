#![allow(unused_imports, non_camel_case_types)]

use crate::models::r5::Element::Element;
use crate::models::r5::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A ValueSet resource instance specifies a set of codes drawn from one or more
/// code systems, intended for use in a particular context. Value sets link between
/// [CodeSystem](codesystem.html) definitions and their use in [coded
/// elements](terminologies.html).

#[derive(Debug)]
pub struct ValueSet_Scope<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl ValueSet_Scope<'_> {
    pub fn new(value: &Value) -> ValueSet_Scope {
        ValueSet_Scope {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for exclusionCriteria
    pub fn _exclusion_criteria(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_exclusionCriteria") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for focus
    pub fn _focus(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_focus") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for inclusionCriteria
    pub fn _inclusion_criteria(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_inclusionCriteria") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Criteria describing which concepts or codes should be excluded and why.
    pub fn exclusion_criteria(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("exclusionCriteria") {
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

    /// The general focus of the Value Set as it relates to the intended semantic space.
    /// This can be the information about clinical relevancy or the statement about the
    /// general focus of the Value Set, such as a description of types of messages,
    /// payment options, geographic locations, etc.
    pub fn focus(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("focus") {
            return Some(string);
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

    /// Criteria describing which concepts or codes should be included and why.
    pub fn inclusion_criteria(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("inclusionCriteria") {
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

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._exclusion_criteria() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._focus() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._inclusion_criteria() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.exclusion_criteria() {}
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.focus() {}
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.inclusion_criteria() {}
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct ValueSet_ScopeBuilder {
    pub(crate) value: Value,
}

impl ValueSet_ScopeBuilder {
    pub fn build(&self) -> ValueSet_Scope {
        ValueSet_Scope {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: ValueSet_Scope) -> ValueSet_ScopeBuilder {
        ValueSet_ScopeBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> ValueSet_ScopeBuilder {
        let mut __value: Value = json!({});
        return ValueSet_ScopeBuilder { value: __value };
    }

    pub fn _exclusion_criteria<'a>(&'a mut self, val: Element) -> &'a mut ValueSet_ScopeBuilder {
        self.value["_exclusionCriteria"] = json!(val.value);
        return self;
    }

    pub fn _focus<'a>(&'a mut self, val: Element) -> &'a mut ValueSet_ScopeBuilder {
        self.value["_focus"] = json!(val.value);
        return self;
    }

    pub fn _inclusion_criteria<'a>(&'a mut self, val: Element) -> &'a mut ValueSet_ScopeBuilder {
        self.value["_inclusionCriteria"] = json!(val.value);
        return self;
    }

    pub fn exclusion_criteria<'a>(&'a mut self, val: &str) -> &'a mut ValueSet_ScopeBuilder {
        self.value["exclusionCriteria"] = json!(val);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut ValueSet_ScopeBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn focus<'a>(&'a mut self, val: &str) -> &'a mut ValueSet_ScopeBuilder {
        self.value["focus"] = json!(val);
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut ValueSet_ScopeBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn inclusion_criteria<'a>(&'a mut self, val: &str) -> &'a mut ValueSet_ScopeBuilder {
        self.value["inclusionCriteria"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ValueSet_ScopeBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }
}
