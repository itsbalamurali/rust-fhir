#![allow(unused_imports, non_camel_case_types)]

use crate::models::r5::CodeableConcept::CodeableConcept;
use crate::models::r5::Element::Element;
use crate::models::r5::Extension::Extension;
use crate::models::r5::Identifier::Identifier;
use crate::models::r5::Reference::Reference;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A process where a researcher or organization plans and then executes a series of
/// steps intended to increase the field of healthcare-related knowledge.  This
/// includes studies of safety, efficacy, comparative effectiveness and other
/// information about medications, devices, therapies and other interventional and
/// investigative techniques. A ResearchStudy involves the gathering of information
/// about human or animal subjects or stability data about drug products or drug
/// substances.

#[derive(Debug)]
pub struct ResearchStudy_ComparisonGroup<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl ResearchStudy_ComparisonGroup<'_> {
    pub fn new(value: &Value) -> ResearchStudy_ComparisonGroup {
        ResearchStudy_ComparisonGroup {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for description
    pub fn _description(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_description") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for identifierUri
    pub fn _identifier_uri(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_identifierUri") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for name
    pub fn _name(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_name") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A succinct description of the path through the study that would be followed by a
    /// subject adhering to this comparisonGroup.
    pub fn description(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("description") {
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

    /// Allows the comparisonGroup for the study and the comparisonGroup for the subject
    /// to be linked easily.
    pub fn identifier_identifier(&self) -> Option<Identifier> {
        if let Some(val) = self.value.get("identifierIdentifier") {
            return Some(Identifier {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Allows the comparisonGroup for the study and the comparisonGroup for the subject
    /// to be linked easily.
    pub fn identifier_uri(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("identifierUri") {
            return Some(string);
        }
        return None;
    }

    /// Interventions or exposures in this comparisonGroup or cohort.
    pub fn intended_exposure(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("intendedExposure") {
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

    /// Unique, human-readable label for this comparisonGroup of the study.
    pub fn name(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("name") {
            return Some(string);
        }
        return None;
    }

    /// Group of participants who were enrolled in study comparisonGroup.
    pub fn observed_group(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("observedGroup") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Categorization of study comparisonGroup, e.g. experimental, active comparator,
    /// placebo comparater.
    pub fn fhir_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("type") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._description() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._identifier_uri() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._name() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.description() {}
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.identifier_identifier() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.identifier_uri() {}
        if let Some(_val) = self.intended_exposure() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.name() {}
        if let Some(_val) = self.observed_group() {
            if !_val.validate() {
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
pub struct ResearchStudy_ComparisonGroupBuilder {
    pub(crate) value: Value,
}

impl ResearchStudy_ComparisonGroupBuilder {
    pub fn build(&self) -> ResearchStudy_ComparisonGroup {
        ResearchStudy_ComparisonGroup {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: ResearchStudy_ComparisonGroup) -> ResearchStudy_ComparisonGroupBuilder {
        ResearchStudy_ComparisonGroupBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> ResearchStudy_ComparisonGroupBuilder {
        let mut __value: Value = json!({});
        return ResearchStudy_ComparisonGroupBuilder { value: __value };
    }

    pub fn _description<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut ResearchStudy_ComparisonGroupBuilder {
        self.value["_description"] = json!(val.value);
        return self;
    }

    pub fn _identifier_uri<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut ResearchStudy_ComparisonGroupBuilder {
        self.value["_identifierUri"] = json!(val.value);
        return self;
    }

    pub fn _name<'a>(&'a mut self, val: Element) -> &'a mut ResearchStudy_ComparisonGroupBuilder {
        self.value["_name"] = json!(val.value);
        return self;
    }

    pub fn description<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut ResearchStudy_ComparisonGroupBuilder {
        self.value["description"] = json!(val);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ResearchStudy_ComparisonGroupBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut ResearchStudy_ComparisonGroupBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn identifier_identifier<'a>(
        &'a mut self,
        val: Identifier,
    ) -> &'a mut ResearchStudy_ComparisonGroupBuilder {
        self.value["identifierIdentifier"] = json!(val.value);
        return self;
    }

    pub fn identifier_uri<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut ResearchStudy_ComparisonGroupBuilder {
        self.value["identifierUri"] = json!(val);
        return self;
    }

    pub fn intended_exposure<'a>(
        &'a mut self,
        val: Vec<Reference>,
    ) -> &'a mut ResearchStudy_ComparisonGroupBuilder {
        self.value["intendedExposure"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ResearchStudy_ComparisonGroupBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn name<'a>(&'a mut self, val: &str) -> &'a mut ResearchStudy_ComparisonGroupBuilder {
        self.value["name"] = json!(val);
        return self;
    }

    pub fn observed_group<'a>(
        &'a mut self,
        val: Reference,
    ) -> &'a mut ResearchStudy_ComparisonGroupBuilder {
        self.value["observedGroup"] = json!(val.value);
        return self;
    }

    pub fn fhir_type<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut ResearchStudy_ComparisonGroupBuilder {
        self.value["type"] = json!(val.value);
        return self;
    }
}
