#![allow(unused_imports, non_camel_case_types)]

use crate::models::r4b::ClinicalUseDefinition_OtherTherapy::ClinicalUseDefinition_OtherTherapy;
use crate::models::r4b::CodeableReference::CodeableReference;
use crate::models::r4b::Element::Element;
use crate::models::r4b::Extension::Extension;
use crate::models::r4b::Range::Range;
use crate::models::r4b::Reference::Reference;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A single issue - either an indication, contraindication, interaction or an
/// undesirable effect for a medicinal product, medication, device or procedure.

#[derive(Debug)]
pub struct ClinicalUseDefinition_Indication<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl ClinicalUseDefinition_Indication<'_> {
    pub fn new(value: &Value) -> ClinicalUseDefinition_Indication {
        ClinicalUseDefinition_Indication {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for durationString
    pub fn _duration_string(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_durationString") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A comorbidity (concurrent condition) or coinfection as part of the indication.
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

    /// The status of the disease or symptom for the indication, for example "chronic"
    /// or "metastatic".
    pub fn disease_status(&self) -> Option<CodeableReference> {
        if let Some(val) = self.value.get("diseaseStatus") {
            return Some(CodeableReference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The situation that is being documented as an indicaton for this item.
    pub fn disease_symptom_procedure(&self) -> Option<CodeableReference> {
        if let Some(val) = self.value.get("diseaseSymptomProcedure") {
            return Some(CodeableReference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Timing or duration information, that may be associated with use with the
    /// indicated condition e.g. Adult patients suffering from myocardial infarction
    /// (from a few days until less than 35 days), ischaemic stroke (from 7 days until
    /// less than 6 months).
    pub fn duration_range(&self) -> Option<Range> {
        if let Some(val) = self.value.get("durationRange") {
            return Some(Range {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Timing or duration information, that may be associated with use with the
    /// indicated condition e.g. Adult patients suffering from myocardial infarction
    /// (from a few days until less than 35 days), ischaemic stroke (from 7 days until
    /// less than 6 months).
    pub fn duration_string(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("durationString") {
            return Some(string);
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

    /// The intended effect, aim or strategy to be achieved.
    pub fn intended_effect(&self) -> Option<CodeableReference> {
        if let Some(val) = self.value.get("intendedEffect") {
            return Some(CodeableReference {
                value: Cow::Borrowed(val),
            });
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
    /// therapies described as part of the indication.
    pub fn other_therapy(&self) -> Option<Vec<ClinicalUseDefinition_OtherTherapy>> {
        if let Some(Value::Array(val)) = self.value.get("otherTherapy") {
            return Some(
                val.into_iter()
                    .map(|e| ClinicalUseDefinition_OtherTherapy {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// An unwanted side effect or negative outcome that may happen if you use the drug
    /// (or other subject of this resource) for this indication.
    pub fn undesirable_effect(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("undesirableEffect") {
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

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._duration_string() {
            if !_val.validate() {
                return false;
            }
        }
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
        if let Some(_val) = self.duration_range() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.duration_string() {}
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.intended_effect() {
            if !_val.validate() {
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
        if let Some(_val) = self.undesirable_effect() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct ClinicalUseDefinition_IndicationBuilder {
    pub(crate) value: Value,
}

impl ClinicalUseDefinition_IndicationBuilder {
    pub fn build(&self) -> ClinicalUseDefinition_Indication {
        ClinicalUseDefinition_Indication {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(
        existing: ClinicalUseDefinition_Indication,
    ) -> ClinicalUseDefinition_IndicationBuilder {
        ClinicalUseDefinition_IndicationBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> ClinicalUseDefinition_IndicationBuilder {
        let mut __value: Value = json!({});
        return ClinicalUseDefinition_IndicationBuilder { value: __value };
    }

    pub fn _duration_string<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut ClinicalUseDefinition_IndicationBuilder {
        self.value["_durationString"] = json!(val.value);
        return self;
    }

    pub fn comorbidity<'a>(
        &'a mut self,
        val: Vec<CodeableReference>,
    ) -> &'a mut ClinicalUseDefinition_IndicationBuilder {
        self.value["comorbidity"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn disease_status<'a>(
        &'a mut self,
        val: CodeableReference,
    ) -> &'a mut ClinicalUseDefinition_IndicationBuilder {
        self.value["diseaseStatus"] = json!(val.value);
        return self;
    }

    pub fn disease_symptom_procedure<'a>(
        &'a mut self,
        val: CodeableReference,
    ) -> &'a mut ClinicalUseDefinition_IndicationBuilder {
        self.value["diseaseSymptomProcedure"] = json!(val.value);
        return self;
    }

    pub fn duration_range<'a>(
        &'a mut self,
        val: Range,
    ) -> &'a mut ClinicalUseDefinition_IndicationBuilder {
        self.value["durationRange"] = json!(val.value);
        return self;
    }

    pub fn duration_string<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut ClinicalUseDefinition_IndicationBuilder {
        self.value["durationString"] = json!(val);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ClinicalUseDefinition_IndicationBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut ClinicalUseDefinition_IndicationBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn intended_effect<'a>(
        &'a mut self,
        val: CodeableReference,
    ) -> &'a mut ClinicalUseDefinition_IndicationBuilder {
        self.value["intendedEffect"] = json!(val.value);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ClinicalUseDefinition_IndicationBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn other_therapy<'a>(
        &'a mut self,
        val: Vec<ClinicalUseDefinition_OtherTherapy>,
    ) -> &'a mut ClinicalUseDefinition_IndicationBuilder {
        self.value["otherTherapy"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn undesirable_effect<'a>(
        &'a mut self,
        val: Vec<Reference>,
    ) -> &'a mut ClinicalUseDefinition_IndicationBuilder {
        self.value["undesirableEffect"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }
}
