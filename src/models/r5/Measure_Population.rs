#![allow(unused_imports, non_camel_case_types)]

use crate::models::r5::CodeableConcept::CodeableConcept;
use crate::models::r5::Element::Element;
use crate::models::r5::Expression::Expression;
use crate::models::r5::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// The Measure resource provides the definition of a quality measure.

#[derive(Debug)]
pub struct Measure_Population<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Measure_Population<'_> {
    pub fn new(value: &Value) -> Measure_Population {
        Measure_Population {
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

    /// Extensions for inputPopulationId
    pub fn _input_population_id(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_inputPopulationId") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Specifies which method should be used to aggregate measure observation values.
    /// For most scoring types, this is implied by scoring (e.g. a proportion measure
    /// counts members of the populations). For continuous variables, however, this
    /// information must be specified to ensure correct calculation.
    pub fn aggregate_method(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("aggregateMethod") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The type of population criteria.
    pub fn code(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("code") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// An expression that specifies the criteria for the population, typically the name
    /// of an expression in a library.
    pub fn criteria(&self) -> Expression {
        Expression {
            value: Cow::Borrowed(&self.value["criteria"]),
        }
    }

    /// The human readable description of this population criteria.
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

    /// The id of a population element in this measure that provides the input for this
    /// population criteria. In most cases, the scoring structure of the measure implies
    /// specific relationships (e.g. the Numerator uses the Denominator as the source in
    /// a proportion scoring). In some cases, however, multiple possible choices exist
    /// and must be resolved explicitly. For example in a ratio measure with multiple
    /// initial populations, the denominator must specify which population should be
    /// used as the starting point.
    pub fn input_population_id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("inputPopulationId") {
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

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._description() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._input_population_id() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.aggregate_method() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.code() {
            if !_val.validate() {
                return false;
            }
        }
        if !self.criteria().validate() {
            return false;
        }
        if let Some(_val) = self.description() {}
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.input_population_id() {}
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct Measure_PopulationBuilder {
    pub(crate) value: Value,
}

impl Measure_PopulationBuilder {
    pub fn build(&self) -> Measure_Population {
        Measure_Population {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Measure_Population) -> Measure_PopulationBuilder {
        Measure_PopulationBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(criteria: Expression) -> Measure_PopulationBuilder {
        let mut __value: Value = json!({});
        __value["criteria"] = json!(criteria.value);
        return Measure_PopulationBuilder { value: __value };
    }

    pub fn _description<'a>(&'a mut self, val: Element) -> &'a mut Measure_PopulationBuilder {
        self.value["_description"] = json!(val.value);
        return self;
    }

    pub fn _input_population_id<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut Measure_PopulationBuilder {
        self.value["_inputPopulationId"] = json!(val.value);
        return self;
    }

    pub fn aggregate_method<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut Measure_PopulationBuilder {
        self.value["aggregateMethod"] = json!(val.value);
        return self;
    }

    pub fn code<'a>(&'a mut self, val: CodeableConcept) -> &'a mut Measure_PopulationBuilder {
        self.value["code"] = json!(val.value);
        return self;
    }

    pub fn description<'a>(&'a mut self, val: &str) -> &'a mut Measure_PopulationBuilder {
        self.value["description"] = json!(val);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut Measure_PopulationBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut Measure_PopulationBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn input_population_id<'a>(&'a mut self, val: &str) -> &'a mut Measure_PopulationBuilder {
        self.value["inputPopulationId"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut Measure_PopulationBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }
}
