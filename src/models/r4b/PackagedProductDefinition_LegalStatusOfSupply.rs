#![allow(unused_imports, non_camel_case_types)]

use crate::models::r4b::CodeableConcept::CodeableConcept;
use crate::models::r4b::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A medically related item or items, in a container or package.

#[derive(Debug)]
pub struct PackagedProductDefinition_LegalStatusOfSupply<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl PackagedProductDefinition_LegalStatusOfSupply<'_> {
    pub fn new(value: &Value) -> PackagedProductDefinition_LegalStatusOfSupply {
        PackagedProductDefinition_LegalStatusOfSupply {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// The actual status of supply. Conveys in what situation this package type may be
    /// supplied for use.
    pub fn code(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("code") {
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

    /// The place where the legal status of supply applies. When not specified, this
    /// indicates it is unknown in this context.
    pub fn jurisdiction(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("jurisdiction") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
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

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.code() {
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
        if let Some(_val) = self.jurisdiction() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct PackagedProductDefinition_LegalStatusOfSupplyBuilder {
    pub(crate) value: Value,
}

impl PackagedProductDefinition_LegalStatusOfSupplyBuilder {
    pub fn build(&self) -> PackagedProductDefinition_LegalStatusOfSupply {
        PackagedProductDefinition_LegalStatusOfSupply {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(
        existing: PackagedProductDefinition_LegalStatusOfSupply,
    ) -> PackagedProductDefinition_LegalStatusOfSupplyBuilder {
        PackagedProductDefinition_LegalStatusOfSupplyBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> PackagedProductDefinition_LegalStatusOfSupplyBuilder {
        let mut __value: Value = json!({});
        return PackagedProductDefinition_LegalStatusOfSupplyBuilder { value: __value };
    }

    pub fn code<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut PackagedProductDefinition_LegalStatusOfSupplyBuilder {
        self.value["code"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut PackagedProductDefinition_LegalStatusOfSupplyBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut PackagedProductDefinition_LegalStatusOfSupplyBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn jurisdiction<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut PackagedProductDefinition_LegalStatusOfSupplyBuilder {
        self.value["jurisdiction"] = json!(val.value);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut PackagedProductDefinition_LegalStatusOfSupplyBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }
}
