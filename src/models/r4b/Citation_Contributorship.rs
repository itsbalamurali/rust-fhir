#![allow(unused_imports, non_camel_case_types)]

use crate::models::r4b::Citation_Entry::Citation_Entry;
use crate::models::r4b::Citation_Summary1::Citation_Summary1;
use crate::models::r4b::Element::Element;
use crate::models::r4b::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// The Citation Resource enables reference to any knowledge artifact for purposes of
/// identification and attribution. The Citation Resource supports existing reference
/// structures and developing publication practices such as versioning, expressing
/// complex contributorship roles, and referencing computable resources.

#[derive(Debug)]
pub struct Citation_Contributorship<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Citation_Contributorship<'_> {
    pub fn new(value: &Value) -> Citation_Contributorship {
        Citation_Contributorship {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for complete
    pub fn _complete(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_complete") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Indicates if the list includes all authors and/or contributors.
    pub fn complete(&self) -> Option<bool> {
        if let Some(val) = self.value.get("complete") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    /// An individual entity named in the author list or contributor list.
    pub fn entry(&self) -> Option<Vec<Citation_Entry>> {
        if let Some(Value::Array(val)) = self.value.get("entry") {
            return Some(
                val.into_iter()
                    .map(|e| Citation_Entry {
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

    /// Used to record a display of the author/contributor list without separate coding
    /// for each list member.
    pub fn summary(&self) -> Option<Vec<Citation_Summary1>> {
        if let Some(Value::Array(val)) = self.value.get("summary") {
            return Some(
                val.into_iter()
                    .map(|e| Citation_Summary1 {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._complete() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.complete() {}
        if let Some(_val) = self.entry() {
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
        if let Some(_val) = self.summary() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct Citation_ContributorshipBuilder {
    pub(crate) value: Value,
}

impl Citation_ContributorshipBuilder {
    pub fn build(&self) -> Citation_Contributorship {
        Citation_Contributorship {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Citation_Contributorship) -> Citation_ContributorshipBuilder {
        Citation_ContributorshipBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> Citation_ContributorshipBuilder {
        let mut __value: Value = json!({});
        return Citation_ContributorshipBuilder { value: __value };
    }

    pub fn _complete<'a>(&'a mut self, val: Element) -> &'a mut Citation_ContributorshipBuilder {
        self.value["_complete"] = json!(val.value);
        return self;
    }

    pub fn complete<'a>(&'a mut self, val: bool) -> &'a mut Citation_ContributorshipBuilder {
        self.value["complete"] = json!(val);
        return self;
    }

    pub fn entry<'a>(
        &'a mut self,
        val: Vec<Citation_Entry>,
    ) -> &'a mut Citation_ContributorshipBuilder {
        self.value["entry"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut Citation_ContributorshipBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut Citation_ContributorshipBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut Citation_ContributorshipBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn summary<'a>(
        &'a mut self,
        val: Vec<Citation_Summary1>,
    ) -> &'a mut Citation_ContributorshipBuilder {
        self.value["summary"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }
}
