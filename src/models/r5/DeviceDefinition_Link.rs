#![allow(unused_imports, non_camel_case_types)]

use crate::models::r5::CodeableReference::CodeableReference;
use crate::models::r5::Coding::Coding;
use crate::models::r5::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// The characteristics, operational status and capabilities of a medical-related
/// component of a medical device.

#[derive(Debug)]
pub struct DeviceDefinition_Link<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl DeviceDefinition_Link<'_> {
    pub fn new(value: &Value) -> DeviceDefinition_Link {
        DeviceDefinition_Link {
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

    /// A reference to the linked device.
    pub fn related_device(&self) -> CodeableReference {
        CodeableReference {
            value: Cow::Borrowed(&self.value["relatedDevice"]),
        }
    }

    /// The type indicates the relationship of the related device to the device instance.
    pub fn relation(&self) -> Coding {
        Coding {
            value: Cow::Borrowed(&self.value["relation"]),
        }
    }

    pub fn validate(&self) -> bool {
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
        if !self.related_device().validate() {
            return false;
        }
        if !self.relation().validate() {
            return false;
        }
        return true;
    }
}

#[derive(Debug)]
pub struct DeviceDefinition_LinkBuilder {
    pub(crate) value: Value,
}

impl DeviceDefinition_LinkBuilder {
    pub fn build(&self) -> DeviceDefinition_Link {
        DeviceDefinition_Link {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: DeviceDefinition_Link) -> DeviceDefinition_LinkBuilder {
        DeviceDefinition_LinkBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(
        related_device: CodeableReference,
        relation: Coding,
    ) -> DeviceDefinition_LinkBuilder {
        let mut __value: Value = json!({});
        __value["relatedDevice"] = json!(related_device.value);
        __value["relation"] = json!(relation.value);
        return DeviceDefinition_LinkBuilder { value: __value };
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut DeviceDefinition_LinkBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut DeviceDefinition_LinkBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut DeviceDefinition_LinkBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }
}
