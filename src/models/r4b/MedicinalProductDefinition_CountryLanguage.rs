#![allow(unused_imports, non_camel_case_types)]

use crate::models::r4b::CodeableConcept::CodeableConcept;
use crate::models::r4b::Extension::Extension;
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
pub struct MedicinalProductDefinition_CountryLanguage<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl MedicinalProductDefinition_CountryLanguage<'_> {
    pub fn new(value: &Value) -> MedicinalProductDefinition_CountryLanguage {
        MedicinalProductDefinition_CountryLanguage {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Country code for where this name applies.
    pub fn country(&self) -> CodeableConcept {
        CodeableConcept {
            value: Cow::Borrowed(&self.value["country"]),
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

    /// Unique id for the element within a resource (for internal references). This may be
    /// any string value that does not contain spaces.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
        }
        return None;
    }

    /// Jurisdiction code for where this name applies. A jurisdiction may be a sub- or
    /// supra-national entity (e.g. a state or a geographic region).
    pub fn jurisdiction(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("jurisdiction") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Language code for this name.
    pub fn language(&self) -> CodeableConcept {
        CodeableConcept {
            value: Cow::Borrowed(&self.value["language"]),
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

    pub fn validate(&self) -> bool {
        if !self.country().validate() {
            return false;
        }
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.jurisdiction() {
            if !_val.validate() {
                return false;
            }
        }
        if !self.language().validate() {
            return false;
        }
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct MedicinalProductDefinition_CountryLanguageBuilder {
    pub(crate) value: Value,
}

impl MedicinalProductDefinition_CountryLanguageBuilder {
    pub fn build(&self) -> MedicinalProductDefinition_CountryLanguage {
        MedicinalProductDefinition_CountryLanguage {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(
        existing: MedicinalProductDefinition_CountryLanguage,
    ) -> MedicinalProductDefinition_CountryLanguageBuilder {
        MedicinalProductDefinition_CountryLanguageBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(
        country: CodeableConcept,
        language: CodeableConcept,
    ) -> MedicinalProductDefinition_CountryLanguageBuilder {
        let mut __value: Value = json!({});
        __value["country"] = json!(country.value);
        __value["language"] = json!(language.value);
        return MedicinalProductDefinition_CountryLanguageBuilder { value: __value };
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut MedicinalProductDefinition_CountryLanguageBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut MedicinalProductDefinition_CountryLanguageBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn jurisdiction<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut MedicinalProductDefinition_CountryLanguageBuilder {
        self.value["jurisdiction"] = json!(val.value);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut MedicinalProductDefinition_CountryLanguageBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }
}
