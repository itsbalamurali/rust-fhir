#![allow(unused_imports, non_camel_case_types)]

use crate::models::r5::CodeableConcept::CodeableConcept;
use crate::models::r5::Element::Element;
use crate::models::r5::Extension::Extension;
use crate::models::r5::Reference::Reference;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A group of related requests that can be used to capture intended activities that
/// have inter-dependencies such as "give this medication after that one".

#[derive(Debug)]
pub struct RequestGroup_Participant<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl RequestGroup_Participant<'_> {
    pub fn new(value: &Value) -> RequestGroup_Participant {
        RequestGroup_Participant {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for type
    pub fn _type(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_type") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A reference to the actual participant.
    pub fn actor(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("actor") {
            return Some(Reference {
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

    /// Indicates how the actor will be involved in the action - author, reviewer,
    /// witness, etc.
    pub fn function(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("function") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
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

    /// The role the participant should play in performing the described action.
    pub fn role(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("role") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The type of participant in the action.
    pub fn fhir_type(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("type") {
            return Some(string);
        }
        return None;
    }

    /// The type of participant in the action.
    pub fn type_reference(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("typeReference") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._type() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.actor() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.function() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.role() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.fhir_type() {}
        if let Some(_val) = self.type_reference() {
            if !_val.validate() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct RequestGroup_ParticipantBuilder {
    pub(crate) value: Value,
}

impl RequestGroup_ParticipantBuilder {
    pub fn build(&self) -> RequestGroup_Participant {
        RequestGroup_Participant {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: RequestGroup_Participant) -> RequestGroup_ParticipantBuilder {
        RequestGroup_ParticipantBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> RequestGroup_ParticipantBuilder {
        let mut __value: Value = json!({});
        return RequestGroup_ParticipantBuilder { value: __value };
    }

    pub fn _type<'a>(&'a mut self, val: Element) -> &'a mut RequestGroup_ParticipantBuilder {
        self.value["_type"] = json!(val.value);
        return self;
    }

    pub fn actor<'a>(&'a mut self, val: Reference) -> &'a mut RequestGroup_ParticipantBuilder {
        self.value["actor"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut RequestGroup_ParticipantBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn function<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut RequestGroup_ParticipantBuilder {
        self.value["function"] = json!(val.value);
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut RequestGroup_ParticipantBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut RequestGroup_ParticipantBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn role<'a>(&'a mut self, val: CodeableConcept) -> &'a mut RequestGroup_ParticipantBuilder {
        self.value["role"] = json!(val.value);
        return self;
    }

    pub fn fhir_type<'a>(&'a mut self, val: &str) -> &'a mut RequestGroup_ParticipantBuilder {
        self.value["type"] = json!(val);
        return self;
    }

    pub fn type_reference<'a>(
        &'a mut self,
        val: Reference,
    ) -> &'a mut RequestGroup_ParticipantBuilder {
        self.value["typeReference"] = json!(val.value);
        return self;
    }
}
