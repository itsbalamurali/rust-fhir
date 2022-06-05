#![allow(unused_imports, non_camel_case_types)]

use crate::models::r4b::CodeableConcept::CodeableConcept;
use crate::models::r4b::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// The detailed description of a substance, typically at a level beyond what is
/// used for prescribing.

#[derive(Debug)]
pub struct SubstanceDefinition_SourceMaterial<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl SubstanceDefinition_SourceMaterial<'_> {
    pub fn new(value: &Value) -> SubstanceDefinition_SourceMaterial {
        SubstanceDefinition_SourceMaterial {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// The country or countries where the material is harvested.
    pub fn country_of_origin(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("countryOfOrigin") {
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

    /// The genus of an organism, typically referring to the Latin epithet of the genus
    /// element of the plant/animal scientific name.
    pub fn genus(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("genus") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
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

    /// An anatomical origin of the source material within an organism.
    pub fn part(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("part") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The species of an organism, typically referring to the Latin epithet of the
    /// species of the plant/animal.
    pub fn species(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("species") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A classification that provides the origin of the raw material. Example: cat hair
    /// would be an Animal source type.
    pub fn fhir_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("type") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.country_of_origin() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.genus() {
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
        if let Some(_val) = self.part() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.species() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.fhir_type() {
            if !_val.validate() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct SubstanceDefinition_SourceMaterialBuilder {
    pub(crate) value: Value,
}

impl SubstanceDefinition_SourceMaterialBuilder {
    pub fn build(&self) -> SubstanceDefinition_SourceMaterial {
        SubstanceDefinition_SourceMaterial {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(
        existing: SubstanceDefinition_SourceMaterial,
    ) -> SubstanceDefinition_SourceMaterialBuilder {
        SubstanceDefinition_SourceMaterialBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> SubstanceDefinition_SourceMaterialBuilder {
        let mut __value: Value = json!({});
        return SubstanceDefinition_SourceMaterialBuilder { value: __value };
    }

    pub fn country_of_origin<'a>(
        &'a mut self,
        val: Vec<CodeableConcept>,
    ) -> &'a mut SubstanceDefinition_SourceMaterialBuilder {
        self.value["countryOfOrigin"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut SubstanceDefinition_SourceMaterialBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn genus<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut SubstanceDefinition_SourceMaterialBuilder {
        self.value["genus"] = json!(val.value);
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut SubstanceDefinition_SourceMaterialBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut SubstanceDefinition_SourceMaterialBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn part<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut SubstanceDefinition_SourceMaterialBuilder {
        self.value["part"] = json!(val.value);
        return self;
    }

    pub fn species<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut SubstanceDefinition_SourceMaterialBuilder {
        self.value["species"] = json!(val.value);
        return self;
    }

    pub fn fhir_type<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut SubstanceDefinition_SourceMaterialBuilder {
        self.value["type"] = json!(val.value);
        return self;
    }
}
