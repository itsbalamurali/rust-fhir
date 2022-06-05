#![allow(unused_imports, non_camel_case_types)]

use crate::models::r5::CodeableConcept::CodeableConcept;
use crate::models::r5::Element::Element;
use crate::models::r5::Extension::Extension;
use crate::models::r5::Identifier::Identifier;
use crate::models::r5::Meta::Meta;
use crate::models::r5::Narrative::Narrative;
use crate::models::r5::Reference::Reference;
use crate::models::r5::ResourceList::ResourceList;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Describes a comparison of an immunization event against published recommendations
/// to determine if the administration is "valid" in relation to those
/// recommendations.

#[derive(Debug)]
pub struct ImmunizationEvaluation<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl ImmunizationEvaluation<'_> {
    pub fn new(value: &Value) -> ImmunizationEvaluation {
        ImmunizationEvaluation {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for date
    pub fn _date(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_date") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
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

    /// Extensions for doseNumber
    pub fn _dose_number(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_doseNumber") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
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

    /// Extensions for series
    pub fn _series(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_series") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for seriesDoses
    pub fn _series_doses(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_seriesDoses") {
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

    /// Indicates the authority who published the protocol (e.g. ACIP).
    pub fn authority(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("authority") {
            return Some(Reference {
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

    /// The date the evaluation of the vaccine administration event was performed.
    pub fn date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("date") {
            return Some(string);
        }
        return None;
    }

    /// Additional information about the evaluation.
    pub fn description(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("description") {
            return Some(string);
        }
        return None;
    }

    /// Nominal position in a series as determined by the outcome of the evaluation
    /// process.
    pub fn dose_number(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("doseNumber") {
            return Some(string);
        }
        return None;
    }

    /// Indicates if the dose is valid or not valid with respect to the published
    /// recommendations.
    pub fn dose_status(&self) -> CodeableConcept {
        CodeableConcept {
            value: Cow::Borrowed(&self.value["doseStatus"]),
        }
    }

    /// Provides an explanation as to why the vaccine administration event is valid or not
    /// relative to the published recommendations.
    pub fn dose_status_reason(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("doseStatusReason") {
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

    /// A unique identifier assigned to this immunization evaluation record.
    pub fn identifier(&self) -> Option<Vec<Identifier>> {
        if let Some(Value::Array(val)) = self.value.get("identifier") {
            return Some(
                val.into_iter()
                    .map(|e| Identifier {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The vaccine administration event being evaluated.
    pub fn immunization_event(&self) -> Reference {
        Reference {
            value: Cow::Borrowed(&self.value["immunizationEvent"]),
        }
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

    /// The base language in which the resource is written.
    pub fn language(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("language") {
            return Some(string);
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

    /// The individual for whom the evaluation is being done.
    pub fn patient(&self) -> Reference {
        Reference {
            value: Cow::Borrowed(&self.value["patient"]),
        }
    }

    /// One possible path to achieve presumed immunity against a disease - within the
    /// context of an authority.
    pub fn series(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("series") {
            return Some(string);
        }
        return None;
    }

    /// The recommended number of doses to achieve immunity as determined by the outcome
    /// of the evaluation process.
    pub fn series_doses(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("seriesDoses") {
            return Some(string);
        }
        return None;
    }

    /// Indicates the current status of the evaluation of the vaccination administration
    /// event.
    pub fn status(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("status") {
            return Some(string);
        }
        return None;
    }

    /// The vaccine preventable disease the dose is being evaluated against.
    pub fn target_disease(&self) -> CodeableConcept {
        CodeableConcept {
            value: Cow::Borrowed(&self.value["targetDisease"]),
        }
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
        if let Some(_val) = self._date() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._description() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._dose_number() {
            if !_val.validate() {
                return false;
            }
        }
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
        if let Some(_val) = self._series() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._series_doses() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._status() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.authority() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.contained() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.date() {}
        if let Some(_val) = self.description() {}
        if let Some(_val) = self.dose_number() {}
        if !self.dose_status().validate() {
            return false;
        }
        if let Some(_val) = self.dose_status_reason() {
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
        if let Some(_val) = self.identifier() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if !self.immunization_event().validate() {
            return false;
        }
        if let Some(_val) = self.implicit_rules() {}
        if let Some(_val) = self.language() {}
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
        if !self.patient().validate() {
            return false;
        }
        if let Some(_val) = self.series() {}
        if let Some(_val) = self.series_doses() {}
        if let Some(_val) = self.status() {}
        if !self.target_disease().validate() {
            return false;
        }
        if let Some(_val) = self.text() {
            if !_val.validate() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct ImmunizationEvaluationBuilder {
    pub(crate) value: Value,
}

impl ImmunizationEvaluationBuilder {
    pub fn build(&self) -> ImmunizationEvaluation {
        ImmunizationEvaluation {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: ImmunizationEvaluation) -> ImmunizationEvaluationBuilder {
        ImmunizationEvaluationBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(
        dose_status: CodeableConcept,
        immunization_event: Reference,
        patient: Reference,
        target_disease: CodeableConcept,
    ) -> ImmunizationEvaluationBuilder {
        let mut __value: Value = json!({});
        __value["doseStatus"] = json!(dose_status.value);
        __value["immunizationEvent"] = json!(immunization_event.value);
        __value["patient"] = json!(patient.value);
        __value["targetDisease"] = json!(target_disease.value);
        return ImmunizationEvaluationBuilder { value: __value };
    }

    pub fn _date<'a>(&'a mut self, val: Element) -> &'a mut ImmunizationEvaluationBuilder {
        self.value["_date"] = json!(val.value);
        return self;
    }

    pub fn _description<'a>(&'a mut self, val: Element) -> &'a mut ImmunizationEvaluationBuilder {
        self.value["_description"] = json!(val.value);
        return self;
    }

    pub fn _dose_number<'a>(&'a mut self, val: Element) -> &'a mut ImmunizationEvaluationBuilder {
        self.value["_doseNumber"] = json!(val.value);
        return self;
    }

    pub fn _implicit_rules<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut ImmunizationEvaluationBuilder {
        self.value["_implicitRules"] = json!(val.value);
        return self;
    }

    pub fn _language<'a>(&'a mut self, val: Element) -> &'a mut ImmunizationEvaluationBuilder {
        self.value["_language"] = json!(val.value);
        return self;
    }

    pub fn _series<'a>(&'a mut self, val: Element) -> &'a mut ImmunizationEvaluationBuilder {
        self.value["_series"] = json!(val.value);
        return self;
    }

    pub fn _series_doses<'a>(&'a mut self, val: Element) -> &'a mut ImmunizationEvaluationBuilder {
        self.value["_seriesDoses"] = json!(val.value);
        return self;
    }

    pub fn _status<'a>(&'a mut self, val: Element) -> &'a mut ImmunizationEvaluationBuilder {
        self.value["_status"] = json!(val.value);
        return self;
    }

    pub fn authority<'a>(&'a mut self, val: Reference) -> &'a mut ImmunizationEvaluationBuilder {
        self.value["authority"] = json!(val.value);
        return self;
    }

    pub fn contained<'a>(
        &'a mut self,
        val: Vec<ResourceList>,
    ) -> &'a mut ImmunizationEvaluationBuilder {
        self.value["contained"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn date<'a>(&'a mut self, val: &str) -> &'a mut ImmunizationEvaluationBuilder {
        self.value["date"] = json!(val);
        return self;
    }

    pub fn description<'a>(&'a mut self, val: &str) -> &'a mut ImmunizationEvaluationBuilder {
        self.value["description"] = json!(val);
        return self;
    }

    pub fn dose_number<'a>(&'a mut self, val: &str) -> &'a mut ImmunizationEvaluationBuilder {
        self.value["doseNumber"] = json!(val);
        return self;
    }

    pub fn dose_status_reason<'a>(
        &'a mut self,
        val: Vec<CodeableConcept>,
    ) -> &'a mut ImmunizationEvaluationBuilder {
        self.value["doseStatusReason"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ImmunizationEvaluationBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut ImmunizationEvaluationBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn identifier<'a>(
        &'a mut self,
        val: Vec<Identifier>,
    ) -> &'a mut ImmunizationEvaluationBuilder {
        self.value["identifier"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn implicit_rules<'a>(&'a mut self, val: &str) -> &'a mut ImmunizationEvaluationBuilder {
        self.value["implicitRules"] = json!(val);
        return self;
    }

    pub fn language<'a>(&'a mut self, val: &str) -> &'a mut ImmunizationEvaluationBuilder {
        self.value["language"] = json!(val);
        return self;
    }

    pub fn meta<'a>(&'a mut self, val: Meta) -> &'a mut ImmunizationEvaluationBuilder {
        self.value["meta"] = json!(val.value);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ImmunizationEvaluationBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn series<'a>(&'a mut self, val: &str) -> &'a mut ImmunizationEvaluationBuilder {
        self.value["series"] = json!(val);
        return self;
    }

    pub fn series_doses<'a>(&'a mut self, val: &str) -> &'a mut ImmunizationEvaluationBuilder {
        self.value["seriesDoses"] = json!(val);
        return self;
    }

    pub fn status<'a>(&'a mut self, val: &str) -> &'a mut ImmunizationEvaluationBuilder {
        self.value["status"] = json!(val);
        return self;
    }

    pub fn text<'a>(&'a mut self, val: Narrative) -> &'a mut ImmunizationEvaluationBuilder {
        self.value["text"] = json!(val.value);
        return self;
    }
}
