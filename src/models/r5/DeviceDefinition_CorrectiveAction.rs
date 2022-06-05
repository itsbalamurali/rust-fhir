#![allow(unused_imports, non_camel_case_types)]

use crate::models::r5::Element::Element;
use crate::models::r5::Extension::Extension;
use crate::models::r5::Period::Period;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// The characteristics, operational status and capabilities of a medical-related
/// component of a medical device.

#[derive(Debug)]
pub struct DeviceDefinition_CorrectiveAction<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl DeviceDefinition_CorrectiveAction<'_> {
    pub fn new(value: &Value) -> DeviceDefinition_CorrectiveAction {
        DeviceDefinition_CorrectiveAction {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for recall
    pub fn _recall(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_recall") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for scope
    pub fn _scope(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_scope") {
            return Some(Element {
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

    /// Start and end dates of the  corrective action.
    pub fn period(&self) -> Period {
        Period {
            value: Cow::Borrowed(&self.value["period"]),
        }
    }

    /// Whether the last corrective action known for this device was a recall.
    pub fn recall(&self) -> Option<bool> {
        if let Some(val) = self.value.get("recall") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    /// The scope of the corrective action - whether the action targeted all units of a
    /// given device model, or only a specific set of batches identified by lot numbers,
    /// or individually identified devices identified by the serial name.
    pub fn scope(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("scope") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._recall() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._scope() {
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
        if !self.period().validate() {
            return false;
        }
        if let Some(_val) = self.recall() {}
        if let Some(_val) = self.scope() {}
        return true;
    }
}

#[derive(Debug)]
pub struct DeviceDefinition_CorrectiveActionBuilder {
    pub(crate) value: Value,
}

impl DeviceDefinition_CorrectiveActionBuilder {
    pub fn build(&self) -> DeviceDefinition_CorrectiveAction {
        DeviceDefinition_CorrectiveAction {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(
        existing: DeviceDefinition_CorrectiveAction,
    ) -> DeviceDefinition_CorrectiveActionBuilder {
        DeviceDefinition_CorrectiveActionBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(period: Period) -> DeviceDefinition_CorrectiveActionBuilder {
        let mut __value: Value = json!({});
        __value["period"] = json!(period.value);
        return DeviceDefinition_CorrectiveActionBuilder { value: __value };
    }

    pub fn _recall<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut DeviceDefinition_CorrectiveActionBuilder {
        self.value["_recall"] = json!(val.value);
        return self;
    }

    pub fn _scope<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut DeviceDefinition_CorrectiveActionBuilder {
        self.value["_scope"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut DeviceDefinition_CorrectiveActionBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut DeviceDefinition_CorrectiveActionBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut DeviceDefinition_CorrectiveActionBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn recall<'a>(&'a mut self, val: bool) -> &'a mut DeviceDefinition_CorrectiveActionBuilder {
        self.value["recall"] = json!(val);
        return self;
    }

    pub fn scope<'a>(&'a mut self, val: &str) -> &'a mut DeviceDefinition_CorrectiveActionBuilder {
        self.value["scope"] = json!(val);
        return self;
    }
}
