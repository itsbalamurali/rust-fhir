#![allow(unused_imports, non_camel_case_types)]

use crate::models::r5::CodeableConcept::CodeableConcept;
use crate::models::r5::Element::Element;
use crate::models::r5::Extension::Extension;
use crate::models::r5::Quantity::Quantity;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Properties of a substance specific to it being a polymer.

#[derive(Debug)]
pub struct SubstancePolymer_StartingMaterial<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl SubstancePolymer_StartingMaterial<'_> {
    pub fn new(value: &Value) -> SubstancePolymer_StartingMaterial {
        SubstancePolymer_StartingMaterial {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for isDefining
    pub fn _is_defining(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_isDefining") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A percentage.
    pub fn amount(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("amount") {
            return Some(Quantity {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Substance high level category, e.g. chemical substance.
    pub fn category(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("category") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The type of substance for this starting material.
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

    /// Unique id for the element within a resource (for internal references). This may be
    /// any string value that does not contain spaces.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
        }
        return None;
    }

    /// Used to specify whether the attribute described is a defining element for the
    /// unique identification of the polymer.
    pub fn is_defining(&self) -> Option<bool> {
        if let Some(val) = self.value.get("isDefining") {
            return Some(val.as_bool().unwrap());
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

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._is_defining() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.amount() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.category() {
            if !_val.validate() {
                return false;
            }
        }
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
        if let Some(_val) = self.is_defining() {}
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct SubstancePolymer_StartingMaterialBuilder {
    pub(crate) value: Value,
}

impl SubstancePolymer_StartingMaterialBuilder {
    pub fn build(&self) -> SubstancePolymer_StartingMaterial {
        SubstancePolymer_StartingMaterial {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(
        existing: SubstancePolymer_StartingMaterial,
    ) -> SubstancePolymer_StartingMaterialBuilder {
        SubstancePolymer_StartingMaterialBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> SubstancePolymer_StartingMaterialBuilder {
        let mut __value: Value = json!({});
        return SubstancePolymer_StartingMaterialBuilder { value: __value };
    }

    pub fn _is_defining<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut SubstancePolymer_StartingMaterialBuilder {
        self.value["_isDefining"] = json!(val.value);
        return self;
    }

    pub fn amount<'a>(
        &'a mut self,
        val: Quantity,
    ) -> &'a mut SubstancePolymer_StartingMaterialBuilder {
        self.value["amount"] = json!(val.value);
        return self;
    }

    pub fn category<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut SubstancePolymer_StartingMaterialBuilder {
        self.value["category"] = json!(val.value);
        return self;
    }

    pub fn code<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut SubstancePolymer_StartingMaterialBuilder {
        self.value["code"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut SubstancePolymer_StartingMaterialBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut SubstancePolymer_StartingMaterialBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn is_defining<'a>(
        &'a mut self,
        val: bool,
    ) -> &'a mut SubstancePolymer_StartingMaterialBuilder {
        self.value["isDefining"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut SubstancePolymer_StartingMaterialBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }
}
