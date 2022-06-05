#![allow(unused_imports, non_camel_case_types)]

use crate::models::r5::CodeableConcept::CodeableConcept;
use crate::models::r5::Extension::Extension;
use crate::models::r5::Reference::Reference;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A record of a healthcare consumer’s  choices  or choices made on their behalf
/// by a third party, which permits or denies identified recipient(s) or recipient
/// role(s) to perform one or more actions within a given policy context, for specific
/// purposes and periods of time.

#[derive(Debug)]
pub struct Consent_Actor<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Consent_Actor<'_> {
    pub fn new(value: &Value) -> Consent_Actor {
        Consent_Actor {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
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

    /// The resource that identifies the actor. To identify actors by type, use group to
    /// identify a set of actors by some property they share (e.g. 'admitting officers').
    pub fn reference(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("reference") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// How the individual is involved in the resources content that is described in the
    /// exception.
    pub fn role(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("role") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
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
        if let Some(_val) = self.reference() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.role() {
            if !_val.validate() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct Consent_ActorBuilder {
    pub(crate) value: Value,
}

impl Consent_ActorBuilder {
    pub fn build(&self) -> Consent_Actor {
        Consent_Actor {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Consent_Actor) -> Consent_ActorBuilder {
        Consent_ActorBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> Consent_ActorBuilder {
        let mut __value: Value = json!({});
        return Consent_ActorBuilder { value: __value };
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut Consent_ActorBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut Consent_ActorBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut Consent_ActorBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn reference<'a>(&'a mut self, val: Reference) -> &'a mut Consent_ActorBuilder {
        self.value["reference"] = json!(val.value);
        return self;
    }

    pub fn role<'a>(&'a mut self, val: CodeableConcept) -> &'a mut Consent_ActorBuilder {
        self.value["role"] = json!(val.value);
        return self;
    }
}
