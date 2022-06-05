#![allow(unused_imports, non_camel_case_types)]

use crate::models::r5::CodeableConcept::CodeableConcept;
use crate::models::r5::Element::Element;
use crate::models::r5::Extension::Extension;
use crate::models::r5::Identifier::Identifier;
use crate::models::r5::Period::Period;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Regulatory approval, clearance or licencing related to a regulated product,
/// treatment, facility or activity that is cited in a guidance, regulation, rule
/// or legislative act. An example is Market Authorization relating to a Medicinal
/// Product.

#[derive(Debug)]
pub struct RegulatedAuthorization_Case<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl RegulatedAuthorization_Case<'_> {
    pub fn new(value: &Value) -> RegulatedAuthorization_Case {
        RegulatedAuthorization_Case {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for dateDateTime
    pub fn _date_date_time(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_dateDateTime") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Applications submitted to obtain a marketing authorization. Steps within the
    /// longer running case or procedure.
    pub fn application(&self) -> Option<Vec<RegulatedAuthorization_Case>> {
        if let Some(Value::Array(val)) = self.value.get("application") {
            return Some(
                val.into_iter()
                    .map(|e| RegulatedAuthorization_Case {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Relevant date for this of case.
    pub fn date_date_time(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("dateDateTime") {
            return Some(string);
        }
        return None;
    }

    /// Relevant date for this of case.
    pub fn date_period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("datePeriod") {
            return Some(Period {
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

    /// Unique id for the element within a resource (for internal references). This may be
    /// any string value that does not contain spaces.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
        }
        return None;
    }

    /// Identifier by which this case can be referenced.
    pub fn identifier(&self) -> Option<Identifier> {
        if let Some(val) = self.value.get("identifier") {
            return Some(Identifier {
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

    /// The status associated with the case.
    pub fn status(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("status") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The defining type of case.
    pub fn fhir_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("type") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._date_date_time() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.application() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.date_date_time() {}
        if let Some(_val) = self.date_period() {
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
        if let Some(_val) = self.identifier() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.status() {
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
pub struct RegulatedAuthorization_CaseBuilder {
    pub(crate) value: Value,
}

impl RegulatedAuthorization_CaseBuilder {
    pub fn build(&self) -> RegulatedAuthorization_Case {
        RegulatedAuthorization_Case {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: RegulatedAuthorization_Case) -> RegulatedAuthorization_CaseBuilder {
        RegulatedAuthorization_CaseBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> RegulatedAuthorization_CaseBuilder {
        let mut __value: Value = json!({});
        return RegulatedAuthorization_CaseBuilder { value: __value };
    }

    pub fn _date_date_time<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut RegulatedAuthorization_CaseBuilder {
        self.value["_dateDateTime"] = json!(val.value);
        return self;
    }

    pub fn application<'a>(
        &'a mut self,
        val: Vec<RegulatedAuthorization_Case>,
    ) -> &'a mut RegulatedAuthorization_CaseBuilder {
        self.value["application"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn date_date_time<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut RegulatedAuthorization_CaseBuilder {
        self.value["dateDateTime"] = json!(val);
        return self;
    }

    pub fn date_period<'a>(
        &'a mut self,
        val: Period,
    ) -> &'a mut RegulatedAuthorization_CaseBuilder {
        self.value["datePeriod"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut RegulatedAuthorization_CaseBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut RegulatedAuthorization_CaseBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn identifier<'a>(
        &'a mut self,
        val: Identifier,
    ) -> &'a mut RegulatedAuthorization_CaseBuilder {
        self.value["identifier"] = json!(val.value);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut RegulatedAuthorization_CaseBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn status<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut RegulatedAuthorization_CaseBuilder {
        self.value["status"] = json!(val.value);
        return self;
    }

    pub fn fhir_type<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut RegulatedAuthorization_CaseBuilder {
        self.value["type"] = json!(val.value);
        return self;
    }
}
