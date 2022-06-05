#![allow(unused_imports, non_camel_case_types)]

use crate::models::r5::Element::Element;
use crate::models::r5::Extension::Extension;
use crate::models::r5::Reference::Reference;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// The Citation Resource enables reference to any knowledge artifact for purposes
/// of identification and attribution. The Citation Resource supports existing
/// reference structures and developing publication practices such as versioning,
/// expressing complex contributorship roles, and referencing computable resources.

#[derive(Debug)]
pub struct Citation_WhoClassified<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Citation_WhoClassified<'_> {
    pub fn new(value: &Value) -> Citation_WhoClassified {
        Citation_WhoClassified {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for classifierCopyright
    pub fn _classifier_copyright(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_classifierCopyright") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for freeToShare
    pub fn _free_to_share(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_freeToShare") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Rights management statement for the classification.
    pub fn classifier_copyright(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("classifierCopyright") {
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

    /// Acceptable to re-use the classification.
    pub fn free_to_share(&self) -> Option<bool> {
        if let Some(val) = self.value.get("freeToShare") {
            return Some(val.as_bool().unwrap());
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

    /// Organization who created the classification.
    pub fn organization(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("organization") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Person who created the classification.
    pub fn person(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("person") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The publisher of the classification, not the publisher of the article or
    /// artifact being cited.
    pub fn publisher(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("publisher") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._classifier_copyright() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._free_to_share() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.classifier_copyright() {}
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.free_to_share() {}
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.organization() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.person() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.publisher() {
            if !_val.validate() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct Citation_WhoClassifiedBuilder {
    pub(crate) value: Value,
}

impl Citation_WhoClassifiedBuilder {
    pub fn build(&self) -> Citation_WhoClassified {
        Citation_WhoClassified {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Citation_WhoClassified) -> Citation_WhoClassifiedBuilder {
        Citation_WhoClassifiedBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> Citation_WhoClassifiedBuilder {
        let mut __value: Value = json!({});
        return Citation_WhoClassifiedBuilder { value: __value };
    }

    pub fn _classifier_copyright<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut Citation_WhoClassifiedBuilder {
        self.value["_classifierCopyright"] = json!(val.value);
        return self;
    }

    pub fn _free_to_share<'a>(&'a mut self, val: Element) -> &'a mut Citation_WhoClassifiedBuilder {
        self.value["_freeToShare"] = json!(val.value);
        return self;
    }

    pub fn classifier_copyright<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut Citation_WhoClassifiedBuilder {
        self.value["classifierCopyright"] = json!(val);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut Citation_WhoClassifiedBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn free_to_share<'a>(&'a mut self, val: bool) -> &'a mut Citation_WhoClassifiedBuilder {
        self.value["freeToShare"] = json!(val);
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut Citation_WhoClassifiedBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut Citation_WhoClassifiedBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn organization<'a>(&'a mut self, val: Reference) -> &'a mut Citation_WhoClassifiedBuilder {
        self.value["organization"] = json!(val.value);
        return self;
    }

    pub fn person<'a>(&'a mut self, val: Reference) -> &'a mut Citation_WhoClassifiedBuilder {
        self.value["person"] = json!(val.value);
        return self;
    }

    pub fn publisher<'a>(&'a mut self, val: Reference) -> &'a mut Citation_WhoClassifiedBuilder {
        self.value["publisher"] = json!(val.value);
        return self;
    }
}
