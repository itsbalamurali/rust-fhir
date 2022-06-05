#![allow(unused_imports, non_camel_case_types)]

use crate::models::r5::Element::Element;
use crate::models::r5::Extension::Extension;
use crate::models::r5::Reference::Reference;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A definition of a condition and information relevant to managing it.

#[derive(Debug)]
pub struct ConditionDefinition_Questionnaire<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl ConditionDefinition_Questionnaire<'_> {
    pub fn new(value: &Value) -> ConditionDefinition_Questionnaire {
        ConditionDefinition_Questionnaire {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for purpose
    pub fn _purpose(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_purpose") {
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

    /// Use of the questionnaire.
    pub fn purpose(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("purpose") {
            return Some(string);
        }
        return None;
    }

    /// Specific Questionnaire.
    pub fn reference(&self) -> Reference {
        Reference {
            value: Cow::Borrowed(&self.value["reference"]),
        }
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._purpose() {
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
        if let Some(_val) = self.purpose() {}
        if !self.reference().validate() {
            return false;
        }
        return true;
    }
}

#[derive(Debug)]
pub struct ConditionDefinition_QuestionnaireBuilder {
    pub(crate) value: Value,
}

impl ConditionDefinition_QuestionnaireBuilder {
    pub fn build(&self) -> ConditionDefinition_Questionnaire {
        ConditionDefinition_Questionnaire {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(
        existing: ConditionDefinition_Questionnaire,
    ) -> ConditionDefinition_QuestionnaireBuilder {
        ConditionDefinition_QuestionnaireBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(reference: Reference) -> ConditionDefinition_QuestionnaireBuilder {
        let mut __value: Value = json!({});
        __value["reference"] = json!(reference.value);
        return ConditionDefinition_QuestionnaireBuilder { value: __value };
    }

    pub fn _purpose<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut ConditionDefinition_QuestionnaireBuilder {
        self.value["_purpose"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ConditionDefinition_QuestionnaireBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut ConditionDefinition_QuestionnaireBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ConditionDefinition_QuestionnaireBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn purpose<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut ConditionDefinition_QuestionnaireBuilder {
        self.value["purpose"] = json!(val);
        return self;
    }
}
