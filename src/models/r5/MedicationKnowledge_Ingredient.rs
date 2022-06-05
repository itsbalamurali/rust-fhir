#![allow(unused_imports, non_camel_case_types)]

use crate::models::r5::CodeableConcept::CodeableConcept;
use crate::models::r5::CodeableReference::CodeableReference;
use crate::models::r5::Extension::Extension;
use crate::models::r5::Quantity::Quantity;
use crate::models::r5::Ratio::Ratio;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Information about a medication that is used to support knowledge.

#[derive(Debug)]
pub struct MedicationKnowledge_Ingredient<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl MedicationKnowledge_Ingredient<'_> {
    pub fn new(value: &Value) -> MedicationKnowledge_Ingredient {
        MedicationKnowledge_Ingredient {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
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

    /// A reference to the resource that provides information about the ingredient.
    pub fn item(&self) -> CodeableReference {
        CodeableReference {
            value: Cow::Borrowed(&self.value["item"]),
        }
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

    /// Specifies how many (or how much) of the items there are in this Medication.  For
    /// example, 250 mg per tablet.  This is expressed as a ratio where the numerator is
    /// 250mg and the denominator is 1 tablet but can also be expressed a quantity when
    /// the denominator is assumed to be 1 tablet.
    pub fn strength_codeable_concept(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("strengthCodeableConcept") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Specifies how many (or how much) of the items there are in this Medication.  For
    /// example, 250 mg per tablet.  This is expressed as a ratio where the numerator is
    /// 250mg and the denominator is 1 tablet but can also be expressed a quantity when
    /// the denominator is assumed to be 1 tablet.
    pub fn strength_quantity(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("strengthQuantity") {
            return Some(Quantity {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Specifies how many (or how much) of the items there are in this Medication.  For
    /// example, 250 mg per tablet.  This is expressed as a ratio where the numerator is
    /// 250mg and the denominator is 1 tablet but can also be expressed a quantity when
    /// the denominator is assumed to be 1 tablet.
    pub fn strength_ratio(&self) -> Option<Ratio> {
        if let Some(val) = self.value.get("strengthRatio") {
            return Some(Ratio {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Indication of whether this ingredient affects the therapeutic action of the drug.
    pub fn fhir_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("type") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if !self.item().validate() {
            return false;
        }
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.strength_codeable_concept() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.strength_quantity() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.strength_ratio() {
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
pub struct MedicationKnowledge_IngredientBuilder {
    pub(crate) value: Value,
}

impl MedicationKnowledge_IngredientBuilder {
    pub fn build(&self) -> MedicationKnowledge_Ingredient {
        MedicationKnowledge_Ingredient {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: MedicationKnowledge_Ingredient) -> MedicationKnowledge_IngredientBuilder {
        MedicationKnowledge_IngredientBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(item: CodeableReference) -> MedicationKnowledge_IngredientBuilder {
        let mut __value: Value = json!({});
        __value["item"] = json!(item.value);
        return MedicationKnowledge_IngredientBuilder { value: __value };
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut MedicationKnowledge_IngredientBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut MedicationKnowledge_IngredientBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut MedicationKnowledge_IngredientBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn strength_codeable_concept<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut MedicationKnowledge_IngredientBuilder {
        self.value["strengthCodeableConcept"] = json!(val.value);
        return self;
    }

    pub fn strength_quantity<'a>(
        &'a mut self,
        val: Quantity,
    ) -> &'a mut MedicationKnowledge_IngredientBuilder {
        self.value["strengthQuantity"] = json!(val.value);
        return self;
    }

    pub fn strength_ratio<'a>(
        &'a mut self,
        val: Ratio,
    ) -> &'a mut MedicationKnowledge_IngredientBuilder {
        self.value["strengthRatio"] = json!(val.value);
        return self;
    }

    pub fn fhir_type<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut MedicationKnowledge_IngredientBuilder {
        self.value["type"] = json!(val.value);
        return self;
    }
}
