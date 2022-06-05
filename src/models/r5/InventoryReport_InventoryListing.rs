#![allow(unused_imports, non_camel_case_types)]

use crate::models::r5::CodeableConcept::CodeableConcept;
use crate::models::r5::Element::Element;
use crate::models::r5::Extension::Extension;
use crate::models::r5::InventoryReport_Items::InventoryReport_Items;
use crate::models::r5::Reference::Reference;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A report of inventory or stock items.

#[derive(Debug)]
pub struct InventoryReport_InventoryListing<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl InventoryReport_InventoryListing<'_> {
    pub fn new(value: &Value) -> InventoryReport_InventoryListing {
        InventoryReport_InventoryListing {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for countingDateTime
    pub fn _counting_date_time(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_countingDateTime") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The date and time when the items were counted.
    pub fn counting_date_time(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("countingDateTime") {
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

    /// The status of the items.
    pub fn item_status(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("itemStatus") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The item or items in this listing.
    pub fn items(&self) -> Option<Vec<InventoryReport_Items>> {
        if let Some(Value::Array(val)) = self.value.get("items") {
            return Some(
                val.into_iter()
                    .map(|e| InventoryReport_Items {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Location of the inventory items.
    pub fn location(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("location") {
            return Some(Reference {
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
        if let Some(_val) = self._counting_date_time() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.counting_date_time() {}
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.item_status() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.items() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.location() {
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
pub struct InventoryReport_InventoryListingBuilder {
    pub(crate) value: Value,
}

impl InventoryReport_InventoryListingBuilder {
    pub fn build(&self) -> InventoryReport_InventoryListing {
        InventoryReport_InventoryListing {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(
        existing: InventoryReport_InventoryListing,
    ) -> InventoryReport_InventoryListingBuilder {
        InventoryReport_InventoryListingBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> InventoryReport_InventoryListingBuilder {
        let mut __value: Value = json!({});
        return InventoryReport_InventoryListingBuilder { value: __value };
    }

    pub fn _counting_date_time<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut InventoryReport_InventoryListingBuilder {
        self.value["_countingDateTime"] = json!(val.value);
        return self;
    }

    pub fn counting_date_time<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut InventoryReport_InventoryListingBuilder {
        self.value["countingDateTime"] = json!(val);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut InventoryReport_InventoryListingBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut InventoryReport_InventoryListingBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn item_status<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut InventoryReport_InventoryListingBuilder {
        self.value["itemStatus"] = json!(val.value);
        return self;
    }

    pub fn items<'a>(
        &'a mut self,
        val: Vec<InventoryReport_Items>,
    ) -> &'a mut InventoryReport_InventoryListingBuilder {
        self.value["items"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn location<'a>(
        &'a mut self,
        val: Reference,
    ) -> &'a mut InventoryReport_InventoryListingBuilder {
        self.value["location"] = json!(val.value);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut InventoryReport_InventoryListingBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }
}
