#![allow(unused_imports, non_camel_case_types)]

use crate::models::r4b::CodeableConcept::CodeableConcept;
use crate::models::r4b::Evidence_AttributeEstimate::Evidence_AttributeEstimate;
use crate::models::r4b::Evidence_Variable::Evidence_Variable;
use crate::models::r4b::Extension::Extension;
use crate::models::r4b::Quantity::Quantity;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// The Evidence Resource provides a machine-interpretable expression of an evidence
/// concept including the evidence variables (eg population, exposures/interventions,
/// comparators, outcomes, measured variables, confounding variables), the statistics,
/// and the certainty of this evidence.

#[derive(Debug)]
pub struct Evidence_ModelCharacteristic<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Evidence_ModelCharacteristic<'_> {
    pub fn new(value: &Value) -> Evidence_ModelCharacteristic {
        Evidence_ModelCharacteristic {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// An attribute of the statistic used as a model characteristic.
    pub fn attribute_estimate(&self) -> Option<Vec<Evidence_AttributeEstimate>> {
        if let Some(Value::Array(val)) = self.value.get("attributeEstimate") {
            return Some(
                val.into_iter()
                    .map(|e| Evidence_AttributeEstimate {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Description of a component of the method to generate the statistic.
    pub fn code(&self) -> CodeableConcept {
        CodeableConcept {
            value: Cow::Borrowed(&self.value["code"]),
        }
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

    /// Further specification of the quantified value of the component of the method to
    /// generate the statistic.
    pub fn value(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("value") {
            return Some(Quantity {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A variable adjusted for in the adjusted analysis.
    pub fn variable(&self) -> Option<Vec<Evidence_Variable>> {
        if let Some(Value::Array(val)) = self.value.get("variable") {
            return Some(
                val.into_iter()
                    .map(|e| Evidence_Variable {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.attribute_estimate() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if !self.code().validate() {
            return false;
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
        if let Some(_val) = self.value() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.variable() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct Evidence_ModelCharacteristicBuilder {
    pub(crate) value: Value,
}

impl Evidence_ModelCharacteristicBuilder {
    pub fn build(&self) -> Evidence_ModelCharacteristic {
        Evidence_ModelCharacteristic {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Evidence_ModelCharacteristic) -> Evidence_ModelCharacteristicBuilder {
        Evidence_ModelCharacteristicBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(code: CodeableConcept) -> Evidence_ModelCharacteristicBuilder {
        let mut __value: Value = json!({});
        __value["code"] = json!(code.value);
        return Evidence_ModelCharacteristicBuilder { value: __value };
    }

    pub fn attribute_estimate<'a>(
        &'a mut self,
        val: Vec<Evidence_AttributeEstimate>,
    ) -> &'a mut Evidence_ModelCharacteristicBuilder {
        self.value["attributeEstimate"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut Evidence_ModelCharacteristicBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut Evidence_ModelCharacteristicBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut Evidence_ModelCharacteristicBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn value<'a>(&'a mut self, val: Quantity) -> &'a mut Evidence_ModelCharacteristicBuilder {
        self.value["value"] = json!(val.value);
        return self;
    }

    pub fn variable<'a>(
        &'a mut self,
        val: Vec<Evidence_Variable>,
    ) -> &'a mut Evidence_ModelCharacteristicBuilder {
        self.value["variable"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }
}
