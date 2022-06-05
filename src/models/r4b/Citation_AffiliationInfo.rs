#![allow(unused_imports, non_camel_case_types)]

use crate::models::r4b::Element::Element;
use crate::models::r4b::Extension::Extension;
use crate::models::r4b::Identifier::Identifier;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// The Citation Resource enables reference to any knowledge artifact for purposes
/// of identification and attribution. The Citation Resource supports existing
/// reference structures and developing publication practices such as versioning,
/// expressing complex contributorship roles, and referencing computable resources.

#[derive(Debug)]
pub struct Citation_AffiliationInfo<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Citation_AffiliationInfo<'_> {
    pub fn new(value: &Value) -> Citation_AffiliationInfo {
        Citation_AffiliationInfo {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for affiliation
    pub fn _affiliation(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_affiliation") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for role
    pub fn _role(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_role") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Display for the organization.
    pub fn affiliation(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("affiliation") {
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

    /// Identifier for the organization.
    pub fn identifier(&self) -> Option<Vec<Identifier>> {
        if let Some(Value::Array(val)) = self.value.get("identifier") {
            return Some(
                val.into_iter()
                    .map(|e| Identifier {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
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

    /// Role within the organization, such as professional title.
    pub fn role(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("role") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._affiliation() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._role() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.affiliation() {}
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.identifier() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.role() {}
        return true;
    }
}

#[derive(Debug)]
pub struct Citation_AffiliationInfoBuilder {
    pub(crate) value: Value,
}

impl Citation_AffiliationInfoBuilder {
    pub fn build(&self) -> Citation_AffiliationInfo {
        Citation_AffiliationInfo {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Citation_AffiliationInfo) -> Citation_AffiliationInfoBuilder {
        Citation_AffiliationInfoBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> Citation_AffiliationInfoBuilder {
        let mut __value: Value = json!({});
        return Citation_AffiliationInfoBuilder { value: __value };
    }

    pub fn _affiliation<'a>(&'a mut self, val: Element) -> &'a mut Citation_AffiliationInfoBuilder {
        self.value["_affiliation"] = json!(val.value);
        return self;
    }

    pub fn _role<'a>(&'a mut self, val: Element) -> &'a mut Citation_AffiliationInfoBuilder {
        self.value["_role"] = json!(val.value);
        return self;
    }

    pub fn affiliation<'a>(&'a mut self, val: &str) -> &'a mut Citation_AffiliationInfoBuilder {
        self.value["affiliation"] = json!(val);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut Citation_AffiliationInfoBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut Citation_AffiliationInfoBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn identifier<'a>(
        &'a mut self,
        val: Vec<Identifier>,
    ) -> &'a mut Citation_AffiliationInfoBuilder {
        self.value["identifier"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut Citation_AffiliationInfoBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn role<'a>(&'a mut self, val: &str) -> &'a mut Citation_AffiliationInfoBuilder {
        self.value["role"] = json!(val);
        return self;
    }
}
