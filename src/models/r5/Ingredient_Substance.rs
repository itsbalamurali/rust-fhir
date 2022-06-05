#![allow(unused_imports, non_camel_case_types)]

use crate::models::r5::CodeableReference::CodeableReference;
use crate::models::r5::Extension::Extension;
use crate::models::r5::Ingredient_Strength::Ingredient_Strength;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// An ingredient of a manufactured item or pharmaceutical product.

#[derive(Debug)]
pub struct Ingredient_Substance<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Ingredient_Substance<'_> {
    pub fn new(value: &Value) -> Ingredient_Substance {
        Ingredient_Substance {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// A code or full resource that represents the ingredient substance.
    pub fn code(&self) -> CodeableReference {
        CodeableReference {
            value: Cow::Borrowed(&self.value["code"]),
        }
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

    /// The quantity of substance in the unit of presentation, or in the volume (or mass)
    /// of the single pharmaceutical product or manufactured item.
    pub fn strength(&self) -> Option<Vec<Ingredient_Strength>> {
        if let Some(Value::Array(val)) = self.value.get("strength") {
            return Some(
                val.into_iter()
                    .map(|e| Ingredient_Strength {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if !self.code().validate() {
            return false;
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
        if let Some(_val) = self.strength() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct Ingredient_SubstanceBuilder {
    pub(crate) value: Value,
}

impl Ingredient_SubstanceBuilder {
    pub fn build(&self) -> Ingredient_Substance {
        Ingredient_Substance {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Ingredient_Substance) -> Ingredient_SubstanceBuilder {
        Ingredient_SubstanceBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(code: CodeableReference) -> Ingredient_SubstanceBuilder {
        let mut __value: Value = json!({});
        __value["code"] = json!(code.value);
        return Ingredient_SubstanceBuilder { value: __value };
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut Ingredient_SubstanceBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut Ingredient_SubstanceBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut Ingredient_SubstanceBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn strength<'a>(
        &'a mut self,
        val: Vec<Ingredient_Strength>,
    ) -> &'a mut Ingredient_SubstanceBuilder {
        self.value["strength"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }
}
