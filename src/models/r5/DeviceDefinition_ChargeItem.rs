#![allow(unused_imports, non_camel_case_types)]

use crate::models::r5::CodeableReference::CodeableReference;
use crate::models::r5::Extension::Extension;
use crate::models::r5::Period::Period;
use crate::models::r5::Quantity::Quantity;
use crate::models::r5::UsageContext::UsageContext;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// The characteristics, operational status and capabilities of a medical-related
/// component of a medical device.

#[derive(Debug)]
pub struct DeviceDefinition_ChargeItem<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl DeviceDefinition_ChargeItem<'_> {
    pub fn new(value: &Value) -> DeviceDefinition_ChargeItem {
        DeviceDefinition_ChargeItem {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// The code or reference for the charge item.
    pub fn charge_item_code(&self) -> CodeableReference {
        CodeableReference {
            value: Cow::Borrowed(&self.value["chargeItemCode"]),
        }
    }

    /// Coefficient applicable to the billing code.
    pub fn count(&self) -> Quantity {
        Quantity {
            value: Cow::Borrowed(&self.value["count"]),
        }
    }

    /// A specific time period in which this charge item applies.
    pub fn effective_period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("effectivePeriod") {
            return Some(Period {
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

    /// The context to which this charge item applies.
    pub fn use_context(&self) -> Option<Vec<UsageContext>> {
        if let Some(Value::Array(val)) = self.value.get("useContext") {
            return Some(
                val.into_iter()
                    .map(|e| UsageContext {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if !self.charge_item_code().validate() {
            return false;
        }
        if !self.count().validate() {
            return false;
        }
        if let Some(_val) = self.effective_period() {
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
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.use_context() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct DeviceDefinition_ChargeItemBuilder {
    pub(crate) value: Value,
}

impl DeviceDefinition_ChargeItemBuilder {
    pub fn build(&self) -> DeviceDefinition_ChargeItem {
        DeviceDefinition_ChargeItem {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: DeviceDefinition_ChargeItem) -> DeviceDefinition_ChargeItemBuilder {
        DeviceDefinition_ChargeItemBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(
        charge_item_code: CodeableReference,
        count: Quantity,
    ) -> DeviceDefinition_ChargeItemBuilder {
        let mut __value: Value = json!({});
        __value["chargeItemCode"] = json!(charge_item_code.value);
        __value["count"] = json!(count.value);
        return DeviceDefinition_ChargeItemBuilder { value: __value };
    }

    pub fn effective_period<'a>(
        &'a mut self,
        val: Period,
    ) -> &'a mut DeviceDefinition_ChargeItemBuilder {
        self.value["effectivePeriod"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut DeviceDefinition_ChargeItemBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut DeviceDefinition_ChargeItemBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut DeviceDefinition_ChargeItemBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn use_context<'a>(
        &'a mut self,
        val: Vec<UsageContext>,
    ) -> &'a mut DeviceDefinition_ChargeItemBuilder {
        self.value["useContext"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }
}
