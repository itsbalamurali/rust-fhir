#![allow(unused_imports, non_camel_case_types)]

use crate::models::r5::CodeableConcept::CodeableConcept;
use crate::models::r5::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Record details about an anatomical structure.  This resource may be used when a
/// coded concept does not provide the necessary detail needed for the use case.

#[derive(Debug)]
pub struct BodyStructure_IncludedStructure<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl BodyStructure_IncludedStructure<'_> {
    pub fn new(value: &Value) -> BodyStructure_IncludedStructure {
        BodyStructure_IncludedStructure {
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

    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
        }
        return None;
    }

    /// Code that represents the included structure laterality.
    pub fn laterality(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("laterality") {
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

    /// Code that represents the included structure qualifier.
    pub fn qualifier(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("qualifier") {
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

    /// Code that represents the included structure.
    pub fn structure(&self) -> CodeableConcept {
        CodeableConcept {
            value: Cow::Borrowed(&self.value["structure"]),
        }
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.laterality() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.qualifier() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if !self.structure().validate() {
            return false;
        }
        return true;
    }
}

#[derive(Debug)]
pub struct BodyStructure_IncludedStructureBuilder {
    pub(crate) value: Value,
}

impl BodyStructure_IncludedStructureBuilder {
    pub fn build(&self) -> BodyStructure_IncludedStructure {
        BodyStructure_IncludedStructure {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(
        existing: BodyStructure_IncludedStructure,
    ) -> BodyStructure_IncludedStructureBuilder {
        BodyStructure_IncludedStructureBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(structure: CodeableConcept) -> BodyStructure_IncludedStructureBuilder {
        let mut __value: Value = json!({});
        __value["structure"] = json!(structure.value);
        return BodyStructure_IncludedStructureBuilder { value: __value };
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut BodyStructure_IncludedStructureBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut BodyStructure_IncludedStructureBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn laterality<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut BodyStructure_IncludedStructureBuilder {
        self.value["laterality"] = json!(val.value);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut BodyStructure_IncludedStructureBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn qualifier<'a>(
        &'a mut self,
        val: Vec<CodeableConcept>,
    ) -> &'a mut BodyStructure_IncludedStructureBuilder {
        self.value["qualifier"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }
}
