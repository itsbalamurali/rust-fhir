#![allow(unused_imports, non_camel_case_types)]

use crate::models::r5::CodeableConcept::CodeableConcept;
use crate::models::r5::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A record of a medication that is being consumed by a patient.   A
/// MedicationUsage may indicate that the patient may be taking the medication now
/// or has taken the medication in the past or will be taking the medication in the
/// future.  The source of this information can be the patient, significant other
/// (such as a family member or spouse), or a clinician.  A common scenario where
/// this information is captured is during the history taking process during a
/// patient visit or stay.   The medication information may come from sources such
/// as the patient's memory, from a prescription bottle,  or from a list of
/// medications the patient, clinician or other party maintains.

/// The primary difference between a medicationusage and a medicationadministration
/// is that the medication administration has complete administration information
/// and is based on actual administration information from the person who
/// administered the medication.  A medicationusage is often, if not always, less
/// specific.  There is no required date/time when the medication was administered,
/// in fact we only know that a source has reported the patient is taking this
/// medication, where details such as time, quantity, or rate or even medication
/// product may be incomplete or missing or less precise.  As stated earlier, the
/// Medication Usage information may come from the patient's memory, from a
/// prescription bottle or from a list of medications the patient, clinician or
/// other party maintains.  Medication administration is more formal and is not
/// missing detailed information.

/// The MedicationUsage resource was previously called MedicationStatement.

#[derive(Debug)]
pub struct MedicationUsage_Adherence<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl MedicationUsage_Adherence<'_> {
    pub fn new(value: &Value) -> MedicationUsage_Adherence {
        MedicationUsage_Adherence {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Type of the adherence for the medication.
    pub fn code(&self) -> CodeableConcept {
        CodeableConcept {
            value: Cow::Borrowed(&self.value["code"]),
        }
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

    /// Captures the reason for the current use or adherence of a medication.
    pub fn reason(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("reason") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if !self.code().validate() {
            return false;
        }
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
        if let Some(_val) = self.reason() {
            if !_val.validate() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct MedicationUsage_AdherenceBuilder {
    pub(crate) value: Value,
}

impl MedicationUsage_AdherenceBuilder {
    pub fn build(&self) -> MedicationUsage_Adherence {
        MedicationUsage_Adherence {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: MedicationUsage_Adherence) -> MedicationUsage_AdherenceBuilder {
        MedicationUsage_AdherenceBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(code: CodeableConcept) -> MedicationUsage_AdherenceBuilder {
        let mut __value: Value = json!({});
        __value["code"] = json!(code.value);
        return MedicationUsage_AdherenceBuilder { value: __value };
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut MedicationUsage_AdherenceBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut MedicationUsage_AdherenceBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut MedicationUsage_AdherenceBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn reason<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut MedicationUsage_AdherenceBuilder {
        self.value["reason"] = json!(val.value);
        return self;
    }
}
