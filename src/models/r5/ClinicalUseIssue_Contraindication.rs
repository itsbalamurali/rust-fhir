#![allow(unused_imports, non_camel_case_types)]

use crate::models::r5::ClinicalUseIssue_OtherTherapy::ClinicalUseIssue_OtherTherapy;
use crate::models::r5::CodeableReference::CodeableReference;
use crate::models::r5::Extension::Extension;
use crate::models::r5::Reference::Reference;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A single issue - either an indication, contraindication, interaction or an
/// undesirable effect for a medicinal product, medication, device or procedure.

#[derive(Debug)]
pub struct ClinicalUseIssue_Contraindication<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl ClinicalUseIssue_Contraindication<'_> {
    pub fn new(value: &Value) -> ClinicalUseIssue_Contraindication {
        ClinicalUseIssue_Contraindication {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// A comorbidity (concurrent condition) or coinfection.
    pub fn comorbidity(&self) -> Option<Vec<CodeableReference>> {
        if let Some(Value::Array(val)) = self.value.get("comorbidity") {
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

    /// The status of the disease or symptom for the contraindication.
    pub fn disease_status(&self) -> Option<CodeableReference> {
        if let Some(val) = self.value.get("diseaseStatus") {
            return Some(CodeableReference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The situation that is being documented as contraindicating against this item.
    pub fn disease_symptom_procedure(&self) -> Option<CodeableReference> {
        if let Some(val) = self.value.get("diseaseSymptomProcedure") {
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

    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
        }
        return None;
    }

    /// The indication which this is a contraidication for.
    pub fn indication(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("indication") {
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

    /// Information about the use of the medicinal product in relation to other
    /// therapies described as part of the contraindication.
    pub fn other_therapy(&self) -> Option<Vec<ClinicalUseIssue_OtherTherapy>> {
        if let Some(Value::Array(val)) = self.value.get("otherTherapy") {
            return Some(
                val.into_iter()
                    .map(|e| ClinicalUseIssue_OtherTherapy {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.comorbidity() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.disease_status() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.disease_symptom_procedure() {
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
        if let Some(_val) = self.indication() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.other_therapy() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct ClinicalUseIssue_ContraindicationBuilder {
    pub(crate) value: Value,
}

impl ClinicalUseIssue_ContraindicationBuilder {
    pub fn build(&self) -> ClinicalUseIssue_Contraindication {
        ClinicalUseIssue_Contraindication {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(
        existing: ClinicalUseIssue_Contraindication,
    ) -> ClinicalUseIssue_ContraindicationBuilder {
        ClinicalUseIssue_ContraindicationBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> ClinicalUseIssue_ContraindicationBuilder {
        let mut __value: Value = json!({});
        return ClinicalUseIssue_ContraindicationBuilder { value: __value };
    }

    pub fn comorbidity<'a>(
        &'a mut self,
        val: Vec<CodeableReference>,
    ) -> &'a mut ClinicalUseIssue_ContraindicationBuilder {
        self.value["comorbidity"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn disease_status<'a>(
        &'a mut self,
        val: CodeableReference,
    ) -> &'a mut ClinicalUseIssue_ContraindicationBuilder {
        self.value["diseaseStatus"] = json!(val.value);
        return self;
    }

    pub fn disease_symptom_procedure<'a>(
        &'a mut self,
        val: CodeableReference,
    ) -> &'a mut ClinicalUseIssue_ContraindicationBuilder {
        self.value["diseaseSymptomProcedure"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ClinicalUseIssue_ContraindicationBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut ClinicalUseIssue_ContraindicationBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn indication<'a>(
        &'a mut self,
        val: Vec<Reference>,
    ) -> &'a mut ClinicalUseIssue_ContraindicationBuilder {
        self.value["indication"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ClinicalUseIssue_ContraindicationBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn other_therapy<'a>(
        &'a mut self,
        val: Vec<ClinicalUseIssue_OtherTherapy>,
    ) -> &'a mut ClinicalUseIssue_ContraindicationBuilder {
        self.value["otherTherapy"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }
}
