#![allow(unused_imports, non_camel_case_types)]

use crate::models::r5::ConceptMap2_Target::ConceptMap2_Target;
use crate::models::r5::Element::Element;
use crate::models::r5::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A statement of relationships from one set of concepts to one or more other
/// concepts - either concepts in code systems, or data element/data element
/// concepts, or classes in class models.

#[derive(Debug)]
pub struct ConceptMap2_Element<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl ConceptMap2_Element<'_> {
    pub fn new(value: &Value) -> ConceptMap2_Element {
        ConceptMap2_Element {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for code
    pub fn _code(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_code") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for display
    pub fn _display(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_display") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for noMap
    pub fn _no_map(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_noMap") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Identity (code or path) or the element/item being mapped.
    pub fn code(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("code") {
            return Some(string);
        }
        return None;
    }

    /// The display for the code. The display is only provided to help editors when
    /// editing the concept map.
    pub fn display(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("display") {
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

    /// If noMap = true this indicates that no mapping to a target concept exists for
    /// this source concept.
    pub fn no_map(&self) -> Option<bool> {
        if let Some(val) = self.value.get("noMap") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    /// A concept from the target value set that this concept maps to.
    pub fn target(&self) -> Option<Vec<ConceptMap2_Target>> {
        if let Some(Value::Array(val)) = self.value.get("target") {
            return Some(
                val.into_iter()
                    .map(|e| ConceptMap2_Target {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The set of codes being mapped.
    pub fn value_set(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("valueSet") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._code() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._display() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._no_map() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.code() {}
        if let Some(_val) = self.display() {}
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
        if let Some(_val) = self.no_map() {}
        if let Some(_val) = self.target() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.value_set() {}
        return true;
    }
}

#[derive(Debug)]
pub struct ConceptMap2_ElementBuilder {
    pub(crate) value: Value,
}

impl ConceptMap2_ElementBuilder {
    pub fn build(&self) -> ConceptMap2_Element {
        ConceptMap2_Element {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: ConceptMap2_Element) -> ConceptMap2_ElementBuilder {
        ConceptMap2_ElementBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> ConceptMap2_ElementBuilder {
        let mut __value: Value = json!({});
        return ConceptMap2_ElementBuilder { value: __value };
    }

    pub fn _code<'a>(&'a mut self, val: Element) -> &'a mut ConceptMap2_ElementBuilder {
        self.value["_code"] = json!(val.value);
        return self;
    }

    pub fn _display<'a>(&'a mut self, val: Element) -> &'a mut ConceptMap2_ElementBuilder {
        self.value["_display"] = json!(val.value);
        return self;
    }

    pub fn _no_map<'a>(&'a mut self, val: Element) -> &'a mut ConceptMap2_ElementBuilder {
        self.value["_noMap"] = json!(val.value);
        return self;
    }

    pub fn code<'a>(&'a mut self, val: &str) -> &'a mut ConceptMap2_ElementBuilder {
        self.value["code"] = json!(val);
        return self;
    }

    pub fn display<'a>(&'a mut self, val: &str) -> &'a mut ConceptMap2_ElementBuilder {
        self.value["display"] = json!(val);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut ConceptMap2_ElementBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut ConceptMap2_ElementBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ConceptMap2_ElementBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn no_map<'a>(&'a mut self, val: bool) -> &'a mut ConceptMap2_ElementBuilder {
        self.value["noMap"] = json!(val);
        return self;
    }

    pub fn target<'a>(
        &'a mut self,
        val: Vec<ConceptMap2_Target>,
    ) -> &'a mut ConceptMap2_ElementBuilder {
        self.value["target"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn value_set<'a>(&'a mut self, val: &str) -> &'a mut ConceptMap2_ElementBuilder {
        self.value["valueSet"] = json!(val);
        return self;
    }
}
