#![allow(unused_imports, non_camel_case_types)]

use crate::models::r4b::Attachment::Attachment;
use crate::models::r4b::CodeableConcept::CodeableConcept;
use crate::models::r4b::Element::Element;
use crate::models::r4b::Extension::Extension;
use crate::models::r4b::Identifier::Identifier;
use crate::models::r4b::Reference::Reference;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// The Citation Resource enables reference to any knowledge artifact for purposes of
/// identification and attribution. The Citation Resource supports existing reference
/// structures and developing publication practices such as versioning, expressing
/// complex contributorship roles, and referencing computable resources.

#[derive(Debug)]
pub struct Citation_RelatesTo<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Citation_RelatesTo<'_> {
    pub fn new(value: &Value) -> Citation_RelatesTo {
        Citation_RelatesTo {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for targetUri
    pub fn _target_uri(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_targetUri") {
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

    /// How the Citation resource relates to the target artifact.
    pub fn relationship_type(&self) -> CodeableConcept {
        CodeableConcept {
            value: Cow::Borrowed(&self.value["relationshipType"]),
        }
    }

    /// The article or artifact that the Citation Resource is related to.
    pub fn target_attachment(&self) -> Option<Attachment> {
        if let Some(val) = self.value.get("targetAttachment") {
            return Some(Attachment {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The clasification of the related artifact.
    pub fn target_classifier(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("targetClassifier") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The article or artifact that the Citation Resource is related to.
    pub fn target_identifier(&self) -> Option<Identifier> {
        if let Some(val) = self.value.get("targetIdentifier") {
            return Some(Identifier {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The article or artifact that the Citation Resource is related to.
    pub fn target_reference(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("targetReference") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The article or artifact that the Citation Resource is related to.
    pub fn target_uri(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("targetUri") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._target_uri() {
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
        if !self.relationship_type().validate() {
            return false;
        }
        if let Some(_val) = self.target_attachment() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.target_classifier() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.target_identifier() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.target_reference() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.target_uri() {}
        return true;
    }
}

#[derive(Debug)]
pub struct Citation_RelatesToBuilder {
    pub(crate) value: Value,
}

impl Citation_RelatesToBuilder {
    pub fn build(&self) -> Citation_RelatesTo {
        Citation_RelatesTo {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Citation_RelatesTo) -> Citation_RelatesToBuilder {
        Citation_RelatesToBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(relationship_type: CodeableConcept) -> Citation_RelatesToBuilder {
        let mut __value: Value = json!({});
        __value["relationshipType"] = json!(relationship_type.value);
        return Citation_RelatesToBuilder { value: __value };
    }

    pub fn _target_uri<'a>(&'a mut self, val: Element) -> &'a mut Citation_RelatesToBuilder {
        self.value["_targetUri"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut Citation_RelatesToBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut Citation_RelatesToBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut Citation_RelatesToBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn target_attachment<'a>(
        &'a mut self,
        val: Attachment,
    ) -> &'a mut Citation_RelatesToBuilder {
        self.value["targetAttachment"] = json!(val.value);
        return self;
    }

    pub fn target_classifier<'a>(
        &'a mut self,
        val: Vec<CodeableConcept>,
    ) -> &'a mut Citation_RelatesToBuilder {
        self.value["targetClassifier"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn target_identifier<'a>(
        &'a mut self,
        val: Identifier,
    ) -> &'a mut Citation_RelatesToBuilder {
        self.value["targetIdentifier"] = json!(val.value);
        return self;
    }

    pub fn target_reference<'a>(&'a mut self, val: Reference) -> &'a mut Citation_RelatesToBuilder {
        self.value["targetReference"] = json!(val.value);
        return self;
    }

    pub fn target_uri<'a>(&'a mut self, val: &str) -> &'a mut Citation_RelatesToBuilder {
        self.value["targetUri"] = json!(val);
        return self;
    }
}
