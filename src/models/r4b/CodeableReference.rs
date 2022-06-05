#![allow(unused_imports, non_camel_case_types)]

use crate::models::r4b::CodeableConcept::CodeableConcept;
use crate::models::r4b::Extension::Extension;
use crate::models::r4b::Reference::Reference;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A reference to a resource (by instance), or instead, a reference to a concept
/// defined in a terminology or ontology (by class).

#[derive(Debug)]
pub struct CodeableReference<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl CodeableReference<'_> {
    pub fn new(value: &Value) -> CodeableReference {
        CodeableReference {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// A reference to a concept - e.g. the information is identified by its general
    /// class to the degree of precision found in the terminology.
    pub fn concept(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("concept") {
            return Some(CodeableConcept {
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

    /// A reference to a resource the provides exact details about the information being
    /// referenced.
    pub fn reference(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("reference") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.concept() {
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
        if let Some(_val) = self.reference() {
            if !_val.validate() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct CodeableReferenceBuilder {
    pub(crate) value: Value,
}

impl CodeableReferenceBuilder {
    pub fn build(&self) -> CodeableReference {
        CodeableReference {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: CodeableReference) -> CodeableReferenceBuilder {
        CodeableReferenceBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> CodeableReferenceBuilder {
        let mut __value: Value = json!({});
        return CodeableReferenceBuilder { value: __value };
    }

    pub fn concept<'a>(&'a mut self, val: CodeableConcept) -> &'a mut CodeableReferenceBuilder {
        self.value["concept"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut CodeableReferenceBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut CodeableReferenceBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn reference<'a>(&'a mut self, val: Reference) -> &'a mut CodeableReferenceBuilder {
        self.value["reference"] = json!(val.value);
        return self;
    }
}
