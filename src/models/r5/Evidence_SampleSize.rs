#![allow(unused_imports, non_camel_case_types)]

use crate::models::r5::Annotation::Annotation;
use crate::models::r5::Element::Element;
use crate::models::r5::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// The Evidence Resource provides a machine-interpretable expression of an
/// evidence concept including the evidence variables (e.g., population, exposures/
/// interventions, comparators, outcomes, measured variables, confounding variables),
/// the statistics, and the certainty of this evidence.

#[derive(Debug)]
pub struct Evidence_SampleSize<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Evidence_SampleSize<'_> {
    pub fn new(value: &Value) -> Evidence_SampleSize {
        Evidence_SampleSize {
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

    /// Extensions for knownDataCount
    pub fn _known_data_count(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_knownDataCount") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for numberOfParticipants
    pub fn _number_of_participants(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_numberOfParticipants") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for numberOfStudies
    pub fn _number_of_studies(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_numberOfStudies") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Human-readable summary of population sample size.
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

    /// Number of participants with known results for measured variables.
    pub fn known_data_count(&self) -> Option<u64> {
        if let Some(val) = self.value.get("knownDataCount") {
            return Some(val.as_u64().unwrap());
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

    /// Footnote or explanatory note about the sample size.
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

    /// A human-readable string to clarify or explain concepts about the sample size.
    pub fn number_of_participants(&self) -> Option<u64> {
        if let Some(val) = self.value.get("numberOfParticipants") {
            return Some(val.as_u64().unwrap());
        }
        return None;
    }

    /// Number of participants in the population.
    pub fn number_of_studies(&self) -> Option<u64> {
        if let Some(val) = self.value.get("numberOfStudies") {
            return Some(val.as_u64().unwrap());
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._description() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._known_data_count() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._number_of_participants() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._number_of_studies() {
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
        if let Some(_val) = self.known_data_count() {}
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
        if let Some(_val) = self.number_of_participants() {}
        if let Some(_val) = self.number_of_studies() {}
        return true;
    }
}

#[derive(Debug)]
pub struct Evidence_SampleSizeBuilder {
    pub(crate) value: Value,
}

impl Evidence_SampleSizeBuilder {
    pub fn build(&self) -> Evidence_SampleSize {
        Evidence_SampleSize {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Evidence_SampleSize) -> Evidence_SampleSizeBuilder {
        Evidence_SampleSizeBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> Evidence_SampleSizeBuilder {
        let mut __value: Value = json!({});
        return Evidence_SampleSizeBuilder { value: __value };
    }

    pub fn _description<'a>(&'a mut self, val: Element) -> &'a mut Evidence_SampleSizeBuilder {
        self.value["_description"] = json!(val.value);
        return self;
    }

    pub fn _known_data_count<'a>(&'a mut self, val: Element) -> &'a mut Evidence_SampleSizeBuilder {
        self.value["_knownDataCount"] = json!(val.value);
        return self;
    }

    pub fn _number_of_participants<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut Evidence_SampleSizeBuilder {
        self.value["_numberOfParticipants"] = json!(val.value);
        return self;
    }

    pub fn _number_of_studies<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut Evidence_SampleSizeBuilder {
        self.value["_numberOfStudies"] = json!(val.value);
        return self;
    }

    pub fn description<'a>(&'a mut self, val: &str) -> &'a mut Evidence_SampleSizeBuilder {
        self.value["description"] = json!(val);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut Evidence_SampleSizeBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut Evidence_SampleSizeBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn known_data_count<'a>(&'a mut self, val: u64) -> &'a mut Evidence_SampleSizeBuilder {
        self.value["knownDataCount"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut Evidence_SampleSizeBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn note<'a>(&'a mut self, val: Vec<Annotation>) -> &'a mut Evidence_SampleSizeBuilder {
        self.value["note"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn number_of_participants<'a>(
        &'a mut self,
        val: u64,
    ) -> &'a mut Evidence_SampleSizeBuilder {
        self.value["numberOfParticipants"] = json!(val);
        return self;
    }

    pub fn number_of_studies<'a>(&'a mut self, val: u64) -> &'a mut Evidence_SampleSizeBuilder {
        self.value["numberOfStudies"] = json!(val);
        return self;
    }
}
