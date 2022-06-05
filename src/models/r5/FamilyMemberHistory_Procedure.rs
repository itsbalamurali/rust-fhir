#![allow(unused_imports, non_camel_case_types)]

use crate::models::r5::Age::Age;
use crate::models::r5::Annotation::Annotation;
use crate::models::r5::CodeableConcept::CodeableConcept;
use crate::models::r5::Element::Element;
use crate::models::r5::Extension::Extension;
use crate::models::r5::Period::Period;
use crate::models::r5::Range::Range;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Significant health conditions for a person related to the patient relevant in the
/// context of care for the patient.

#[derive(Debug)]
pub struct FamilyMemberHistory_Procedure<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl FamilyMemberHistory_Procedure<'_> {
    pub fn new(value: &Value) -> FamilyMemberHistory_Procedure {
        FamilyMemberHistory_Procedure {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for contributedToDeath
    pub fn _contributed_to_death(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_contributedToDeath") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for performedDateTime
    pub fn _performed_date_time(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_performedDateTime") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for performedString
    pub fn _performed_string(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_performedString") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The actual procedure specified. Could be a coded procedure or a less specific
    /// string depending on how much is known about the procedure and the capabilities of
    /// the creating system.
    pub fn code(&self) -> CodeableConcept {
        CodeableConcept {
            value: Cow::Borrowed(&self.value["code"]),
        }
    }

    /// This procedure contributed to the cause of death of the related person. If
    /// contributedToDeath is not populated, then it is unknown.
    pub fn contributed_to_death(&self) -> Option<bool> {
        if let Some(val) = self.value.get("contributedToDeath") {
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

    /// Unique id for the element within a resource (for internal references). This may be
    /// any string value that does not contain spaces.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
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

    /// An area where general notes can be placed about this specific procedure.
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

    /// Indicates what happened following the procedure. If the procedure resulted in
    /// death, deceased date is captured on the relation.
    pub fn outcome(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("outcome") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Estimated or actual date, date-time, period, or age when the procedure was
    /// performed. Allows a period to support complex procedures that span more than one
    /// date, and also allows for the length of the procedure to be captured.
    pub fn performed_age(&self) -> Option<Age> {
        if let Some(val) = self.value.get("performedAge") {
            return Some(Age {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Estimated or actual date, date-time, period, or age when the procedure was
    /// performed. Allows a period to support complex procedures that span more than one
    /// date, and also allows for the length of the procedure to be captured.
    pub fn performed_date_time(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("performedDateTime") {
            return Some(string);
        }
        return None;
    }

    /// Estimated or actual date, date-time, period, or age when the procedure was
    /// performed. Allows a period to support complex procedures that span more than one
    /// date, and also allows for the length of the procedure to be captured.
    pub fn performed_period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("performedPeriod") {
            return Some(Period {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Estimated or actual date, date-time, period, or age when the procedure was
    /// performed. Allows a period to support complex procedures that span more than one
    /// date, and also allows for the length of the procedure to be captured.
    pub fn performed_range(&self) -> Option<Range> {
        if let Some(val) = self.value.get("performedRange") {
            return Some(Range {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Estimated or actual date, date-time, period, or age when the procedure was
    /// performed. Allows a period to support complex procedures that span more than one
    /// date, and also allows for the length of the procedure to be captured.
    pub fn performed_string(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("performedString") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._contributed_to_death() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._performed_date_time() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._performed_string() {
            if !_val.validate() {
                return false;
            }
        }
        if !self.code().validate() {
            return false;
        }
        if let Some(_val) = self.contributed_to_death() {}
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
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
        if let Some(_val) = self.outcome() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.performed_age() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.performed_date_time() {}
        if let Some(_val) = self.performed_period() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.performed_range() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.performed_string() {}
        return true;
    }
}

#[derive(Debug)]
pub struct FamilyMemberHistory_ProcedureBuilder {
    pub(crate) value: Value,
}

impl FamilyMemberHistory_ProcedureBuilder {
    pub fn build(&self) -> FamilyMemberHistory_Procedure {
        FamilyMemberHistory_Procedure {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: FamilyMemberHistory_Procedure) -> FamilyMemberHistory_ProcedureBuilder {
        FamilyMemberHistory_ProcedureBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(code: CodeableConcept) -> FamilyMemberHistory_ProcedureBuilder {
        let mut __value: Value = json!({});
        __value["code"] = json!(code.value);
        return FamilyMemberHistory_ProcedureBuilder { value: __value };
    }

    pub fn _contributed_to_death<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut FamilyMemberHistory_ProcedureBuilder {
        self.value["_contributedToDeath"] = json!(val.value);
        return self;
    }

    pub fn _performed_date_time<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut FamilyMemberHistory_ProcedureBuilder {
        self.value["_performedDateTime"] = json!(val.value);
        return self;
    }

    pub fn _performed_string<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut FamilyMemberHistory_ProcedureBuilder {
        self.value["_performedString"] = json!(val.value);
        return self;
    }

    pub fn contributed_to_death<'a>(
        &'a mut self,
        val: bool,
    ) -> &'a mut FamilyMemberHistory_ProcedureBuilder {
        self.value["contributedToDeath"] = json!(val);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut FamilyMemberHistory_ProcedureBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut FamilyMemberHistory_ProcedureBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut FamilyMemberHistory_ProcedureBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn note<'a>(
        &'a mut self,
        val: Vec<Annotation>,
    ) -> &'a mut FamilyMemberHistory_ProcedureBuilder {
        self.value["note"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn outcome<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut FamilyMemberHistory_ProcedureBuilder {
        self.value["outcome"] = json!(val.value);
        return self;
    }

    pub fn performed_age<'a>(
        &'a mut self,
        val: Age,
    ) -> &'a mut FamilyMemberHistory_ProcedureBuilder {
        self.value["performedAge"] = json!(val.value);
        return self;
    }

    pub fn performed_date_time<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut FamilyMemberHistory_ProcedureBuilder {
        self.value["performedDateTime"] = json!(val);
        return self;
    }

    pub fn performed_period<'a>(
        &'a mut self,
        val: Period,
    ) -> &'a mut FamilyMemberHistory_ProcedureBuilder {
        self.value["performedPeriod"] = json!(val.value);
        return self;
    }

    pub fn performed_range<'a>(
        &'a mut self,
        val: Range,
    ) -> &'a mut FamilyMemberHistory_ProcedureBuilder {
        self.value["performedRange"] = json!(val.value);
        return self;
    }

    pub fn performed_string<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut FamilyMemberHistory_ProcedureBuilder {
        self.value["performedString"] = json!(val);
        return self;
    }
}
