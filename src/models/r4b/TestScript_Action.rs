#![allow(unused_imports, non_camel_case_types)]

use crate::models::r4b::Extension::Extension;
use crate::models::r4b::TestScript_Assert::TestScript_Assert;
use crate::models::r4b::TestScript_Operation::TestScript_Operation;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A structured set of tests against a FHIR server or client implementation to
/// determine compliance against the FHIR specification.

#[derive(Debug)]
pub struct TestScript_Action<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl TestScript_Action<'_> {
    pub fn new(value: &Value) -> TestScript_Action {
        TestScript_Action {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Evaluates the results of previous operations to determine if the server under test
    /// behaves appropriately.
    pub fn assert(&self) -> Option<TestScript_Assert> {
        if let Some(val) = self.value.get("assert") {
            return Some(TestScript_Assert {
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

    /// Unique id for the element within a resource (for internal references). This may be
    /// any string value that does not contain spaces.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
        }
        return None;
    }

    /// May be used to represent additional information that is not part of the basic
    /// definition of the element and that modifies the understanding of the element
    /// in which it is contained and/or the understanding of the containing element's
    /// descendants. Usually modifier elements provide negation or qualification. To make
    /// the use of extensions safe and manageable, there is a strict set of governance
    /// applied to the definition and use of extensions. Though any implementer can define
    /// an extension, there is a set of requirements that SHALL be met as part of the
    /// definition of the extension. Applications processing a resource are required to
    /// check for modifier extensions.    Modifier extensions SHALL NOT change the meaning
    /// of any elements on Resource or DomainResource (including cannot change the meaning
    /// of modifierExtension itself).
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

    /// The operation to perform.
    pub fn operation(&self) -> Option<TestScript_Operation> {
        if let Some(val) = self.value.get("operation") {
            return Some(TestScript_Operation {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.assert() {
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
        if let Some(_val) = self.operation() {
            if !_val.validate() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct TestScript_ActionBuilder {
    pub(crate) value: Value,
}

impl TestScript_ActionBuilder {
    pub fn build(&self) -> TestScript_Action {
        TestScript_Action {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: TestScript_Action) -> TestScript_ActionBuilder {
        TestScript_ActionBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> TestScript_ActionBuilder {
        let mut __value: Value = json!({});
        return TestScript_ActionBuilder { value: __value };
    }

    pub fn assert<'a>(&'a mut self, val: TestScript_Assert) -> &'a mut TestScript_ActionBuilder {
        self.value["assert"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut TestScript_ActionBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut TestScript_ActionBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut TestScript_ActionBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn operation<'a>(
        &'a mut self,
        val: TestScript_Operation,
    ) -> &'a mut TestScript_ActionBuilder {
        self.value["operation"] = json!(val.value);
        return self;
    }
}
