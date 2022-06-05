#![allow(unused_imports, non_camel_case_types)]

use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Base definition for all types defined in FHIR type system.

#[derive(Debug)]
pub struct Base<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Base<'_> {
    pub fn new(value: &Value) -> Base {
        Base {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    pub fn validate(&self) -> bool {
        return true;
    }
}

#[derive(Debug)]
pub struct BaseBuilder {
    pub(crate) value: Value,
}

impl BaseBuilder {
    pub fn build(&self) -> Base {
        Base {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Base) -> BaseBuilder {
        BaseBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> BaseBuilder {
        let mut __value: Value = json!({});
        return BaseBuilder { value: __value };
    }
}
