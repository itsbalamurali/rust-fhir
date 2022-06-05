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
pub struct DeviceDefinition_MarketDistribution<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl DeviceDefinition_MarketDistribution<'_> {
    pub fn new(value: &Value) -> DeviceDefinition_MarketDistribution {
        DeviceDefinition_MarketDistribution {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for subJurisdiction
    pub fn _sub_jurisdiction(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_subJurisdiction") {
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

    /// Begin and end dates for the commercial distribution of the device.
    pub fn market_period(&self) -> Period {
        Period {
            value: Cow::Borrowed(&self.value["marketPeriod"]),
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

    /// National state or territory to which the marketDistribution recers, typically
    /// where the device is commercialized.
    pub fn sub_jurisdiction(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("subJurisdiction") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._sub_jurisdiction() {
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
        if !self.market_period().validate() {
            return false;
        }
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.sub_jurisdiction() {}
        return true;
    }
}

#[derive(Debug)]
pub struct DeviceDefinition_MarketDistributionBuilder {
    pub(crate) value: Value,
}

impl DeviceDefinition_MarketDistributionBuilder {
    pub fn build(&self) -> DeviceDefinition_MarketDistribution {
        DeviceDefinition_MarketDistribution {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(
        existing: DeviceDefinition_MarketDistribution,
    ) -> DeviceDefinition_MarketDistributionBuilder {
        DeviceDefinition_MarketDistributionBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(market_period: Period) -> DeviceDefinition_MarketDistributionBuilder {
        let mut __value: Value = json!({});
        __value["marketPeriod"] = json!(market_period.value);
        return DeviceDefinition_MarketDistributionBuilder { value: __value };
    }

    pub fn _sub_jurisdiction<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut DeviceDefinition_MarketDistributionBuilder {
        self.value["_subJurisdiction"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut DeviceDefinition_MarketDistributionBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut DeviceDefinition_MarketDistributionBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut DeviceDefinition_MarketDistributionBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn sub_jurisdiction<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut DeviceDefinition_MarketDistributionBuilder {
        self.value["subJurisdiction"] = json!(val);
        return self;
    }
}
