#![allow(unused_imports, non_camel_case_types)]

use crate::models::r5::CodeableConcept::CodeableConcept;
use crate::models::r5::Element::Element;
use crate::models::r5::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Properties of a substance specific to it being a polymer.

#[derive(Debug)]
pub struct SubstancePolymer_DegreeOfPolymerisation<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl SubstancePolymer_DegreeOfPolymerisation<'_> {
    pub fn new(value: &Value) -> SubstancePolymer_DegreeOfPolymerisation {
        SubstancePolymer_DegreeOfPolymerisation {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for average
    pub fn _average(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_average") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for high
    pub fn _high(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_high") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for low
    pub fn _low(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_low") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// An average amount of polymerisation.
    pub fn average(&self) -> Option<i64> {
        if let Some(val) = self.value.get("average") {
            return Some(val.as_i64().unwrap());
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

    /// A high expected limit of the amount.
    pub fn high(&self) -> Option<i64> {
        if let Some(val) = self.value.get("high") {
            return Some(val.as_i64().unwrap());
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

    /// A low expected limit of the amount.
    pub fn low(&self) -> Option<i64> {
        if let Some(val) = self.value.get("low") {
            return Some(val.as_i64().unwrap());
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

    /// The type of the degree of polymerisation shall be described, e.g. SRU/Polymer
    /// Ratio.
    pub fn fhir_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("type") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._average() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._high() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._low() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.average() {}
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.high() {}
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.low() {}
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
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
pub struct SubstancePolymer_DegreeOfPolymerisationBuilder {
    pub(crate) value: Value,
}

impl SubstancePolymer_DegreeOfPolymerisationBuilder {
    pub fn build(&self) -> SubstancePolymer_DegreeOfPolymerisation {
        SubstancePolymer_DegreeOfPolymerisation {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(
        existing: SubstancePolymer_DegreeOfPolymerisation,
    ) -> SubstancePolymer_DegreeOfPolymerisationBuilder {
        SubstancePolymer_DegreeOfPolymerisationBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> SubstancePolymer_DegreeOfPolymerisationBuilder {
        let mut __value: Value = json!({});
        return SubstancePolymer_DegreeOfPolymerisationBuilder { value: __value };
    }

    pub fn _average<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut SubstancePolymer_DegreeOfPolymerisationBuilder {
        self.value["_average"] = json!(val.value);
        return self;
    }

    pub fn _high<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut SubstancePolymer_DegreeOfPolymerisationBuilder {
        self.value["_high"] = json!(val.value);
        return self;
    }

    pub fn _low<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut SubstancePolymer_DegreeOfPolymerisationBuilder {
        self.value["_low"] = json!(val.value);
        return self;
    }

    pub fn average<'a>(
        &'a mut self,
        val: i64,
    ) -> &'a mut SubstancePolymer_DegreeOfPolymerisationBuilder {
        self.value["average"] = json!(val);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut SubstancePolymer_DegreeOfPolymerisationBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn high<'a>(
        &'a mut self,
        val: i64,
    ) -> &'a mut SubstancePolymer_DegreeOfPolymerisationBuilder {
        self.value["high"] = json!(val);
        return self;
    }

    pub fn id<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut SubstancePolymer_DegreeOfPolymerisationBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn low<'a>(
        &'a mut self,
        val: i64,
    ) -> &'a mut SubstancePolymer_DegreeOfPolymerisationBuilder {
        self.value["low"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut SubstancePolymer_DegreeOfPolymerisationBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn fhir_type<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut SubstancePolymer_DegreeOfPolymerisationBuilder {
        self.value["type"] = json!(val.value);
        return self;
    }
}
