#![allow(unused_imports, non_camel_case_types)]

use crate::models::r5::CodeableConcept::CodeableConcept;
use crate::models::r5::Element::Element;
use crate::models::r5::Extension::Extension;
use crate::models::r5::Measure_Population::Measure_Population;
use crate::models::r5::Measure_Stratifier::Measure_Stratifier;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// The Measure resource provides the definition of a quality measure.

#[derive(Debug)]
pub struct Measure_Group<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Measure_Group<'_> {
    pub fn new(value: &Value) -> Measure_Group {
        Measure_Group {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for basis
    pub fn _basis(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_basis") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
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

    /// The population basis specifies the type of elements in the population. For a
    /// subject-based measure, this is boolean (because the subject and the population
    /// basis are the same, and the population criteria define yes/no values for each
    /// individual in the population). For measures that have a population basis that is
    /// different than the subject, this element specifies the type of the population
    /// basis. For example, an encounter-based measure has a subject of Patient and a
    /// population basis of Encounter, and the population criteria all return lists of
    /// Encounters.
    pub fn basis(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("basis") {
            return Some(string);
        }
        return None;
    }

    /// Indicates a meaning for the group. This can be as simple as a unique identifier,
    /// or it can establish meaning in a broader context by drawing from a terminology,
    /// allowing groups to be correlated across measures.
    pub fn code(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("code") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The human readable description of this population group.
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

    /// Information on whether an increase or decrease in score is the preferred result
    /// (e.g., a higher score indicates better quality OR a lower score indicates better
    /// quality OR quality is within a range).
    pub fn improvement_notation(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("improvementNotation") {
            return Some(CodeableConcept {
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

    /// A population criteria for the measure.
    pub fn population(&self) -> Option<Vec<Measure_Population>> {
        if let Some(Value::Array(val)) = self.value.get("population") {
            return Some(
                val.into_iter()
                    .map(|e| Measure_Population {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Indicates how the calculation is performed for the measure, including
    /// proportion, ratio, continuous-variable, and cohort. The value set is extensible,
    /// allowing additional measure scoring types to be represented.
    pub fn scoring(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("scoring") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Defines the expected units of measure for the measure score. This element SHOULD
    /// be specified as a UCUM unit.
    pub fn scoring_unit(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("scoringUnit") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The stratifier criteria for the measure report, specified as either the name of
    /// a valid CQL expression defined within a referenced library or a valid FHIR
    /// Resource Path.
    pub fn stratifier(&self) -> Option<Vec<Measure_Stratifier>> {
        if let Some(Value::Array(val)) = self.value.get("stratifier") {
            return Some(
                val.into_iter()
                    .map(|e| Measure_Stratifier {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Indicates whether the measure is used to examine a process, an outcome over
    /// time, a patient-reported outcome, or a structure measure such as utilization.
    pub fn fhir_type(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("type") {
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

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._basis() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._description() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.basis() {}
        if let Some(_val) = self.code() {
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
        if let Some(_val) = self.improvement_notation() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.population() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.scoring() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.scoring_unit() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.stratifier() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.fhir_type() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct Measure_GroupBuilder {
    pub(crate) value: Value,
}

impl Measure_GroupBuilder {
    pub fn build(&self) -> Measure_Group {
        Measure_Group {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Measure_Group) -> Measure_GroupBuilder {
        Measure_GroupBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> Measure_GroupBuilder {
        let mut __value: Value = json!({});
        return Measure_GroupBuilder { value: __value };
    }

    pub fn _basis<'a>(&'a mut self, val: Element) -> &'a mut Measure_GroupBuilder {
        self.value["_basis"] = json!(val.value);
        return self;
    }

    pub fn _description<'a>(&'a mut self, val: Element) -> &'a mut Measure_GroupBuilder {
        self.value["_description"] = json!(val.value);
        return self;
    }

    pub fn basis<'a>(&'a mut self, val: &str) -> &'a mut Measure_GroupBuilder {
        self.value["basis"] = json!(val);
        return self;
    }

    pub fn code<'a>(&'a mut self, val: CodeableConcept) -> &'a mut Measure_GroupBuilder {
        self.value["code"] = json!(val.value);
        return self;
    }

    pub fn description<'a>(&'a mut self, val: &str) -> &'a mut Measure_GroupBuilder {
        self.value["description"] = json!(val);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut Measure_GroupBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut Measure_GroupBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn improvement_notation<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut Measure_GroupBuilder {
        self.value["improvementNotation"] = json!(val.value);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut Measure_GroupBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn population<'a>(
        &'a mut self,
        val: Vec<Measure_Population>,
    ) -> &'a mut Measure_GroupBuilder {
        self.value["population"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn scoring<'a>(&'a mut self, val: CodeableConcept) -> &'a mut Measure_GroupBuilder {
        self.value["scoring"] = json!(val.value);
        return self;
    }

    pub fn scoring_unit<'a>(&'a mut self, val: CodeableConcept) -> &'a mut Measure_GroupBuilder {
        self.value["scoringUnit"] = json!(val.value);
        return self;
    }

    pub fn stratifier<'a>(
        &'a mut self,
        val: Vec<Measure_Stratifier>,
    ) -> &'a mut Measure_GroupBuilder {
        self.value["stratifier"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn fhir_type<'a>(&'a mut self, val: Vec<CodeableConcept>) -> &'a mut Measure_GroupBuilder {
        self.value["type"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }
}
