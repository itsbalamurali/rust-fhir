#![allow(unused_imports, non_camel_case_types)]

use crate::models::r4b::Annotation::Annotation;
use crate::models::r4b::CodeableConcept::CodeableConcept;
use crate::models::r4b::Element::Element;
use crate::models::r4b::Extension::Extension;
use crate::models::r4b::Reference::Reference;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// The Evidence Resource provides a machine-interpretable expression of an evidence
/// concept including the evidence variables (eg population,
/// exposures/interventions, comparators, outcomes, measured variables, confounding
/// variables), the statistics, and the certainty of this evidence.

#[derive(Debug)]
pub struct Evidence_VariableDefinition<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Evidence_VariableDefinition<'_> {
    pub fn new(value: &Value) -> Evidence_VariableDefinition {
        Evidence_VariableDefinition {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for description
    pub fn _description(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_description") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A text description or summary of the variable.
    pub fn description(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("description") {
            return Some(string);
        }
        return None;
    }

    /// Indication of quality of match between intended variable to actual variable.
    pub fn directness_match(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("directnessMatch") {
            return Some(CodeableConcept {
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

    /// Definition of the intended variable related to the Evidence.
    pub fn intended(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("intended") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
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

    /// Footnotes and/or explanatory notes.
    pub fn note(&self) -> Option<Vec<Annotation>> {
        if let Some(Value::Array(val)) = self.value.get("note") {
            return Some(
                val.into_iter()
                    .map(|e| Annotation {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Definition of the actual variable related to the statistic(s).
    pub fn observed(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("observed") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// population | subpopulation | exposure | referenceExposure | measuredVariable |
    /// confounder.
    pub fn variable_role(&self) -> CodeableConcept {
        CodeableConcept {
            value: Cow::Borrowed(&self.value["variableRole"]),
        }
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._description() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.description() {}
        if let Some(_val) = self.directness_match() {
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
        if let Some(_val) = self.intended() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.note() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.observed() {
            if !_val.validate() {
                return false;
            }
        }
        if !self.variable_role().validate() {
            return false;
        }
        return true;
    }
}

#[derive(Debug)]
pub struct Evidence_VariableDefinitionBuilder {
    pub(crate) value: Value,
}

impl Evidence_VariableDefinitionBuilder {
    pub fn build(&self) -> Evidence_VariableDefinition {
        Evidence_VariableDefinition {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Evidence_VariableDefinition) -> Evidence_VariableDefinitionBuilder {
        Evidence_VariableDefinitionBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(variable_role: CodeableConcept) -> Evidence_VariableDefinitionBuilder {
        let mut __value: Value = json!({});
        __value["variableRole"] = json!(variable_role.value);
        return Evidence_VariableDefinitionBuilder { value: __value };
    }

    pub fn _description<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut Evidence_VariableDefinitionBuilder {
        self.value["_description"] = json!(val.value);
        return self;
    }

    pub fn description<'a>(&'a mut self, val: &str) -> &'a mut Evidence_VariableDefinitionBuilder {
        self.value["description"] = json!(val);
        return self;
    }

    pub fn directness_match<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut Evidence_VariableDefinitionBuilder {
        self.value["directnessMatch"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut Evidence_VariableDefinitionBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut Evidence_VariableDefinitionBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn intended<'a>(
        &'a mut self,
        val: Reference,
    ) -> &'a mut Evidence_VariableDefinitionBuilder {
        self.value["intended"] = json!(val.value);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut Evidence_VariableDefinitionBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn note<'a>(
        &'a mut self,
        val: Vec<Annotation>,
    ) -> &'a mut Evidence_VariableDefinitionBuilder {
        self.value["note"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn observed<'a>(
        &'a mut self,
        val: Reference,
    ) -> &'a mut Evidence_VariableDefinitionBuilder {
        self.value["observed"] = json!(val.value);
        return self;
    }
}
