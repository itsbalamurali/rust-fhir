#![allow(unused_imports, non_camel_case_types)]

use crate::models::r4b::CodeableConcept::CodeableConcept;
use crate::models::r4b::Element::Element;
use crate::models::r4b::Extension::Extension;
use crate::models::r4b::Quantity::Quantity;
use crate::models::r4b::Range::Range;
use crate::models::r4b::Reference::Reference;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// The Evidence Resource provides a machine-interpretable expression of an evidence
/// concept including the evidence variables (eg population,
/// exposures/interventions, comparators, outcomes, measured variables, confounding
/// variables), the statistics, and the certainty of this evidence.

#[derive(Debug)]
pub struct Evidence_Variable<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Evidence_Variable<'_> {
    pub fn new(value: &Value) -> Evidence_Variable {
        Evidence_Variable {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for handling
    pub fn _handling(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_handling") {
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

    /// How the variable is classified for use in adjusted analysis.
    pub fn handling(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("handling") {
            return Some(string);
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

    /// Description for grouping of ordinal or polychotomous variables.
    pub fn value_category(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("valueCategory") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Discrete value for grouping of ordinal or polychotomous variables.
    pub fn value_quantity(&self) -> Option<Vec<Quantity>> {
        if let Some(Value::Array(val)) = self.value.get("valueQuantity") {
            return Some(
                val.into_iter()
                    .map(|e| Quantity {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Range of values for grouping of ordinal or polychotomous variables.
    pub fn value_range(&self) -> Option<Vec<Range>> {
        if let Some(Value::Array(val)) = self.value.get("valueRange") {
            return Some(
                val.into_iter()
                    .map(|e| Range {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Description of the variable.
    pub fn variable_definition(&self) -> Reference {
        Reference {
            value: Cow::Borrowed(&self.value["variableDefinition"]),
        }
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._handling() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.handling() {}
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.value_category() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.value_quantity() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.value_range() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if !self.variable_definition().validate() {
            return false;
        }
        return true;
    }
}

#[derive(Debug)]
pub struct Evidence_VariableBuilder {
    pub(crate) value: Value,
}

impl Evidence_VariableBuilder {
    pub fn build(&self) -> Evidence_Variable {
        Evidence_Variable {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Evidence_Variable) -> Evidence_VariableBuilder {
        Evidence_VariableBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(variable_definition: Reference) -> Evidence_VariableBuilder {
        let mut __value: Value = json!({});
        __value["variableDefinition"] = json!(variable_definition.value);
        return Evidence_VariableBuilder { value: __value };
    }

    pub fn _handling<'a>(&'a mut self, val: Element) -> &'a mut Evidence_VariableBuilder {
        self.value["_handling"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut Evidence_VariableBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn handling<'a>(&'a mut self, val: &str) -> &'a mut Evidence_VariableBuilder {
        self.value["handling"] = json!(val);
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut Evidence_VariableBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut Evidence_VariableBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn value_category<'a>(
        &'a mut self,
        val: Vec<CodeableConcept>,
    ) -> &'a mut Evidence_VariableBuilder {
        self.value["valueCategory"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn value_quantity<'a>(
        &'a mut self,
        val: Vec<Quantity>,
    ) -> &'a mut Evidence_VariableBuilder {
        self.value["valueQuantity"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn value_range<'a>(&'a mut self, val: Vec<Range>) -> &'a mut Evidence_VariableBuilder {
        self.value["valueRange"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }
}
