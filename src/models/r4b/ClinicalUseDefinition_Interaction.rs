#![allow(unused_imports, non_camel_case_types)]

use crate::models::r4b::ClinicalUseDefinition_Interactant::ClinicalUseDefinition_Interactant;
use crate::models::r4b::CodeableConcept::CodeableConcept;
use crate::models::r4b::CodeableReference::CodeableReference;
use crate::models::r4b::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A single issue - either an indication, contraindication, interaction or an
/// undesirable effect for a medicinal product, medication, device or procedure.

#[derive(Debug)]
pub struct ClinicalUseDefinition_Interaction<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl ClinicalUseDefinition_Interaction<'_> {
    pub fn new(value: &Value) -> ClinicalUseDefinition_Interaction {
        ClinicalUseDefinition_Interaction {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// The effect of the interaction, for example "reduced gastric absorption of primary
    /// medication".
    pub fn effect(&self) -> Option<CodeableReference> {
        if let Some(val) = self.value.get("effect") {
            return Some(CodeableReference {
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

    /// The incidence of the interaction, e.g. theoretical, observed.
    pub fn incidence(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("incidence") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The specific medication, food, substance or laboratory test that interacts.
    pub fn interactant(&self) -> Option<Vec<ClinicalUseDefinition_Interactant>> {
        if let Some(Value::Array(val)) = self.value.get("interactant") {
            return Some(
                val.into_iter()
                    .map(|e| ClinicalUseDefinition_Interactant {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Actions for managing the interaction.
    pub fn management(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("management") {
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

    /// The type of the interaction e.g. drug-drug interaction, drug-food interaction,
    /// drug-lab test interaction.
    pub fn fhir_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("type") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.effect() {
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
        if let Some(_val) = self.incidence() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.interactant() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.management() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
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
pub struct ClinicalUseDefinition_InteractionBuilder {
    pub(crate) value: Value,
}

impl ClinicalUseDefinition_InteractionBuilder {
    pub fn build(&self) -> ClinicalUseDefinition_Interaction {
        ClinicalUseDefinition_Interaction {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(
        existing: ClinicalUseDefinition_Interaction,
    ) -> ClinicalUseDefinition_InteractionBuilder {
        ClinicalUseDefinition_InteractionBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> ClinicalUseDefinition_InteractionBuilder {
        let mut __value: Value = json!({});
        return ClinicalUseDefinition_InteractionBuilder { value: __value };
    }

    pub fn effect<'a>(
        &'a mut self,
        val: CodeableReference,
    ) -> &'a mut ClinicalUseDefinition_InteractionBuilder {
        self.value["effect"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ClinicalUseDefinition_InteractionBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut ClinicalUseDefinition_InteractionBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn incidence<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut ClinicalUseDefinition_InteractionBuilder {
        self.value["incidence"] = json!(val.value);
        return self;
    }

    pub fn interactant<'a>(
        &'a mut self,
        val: Vec<ClinicalUseDefinition_Interactant>,
    ) -> &'a mut ClinicalUseDefinition_InteractionBuilder {
        self.value["interactant"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn management<'a>(
        &'a mut self,
        val: Vec<CodeableConcept>,
    ) -> &'a mut ClinicalUseDefinition_InteractionBuilder {
        self.value["management"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ClinicalUseDefinition_InteractionBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn fhir_type<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut ClinicalUseDefinition_InteractionBuilder {
        self.value["type"] = json!(val.value);
        return self;
    }
}
