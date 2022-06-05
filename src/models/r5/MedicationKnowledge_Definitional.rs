#![allow(unused_imports, non_camel_case_types)]

use crate::models::r5::CodeableConcept::CodeableConcept;
use crate::models::r5::Extension::Extension;
use crate::models::r5::MedicationKnowledge_DrugCharacteristic::MedicationKnowledge_DrugCharacteristic;
use crate::models::r5::MedicationKnowledge_Ingredient::MedicationKnowledge_Ingredient;
use crate::models::r5::Reference::Reference;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Information about a medication that is used to support knowledge.

#[derive(Debug)]
pub struct MedicationKnowledge_Definitional<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl MedicationKnowledge_Definitional<'_> {
    pub fn new(value: &Value) -> MedicationKnowledge_Definitional {
        MedicationKnowledge_Definitional {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Associated definitions for this medication.
    pub fn definition(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("definition") {
            return Some(
                val.into_iter()
                    .map(|e| Reference {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Describes the form of the item.  Powder; tablets; capsule.
    pub fn dose_form(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("doseForm") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Specifies descriptive properties of the medicine, such as color, shape, imprints,
    /// etc.
    pub fn drug_characteristic(&self) -> Option<Vec<MedicationKnowledge_DrugCharacteristic>> {
        if let Some(Value::Array(val)) = self.value.get("drugCharacteristic") {
            return Some(
                val.into_iter()
                    .map(|e| MedicationKnowledge_DrugCharacteristic {
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

    /// Identifies a particular constituent of interest in the product.
    pub fn ingredient(&self) -> Option<Vec<MedicationKnowledge_Ingredient>> {
        if let Some(Value::Array(val)) = self.value.get("ingredient") {
            return Some(
                val.into_iter()
                    .map(|e| MedicationKnowledge_Ingredient {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The intended or approved route of administration.
    pub fn intended_route(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("intendedRoute") {
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

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.definition() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.dose_form() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.drug_characteristic() {
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
        if let Some(_val) = self.ingredient() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.intended_route() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct MedicationKnowledge_DefinitionalBuilder {
    pub(crate) value: Value,
}

impl MedicationKnowledge_DefinitionalBuilder {
    pub fn build(&self) -> MedicationKnowledge_Definitional {
        MedicationKnowledge_Definitional {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(
        existing: MedicationKnowledge_Definitional,
    ) -> MedicationKnowledge_DefinitionalBuilder {
        MedicationKnowledge_DefinitionalBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> MedicationKnowledge_DefinitionalBuilder {
        let mut __value: Value = json!({});
        return MedicationKnowledge_DefinitionalBuilder { value: __value };
    }

    pub fn definition<'a>(
        &'a mut self,
        val: Vec<Reference>,
    ) -> &'a mut MedicationKnowledge_DefinitionalBuilder {
        self.value["definition"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn dose_form<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut MedicationKnowledge_DefinitionalBuilder {
        self.value["doseForm"] = json!(val.value);
        return self;
    }

    pub fn drug_characteristic<'a>(
        &'a mut self,
        val: Vec<MedicationKnowledge_DrugCharacteristic>,
    ) -> &'a mut MedicationKnowledge_DefinitionalBuilder {
        self.value["drugCharacteristic"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut MedicationKnowledge_DefinitionalBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut MedicationKnowledge_DefinitionalBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn ingredient<'a>(
        &'a mut self,
        val: Vec<MedicationKnowledge_Ingredient>,
    ) -> &'a mut MedicationKnowledge_DefinitionalBuilder {
        self.value["ingredient"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn intended_route<'a>(
        &'a mut self,
        val: Vec<CodeableConcept>,
    ) -> &'a mut MedicationKnowledge_DefinitionalBuilder {
        self.value["intendedRoute"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut MedicationKnowledge_DefinitionalBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }
}
