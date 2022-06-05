#![allow(unused_imports, non_camel_case_types)]

use crate::models::r5::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// The base class for all re-useable types defined as part of the FHIR
/// Specification.

#[derive(Debug)]
pub struct DataType<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl DataType<'_> {
    pub fn new(value: &Value) -> DataType {
        DataType {
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

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        return true;
    }
}

#[derive(Debug)]
pub struct DataTypeBuilder {
    pub(crate) value: Value,
}

impl DataTypeBuilder {
    pub fn build(&self) -> DataType {
        DataType {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: DataType) -> DataTypeBuilder {
        DataTypeBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> DataTypeBuilder {
        let mut __value: Value = json!({});
        return DataTypeBuilder { value: __value };
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut DataTypeBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut DataTypeBuilder {
        self.value["id"] = json!(val);
        return self;
    }
}
