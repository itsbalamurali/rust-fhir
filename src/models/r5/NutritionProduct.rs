#![allow(unused_imports, non_camel_case_types)]

use crate::models::r5::Annotation::Annotation;
use crate::models::r5::CodeableConcept::CodeableConcept;
use crate::models::r5::CodeableReference::CodeableReference;
use crate::models::r5::Element::Element;
use crate::models::r5::Extension::Extension;
use crate::models::r5::Meta::Meta;
use crate::models::r5::Narrative::Narrative;
use crate::models::r5::NutritionProduct_Ingredient::NutritionProduct_Ingredient;
use crate::models::r5::NutritionProduct_Instance::NutritionProduct_Instance;
use crate::models::r5::NutritionProduct_Nutrient::NutritionProduct_Nutrient;
use crate::models::r5::NutritionProduct_ProductCharacteristic::NutritionProduct_ProductCharacteristic;
use crate::models::r5::Reference::Reference;
use crate::models::r5::ResourceList::ResourceList;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A food or fluid product that is consumed by patients.

#[derive(Debug)]
pub struct NutritionProduct<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl NutritionProduct<'_> {
    pub fn new(value: &Value) -> NutritionProduct {
        NutritionProduct {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for implicitRules
    pub fn _implicit_rules(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_implicitRules") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for language
    pub fn _language(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_language") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for status
    pub fn _status(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_status") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Nutrition products can have different classifications - according to its
    /// nutritional properties, preparation methods, etc.
    pub fn category(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("category") {
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

    /// The code assigned to the product, for example a manufacturer number or other
    /// terminology.
    pub fn code(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("code") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// These resources do not have an independent existence apart from the resource that
    /// contains them - they cannot be identified independently, nor can they have their
    /// own independent transaction scope.
    pub fn contained(&self) -> Option<Vec<ResourceList>> {
        if let Some(Value::Array(val)) = self.value.get("contained") {
            return Some(
                val.into_iter()
                    .map(|e| ResourceList {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// May be used to represent additional information that is not part of the basic
    /// definition of the resource. To make the use of extensions safe and manageable,
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

    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
        }
        return None;
    }

    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often, this
    /// is a reference to an implementation guide that defines the special rules along
    /// with other profiles etc.
    pub fn implicit_rules(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("implicitRules") {
            return Some(string);
        }
        return None;
    }

    /// Ingredients contained in this product.
    pub fn ingredient(&self) -> Option<Vec<NutritionProduct_Ingredient>> {
        if let Some(Value::Array(val)) = self.value.get("ingredient") {
            return Some(
                val.into_iter()
                    .map(|e| NutritionProduct_Ingredient {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Conveys instance-level information about this product item. One or several
    /// physical, countable instances or occurrences of the product.
    pub fn instance(&self) -> Option<NutritionProduct_Instance> {
        if let Some(val) = self.value.get("instance") {
            return Some(NutritionProduct_Instance {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Allergens that are known or suspected to be a part of this nutrition product.
    pub fn known_allergen(&self) -> Option<Vec<CodeableReference>> {
        if let Some(Value::Array(val)) = self.value.get("knownAllergen") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableReference {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The base language in which the resource is written.
    pub fn language(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("language") {
            return Some(string);
        }
        return None;
    }

    /// The organisation (manufacturer, representative or legal authorisation holder) that
    /// is responsible for the device.
    pub fn manufacturer(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("manufacturer") {
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

    /// The metadata about the resource. This is content that is maintained by the
    /// infrastructure. Changes to the content might not always be associated with version
    /// changes to the resource.
    pub fn meta(&self) -> Option<Meta> {
        if let Some(val) = self.value.get("meta") {
            return Some(Meta {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// May be used to represent additional information that is not part of the basic
    /// definition of the resource and that modifies the understanding of the element
    /// that contains it and/or the understanding of the containing element's descendants.
    /// Usually modifier elements provide negation or qualification. To make the use of
    /// extensions safe and manageable, there is a strict set of governance applied to
    /// the definition and use of extensions. Though any implementer is allowed to define
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

    /// Comments made about the product.
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

    /// The product's nutritional information expressed by the nutrients.
    pub fn nutrient(&self) -> Option<Vec<NutritionProduct_Nutrient>> {
        if let Some(Value::Array(val)) = self.value.get("nutrient") {
            return Some(
                val.into_iter()
                    .map(|e| NutritionProduct_Nutrient {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Specifies descriptive properties of the nutrition product.
    pub fn product_characteristic(&self) -> Option<Vec<NutritionProduct_ProductCharacteristic>> {
        if let Some(Value::Array(val)) = self.value.get("productCharacteristic") {
            return Some(
                val.into_iter()
                    .map(|e| NutritionProduct_ProductCharacteristic {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The current state of the product.
    pub fn status(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("status") {
            return Some(string);
        }
        return None;
    }

    /// A human-readable narrative that contains a summary of the resource and can be used
    /// to represent the content of the resource to a human. The narrative need not encode
    /// all the structured data, but is required to contain sufficient detail to make it
    /// "clinically safe" for a human to just read the narrative. Resource definitions
    /// may define what content should be represented in the narrative to ensure clinical
    /// safety.
    pub fn text(&self) -> Option<Narrative> {
        if let Some(val) = self.value.get("text") {
            return Some(Narrative {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._implicit_rules() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._language() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._status() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.category() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.code() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.contained() {
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
        if let Some(_val) = self.implicit_rules() {}
        if let Some(_val) = self.ingredient() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.instance() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.known_allergen() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.language() {}
        if let Some(_val) = self.manufacturer() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.meta() {
            if !_val.validate() {
                return false;
            }
        }
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
        if let Some(_val) = self.nutrient() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.product_characteristic() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.status() {}
        if let Some(_val) = self.text() {
            if !_val.validate() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct NutritionProductBuilder {
    pub(crate) value: Value,
}

impl NutritionProductBuilder {
    pub fn build(&self) -> NutritionProduct {
        NutritionProduct {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: NutritionProduct) -> NutritionProductBuilder {
        NutritionProductBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> NutritionProductBuilder {
        let mut __value: Value = json!({});
        return NutritionProductBuilder { value: __value };
    }

    pub fn _implicit_rules<'a>(&'a mut self, val: Element) -> &'a mut NutritionProductBuilder {
        self.value["_implicitRules"] = json!(val.value);
        return self;
    }

    pub fn _language<'a>(&'a mut self, val: Element) -> &'a mut NutritionProductBuilder {
        self.value["_language"] = json!(val.value);
        return self;
    }

    pub fn _status<'a>(&'a mut self, val: Element) -> &'a mut NutritionProductBuilder {
        self.value["_status"] = json!(val.value);
        return self;
    }

    pub fn category<'a>(
        &'a mut self,
        val: Vec<CodeableConcept>,
    ) -> &'a mut NutritionProductBuilder {
        self.value["category"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn code<'a>(&'a mut self, val: CodeableConcept) -> &'a mut NutritionProductBuilder {
        self.value["code"] = json!(val.value);
        return self;
    }

    pub fn contained<'a>(&'a mut self, val: Vec<ResourceList>) -> &'a mut NutritionProductBuilder {
        self.value["contained"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut NutritionProductBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut NutritionProductBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn implicit_rules<'a>(&'a mut self, val: &str) -> &'a mut NutritionProductBuilder {
        self.value["implicitRules"] = json!(val);
        return self;
    }

    pub fn ingredient<'a>(
        &'a mut self,
        val: Vec<NutritionProduct_Ingredient>,
    ) -> &'a mut NutritionProductBuilder {
        self.value["ingredient"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn instance<'a>(
        &'a mut self,
        val: NutritionProduct_Instance,
    ) -> &'a mut NutritionProductBuilder {
        self.value["instance"] = json!(val.value);
        return self;
    }

    pub fn known_allergen<'a>(
        &'a mut self,
        val: Vec<CodeableReference>,
    ) -> &'a mut NutritionProductBuilder {
        self.value["knownAllergen"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn language<'a>(&'a mut self, val: &str) -> &'a mut NutritionProductBuilder {
        self.value["language"] = json!(val);
        return self;
    }

    pub fn manufacturer<'a>(&'a mut self, val: Vec<Reference>) -> &'a mut NutritionProductBuilder {
        self.value["manufacturer"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn meta<'a>(&'a mut self, val: Meta) -> &'a mut NutritionProductBuilder {
        self.value["meta"] = json!(val.value);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut NutritionProductBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn note<'a>(&'a mut self, val: Vec<Annotation>) -> &'a mut NutritionProductBuilder {
        self.value["note"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn nutrient<'a>(
        &'a mut self,
        val: Vec<NutritionProduct_Nutrient>,
    ) -> &'a mut NutritionProductBuilder {
        self.value["nutrient"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn product_characteristic<'a>(
        &'a mut self,
        val: Vec<NutritionProduct_ProductCharacteristic>,
    ) -> &'a mut NutritionProductBuilder {
        self.value["productCharacteristic"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn status<'a>(&'a mut self, val: &str) -> &'a mut NutritionProductBuilder {
        self.value["status"] = json!(val);
        return self;
    }

    pub fn text<'a>(&'a mut self, val: Narrative) -> &'a mut NutritionProductBuilder {
        self.value["text"] = json!(val.value);
        return self;
    }
}
