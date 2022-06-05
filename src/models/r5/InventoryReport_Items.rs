#![allow(unused_imports, non_camel_case_types)]

use crate::models::r5::CodeableConcept::CodeableConcept;
use crate::models::r5::CodeableReference::CodeableReference;
use crate::models::r5::Element::Element;
use crate::models::r5::Extension::Extension;
use crate::models::r5::Quantity::Quantity;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A report of inventory or stock items.

#[derive(Debug)]
pub struct InventoryReport_Items<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl InventoryReport_Items<'_> {
    pub fn new(value: &Value) -> InventoryReport_Items {
        InventoryReport_Items {
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

    /// Extensions for lot
    pub fn _lot(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_lot") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for manufacturingDate
    pub fn _manufacturing_date(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_manufacturingDate") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for serial
    pub fn _serial(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_serial") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The category of the item or items.
    pub fn category(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("category") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The expiry date of the item or items.
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

    /// The code or reference to the item type.
    pub fn item(&self) -> CodeableReference {
        CodeableReference {
            value: Cow::Borrowed(&self.value["item"]),
        }
    }

    /// The lot number of the item or items.
    pub fn lot(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("lot") {
            return Some(string);
        }
        return None;
    }

    /// The manufacturingDate of the item or items.
    pub fn manufacturing_date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("manufacturingDate") {
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

    /// The quantity of the item or items.
    pub fn quantity(&self) -> Quantity {
        Quantity {
            value: Cow::Borrowed(&self.value["quantity"]),
        }
    }

    /// The serial number of the item.
    pub fn serial(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("serial") {
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
        if let Some(_val) = self._lot() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._manufacturing_date() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._serial() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.category() {
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
        if !self.item().validate() {
            return false;
        }
        if let Some(_val) = self.lot() {}
        if let Some(_val) = self.manufacturing_date() {}
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if !self.quantity().validate() {
            return false;
        }
        if let Some(_val) = self.serial() {}
        return true;
    }
}

#[derive(Debug)]
pub struct InventoryReport_ItemsBuilder {
    pub(crate) value: Value,
}

impl InventoryReport_ItemsBuilder {
    pub fn build(&self) -> InventoryReport_Items {
        InventoryReport_Items {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: InventoryReport_Items) -> InventoryReport_ItemsBuilder {
        InventoryReport_ItemsBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(item: CodeableReference, quantity: Quantity) -> InventoryReport_ItemsBuilder {
        let mut __value: Value = json!({});
        __value["item"] = json!(item.value);
        __value["quantity"] = json!(quantity.value);
        return InventoryReport_ItemsBuilder { value: __value };
    }

    pub fn _expiry<'a>(&'a mut self, val: Element) -> &'a mut InventoryReport_ItemsBuilder {
        self.value["_expiry"] = json!(val.value);
        return self;
    }

    pub fn _lot<'a>(&'a mut self, val: Element) -> &'a mut InventoryReport_ItemsBuilder {
        self.value["_lot"] = json!(val.value);
        return self;
    }

    pub fn _manufacturing_date<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut InventoryReport_ItemsBuilder {
        self.value["_manufacturingDate"] = json!(val.value);
        return self;
    }

    pub fn _serial<'a>(&'a mut self, val: Element) -> &'a mut InventoryReport_ItemsBuilder {
        self.value["_serial"] = json!(val.value);
        return self;
    }

    pub fn category<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut InventoryReport_ItemsBuilder {
        self.value["category"] = json!(val.value);
        return self;
    }

    pub fn expiry<'a>(&'a mut self, val: &str) -> &'a mut InventoryReport_ItemsBuilder {
        self.value["expiry"] = json!(val);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut InventoryReport_ItemsBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut InventoryReport_ItemsBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn lot<'a>(&'a mut self, val: &str) -> &'a mut InventoryReport_ItemsBuilder {
        self.value["lot"] = json!(val);
        return self;
    }

    pub fn manufacturing_date<'a>(&'a mut self, val: &str) -> &'a mut InventoryReport_ItemsBuilder {
        self.value["manufacturingDate"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut InventoryReport_ItemsBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn serial<'a>(&'a mut self, val: &str) -> &'a mut InventoryReport_ItemsBuilder {
        self.value["serial"] = json!(val);
        return self;
    }
}
