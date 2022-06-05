#![allow(unused_imports, non_camel_case_types)]

use crate::models::r5::CodeableReference::CodeableReference;
use crate::models::r5::Element::Element;
use crate::models::r5::Extension::Extension;
use crate::models::r5::RelatedArtifact::RelatedArtifact;
use crate::models::r5::UsageContext::UsageContext;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// The characteristics, operational status and capabilities of a medical-related
/// component of a medical device.

#[derive(Debug)]
pub struct DeviceDefinition_Guideline<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl DeviceDefinition_Guideline<'_> {
    pub fn new(value: &Value) -> DeviceDefinition_Guideline {
        DeviceDefinition_Guideline {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for intendedUse
    pub fn _intended_use(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_intendedUse") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for usageInstruction
    pub fn _usage_instruction(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_usageInstruction") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A specific situation when a device should not be used because it may cause harm.
    pub fn contraindication(&self) -> Option<Vec<CodeableReference>> {
        if let Some(Value::Array(val)) = self.value.get("contraindication") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableReference {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
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

    /// A clinical condition for which the device was designed to be used.
    pub fn indication(&self) -> Option<Vec<CodeableReference>> {
        if let Some(Value::Array(val)) = self.value.get("indication") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableReference {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A description of the general purpose or medical use of the device or its
    /// function.
    pub fn intended_use(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("intendedUse") {
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

    /// A source of information or reference for this guideline.
    pub fn related_artifact(&self) -> Option<Vec<RelatedArtifact>> {
        if let Some(Value::Array(val)) = self.value.get("relatedArtifact") {
            return Some(
                val.into_iter()
                    .map(|e| RelatedArtifact {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Detailed written and visual directions for the user on how to use the device.
    pub fn usage_instruction(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("usageInstruction") {
            return Some(string);
        }
        return None;
    }

    /// The circumstances that form the setting for using the device.
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

    /// Specific hazard alert information that a user needs to know before using the
    /// device.
    pub fn warning(&self) -> Option<Vec<CodeableReference>> {
        if let Some(Value::Array(val)) = self.value.get("warning") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableReference {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._intended_use() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._usage_instruction() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.contraindication() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.indication() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.intended_use() {}
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.related_artifact() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.usage_instruction() {}
        if let Some(_val) = self.use_context() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.warning() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct DeviceDefinition_GuidelineBuilder {
    pub(crate) value: Value,
}

impl DeviceDefinition_GuidelineBuilder {
    pub fn build(&self) -> DeviceDefinition_Guideline {
        DeviceDefinition_Guideline {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: DeviceDefinition_Guideline) -> DeviceDefinition_GuidelineBuilder {
        DeviceDefinition_GuidelineBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> DeviceDefinition_GuidelineBuilder {
        let mut __value: Value = json!({});
        return DeviceDefinition_GuidelineBuilder { value: __value };
    }

    pub fn _intended_use<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut DeviceDefinition_GuidelineBuilder {
        self.value["_intendedUse"] = json!(val.value);
        return self;
    }

    pub fn _usage_instruction<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut DeviceDefinition_GuidelineBuilder {
        self.value["_usageInstruction"] = json!(val.value);
        return self;
    }

    pub fn contraindication<'a>(
        &'a mut self,
        val: Vec<CodeableReference>,
    ) -> &'a mut DeviceDefinition_GuidelineBuilder {
        self.value["contraindication"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut DeviceDefinition_GuidelineBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut DeviceDefinition_GuidelineBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn indication<'a>(
        &'a mut self,
        val: Vec<CodeableReference>,
    ) -> &'a mut DeviceDefinition_GuidelineBuilder {
        self.value["indication"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn intended_use<'a>(&'a mut self, val: &str) -> &'a mut DeviceDefinition_GuidelineBuilder {
        self.value["intendedUse"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut DeviceDefinition_GuidelineBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn related_artifact<'a>(
        &'a mut self,
        val: Vec<RelatedArtifact>,
    ) -> &'a mut DeviceDefinition_GuidelineBuilder {
        self.value["relatedArtifact"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn usage_instruction<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut DeviceDefinition_GuidelineBuilder {
        self.value["usageInstruction"] = json!(val);
        return self;
    }

    pub fn use_context<'a>(
        &'a mut self,
        val: Vec<UsageContext>,
    ) -> &'a mut DeviceDefinition_GuidelineBuilder {
        self.value["useContext"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn warning<'a>(
        &'a mut self,
        val: Vec<CodeableReference>,
    ) -> &'a mut DeviceDefinition_GuidelineBuilder {
        self.value["warning"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }
}
