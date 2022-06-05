#![allow(unused_imports, non_camel_case_types)]

use crate::models::r5::CodeableConcept::CodeableConcept;
use crate::models::r5::Duration::Duration;
use crate::models::r5::Element::Element;
use crate::models::r5::Extension::Extension;
use crate::models::r5::MeasureReport_Component::MeasureReport_Component;
use crate::models::r5::MeasureReport_Population1::MeasureReport_Population1;
use crate::models::r5::Period::Period;
use crate::models::r5::Quantity::Quantity;
use crate::models::r5::Range::Range;
use crate::models::r5::Reference::Reference;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// The MeasureReport resource contains the results of the calculation of a measure;
/// and optionally a reference to the resources involved in that calculation.

#[derive(Debug)]
pub struct MeasureReport_Stratum<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl MeasureReport_Stratum<'_> {
    pub fn new(value: &Value) -> MeasureReport_Stratum {
        MeasureReport_Stratum {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for measureScoreDateTime
    pub fn _measure_score_date_time(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_measureScoreDateTime") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for valueBoolean
    pub fn _value_boolean(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_valueBoolean") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A stratifier component value.
    pub fn component(&self) -> Option<Vec<MeasureReport_Component>> {
        if let Some(Value::Array(val)) = self.value.get("component") {
            return Some(
                val.into_iter()
                    .map(|e| MeasureReport_Component {
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

    /// Unique id for the element within a resource (for internal references). This may be
    /// any string value that does not contain spaces.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
        }
        return None;
    }

    /// The measure score for this stratum, calculated as appropriate for the measure type
    /// and scoring method, and based on only the members of this stratum.
    pub fn measure_score_codeable_concept(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("measureScoreCodeableConcept") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The measure score for this stratum, calculated as appropriate for the measure type
    /// and scoring method, and based on only the members of this stratum.
    pub fn measure_score_date_time(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("measureScoreDateTime") {
            return Some(string);
        }
        return None;
    }

    /// The measure score for this stratum, calculated as appropriate for the measure type
    /// and scoring method, and based on only the members of this stratum.
    pub fn measure_score_duration(&self) -> Option<Duration> {
        if let Some(val) = self.value.get("measureScoreDuration") {
            return Some(Duration {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The measure score for this stratum, calculated as appropriate for the measure type
    /// and scoring method, and based on only the members of this stratum.
    pub fn measure_score_period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("measureScorePeriod") {
            return Some(Period {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The measure score for this stratum, calculated as appropriate for the measure type
    /// and scoring method, and based on only the members of this stratum.
    pub fn measure_score_quantity(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("measureScoreQuantity") {
            return Some(Quantity {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The measure score for this stratum, calculated as appropriate for the measure type
    /// and scoring method, and based on only the members of this stratum.
    pub fn measure_score_range(&self) -> Option<Range> {
        if let Some(val) = self.value.get("measureScoreRange") {
            return Some(Range {
                value: Cow::Borrowed(val),
            });
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

    /// The populations that make up the stratum, one for each type of population
    /// appropriate to the measure.
    pub fn population(&self) -> Option<Vec<MeasureReport_Population1>> {
        if let Some(Value::Array(val)) = self.value.get("population") {
            return Some(
                val.into_iter()
                    .map(|e| MeasureReport_Population1 {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The value for this stratum, expressed as a CodeableConcept. When defining
    /// stratifiers on complex values, the value must be rendered such that the value for
    /// each stratum within the stratifier is unique.
    pub fn value_boolean(&self) -> Option<bool> {
        if let Some(val) = self.value.get("valueBoolean") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    /// The value for this stratum, expressed as a CodeableConcept. When defining
    /// stratifiers on complex values, the value must be rendered such that the value for
    /// each stratum within the stratifier is unique.
    pub fn value_codeable_concept(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("valueCodeableConcept") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The value for this stratum, expressed as a CodeableConcept. When defining
    /// stratifiers on complex values, the value must be rendered such that the value for
    /// each stratum within the stratifier is unique.
    pub fn value_quantity(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("valueQuantity") {
            return Some(Quantity {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The value for this stratum, expressed as a CodeableConcept. When defining
    /// stratifiers on complex values, the value must be rendered such that the value for
    /// each stratum within the stratifier is unique.
    pub fn value_range(&self) -> Option<Range> {
        if let Some(val) = self.value.get("valueRange") {
            return Some(Range {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The value for this stratum, expressed as a CodeableConcept. When defining
    /// stratifiers on complex values, the value must be rendered such that the value for
    /// each stratum within the stratifier is unique.
    pub fn value_reference(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("valueReference") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._measure_score_date_time() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._value_boolean() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.component() {
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
        if let Some(_val) = self.measure_score_codeable_concept() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.measure_score_date_time() {}
        if let Some(_val) = self.measure_score_duration() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.measure_score_period() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.measure_score_quantity() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.measure_score_range() {
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
        if let Some(_val) = self.value_boolean() {}
        if let Some(_val) = self.value_codeable_concept() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.value_quantity() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.value_range() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.value_reference() {
            if !_val.validate() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct MeasureReport_StratumBuilder {
    pub(crate) value: Value,
}

impl MeasureReport_StratumBuilder {
    pub fn build(&self) -> MeasureReport_Stratum {
        MeasureReport_Stratum {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: MeasureReport_Stratum) -> MeasureReport_StratumBuilder {
        MeasureReport_StratumBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> MeasureReport_StratumBuilder {
        let mut __value: Value = json!({});
        return MeasureReport_StratumBuilder { value: __value };
    }

    pub fn _measure_score_date_time<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut MeasureReport_StratumBuilder {
        self.value["_measureScoreDateTime"] = json!(val.value);
        return self;
    }

    pub fn _value_boolean<'a>(&'a mut self, val: Element) -> &'a mut MeasureReport_StratumBuilder {
        self.value["_valueBoolean"] = json!(val.value);
        return self;
    }

    pub fn component<'a>(
        &'a mut self,
        val: Vec<MeasureReport_Component>,
    ) -> &'a mut MeasureReport_StratumBuilder {
        self.value["component"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut MeasureReport_StratumBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut MeasureReport_StratumBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn measure_score_codeable_concept<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut MeasureReport_StratumBuilder {
        self.value["measureScoreCodeableConcept"] = json!(val.value);
        return self;
    }

    pub fn measure_score_date_time<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut MeasureReport_StratumBuilder {
        self.value["measureScoreDateTime"] = json!(val);
        return self;
    }

    pub fn measure_score_duration<'a>(
        &'a mut self,
        val: Duration,
    ) -> &'a mut MeasureReport_StratumBuilder {
        self.value["measureScoreDuration"] = json!(val.value);
        return self;
    }

    pub fn measure_score_period<'a>(
        &'a mut self,
        val: Period,
    ) -> &'a mut MeasureReport_StratumBuilder {
        self.value["measureScorePeriod"] = json!(val.value);
        return self;
    }

    pub fn measure_score_quantity<'a>(
        &'a mut self,
        val: Quantity,
    ) -> &'a mut MeasureReport_StratumBuilder {
        self.value["measureScoreQuantity"] = json!(val.value);
        return self;
    }

    pub fn measure_score_range<'a>(
        &'a mut self,
        val: Range,
    ) -> &'a mut MeasureReport_StratumBuilder {
        self.value["measureScoreRange"] = json!(val.value);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut MeasureReport_StratumBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn population<'a>(
        &'a mut self,
        val: Vec<MeasureReport_Population1>,
    ) -> &'a mut MeasureReport_StratumBuilder {
        self.value["population"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn value_boolean<'a>(&'a mut self, val: bool) -> &'a mut MeasureReport_StratumBuilder {
        self.value["valueBoolean"] = json!(val);
        return self;
    }

    pub fn value_codeable_concept<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut MeasureReport_StratumBuilder {
        self.value["valueCodeableConcept"] = json!(val.value);
        return self;
    }

    pub fn value_quantity<'a>(&'a mut self, val: Quantity) -> &'a mut MeasureReport_StratumBuilder {
        self.value["valueQuantity"] = json!(val.value);
        return self;
    }

    pub fn value_range<'a>(&'a mut self, val: Range) -> &'a mut MeasureReport_StratumBuilder {
        self.value["valueRange"] = json!(val.value);
        return self;
    }

    pub fn value_reference<'a>(
        &'a mut self,
        val: Reference,
    ) -> &'a mut MeasureReport_StratumBuilder {
        self.value["valueReference"] = json!(val.value);
        return self;
    }
}
