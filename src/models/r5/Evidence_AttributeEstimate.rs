#![allow(unused_imports, non_camel_case_types)]

use crate::models::r5::Annotation::Annotation;
use crate::models::r5::CodeableConcept::CodeableConcept;
use crate::models::r5::Element::Element;
use crate::models::r5::Extension::Extension;
use crate::models::r5::Quantity::Quantity;
use crate::models::r5::Range::Range;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// The Evidence Resource provides a machine-interpretable expression of an evidence
/// concept including the evidence variables (e.g., population,
/// exposures/interventions, comparators, outcomes, measured variables, confounding
/// variables), the statistics, and the certainty of this evidence.

#[derive(Debug)]
pub struct Evidence_AttributeEstimate<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Evidence_AttributeEstimate<'_> {
    pub fn new(value: &Value) -> Evidence_AttributeEstimate {
        Evidence_AttributeEstimate {
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

    /// Extensions for level
    pub fn _level(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_level") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A nested attribute estimate; which is the attribute estimate of an attribute
    /// estimate.
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

    /// Human-readable summary of the estimate.
    pub fn description(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("description") {
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

    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
        }
        return None;
    }

    /// Use 95 for a 95% confidence interval.
    pub fn level(&self) -> Option<f64> {
        if let Some(val) = self.value.get("level") {
            return Some(val.as_f64().unwrap());
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

    /// Footnote or explanatory note about the estimate.
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

    /// The singular quantity of the attribute estimate, for attribute estimates
    /// represented as single values; also used to report unit of measure.
    pub fn quantity(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("quantity") {
            return Some(Quantity {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Lower bound of confidence interval.
    pub fn range(&self) -> Option<Range> {
        if let Some(val) = self.value.get("range") {
            return Some(Range {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The type of attribute estimate, e.g., confidence interval or p value.
    pub fn fhir_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("type") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._description() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._level() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.attribute_estimate() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.description() {}
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.level() {}
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
        if let Some(_val) = self.quantity() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.range() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.fhir_type() {
            if !_val.validate() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct Evidence_AttributeEstimateBuilder {
    pub(crate) value: Value,
}

impl Evidence_AttributeEstimateBuilder {
    pub fn build(&self) -> Evidence_AttributeEstimate {
        Evidence_AttributeEstimate {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Evidence_AttributeEstimate) -> Evidence_AttributeEstimateBuilder {
        Evidence_AttributeEstimateBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> Evidence_AttributeEstimateBuilder {
        let mut __value: Value = json!({});
        return Evidence_AttributeEstimateBuilder { value: __value };
    }

    pub fn _description<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut Evidence_AttributeEstimateBuilder {
        self.value["_description"] = json!(val.value);
        return self;
    }

    pub fn _level<'a>(&'a mut self, val: Element) -> &'a mut Evidence_AttributeEstimateBuilder {
        self.value["_level"] = json!(val.value);
        return self;
    }

    pub fn attribute_estimate<'a>(
        &'a mut self,
        val: Vec<Evidence_AttributeEstimate>,
    ) -> &'a mut Evidence_AttributeEstimateBuilder {
        self.value["attributeEstimate"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn description<'a>(&'a mut self, val: &str) -> &'a mut Evidence_AttributeEstimateBuilder {
        self.value["description"] = json!(val);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut Evidence_AttributeEstimateBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut Evidence_AttributeEstimateBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn level<'a>(&'a mut self, val: f64) -> &'a mut Evidence_AttributeEstimateBuilder {
        self.value["level"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut Evidence_AttributeEstimateBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn note<'a>(
        &'a mut self,
        val: Vec<Annotation>,
    ) -> &'a mut Evidence_AttributeEstimateBuilder {
        self.value["note"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn quantity<'a>(&'a mut self, val: Quantity) -> &'a mut Evidence_AttributeEstimateBuilder {
        self.value["quantity"] = json!(val.value);
        return self;
    }

    pub fn range<'a>(&'a mut self, val: Range) -> &'a mut Evidence_AttributeEstimateBuilder {
        self.value["range"] = json!(val.value);
        return self;
    }

    pub fn fhir_type<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut Evidence_AttributeEstimateBuilder {
        self.value["type"] = json!(val.value);
        return self;
    }
}
