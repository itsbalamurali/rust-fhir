#![allow(unused_imports, non_camel_case_types)]

use crate::models::r4b::CodeableConcept::CodeableConcept;
use crate::models::r4b::CodeableReference::CodeableReference;
use crate::models::r4b::Extension::Extension;
use crate::models::r4b::Period::Period;
use crate::models::r4b::Reference::Reference;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A medicinal product, being a substance or combination of substances that is
/// intended to treat, prevent or diagnose a disease, or to restore, correct or modify
/// physiological functions by exerting a pharmacological, immunological or metabolic
/// action. This resource is intended to define and detail such products and their
/// properties, for uses other than direct patient care (e.g. regulatory use, or drug
/// catalogs).

#[derive(Debug)]
pub struct MedicinalProductDefinition_Operation<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl MedicinalProductDefinition_Operation<'_> {
    pub fn new(value: &Value) -> MedicinalProductDefinition_Operation {
        MedicinalProductDefinition_Operation {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Specifies whether this particular business or manufacturing process is considered
    /// proprietary or confidential.
    pub fn confidentiality_indicator(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("confidentialityIndicator") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Date range of applicability.
    pub fn effective_date(&self) -> Option<Period> {
        if let Some(val) = self.value.get("effectiveDate") {
            return Some(Period {
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

    /// The organization or establishment responsible for (or associated with) the
    /// particular process or step, examples include the manufacturer, importer, agent.
    pub fn organization(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("organization") {
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

    /// The type of manufacturing operation e.g. manufacturing itself, re-packaging. For
    /// the authorization of this, a RegulatedAuthorization would point to the same plan
    /// or activity referenced here.
    pub fn fhir_type(&self) -> Option<CodeableReference> {
        if let Some(val) = self.value.get("type") {
            return Some(CodeableReference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.confidentiality_indicator() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.effective_date() {
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
        if let Some(_val) = self.organization() {
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
pub struct MedicinalProductDefinition_OperationBuilder {
    pub(crate) value: Value,
}

impl MedicinalProductDefinition_OperationBuilder {
    pub fn build(&self) -> MedicinalProductDefinition_Operation {
        MedicinalProductDefinition_Operation {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(
        existing: MedicinalProductDefinition_Operation,
    ) -> MedicinalProductDefinition_OperationBuilder {
        MedicinalProductDefinition_OperationBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> MedicinalProductDefinition_OperationBuilder {
        let mut __value: Value = json!({});
        return MedicinalProductDefinition_OperationBuilder { value: __value };
    }

    pub fn confidentiality_indicator<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut MedicinalProductDefinition_OperationBuilder {
        self.value["confidentialityIndicator"] = json!(val.value);
        return self;
    }

    pub fn effective_date<'a>(
        &'a mut self,
        val: Period,
    ) -> &'a mut MedicinalProductDefinition_OperationBuilder {
        self.value["effectiveDate"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut MedicinalProductDefinition_OperationBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut MedicinalProductDefinition_OperationBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut MedicinalProductDefinition_OperationBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn organization<'a>(
        &'a mut self,
        val: Vec<Reference>,
    ) -> &'a mut MedicinalProductDefinition_OperationBuilder {
        self.value["organization"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn fhir_type<'a>(
        &'a mut self,
        val: CodeableReference,
    ) -> &'a mut MedicinalProductDefinition_OperationBuilder {
        self.value["type"] = json!(val.value);
        return self;
    }
}
