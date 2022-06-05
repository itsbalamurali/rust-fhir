#![allow(unused_imports, non_camel_case_types)]

use crate::models::r5::Element::Element;
use crate::models::r5::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// The EvidenceVariable resource describes an element that knowledge (Evidence) is
/// about.

#[derive(Debug)]
pub struct EvidenceVariable_CharacteristicCombination<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl EvidenceVariable_CharacteristicCombination<'_> {
    pub fn new(value: &Value) -> EvidenceVariable_CharacteristicCombination {
        EvidenceVariable_CharacteristicCombination {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for code
    pub fn _code(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_code") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for threshold
    pub fn _threshold(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_threshold") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Used to specify if two or more characteristics are combined with OR or AND.
    pub fn code(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("code") {
            return Some(string);
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

    /// Provides the value of "n" when "at-least" or "at-most" codes are used.
    pub fn threshold(&self) -> Option<i64> {
        if let Some(val) = self.value.get("threshold") {
            return Some(val.as_i64().unwrap());
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._code() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._threshold() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.code() {}
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
        if let Some(_val) = self.threshold() {}
        return true;
    }
}

#[derive(Debug)]
pub struct EvidenceVariable_CharacteristicCombinationBuilder {
    pub(crate) value: Value,
}

impl EvidenceVariable_CharacteristicCombinationBuilder {
    pub fn build(&self) -> EvidenceVariable_CharacteristicCombination {
        EvidenceVariable_CharacteristicCombination {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(
        existing: EvidenceVariable_CharacteristicCombination,
    ) -> EvidenceVariable_CharacteristicCombinationBuilder {
        EvidenceVariable_CharacteristicCombinationBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> EvidenceVariable_CharacteristicCombinationBuilder {
        let mut __value: Value = json!({});
        return EvidenceVariable_CharacteristicCombinationBuilder { value: __value };
    }

    pub fn _code<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut EvidenceVariable_CharacteristicCombinationBuilder {
        self.value["_code"] = json!(val.value);
        return self;
    }

    pub fn _threshold<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut EvidenceVariable_CharacteristicCombinationBuilder {
        self.value["_threshold"] = json!(val.value);
        return self;
    }

    pub fn code<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut EvidenceVariable_CharacteristicCombinationBuilder {
        self.value["code"] = json!(val);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut EvidenceVariable_CharacteristicCombinationBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut EvidenceVariable_CharacteristicCombinationBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut EvidenceVariable_CharacteristicCombinationBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn threshold<'a>(
        &'a mut self,
        val: i64,
    ) -> &'a mut EvidenceVariable_CharacteristicCombinationBuilder {
        self.value["threshold"] = json!(val);
        return self;
    }
}
