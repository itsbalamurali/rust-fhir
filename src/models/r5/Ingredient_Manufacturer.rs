#![allow(unused_imports, non_camel_case_types)]

use crate::models::r5::Coding::Coding;
use crate::models::r5::Extension::Extension;
use crate::models::r5::Reference::Reference;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// An ingredient of a manufactured item or pharmaceutical product.

#[derive(Debug)]
pub struct Ingredient_Manufacturer<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Ingredient_Manufacturer<'_> {
    pub fn new(value: &Value) -> Ingredient_Manufacturer {
        Ingredient_Manufacturer {
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

    /// An organization that manufactures this ingredient.
    pub fn manufacturer(&self) -> Reference {
        Reference {
            value: Cow::Borrowed(&self.value["manufacturer"]),
        }
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

    /// The way in which this manufacturer is associated with the ingredient. For
    /// example whether it is a possible one (others allowed), or an exclusive
    /// authorized one for this ingredient. Note that this is not the manufacturing
    /// process role.
    pub fn role(&self) -> Option<Coding> {
        if let Some(val) = self.value.get("role") {
            return Some(Coding {
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
        if !self.manufacturer().validate() {
            return false;
        }
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
        return true;
    }
}

#[derive(Debug)]
pub struct Ingredient_ManufacturerBuilder {
    pub(crate) value: Value,
}

impl Ingredient_ManufacturerBuilder {
    pub fn build(&self) -> Ingredient_Manufacturer {
        Ingredient_Manufacturer {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Ingredient_Manufacturer) -> Ingredient_ManufacturerBuilder {
        Ingredient_ManufacturerBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(manufacturer: Reference) -> Ingredient_ManufacturerBuilder {
        let mut __value: Value = json!({});
        __value["manufacturer"] = json!(manufacturer.value);
        return Ingredient_ManufacturerBuilder { value: __value };
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut Ingredient_ManufacturerBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut Ingredient_ManufacturerBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut Ingredient_ManufacturerBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn role<'a>(&'a mut self, val: Coding) -> &'a mut Ingredient_ManufacturerBuilder {
        self.value["role"] = json!(val.value);
        return self;
    }
}
