#![allow(unused_imports, non_camel_case_types)]

use crate::models::r5::Annotation::Annotation;
use crate::models::r5::CodeableConcept::CodeableConcept;
use crate::models::r5::Element::Element;
use crate::models::r5::Evidence_AttributeEstimate::Evidence_AttributeEstimate;
use crate::models::r5::Evidence_ModelCharacteristic::Evidence_ModelCharacteristic;
use crate::models::r5::Evidence_SampleSize::Evidence_SampleSize;
use crate::models::r5::Extension::Extension;
use crate::models::r5::Quantity::Quantity;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// The Evidence Resource provides a machine-interpretable expression of an
/// evidence concept including the evidence variables (e.g., population, exposures/
/// interventions, comparators, outcomes, measured variables, confounding variables),
/// the statistics, and the certainty of this evidence.

#[derive(Debug)]
pub struct Evidence_Statistic<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Evidence_Statistic<'_> {
    pub fn new(value: &Value) -> Evidence_Statistic {
        Evidence_Statistic {
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

    /// Extensions for numberAffected
    pub fn _number_affected(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_numberAffected") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for numberOfEvents
    pub fn _number_of_events(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_numberOfEvents") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A statistical attribute of the statistic such as a measure of heterogeneity.
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

    /// When the measured variable is handled categorically, the category element is used
    /// to define which category the statistic is reporting.
    pub fn category(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("category") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A description of the content value of the statistic.
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

    /// Unique id for the element within a resource (for internal references). This may be
    /// any string value that does not contain spaces.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
        }
        return None;
    }

    /// A component of the method to generate the statistic.
    pub fn model_characteristic(&self) -> Option<Vec<Evidence_ModelCharacteristic>> {
        if let Some(Value::Array(val)) = self.value.get("modelCharacteristic") {
            return Some(
                val.into_iter()
                    .map(|e| Evidence_ModelCharacteristic {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
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

    /// The number of participants affected where the unit of analysis is the same as
    /// sampleSize.knownDataCount and sampleSize.numberOfParticipants.
    pub fn number_affected(&self) -> Option<u64> {
        if let Some(val) = self.value.get("numberAffected") {
            return Some(val.as_u64().unwrap());
        }
        return None;
    }

    /// The number of events associated with the statistic, where the unit of
    /// analysis is different from numberAffected, sampleSize.knownDataCount and
    /// sampleSize.numberOfParticipants.
    pub fn number_of_events(&self) -> Option<u64> {
        if let Some(val) = self.value.get("numberOfEvents") {
            return Some(val.as_u64().unwrap());
        }
        return None;
    }

    /// Statistic value.
    pub fn quantity(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("quantity") {
            return Some(Quantity {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Number of samples in the statistic.
    pub fn sample_size(&self) -> Option<Evidence_SampleSize> {
        if let Some(val) = self.value.get("sampleSize") {
            return Some(Evidence_SampleSize {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Type of statistic, e.g., relative risk.
    pub fn statistic_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("statisticType") {
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
        if let Some(_val) = self._number_affected() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._number_of_events() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.attribute_estimate() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.category() {
            if !_val.validate() {
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
        if let Some(_val) = self.model_characteristic() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
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
        if let Some(_val) = self.number_affected() {}
        if let Some(_val) = self.number_of_events() {}
        if let Some(_val) = self.quantity() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.sample_size() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.statistic_type() {
            if !_val.validate() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct Evidence_StatisticBuilder {
    pub(crate) value: Value,
}

impl Evidence_StatisticBuilder {
    pub fn build(&self) -> Evidence_Statistic {
        Evidence_Statistic {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Evidence_Statistic) -> Evidence_StatisticBuilder {
        Evidence_StatisticBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> Evidence_StatisticBuilder {
        let mut __value: Value = json!({});
        return Evidence_StatisticBuilder { value: __value };
    }

    pub fn _description<'a>(&'a mut self, val: Element) -> &'a mut Evidence_StatisticBuilder {
        self.value["_description"] = json!(val.value);
        return self;
    }

    pub fn _number_affected<'a>(&'a mut self, val: Element) -> &'a mut Evidence_StatisticBuilder {
        self.value["_numberAffected"] = json!(val.value);
        return self;
    }

    pub fn _number_of_events<'a>(&'a mut self, val: Element) -> &'a mut Evidence_StatisticBuilder {
        self.value["_numberOfEvents"] = json!(val.value);
        return self;
    }

    pub fn attribute_estimate<'a>(
        &'a mut self,
        val: Vec<Evidence_AttributeEstimate>,
    ) -> &'a mut Evidence_StatisticBuilder {
        self.value["attributeEstimate"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn category<'a>(&'a mut self, val: CodeableConcept) -> &'a mut Evidence_StatisticBuilder {
        self.value["category"] = json!(val.value);
        return self;
    }

    pub fn description<'a>(&'a mut self, val: &str) -> &'a mut Evidence_StatisticBuilder {
        self.value["description"] = json!(val);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut Evidence_StatisticBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut Evidence_StatisticBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn model_characteristic<'a>(
        &'a mut self,
        val: Vec<Evidence_ModelCharacteristic>,
    ) -> &'a mut Evidence_StatisticBuilder {
        self.value["modelCharacteristic"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut Evidence_StatisticBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn note<'a>(&'a mut self, val: Vec<Annotation>) -> &'a mut Evidence_StatisticBuilder {
        self.value["note"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn number_affected<'a>(&'a mut self, val: u64) -> &'a mut Evidence_StatisticBuilder {
        self.value["numberAffected"] = json!(val);
        return self;
    }

    pub fn number_of_events<'a>(&'a mut self, val: u64) -> &'a mut Evidence_StatisticBuilder {
        self.value["numberOfEvents"] = json!(val);
        return self;
    }

    pub fn quantity<'a>(&'a mut self, val: Quantity) -> &'a mut Evidence_StatisticBuilder {
        self.value["quantity"] = json!(val.value);
        return self;
    }

    pub fn sample_size<'a>(
        &'a mut self,
        val: Evidence_SampleSize,
    ) -> &'a mut Evidence_StatisticBuilder {
        self.value["sampleSize"] = json!(val.value);
        return self;
    }

    pub fn statistic_type<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut Evidence_StatisticBuilder {
        self.value["statisticType"] = json!(val.value);
        return self;
    }
}
