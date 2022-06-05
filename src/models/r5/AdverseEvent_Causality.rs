#![allow(unused_imports, non_camel_case_types)]

use crate::models::r5::CodeableConcept::CodeableConcept;
use crate::models::r5::Extension::Extension;
use crate::models::r5::Reference::Reference;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// An event (i.e. any change to current patient status) that may be related to
/// unintended effects on a patient or research subject.  The unintended effects
/// may require additional monitoring, treatment or hospitalization or may result in
/// death.  The AdverseEvent resource also extends to potential or avoided events that
/// could have had such effects.

#[derive(Debug)]
pub struct AdverseEvent_Causality<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl AdverseEvent_Causality<'_> {
    pub fn new(value: &Value) -> AdverseEvent_Causality {
        AdverseEvent_Causality {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// The method of evaluating the relatedness of the suspected entity to the event.
    pub fn assessment_method(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("assessmentMethod") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The author of the information on the possible cause of the event.
    pub fn author(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("author") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The result of the assessment regarding the relatedness of the suspected entity to
    /// the event.
    pub fn entity_relatedness(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("entityRelatedness") {
            return Some(CodeableConcept {
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

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.assessment_method() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.author() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.entity_relatedness() {
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
        return true;
    }
}

#[derive(Debug)]
pub struct AdverseEvent_CausalityBuilder {
    pub(crate) value: Value,
}

impl AdverseEvent_CausalityBuilder {
    pub fn build(&self) -> AdverseEvent_Causality {
        AdverseEvent_Causality {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: AdverseEvent_Causality) -> AdverseEvent_CausalityBuilder {
        AdverseEvent_CausalityBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> AdverseEvent_CausalityBuilder {
        let mut __value: Value = json!({});
        return AdverseEvent_CausalityBuilder { value: __value };
    }

    pub fn assessment_method<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut AdverseEvent_CausalityBuilder {
        self.value["assessmentMethod"] = json!(val.value);
        return self;
    }

    pub fn author<'a>(&'a mut self, val: Reference) -> &'a mut AdverseEvent_CausalityBuilder {
        self.value["author"] = json!(val.value);
        return self;
    }

    pub fn entity_relatedness<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut AdverseEvent_CausalityBuilder {
        self.value["entityRelatedness"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut AdverseEvent_CausalityBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut AdverseEvent_CausalityBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut AdverseEvent_CausalityBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }
}
