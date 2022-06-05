#![allow(unused_imports, non_camel_case_types)]

use crate::models::r5::Citation_DateOfPublication::Citation_DateOfPublication;
use crate::models::r5::CodeableConcept::CodeableConcept;
use crate::models::r5::Element::Element;
use crate::models::r5::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// The Citation Resource enables reference to any knowledge artifact for purposes
/// of identification and attribution. The Citation Resource supports existing
/// reference structures and developing publication practices such as versioning,
/// expressing complex contributorship roles, and referencing computable resources.

#[derive(Debug)]
pub struct Citation_PeriodicRelease<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Citation_PeriodicRelease<'_> {
    pub fn new(value: &Value) -> Citation_PeriodicRelease {
        Citation_PeriodicRelease {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for issue
    pub fn _issue(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_issue") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for volume
    pub fn _volume(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_volume") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Describes the form of the medium cited. Common codes are "Internet" or "Print".
    pub fn cited_medium(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("citedMedium") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Defining the date on which the issue of the journal was published.
    pub fn date_of_publication(&self) -> Option<Citation_DateOfPublication> {
        if let Some(val) = self.value.get("dateOfPublication") {
            return Some(Citation_DateOfPublication {
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

    /// Issue, part or supplement of journal in which the article is published.
    pub fn issue(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("issue") {
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

    /// Volume number of journal in which the article is published.
    pub fn volume(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("volume") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._issue() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._volume() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.cited_medium() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.date_of_publication() {
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
        if let Some(_val) = self.issue() {}
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.volume() {}
        return true;
    }
}

#[derive(Debug)]
pub struct Citation_PeriodicReleaseBuilder {
    pub(crate) value: Value,
}

impl Citation_PeriodicReleaseBuilder {
    pub fn build(&self) -> Citation_PeriodicRelease {
        Citation_PeriodicRelease {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Citation_PeriodicRelease) -> Citation_PeriodicReleaseBuilder {
        Citation_PeriodicReleaseBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> Citation_PeriodicReleaseBuilder {
        let mut __value: Value = json!({});
        return Citation_PeriodicReleaseBuilder { value: __value };
    }

    pub fn _issue<'a>(&'a mut self, val: Element) -> &'a mut Citation_PeriodicReleaseBuilder {
        self.value["_issue"] = json!(val.value);
        return self;
    }

    pub fn _volume<'a>(&'a mut self, val: Element) -> &'a mut Citation_PeriodicReleaseBuilder {
        self.value["_volume"] = json!(val.value);
        return self;
    }

    pub fn cited_medium<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut Citation_PeriodicReleaseBuilder {
        self.value["citedMedium"] = json!(val.value);
        return self;
    }

    pub fn date_of_publication<'a>(
        &'a mut self,
        val: Citation_DateOfPublication,
    ) -> &'a mut Citation_PeriodicReleaseBuilder {
        self.value["dateOfPublication"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut Citation_PeriodicReleaseBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut Citation_PeriodicReleaseBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn issue<'a>(&'a mut self, val: &str) -> &'a mut Citation_PeriodicReleaseBuilder {
        self.value["issue"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut Citation_PeriodicReleaseBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn volume<'a>(&'a mut self, val: &str) -> &'a mut Citation_PeriodicReleaseBuilder {
        self.value["volume"] = json!(val);
        return self;
    }
}
