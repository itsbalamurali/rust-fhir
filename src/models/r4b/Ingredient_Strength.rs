#![allow(unused_imports, non_camel_case_types)]

use crate::models::r4b::CodeableConcept::CodeableConcept;
use crate::models::r4b::Element::Element;
use crate::models::r4b::Extension::Extension;
use crate::models::r4b::Ingredient_ReferenceStrength::Ingredient_ReferenceStrength;
use crate::models::r4b::Ratio::Ratio;
use crate::models::r4b::RatioRange::RatioRange;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// An ingredient of a manufactured item or pharmaceutical product.

#[derive(Debug)]
pub struct Ingredient_Strength<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Ingredient_Strength<'_> {
    pub fn new(value: &Value) -> Ingredient_Strength {
        Ingredient_Strength {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for measurementPoint
    pub fn _measurement_point(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_measurementPoint") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for textConcentration
    pub fn _text_concentration(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_textConcentration") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for textPresentation
    pub fn _text_presentation(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_textPresentation") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The strength per unitary volume (or mass).
    pub fn concentration_ratio(&self) -> Option<Ratio> {
        if let Some(val) = self.value.get("concentrationRatio") {
            return Some(Ratio {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The strength per unitary volume (or mass).
    pub fn concentration_ratio_range(&self) -> Option<RatioRange> {
        if let Some(val) = self.value.get("concentrationRatioRange") {
            return Some(RatioRange {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The country or countries for which the strength range applies.
    pub fn country(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("country") {
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

    /// For when strength is measured at a particular point or distance. There are
    /// products where strength is measured at a particular point. For example, the
    /// strength of the ingredient in some inhalers is measured at a particular position
    /// relative to the point of aerosolization.
    pub fn measurement_point(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("measurementPoint") {
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

    /// The quantity of substance in the unit of presentation, or in the volume (or
    /// mass) of the single pharmaceutical product or manufactured item. Unit of
    /// presentation refers to the quantity that the item occurs in e.g. a strength per
    /// tablet size, perhaps 'per 20mg' (the size of the tablet). It is not generally
    /// normalized as a unitary unit, which would be 'per mg').
    pub fn presentation_ratio(&self) -> Option<Ratio> {
        if let Some(val) = self.value.get("presentationRatio") {
            return Some(Ratio {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The quantity of substance in the unit of presentation, or in the volume (or
    /// mass) of the single pharmaceutical product or manufactured item. Unit of
    /// presentation refers to the quantity that the item occurs in e.g. a strength per
    /// tablet size, perhaps 'per 20mg' (the size of the tablet). It is not generally
    /// normalized as a unitary unit, which would be 'per mg').
    pub fn presentation_ratio_range(&self) -> Option<RatioRange> {
        if let Some(val) = self.value.get("presentationRatioRange") {
            return Some(RatioRange {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Strength expressed in terms of a reference substance. For when the ingredient
    /// strength is additionally expressed as equivalent to the strength of some other
    /// closely related substance (e.g. salt vs. base). Reference strength represents
    /// the strength (quantitative composition) of the active moiety of the active
    /// substance. There are situations when the active substance and active moiety are
    /// different, therefore both a strength and a reference strength are needed.
    pub fn reference_strength(&self) -> Option<Vec<Ingredient_ReferenceStrength>> {
        if let Some(Value::Array(val)) = self.value.get("referenceStrength") {
            return Some(
                val.into_iter()
                    .map(|e| Ingredient_ReferenceStrength {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A textual represention of either the whole of the concentration strength or a
    /// part of it - with the rest being in Strength.concentration as a ratio.
    pub fn text_concentration(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("textConcentration") {
            return Some(string);
        }
        return None;
    }

    /// A textual represention of either the whole of the presentation strength or a
    /// part of it - with the rest being in Strength.presentation as a ratio.
    pub fn text_presentation(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("textPresentation") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._measurement_point() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._text_concentration() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._text_presentation() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.concentration_ratio() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.concentration_ratio_range() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.country() {
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
        if let Some(_val) = self.measurement_point() {}
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.presentation_ratio() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.presentation_ratio_range() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.reference_strength() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.text_concentration() {}
        if let Some(_val) = self.text_presentation() {}
        return true;
    }
}

#[derive(Debug)]
pub struct Ingredient_StrengthBuilder {
    pub(crate) value: Value,
}

impl Ingredient_StrengthBuilder {
    pub fn build(&self) -> Ingredient_Strength {
        Ingredient_Strength {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Ingredient_Strength) -> Ingredient_StrengthBuilder {
        Ingredient_StrengthBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> Ingredient_StrengthBuilder {
        let mut __value: Value = json!({});
        return Ingredient_StrengthBuilder { value: __value };
    }

    pub fn _measurement_point<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut Ingredient_StrengthBuilder {
        self.value["_measurementPoint"] = json!(val.value);
        return self;
    }

    pub fn _text_concentration<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut Ingredient_StrengthBuilder {
        self.value["_textConcentration"] = json!(val.value);
        return self;
    }

    pub fn _text_presentation<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut Ingredient_StrengthBuilder {
        self.value["_textPresentation"] = json!(val.value);
        return self;
    }

    pub fn concentration_ratio<'a>(&'a mut self, val: Ratio) -> &'a mut Ingredient_StrengthBuilder {
        self.value["concentrationRatio"] = json!(val.value);
        return self;
    }

    pub fn concentration_ratio_range<'a>(
        &'a mut self,
        val: RatioRange,
    ) -> &'a mut Ingredient_StrengthBuilder {
        self.value["concentrationRatioRange"] = json!(val.value);
        return self;
    }

    pub fn country<'a>(
        &'a mut self,
        val: Vec<CodeableConcept>,
    ) -> &'a mut Ingredient_StrengthBuilder {
        self.value["country"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut Ingredient_StrengthBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut Ingredient_StrengthBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn measurement_point<'a>(&'a mut self, val: &str) -> &'a mut Ingredient_StrengthBuilder {
        self.value["measurementPoint"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut Ingredient_StrengthBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn presentation_ratio<'a>(&'a mut self, val: Ratio) -> &'a mut Ingredient_StrengthBuilder {
        self.value["presentationRatio"] = json!(val.value);
        return self;
    }

    pub fn presentation_ratio_range<'a>(
        &'a mut self,
        val: RatioRange,
    ) -> &'a mut Ingredient_StrengthBuilder {
        self.value["presentationRatioRange"] = json!(val.value);
        return self;
    }

    pub fn reference_strength<'a>(
        &'a mut self,
        val: Vec<Ingredient_ReferenceStrength>,
    ) -> &'a mut Ingredient_StrengthBuilder {
        self.value["referenceStrength"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn text_concentration<'a>(&'a mut self, val: &str) -> &'a mut Ingredient_StrengthBuilder {
        self.value["textConcentration"] = json!(val);
        return self;
    }

    pub fn text_presentation<'a>(&'a mut self, val: &str) -> &'a mut Ingredient_StrengthBuilder {
        self.value["textPresentation"] = json!(val);
        return self;
    }
}
