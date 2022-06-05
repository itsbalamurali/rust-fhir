#![allow(unused_imports, non_camel_case_types)]

use crate::models::r5::CodeableConcept::CodeableConcept;
use crate::models::r5::CodeableReference::CodeableReference;
use crate::models::r5::Element::Element;
use crate::models::r5::Extension::Extension;
use crate::models::r5::Quantity::Quantity;
use crate::models::r5::Timing::Timing;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A record of food or fluid that is being consumed by a patient.   A NutritionIntake
/// may indicate that the patient may be consuming the food or fluid now or has
/// consumed the food or fluid in the past.  The source of this information can be the
/// patient, significant other (such as a family member or spouse), or a clinician.  A
/// common scenario where this information is captured is during the history taking
/// process during a patient visit or stay or through an app that tracks food or
/// fluids consumed.   The consumption information may come from sources such as
/// the patient's memory, from a nutrition label,  or from a clinician documenting
/// observed intake.

#[derive(Debug)]
pub struct NutritionIntake_ConsumedItem<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl NutritionIntake_ConsumedItem<'_> {
    pub fn new(value: &Value) -> NutritionIntake_ConsumedItem {
        NutritionIntake_ConsumedItem {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for notConsumed
    pub fn _not_consumed(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_notConsumed") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Quantity of the specified food.
    pub fn amount(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("amount") {
            return Some(Quantity {
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

    /// Indicator when a patient is in a setting where it is helpful to know if food was
    /// not consumed, such as it was refused, held (as in tube feedings), or otherwise not
    /// provided. If a consumption is being recorded from an app, such as MyFitnessPal,
    /// this indicator will likely not be used.
    pub fn not_consumed(&self) -> Option<bool> {
        if let Some(val) = self.value.get("notConsumed") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    /// Document the reason the food or fluid was not consumed, such as refused, held,
    /// etc.
    pub fn not_consumed_reason(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("notConsumedReason") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Identifies the food or fluid product that was consumed. This is potentially a
    /// link to a resource representing the details of the food product (TBD) or a simple
    /// attribute carrying a code that identifies the food from a known list of foods.
    pub fn nutrition_product(&self) -> CodeableReference {
        CodeableReference {
            value: Cow::Borrowed(&self.value["nutritionProduct"]),
        }
    }

    /// Rate at which enteral feeding was administered.
    pub fn rate(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("rate") {
            return Some(Quantity {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Scheduled frequency of consumption.
    pub fn schedule(&self) -> Option<Timing> {
        if let Some(val) = self.value.get("schedule") {
            return Some(Timing {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Indicates what a category of item that was consumed: eg., food, fluid, enteral,
    /// etc.
    pub fn fhir_type(&self) -> CodeableConcept {
        CodeableConcept {
            value: Cow::Borrowed(&self.value["type"]),
        }
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._not_consumed() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.amount() {
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
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.not_consumed() {}
        if let Some(_val) = self.not_consumed_reason() {
            if !_val.validate() {
                return false;
            }
        }
        if !self.nutrition_product().validate() {
            return false;
        }
        if let Some(_val) = self.rate() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.schedule() {
            if !_val.validate() {
                return false;
            }
        }
        if !self.fhir_type().validate() {
            return false;
        }
        return true;
    }
}

#[derive(Debug)]
pub struct NutritionIntake_ConsumedItemBuilder {
    pub(crate) value: Value,
}

impl NutritionIntake_ConsumedItemBuilder {
    pub fn build(&self) -> NutritionIntake_ConsumedItem {
        NutritionIntake_ConsumedItem {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: NutritionIntake_ConsumedItem) -> NutritionIntake_ConsumedItemBuilder {
        NutritionIntake_ConsumedItemBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(
        nutrition_product: CodeableReference,
        fhir_type: CodeableConcept,
    ) -> NutritionIntake_ConsumedItemBuilder {
        let mut __value: Value = json!({});
        __value["nutritionProduct"] = json!(nutrition_product.value);
        __value["type"] = json!(fhir_type.value);
        return NutritionIntake_ConsumedItemBuilder { value: __value };
    }

    pub fn _not_consumed<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut NutritionIntake_ConsumedItemBuilder {
        self.value["_notConsumed"] = json!(val.value);
        return self;
    }

    pub fn amount<'a>(&'a mut self, val: Quantity) -> &'a mut NutritionIntake_ConsumedItemBuilder {
        self.value["amount"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut NutritionIntake_ConsumedItemBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut NutritionIntake_ConsumedItemBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut NutritionIntake_ConsumedItemBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn not_consumed<'a>(
        &'a mut self,
        val: bool,
    ) -> &'a mut NutritionIntake_ConsumedItemBuilder {
        self.value["notConsumed"] = json!(val);
        return self;
    }

    pub fn not_consumed_reason<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut NutritionIntake_ConsumedItemBuilder {
        self.value["notConsumedReason"] = json!(val.value);
        return self;
    }

    pub fn rate<'a>(&'a mut self, val: Quantity) -> &'a mut NutritionIntake_ConsumedItemBuilder {
        self.value["rate"] = json!(val.value);
        return self;
    }

    pub fn schedule<'a>(&'a mut self, val: Timing) -> &'a mut NutritionIntake_ConsumedItemBuilder {
        self.value["schedule"] = json!(val.value);
        return self;
    }
}
