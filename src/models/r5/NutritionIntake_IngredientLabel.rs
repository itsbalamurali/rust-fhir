#![allow(unused_imports, non_camel_case_types)]

use crate::models::r5::CodeableReference::CodeableReference;
use crate::models::r5::Extension::Extension;
use crate::models::r5::Quantity::Quantity;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A record of food or fluid that is being consumed by a patient.   A
/// NutritionIntake may indicate that the patient may be consuming the food or fluid
/// now or has consumed the food or fluid in the past.  The source of this
/// information can be the patient, significant other (such as a family member or
/// spouse), or a clinician.  A common scenario where this information is captured
/// is during the history taking process during a patient visit or stay or through
/// an app that tracks food or fluids consumed.   The consumption information may
/// come from sources such as the patient's memory, from a nutrition label,  or from
/// a clinician documenting observed intake.

#[derive(Debug)]
pub struct NutritionIntake_IngredientLabel<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl NutritionIntake_IngredientLabel<'_> {
    pub fn new(value: &Value) -> NutritionIntake_IngredientLabel {
        NutritionIntake_IngredientLabel {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Total amount of nutrient consumed.
    pub fn amount(&self) -> Quantity {
        Quantity {
            value: Cow::Borrowed(&self.value["amount"]),
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

    /// Total nutrient consumed. This could be a macronutrient (protein, fat,
    /// carbohydrate), or a vitamin and mineral.
    pub fn nutrient(&self) -> CodeableReference {
        CodeableReference {
            value: Cow::Borrowed(&self.value["nutrient"]),
        }
    }

    pub fn validate(&self) -> bool {
        if !self.amount().validate() {
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
        if !self.nutrient().validate() {
            return false;
        }
        return true;
    }
}

#[derive(Debug)]
pub struct NutritionIntake_IngredientLabelBuilder {
    pub(crate) value: Value,
}

impl NutritionIntake_IngredientLabelBuilder {
    pub fn build(&self) -> NutritionIntake_IngredientLabel {
        NutritionIntake_IngredientLabel {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(
        existing: NutritionIntake_IngredientLabel,
    ) -> NutritionIntake_IngredientLabelBuilder {
        NutritionIntake_IngredientLabelBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(
        amount: Quantity,
        nutrient: CodeableReference,
    ) -> NutritionIntake_IngredientLabelBuilder {
        let mut __value: Value = json!({});
        __value["amount"] = json!(amount.value);
        __value["nutrient"] = json!(nutrient.value);
        return NutritionIntake_IngredientLabelBuilder { value: __value };
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut NutritionIntake_IngredientLabelBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut NutritionIntake_IngredientLabelBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut NutritionIntake_IngredientLabelBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }
}
