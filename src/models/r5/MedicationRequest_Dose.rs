#![allow(unused_imports, non_camel_case_types)]

use crate::models::r5::Dosage::Dosage;
use crate::models::r5::Element::Element;
use crate::models::r5::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// An order or request for both supply of the medication and the instructions
/// for administration of the medication to a patient. The resource is called
/// "MedicationRequest" rather than "MedicationPrescription" or "MedicationOrder" to
/// generalize the use across inpatient and outpatient settings, including care plans,
/// etc., and to harmonize with workflow patterns.

#[derive(Debug)]
pub struct MedicationRequest_Dose<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl MedicationRequest_Dose<'_> {
    pub fn new(value: &Value) -> MedicationRequest_Dose {
        MedicationRequest_Dose {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for effectiveDosePeriod
    pub fn _effective_dose_period(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_effectiveDosePeriod") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for renderedDosageInstruction
    pub fn _rendered_dosage_instruction(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_renderedDosageInstruction") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Specific instructions for how the medication is to be used by the patient.
    pub fn dosage_instruction(&self) -> Option<Vec<Dosage>> {
        if let Some(Value::Array(val)) = self.value.get("dosageInstruction") {
            return Some(
                val.into_iter()
                    .map(|e| Dosage {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The period over which the medication is to be taken.  Where there are multiple
    /// dosageInstruction lines (for example, tapering doses), this is the earliest date
    /// and the latest end date of the dosageInstructions.
    pub fn effective_dose_period(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("effectiveDosePeriod") {
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

    /// The full representation of the dose of the medication included in all dosage
    /// instructions.  To be used when multiple dosage instructions are included to
    /// represent complex dosing such as increasing or tapering doses.
    pub fn rendered_dosage_instruction(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("renderedDosageInstruction") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._effective_dose_period() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._rendered_dosage_instruction() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.dosage_instruction() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.effective_dose_period() {}
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
        if let Some(_val) = self.rendered_dosage_instruction() {}
        return true;
    }
}

#[derive(Debug)]
pub struct MedicationRequest_DoseBuilder {
    pub(crate) value: Value,
}

impl MedicationRequest_DoseBuilder {
    pub fn build(&self) -> MedicationRequest_Dose {
        MedicationRequest_Dose {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: MedicationRequest_Dose) -> MedicationRequest_DoseBuilder {
        MedicationRequest_DoseBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> MedicationRequest_DoseBuilder {
        let mut __value: Value = json!({});
        return MedicationRequest_DoseBuilder { value: __value };
    }

    pub fn _effective_dose_period<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut MedicationRequest_DoseBuilder {
        self.value["_effectiveDosePeriod"] = json!(val.value);
        return self;
    }

    pub fn _rendered_dosage_instruction<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut MedicationRequest_DoseBuilder {
        self.value["_renderedDosageInstruction"] = json!(val.value);
        return self;
    }

    pub fn dosage_instruction<'a>(
        &'a mut self,
        val: Vec<Dosage>,
    ) -> &'a mut MedicationRequest_DoseBuilder {
        self.value["dosageInstruction"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn effective_dose_period<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut MedicationRequest_DoseBuilder {
        self.value["effectiveDosePeriod"] = json!(val);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut MedicationRequest_DoseBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut MedicationRequest_DoseBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut MedicationRequest_DoseBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn rendered_dosage_instruction<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut MedicationRequest_DoseBuilder {
        self.value["renderedDosageInstruction"] = json!(val);
        return self;
    }
}
