#![allow(unused_imports, non_camel_case_types)]

use crate::models::r4b::Extension::Extension;
use crate::models::r4b::Quantity::Quantity;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A range of ratios expressed as a low and high numerator and a denominator.

#[derive(Debug)]
pub struct RatioRange<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl RatioRange<'_> {
    pub fn new(value: &Value) -> RatioRange {
        RatioRange {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// The value of the denominator.
    pub fn denominator(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("denominator") {
            return Some(Quantity {
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

    /// The value of the high limit numerator.
    pub fn high_numerator(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("highNumerator") {
            return Some(Quantity {
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

    /// The value of the low limit numerator.
    pub fn low_numerator(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("lowNumerator") {
            return Some(Quantity {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.denominator() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.high_numerator() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.low_numerator() {
            if !_val.validate() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct RatioRangeBuilder {
    pub(crate) value: Value,
}

impl RatioRangeBuilder {
    pub fn build(&self) -> RatioRange {
        RatioRange {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: RatioRange) -> RatioRangeBuilder {
        RatioRangeBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> RatioRangeBuilder {
        let mut __value: Value = json!({});
        return RatioRangeBuilder { value: __value };
    }

    pub fn denominator<'a>(&'a mut self, val: Quantity) -> &'a mut RatioRangeBuilder {
        self.value["denominator"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut RatioRangeBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn high_numerator<'a>(&'a mut self, val: Quantity) -> &'a mut RatioRangeBuilder {
        self.value["highNumerator"] = json!(val.value);
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut RatioRangeBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn low_numerator<'a>(&'a mut self, val: Quantity) -> &'a mut RatioRangeBuilder {
        self.value["lowNumerator"] = json!(val.value);
        return self;
    }
}
