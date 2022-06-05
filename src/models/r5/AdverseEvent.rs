#![allow(unused_imports, non_camel_case_types)]

use crate::models::r5::AdverseEvent_ContributingFactor::AdverseEvent_ContributingFactor;
use crate::models::r5::AdverseEvent_MitigatingAction::AdverseEvent_MitigatingAction;
use crate::models::r5::AdverseEvent_Participant::AdverseEvent_Participant;
use crate::models::r5::AdverseEvent_PreventiveAction::AdverseEvent_PreventiveAction;
use crate::models::r5::AdverseEvent_SupportingInfo::AdverseEvent_SupportingInfo;
use crate::models::r5::AdverseEvent_SuspectEntity::AdverseEvent_SuspectEntity;
use crate::models::r5::CodeableConcept::CodeableConcept;
use crate::models::r5::Element::Element;
use crate::models::r5::Extension::Extension;
use crate::models::r5::Identifier::Identifier;
use crate::models::r5::Meta::Meta;
use crate::models::r5::Narrative::Narrative;
use crate::models::r5::Period::Period;
use crate::models::r5::Reference::Reference;
use crate::models::r5::ResourceList::ResourceList;
use crate::models::r5::Timing::Timing;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// An event (i.e. any change to current patient status) that may be related to
/// unintended effects on a patient or research subject.  The unintended effects may
/// require additional monitoring, treatment or hospitalization or may result in
/// death.  The AdverseEvent resource also extends to potential or avoided events
/// that could have had such effects.

#[derive(Debug)]
pub struct AdverseEvent<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl AdverseEvent<'_> {
    pub fn new(value: &Value) -> AdverseEvent {
        AdverseEvent {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for actuality
    pub fn _actuality(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_actuality") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for detected
    pub fn _detected(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_detected") {
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

    /// Extensions for occurrenceDateTime
    pub fn _occurrence_date_time(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_occurrenceDateTime") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for recordedDate
    pub fn _recorded_date(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_recordedDate") {
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

    /// Whether the event actually happened, or just had the potential to. Note that
    /// this is independent of whether anyone was affected or harmed or how severely.
    pub fn actuality(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("actuality") {
            return Some(string);
        }
        return None;
    }

    /// The overall type of event, intended for search and filtering purposes.
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

    /// Specific event that occurred or that was averted, such as patient fall, wrong
    /// organ removed, or wrong blood transfused.
    pub fn code(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("code") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope.
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

    /// The contributing factors suspected to have increased the probability or severity
    /// of the adverse event.
    pub fn contributing_factor(&self) -> Option<Vec<AdverseEvent_ContributingFactor>> {
        if let Some(Value::Array(val)) = self.value.get("contributingFactor") {
            return Some(
                val.into_iter()
                    .map(|e| AdverseEvent_ContributingFactor {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Estimated or actual date the AdverseEvent began, in the opinion of the reporter.
    pub fn detected(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("detected") {
            return Some(string);
        }
        return None;
    }

    /// The Encounter associated with the start of the AdverseEvent.
    pub fn encounter(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("encounter") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
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

    /// Business identifiers assigned to this adverse event by the performer or other
    /// systems which remain constant as the resource is updated and propagates from
    /// server to server.
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

    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
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

    /// The information about where the adverse event occurred.
    pub fn location(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("location") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The metadata about the resource. This is content that is maintained by the
    /// infrastructure. Changes to the content might not always be associated with
    /// version changes to the resource.
    pub fn meta(&self) -> Option<Meta> {
        if let Some(val) = self.value.get("meta") {
            return Some(Meta {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The ameliorating action taken after the adverse event occured in order to reduce
    /// the extent of harm.
    pub fn mitigating_action(&self) -> Option<Vec<AdverseEvent_MitigatingAction>> {
        if let Some(Value::Array(val)) = self.value.get("mitigatingAction") {
            return Some(
                val.into_iter()
                    .map(|e| AdverseEvent_MitigatingAction {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// May be used to represent additional information that is not part of the basic
    /// definition of the resource and that modifies the understanding of the element
    /// that contains it and/or the understanding of the containing element's
    /// descendants. Usually modifier elements provide negation or qualification. To
    /// make the use of extensions safe and manageable, there is a strict set of
    /// governance applied to the definition and use of extensions. Though any
    /// implementer is allowed to define an extension, there is a set of requirements
    /// that SHALL be met as part of the definition of the extension. Applications
    /// processing a resource are required to check for modifier extensions.    Modifier
    /// extensions SHALL NOT change the meaning of any elements on Resource or
    /// DomainResource (including cannot change the meaning of modifierExtension
    /// itself).
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

    /// The date (and perhaps time) when the adverse event occurred.
    pub fn occurrence_date_time(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("occurrenceDateTime") {
            return Some(string);
        }
        return None;
    }

    /// The date (and perhaps time) when the adverse event occurred.
    pub fn occurrence_period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("occurrencePeriod") {
            return Some(Period {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The date (and perhaps time) when the adverse event occurred.
    pub fn occurrence_timing(&self) -> Option<Timing> {
        if let Some(val) = self.value.get("occurrenceTiming") {
            return Some(Timing {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Describes the type of outcome from the adverse event, such as resolved,
    /// recovering, ongoing, resolved-with-sequelae, or fatal.
    pub fn outcome(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("outcome") {
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

    /// Indicates who or what participated in the adverse event and how they were
    /// involved.
    pub fn participant(&self) -> Option<Vec<AdverseEvent_Participant>> {
        if let Some(Value::Array(val)) = self.value.get("participant") {
            return Some(
                val.into_iter()
                    .map(|e| AdverseEvent_Participant {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Preventive actions that contributed to avoiding the adverse event.
    pub fn preventive_action(&self) -> Option<Vec<AdverseEvent_PreventiveAction>> {
        if let Some(Value::Array(val)) = self.value.get("preventiveAction") {
            return Some(
                val.into_iter()
                    .map(|e| AdverseEvent_PreventiveAction {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The date on which the existence of the AdverseEvent was first recorded.
    pub fn recorded_date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("recordedDate") {
            return Some(string);
        }
        return None;
    }

    /// Information on who recorded the adverse event.  May be the patient or a
    /// practitioner.
    pub fn recorder(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("recorder") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Information about the condition that occurred as a result of the adverse event,
    /// such as hives due to the exposure to a substance (for example, a drug or a
    /// chemical) or a broken leg as a result of the fall.
    pub fn resulting_condition(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("resultingCondition") {
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

    /// Assessment whether this event, or averted event, was of clinical importance.
    pub fn seriousness(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("seriousness") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The current state of the adverse event or potential adverse event.
    pub fn status(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("status") {
            return Some(string);
        }
        return None;
    }

    /// The research study that the subject is enrolled in.
    pub fn study(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("study") {
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

    /// This subject or group impacted by the event.
    pub fn subject(&self) -> Reference {
        Reference {
            value: Cow::Borrowed(&self.value["subject"]),
        }
    }

    /// Supporting information relevant to the event.
    pub fn supporting_info(&self) -> Option<Vec<AdverseEvent_SupportingInfo>> {
        if let Some(Value::Array(val)) = self.value.get("supportingInfo") {
            return Some(
                val.into_iter()
                    .map(|e| AdverseEvent_SupportingInfo {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Describes the entity that is suspected to have caused the adverse event.
    pub fn suspect_entity(&self) -> Option<Vec<AdverseEvent_SuspectEntity>> {
        if let Some(Value::Array(val)) = self.value.get("suspectEntity") {
            return Some(
                val.into_iter()
                    .map(|e| AdverseEvent_SuspectEntity {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need not
    /// encode all the structured data, but is required to contain sufficient detail to
    /// make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub fn text(&self) -> Option<Narrative> {
        if let Some(val) = self.value.get("text") {
            return Some(Narrative {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._actuality() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._detected() {
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
        if let Some(_val) = self._occurrence_date_time() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._recorded_date() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._status() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.actuality() {}
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
        if let Some(_val) = self.contributing_factor() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.detected() {}
        if let Some(_val) = self.encounter() {
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
        if let Some(_val) = self.identifier() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.implicit_rules() {}
        if let Some(_val) = self.language() {}
        if let Some(_val) = self.location() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.meta() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.mitigating_action() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.occurrence_date_time() {}
        if let Some(_val) = self.occurrence_period() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.occurrence_timing() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.outcome() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.participant() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.preventive_action() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.recorded_date() {}
        if let Some(_val) = self.recorder() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.resulting_condition() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.seriousness() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.status() {}
        if let Some(_val) = self.study() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if !self.subject().validate() {
            return false;
        }
        if let Some(_val) = self.supporting_info() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.suspect_entity() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
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
pub struct AdverseEventBuilder {
    pub(crate) value: Value,
}

impl AdverseEventBuilder {
    pub fn build(&self) -> AdverseEvent {
        AdverseEvent {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: AdverseEvent) -> AdverseEventBuilder {
        AdverseEventBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(subject: Reference) -> AdverseEventBuilder {
        let mut __value: Value = json!({});
        __value["subject"] = json!(subject.value);
        return AdverseEventBuilder { value: __value };
    }

    pub fn _actuality<'a>(&'a mut self, val: Element) -> &'a mut AdverseEventBuilder {
        self.value["_actuality"] = json!(val.value);
        return self;
    }

    pub fn _detected<'a>(&'a mut self, val: Element) -> &'a mut AdverseEventBuilder {
        self.value["_detected"] = json!(val.value);
        return self;
    }

    pub fn _implicit_rules<'a>(&'a mut self, val: Element) -> &'a mut AdverseEventBuilder {
        self.value["_implicitRules"] = json!(val.value);
        return self;
    }

    pub fn _language<'a>(&'a mut self, val: Element) -> &'a mut AdverseEventBuilder {
        self.value["_language"] = json!(val.value);
        return self;
    }

    pub fn _occurrence_date_time<'a>(&'a mut self, val: Element) -> &'a mut AdverseEventBuilder {
        self.value["_occurrenceDateTime"] = json!(val.value);
        return self;
    }

    pub fn _recorded_date<'a>(&'a mut self, val: Element) -> &'a mut AdverseEventBuilder {
        self.value["_recordedDate"] = json!(val.value);
        return self;
    }

    pub fn _status<'a>(&'a mut self, val: Element) -> &'a mut AdverseEventBuilder {
        self.value["_status"] = json!(val.value);
        return self;
    }

    pub fn actuality<'a>(&'a mut self, val: &str) -> &'a mut AdverseEventBuilder {
        self.value["actuality"] = json!(val);
        return self;
    }

    pub fn category<'a>(&'a mut self, val: Vec<CodeableConcept>) -> &'a mut AdverseEventBuilder {
        self.value["category"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn code<'a>(&'a mut self, val: CodeableConcept) -> &'a mut AdverseEventBuilder {
        self.value["code"] = json!(val.value);
        return self;
    }

    pub fn contained<'a>(&'a mut self, val: Vec<ResourceList>) -> &'a mut AdverseEventBuilder {
        self.value["contained"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn contributing_factor<'a>(
        &'a mut self,
        val: Vec<AdverseEvent_ContributingFactor>,
    ) -> &'a mut AdverseEventBuilder {
        self.value["contributingFactor"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn detected<'a>(&'a mut self, val: &str) -> &'a mut AdverseEventBuilder {
        self.value["detected"] = json!(val);
        return self;
    }

    pub fn encounter<'a>(&'a mut self, val: Reference) -> &'a mut AdverseEventBuilder {
        self.value["encounter"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut AdverseEventBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut AdverseEventBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn identifier<'a>(&'a mut self, val: Vec<Identifier>) -> &'a mut AdverseEventBuilder {
        self.value["identifier"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn implicit_rules<'a>(&'a mut self, val: &str) -> &'a mut AdverseEventBuilder {
        self.value["implicitRules"] = json!(val);
        return self;
    }

    pub fn language<'a>(&'a mut self, val: &str) -> &'a mut AdverseEventBuilder {
        self.value["language"] = json!(val);
        return self;
    }

    pub fn location<'a>(&'a mut self, val: Reference) -> &'a mut AdverseEventBuilder {
        self.value["location"] = json!(val.value);
        return self;
    }

    pub fn meta<'a>(&'a mut self, val: Meta) -> &'a mut AdverseEventBuilder {
        self.value["meta"] = json!(val.value);
        return self;
    }

    pub fn mitigating_action<'a>(
        &'a mut self,
        val: Vec<AdverseEvent_MitigatingAction>,
    ) -> &'a mut AdverseEventBuilder {
        self.value["mitigatingAction"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut AdverseEventBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn occurrence_date_time<'a>(&'a mut self, val: &str) -> &'a mut AdverseEventBuilder {
        self.value["occurrenceDateTime"] = json!(val);
        return self;
    }

    pub fn occurrence_period<'a>(&'a mut self, val: Period) -> &'a mut AdverseEventBuilder {
        self.value["occurrencePeriod"] = json!(val.value);
        return self;
    }

    pub fn occurrence_timing<'a>(&'a mut self, val: Timing) -> &'a mut AdverseEventBuilder {
        self.value["occurrenceTiming"] = json!(val.value);
        return self;
    }

    pub fn outcome<'a>(&'a mut self, val: Vec<CodeableConcept>) -> &'a mut AdverseEventBuilder {
        self.value["outcome"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn participant<'a>(
        &'a mut self,
        val: Vec<AdverseEvent_Participant>,
    ) -> &'a mut AdverseEventBuilder {
        self.value["participant"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn preventive_action<'a>(
        &'a mut self,
        val: Vec<AdverseEvent_PreventiveAction>,
    ) -> &'a mut AdverseEventBuilder {
        self.value["preventiveAction"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn recorded_date<'a>(&'a mut self, val: &str) -> &'a mut AdverseEventBuilder {
        self.value["recordedDate"] = json!(val);
        return self;
    }

    pub fn recorder<'a>(&'a mut self, val: Reference) -> &'a mut AdverseEventBuilder {
        self.value["recorder"] = json!(val.value);
        return self;
    }

    pub fn resulting_condition<'a>(
        &'a mut self,
        val: Vec<Reference>,
    ) -> &'a mut AdverseEventBuilder {
        self.value["resultingCondition"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn seriousness<'a>(&'a mut self, val: CodeableConcept) -> &'a mut AdverseEventBuilder {
        self.value["seriousness"] = json!(val.value);
        return self;
    }

    pub fn status<'a>(&'a mut self, val: &str) -> &'a mut AdverseEventBuilder {
        self.value["status"] = json!(val);
        return self;
    }

    pub fn study<'a>(&'a mut self, val: Vec<Reference>) -> &'a mut AdverseEventBuilder {
        self.value["study"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn supporting_info<'a>(
        &'a mut self,
        val: Vec<AdverseEvent_SupportingInfo>,
    ) -> &'a mut AdverseEventBuilder {
        self.value["supportingInfo"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn suspect_entity<'a>(
        &'a mut self,
        val: Vec<AdverseEvent_SuspectEntity>,
    ) -> &'a mut AdverseEventBuilder {
        self.value["suspectEntity"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn text<'a>(&'a mut self, val: Narrative) -> &'a mut AdverseEventBuilder {
        self.value["text"] = json!(val.value);
        return self;
    }
}
