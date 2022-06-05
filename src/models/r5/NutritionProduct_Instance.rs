#![allow(unused_imports, non_camel_case_types)]

use crate::models::r5::Element::Element;
use crate::models::r5::Extension::Extension;
use crate::models::r5::Identifier::Identifier;
use crate::models::r5::Quantity::Quantity;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A food or fluid product that is consumed by patients.

#[derive(Debug)]
pub struct NutritionProduct_Instance<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl NutritionProduct_Instance<'_> {
    pub fn new(value: &Value) -> NutritionProduct_Instance {
        NutritionProduct_Instance {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for expiry
    pub fn _expiry(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_expiry") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for lotNumber
    pub fn _lot_number(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_lotNumber") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for useBy
    pub fn _use_by(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_useBy") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// An identifier that supports traceability to the biological entity that is the
    /// source of biological material in the product.
    pub fn biological_source(&self) -> Option<Identifier> {
        if let Some(val) = self.value.get("biologicalSource") {
            return Some(Identifier {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The time after which the product is no longer expected to be in proper
    /// condition, or its use is not advised or not allowed.
    pub fn expiry(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("expiry") {
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

    /// The identifier for the physical instance, typically a serial number.
    pub fn identifier(&self) -> Option<Vec<Identifier>> {
        if let Some(Value::Array(val)) = self.value.get("identifier") {
            return Some(
                val.into_iter()
                    .map(|e| Identifier {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The identification of the batch or lot of the product.
    pub fn lot_number(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("lotNumber") {
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

    /// The amount of items or instances that the resource considers, for instance when
    /// referring to 2 identical units together.
    pub fn quantity(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("quantity") {
            return Some(Quantity {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The time after which the product is no longer expected to be in proper
    /// condition, or its use is not advised or not allowed.
    pub fn use_by(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("useBy") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._expiry() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._lot_number() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._use_by() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.biological_source() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.expiry() {}
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.identifier() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.lot_number() {}
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.quantity() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.use_by() {}
        return true;
    }
}

#[derive(Debug)]
pub struct NutritionProduct_InstanceBuilder {
    pub(crate) value: Value,
}

impl NutritionProduct_InstanceBuilder {
    pub fn build(&self) -> NutritionProduct_Instance {
        NutritionProduct_Instance {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: NutritionProduct_Instance) -> NutritionProduct_InstanceBuilder {
        NutritionProduct_InstanceBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> NutritionProduct_InstanceBuilder {
        let mut __value: Value = json!({});
        return NutritionProduct_InstanceBuilder { value: __value };
    }

    pub fn _expiry<'a>(&'a mut self, val: Element) -> &'a mut NutritionProduct_InstanceBuilder {
        self.value["_expiry"] = json!(val.value);
        return self;
    }

    pub fn _lot_number<'a>(&'a mut self, val: Element) -> &'a mut NutritionProduct_InstanceBuilder {
        self.value["_lotNumber"] = json!(val.value);
        return self;
    }

    pub fn _use_by<'a>(&'a mut self, val: Element) -> &'a mut NutritionProduct_InstanceBuilder {
        self.value["_useBy"] = json!(val.value);
        return self;
    }

    pub fn biological_source<'a>(
        &'a mut self,
        val: Identifier,
    ) -> &'a mut NutritionProduct_InstanceBuilder {
        self.value["biologicalSource"] = json!(val.value);
        return self;
    }

    pub fn expiry<'a>(&'a mut self, val: &str) -> &'a mut NutritionProduct_InstanceBuilder {
        self.value["expiry"] = json!(val);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut NutritionProduct_InstanceBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut NutritionProduct_InstanceBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn identifier<'a>(
        &'a mut self,
        val: Vec<Identifier>,
    ) -> &'a mut NutritionProduct_InstanceBuilder {
        self.value["identifier"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn lot_number<'a>(&'a mut self, val: &str) -> &'a mut NutritionProduct_InstanceBuilder {
        self.value["lotNumber"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut NutritionProduct_InstanceBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn quantity<'a>(&'a mut self, val: Quantity) -> &'a mut NutritionProduct_InstanceBuilder {
        self.value["quantity"] = json!(val.value);
        return self;
    }

    pub fn use_by<'a>(&'a mut self, val: &str) -> &'a mut NutritionProduct_InstanceBuilder {
        self.value["useBy"] = json!(val);
        return self;
    }
}
