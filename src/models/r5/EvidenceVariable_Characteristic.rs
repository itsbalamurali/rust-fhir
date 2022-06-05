#![allow(unused_imports, non_camel_case_types)]

use crate::models::r5::CodeableConcept::CodeableConcept;
use crate::models::r5::Element::Element;
use crate::models::r5::EvidenceVariable_TimeFromEvent::EvidenceVariable_TimeFromEvent;
use crate::models::r5::Expression::Expression;
use crate::models::r5::Extension::Extension;
use crate::models::r5::Reference::Reference;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// The EvidenceVariable resource describes an element that knowledge (Evidence) is
/// about.

#[derive(Debug)]
pub struct EvidenceVariable_Characteristic<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl EvidenceVariable_Characteristic<'_> {
    pub fn new(value: &Value) -> EvidenceVariable_Characteristic {
        EvidenceVariable_Characteristic {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for definitionCanonical
    pub fn _definition_canonical(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_definitionCanonical") {
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

    /// Extensions for exclude
    pub fn _exclude(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_exclude") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for groupMeasure
    pub fn _group_measure(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_groupMeasure") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Define members of the evidence element using Codes (such as condition, medication,
    /// or observation), Expressions ( using an expression language such as FHIRPath or
    /// CQL) or DataRequirements (such as Diabetes diagnosis onset in the last year).
    pub fn definition_canonical(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("definitionCanonical") {
            return Some(string);
        }
        return None;
    }

    /// Define members of the evidence element using Codes (such as condition, medication,
    /// or observation), Expressions ( using an expression language such as FHIRPath or
    /// CQL) or DataRequirements (such as Diabetes diagnosis onset in the last year).
    pub fn definition_codeable_concept(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("definitionCodeableConcept") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Define members of the evidence element using Codes (such as condition, medication,
    /// or observation), Expressions ( using an expression language such as FHIRPath or
    /// CQL) or DataRequirements (such as Diabetes diagnosis onset in the last year).
    pub fn definition_expression(&self) -> Option<Expression> {
        if let Some(val) = self.value.get("definitionExpression") {
            return Some(Expression {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Define members of the evidence element using Codes (such as condition, medication,
    /// or observation), Expressions ( using an expression language such as FHIRPath or
    /// CQL) or DataRequirements (such as Diabetes diagnosis onset in the last year).
    pub fn definition_reference(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("definitionReference") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A short, natural language description of the characteristic that could be used to
    /// communicate the criteria to an end-user.
    pub fn description(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("description") {
            return Some(string);
        }
        return None;
    }

    /// Device used for determining characteristic.
    pub fn device(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("device") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// When true, members with this characteristic are excluded from the element.
    pub fn exclude(&self) -> Option<bool> {
        if let Some(val) = self.value.get("exclude") {
            return Some(val.as_bool().unwrap());
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

    /// Value or set of values that define the grouping.
    pub fn group_measure(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("groupMeasure") {
            return Some(string);
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

    /// Method used for describing characteristic.
    pub fn method(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("method") {
            return Some(CodeableConcept {
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

    /// Observation time from study specified event.
    pub fn time_from_event(&self) -> Option<Vec<EvidenceVariable_TimeFromEvent>> {
        if let Some(Value::Array(val)) = self.value.get("timeFromEvent") {
            return Some(
                val.into_iter()
                    .map(|e| EvidenceVariable_TimeFromEvent {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Used to expressing the type of characteristic.
    pub fn fhir_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("type") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._definition_canonical() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._description() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._exclude() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._group_measure() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.definition_canonical() {}
        if let Some(_val) = self.definition_codeable_concept() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.definition_expression() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.definition_reference() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.description() {}
        if let Some(_val) = self.device() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.exclude() {}
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.group_measure() {}
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.method() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.time_from_event() {
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
pub struct EvidenceVariable_CharacteristicBuilder {
    pub(crate) value: Value,
}

impl EvidenceVariable_CharacteristicBuilder {
    pub fn build(&self) -> EvidenceVariable_Characteristic {
        EvidenceVariable_Characteristic {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(
        existing: EvidenceVariable_Characteristic,
    ) -> EvidenceVariable_CharacteristicBuilder {
        EvidenceVariable_CharacteristicBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> EvidenceVariable_CharacteristicBuilder {
        let mut __value: Value = json!({});
        return EvidenceVariable_CharacteristicBuilder { value: __value };
    }

    pub fn _definition_canonical<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut EvidenceVariable_CharacteristicBuilder {
        self.value["_definitionCanonical"] = json!(val.value);
        return self;
    }

    pub fn _description<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut EvidenceVariable_CharacteristicBuilder {
        self.value["_description"] = json!(val.value);
        return self;
    }

    pub fn _exclude<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut EvidenceVariable_CharacteristicBuilder {
        self.value["_exclude"] = json!(val.value);
        return self;
    }

    pub fn _group_measure<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut EvidenceVariable_CharacteristicBuilder {
        self.value["_groupMeasure"] = json!(val.value);
        return self;
    }

    pub fn definition_canonical<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut EvidenceVariable_CharacteristicBuilder {
        self.value["definitionCanonical"] = json!(val);
        return self;
    }

    pub fn definition_codeable_concept<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut EvidenceVariable_CharacteristicBuilder {
        self.value["definitionCodeableConcept"] = json!(val.value);
        return self;
    }

    pub fn definition_expression<'a>(
        &'a mut self,
        val: Expression,
    ) -> &'a mut EvidenceVariable_CharacteristicBuilder {
        self.value["definitionExpression"] = json!(val.value);
        return self;
    }

    pub fn definition_reference<'a>(
        &'a mut self,
        val: Reference,
    ) -> &'a mut EvidenceVariable_CharacteristicBuilder {
        self.value["definitionReference"] = json!(val.value);
        return self;
    }

    pub fn description<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut EvidenceVariable_CharacteristicBuilder {
        self.value["description"] = json!(val);
        return self;
    }

    pub fn device<'a>(
        &'a mut self,
        val: Reference,
    ) -> &'a mut EvidenceVariable_CharacteristicBuilder {
        self.value["device"] = json!(val.value);
        return self;
    }

    pub fn exclude<'a>(&'a mut self, val: bool) -> &'a mut EvidenceVariable_CharacteristicBuilder {
        self.value["exclude"] = json!(val);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut EvidenceVariable_CharacteristicBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn group_measure<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut EvidenceVariable_CharacteristicBuilder {
        self.value["groupMeasure"] = json!(val);
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut EvidenceVariable_CharacteristicBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn method<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut EvidenceVariable_CharacteristicBuilder {
        self.value["method"] = json!(val.value);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut EvidenceVariable_CharacteristicBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn time_from_event<'a>(
        &'a mut self,
        val: Vec<EvidenceVariable_TimeFromEvent>,
    ) -> &'a mut EvidenceVariable_CharacteristicBuilder {
        self.value["timeFromEvent"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn fhir_type<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut EvidenceVariable_CharacteristicBuilder {
        self.value["type"] = json!(val.value);
        return self;
    }
}
