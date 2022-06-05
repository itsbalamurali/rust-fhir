#![allow(unused_imports, non_camel_case_types)]

use crate::models::r4b::Element::Element;
use crate::models::r4b::Extension::Extension;
use crate::models::r4b::Identifier::Identifier;
use crate::models::r4b::Reference::Reference;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// The EvidenceReport Resource is a specialized container for a collection of
/// resources and codable concepts, adapted to support compositions of Evidence,
/// EvidenceVariable, and Citation resources and related concepts.

#[derive(Debug)]
pub struct EvidenceReport_RelatesTo<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl EvidenceReport_RelatesTo<'_> {
    pub fn new(value: &Value) -> EvidenceReport_RelatesTo {
        EvidenceReport_RelatesTo {
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

    /// The type of relationship that this composition has with anther composition or
    /// document.
    pub fn code(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("code") {
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

    /// The target composition/document of this relationship.
    pub fn target_identifier(&self) -> Option<Identifier> {
        if let Some(val) = self.value.get("targetIdentifier") {
            return Some(Identifier {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The target composition/document of this relationship.
    pub fn target_reference(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("targetReference") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._code() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.code() {}
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
        return true;
    }
}

#[derive(Debug)]
pub struct EvidenceReport_RelatesToBuilder {
    pub(crate) value: Value,
}

impl EvidenceReport_RelatesToBuilder {
    pub fn build(&self) -> EvidenceReport_RelatesTo {
        EvidenceReport_RelatesTo {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: EvidenceReport_RelatesTo) -> EvidenceReport_RelatesToBuilder {
        EvidenceReport_RelatesToBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> EvidenceReport_RelatesToBuilder {
        let mut __value: Value = json!({});
        return EvidenceReport_RelatesToBuilder { value: __value };
    }

    pub fn _code<'a>(&'a mut self, val: Element) -> &'a mut EvidenceReport_RelatesToBuilder {
        self.value["_code"] = json!(val.value);
        return self;
    }

    pub fn code<'a>(&'a mut self, val: &str) -> &'a mut EvidenceReport_RelatesToBuilder {
        self.value["code"] = json!(val);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut EvidenceReport_RelatesToBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut EvidenceReport_RelatesToBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut EvidenceReport_RelatesToBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn target_identifier<'a>(
        &'a mut self,
        val: Identifier,
    ) -> &'a mut EvidenceReport_RelatesToBuilder {
        self.value["targetIdentifier"] = json!(val.value);
        return self;
    }

    pub fn target_reference<'a>(
        &'a mut self,
        val: Reference,
    ) -> &'a mut EvidenceReport_RelatesToBuilder {
        self.value["targetReference"] = json!(val.value);
        return self;
    }
}
