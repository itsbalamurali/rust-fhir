#![allow(unused_imports, non_camel_case_types)]

use crate::models::r5::Age::Age;
use crate::models::r5::CodeableConcept::CodeableConcept;
use crate::models::r5::CodeableReference::CodeableReference;
use crate::models::r5::Duration::Duration;
use crate::models::r5::Element::Element;
use crate::models::r5::Extension::Extension;
use crate::models::r5::PlanDefinition_Condition::PlanDefinition_Condition;
use crate::models::r5::PlanDefinition_DynamicValue::PlanDefinition_DynamicValue;
use crate::models::r5::PlanDefinition_Input::PlanDefinition_Input;
use crate::models::r5::PlanDefinition_Output::PlanDefinition_Output;
use crate::models::r5::PlanDefinition_Participant::PlanDefinition_Participant;
use crate::models::r5::PlanDefinition_RelatedAction::PlanDefinition_RelatedAction;
use crate::models::r5::Range::Range;
use crate::models::r5::Reference::Reference;
use crate::models::r5::RelatedArtifact::RelatedArtifact;
use crate::models::r5::Timing::Timing;
use crate::models::r5::TriggerDefinition::TriggerDefinition;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// This resource allows for the definition of various types of plans as a sharable,
/// consumable, and executable artifact. The resource is general enough to support
/// the description of a broad range of clinical and non-clinical artifacts such
/// as clinical decision support rules, order sets, protocols, and drug quality
/// specifications.

#[derive(Debug)]
pub struct PlanDefinition_Action<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl PlanDefinition_Action<'_> {
    pub fn new(value: &Value) -> PlanDefinition_Action {
        PlanDefinition_Action {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for cardinalityBehavior
    pub fn _cardinality_behavior(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_cardinalityBehavior") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for definitionCanonical
    pub fn _definition_canonical(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_definitionCanonical") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for definitionUri
    pub fn _definition_uri(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_definitionUri") {
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

    /// Extensions for goalId
    pub fn _goal_id(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_goalId") {
            return Some(
                val.into_iter()
                    .map(|e| Element {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for groupingBehavior
    pub fn _grouping_behavior(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_groupingBehavior") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for linkId
    pub fn _link_id(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_linkId") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for precheckBehavior
    pub fn _precheck_behavior(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_precheckBehavior") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for prefix
    pub fn _prefix(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_prefix") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for priority
    pub fn _priority(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_priority") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for requiredBehavior
    pub fn _required_behavior(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_requiredBehavior") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for selectionBehavior
    pub fn _selection_behavior(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_selectionBehavior") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for subjectCanonical
    pub fn _subject_canonical(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_subjectCanonical") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for textEquivalent
    pub fn _text_equivalent(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_textEquivalent") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for title
    pub fn _title(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_title") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Sub actions that are contained within the action. The behavior of this action
    /// determines the functionality of the sub-actions. For example, a selection behavior
    /// of at-most-one indicates that of the sub-actions, at most one may be chosen as
    /// part of realizing the action definition.
    pub fn action(&self) -> Option<Vec<PlanDefinition_Action>> {
        if let Some(Value::Array(val)) = self.value.get("action") {
            return Some(
                val.into_iter()
                    .map(|e| PlanDefinition_Action {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Defines whether the action can be selected multiple times.
    pub fn cardinality_behavior(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("cardinalityBehavior") {
            return Some(string);
        }
        return None;
    }

    /// A code that provides a meaning, grouping, or classification for the action or
    /// action group. For example, a section may have a LOINC code for the section of a
    /// documentation template. In pharmaceutical quality, an action (Test) such as pH
    /// could be classified as a physical property.
    pub fn code(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("code") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// An expression that describes applicability criteria or start/stop conditions for
    /// the action.
    pub fn condition(&self) -> Option<Vec<PlanDefinition_Condition>> {
        if let Some(Value::Array(val)) = self.value.get("condition") {
            return Some(
                val.into_iter()
                    .map(|e| PlanDefinition_Condition {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A reference to an ActivityDefinition that describes the action to be taken
    /// in detail, a PlanDefinition that describes a series of actions to be taken,
    /// a Questionnaire that should be filled out, or an ObservationDefinition that
    /// specifies what observation should be captured.
    pub fn definition_canonical(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("definitionCanonical") {
            return Some(string);
        }
        return None;
    }

    /// A reference to an ActivityDefinition that describes the action to be taken
    /// in detail, a PlanDefinition that describes a series of actions to be taken,
    /// a Questionnaire that should be filled out, or an ObservationDefinition that
    /// specifies what observation should be captured.
    pub fn definition_uri(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("definitionUri") {
            return Some(string);
        }
        return None;
    }

    /// A brief description of the action used to provide a summary to display to the
    /// user.
    pub fn description(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("description") {
            return Some(string);
        }
        return None;
    }

    /// Didactic or other informational resources associated with the action that can
    /// be provided to the CDS recipient. Information resources can include inline text
    /// commentary and links to web resources.
    pub fn documentation(&self) -> Option<Vec<RelatedArtifact>> {
        if let Some(Value::Array(val)) = self.value.get("documentation") {
            return Some(
                val.into_iter()
                    .map(|e| RelatedArtifact {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Customizations that should be applied to the statically defined resource. For
    /// example, if the dosage of a medication must be computed based on the patient's
    /// weight, a customization would be used to specify an expression that calculated the
    /// weight, and the path on the resource that would contain the result.
    pub fn dynamic_value(&self) -> Option<Vec<PlanDefinition_DynamicValue>> {
        if let Some(Value::Array(val)) = self.value.get("dynamicValue") {
            return Some(
                val.into_iter()
                    .map(|e| PlanDefinition_DynamicValue {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
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

    /// Identifies goals that this action supports. The reference must be to a goal
    /// element defined within this plan definition. In pharmaceutical quality, a goal
    /// represents acceptance criteria (Goal) for a given action (Test), so the goalId
    /// would be the unique id of a defined goal element establishing the acceptance
    /// criteria for the action.
    pub fn goal_id(&self) -> Option<Vec<&str>> {
        if let Some(Value::Array(val)) = self.value.get("goalId") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_str().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Defines the grouping behavior for the action and its children.
    pub fn grouping_behavior(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("groupingBehavior") {
            return Some(string);
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

    /// Defines input data requirements for the action.
    pub fn input(&self) -> Option<Vec<PlanDefinition_Input>> {
        if let Some(Value::Array(val)) = self.value.get("input") {
            return Some(
                val.into_iter()
                    .map(|e| PlanDefinition_Input {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// An identifier that is unique within the PlanDefinition to allow linkage within the
    /// realized CarePlan and/or RequestGroup.
    pub fn link_id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("linkId") {
            return Some(string);
        }
        return None;
    }

    /// Identifies the facility where the action will occur; e.g. home, hospital, specific
    /// clinic, etc.
    pub fn location(&self) -> Option<CodeableReference> {
        if let Some(val) = self.value.get("location") {
            return Some(CodeableReference {
                value: Cow::Borrowed(val),
            });
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

    /// Defines the outputs of the action, if any.
    pub fn output(&self) -> Option<Vec<PlanDefinition_Output>> {
        if let Some(Value::Array(val)) = self.value.get("output") {
            return Some(
                val.into_iter()
                    .map(|e| PlanDefinition_Output {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Indicates who should participate in performing the action described.
    pub fn participant(&self) -> Option<Vec<PlanDefinition_Participant>> {
        if let Some(Value::Array(val)) = self.value.get("participant") {
            return Some(
                val.into_iter()
                    .map(|e| PlanDefinition_Participant {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Defines whether the action should usually be preselected.
    pub fn precheck_behavior(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("precheckBehavior") {
            return Some(string);
        }
        return None;
    }

    /// A user-visible prefix for the action. For example a section or item numbering such
    /// as 1. or A.
    pub fn prefix(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("prefix") {
            return Some(string);
        }
        return None;
    }

    /// Indicates how quickly the action should be addressed with respect to other
    /// actions.
    pub fn priority(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("priority") {
            return Some(string);
        }
        return None;
    }

    /// A description of why this action is necessary or appropriate.
    pub fn reason(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("reason") {
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

    /// A relationship to another action such as "before" or "30-60 minutes after start
    /// of".
    pub fn related_action(&self) -> Option<Vec<PlanDefinition_RelatedAction>> {
        if let Some(Value::Array(val)) = self.value.get("relatedAction") {
            return Some(
                val.into_iter()
                    .map(|e| PlanDefinition_RelatedAction {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Defines the required behavior for the action.
    pub fn required_behavior(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("requiredBehavior") {
            return Some(string);
        }
        return None;
    }

    /// Defines the selection behavior for the action and its children.
    pub fn selection_behavior(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("selectionBehavior") {
            return Some(string);
        }
        return None;
    }

    /// A code, group definition, or canonical reference that describes the intended
    /// subject of the action and its children, if any. Canonical references are
    /// allowed to support the definition of protocols for drug and substance quality
    /// specifications, and is allowed to reference a MedicinalProductDefinition,
    /// SubstanceDefinition, AdministrableProductDefinition, ManufacturedItemDefinition,
    /// or PackagedProductDefinition resource.
    pub fn subject_canonical(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("subjectCanonical") {
            return Some(string);
        }
        return None;
    }

    /// A code, group definition, or canonical reference that describes the intended
    /// subject of the action and its children, if any. Canonical references are
    /// allowed to support the definition of protocols for drug and substance quality
    /// specifications, and is allowed to reference a MedicinalProductDefinition,
    /// SubstanceDefinition, AdministrableProductDefinition, ManufacturedItemDefinition,
    /// or PackagedProductDefinition resource.
    pub fn subject_codeable_concept(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("subjectCodeableConcept") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A code, group definition, or canonical reference that describes the intended
    /// subject of the action and its children, if any. Canonical references are
    /// allowed to support the definition of protocols for drug and substance quality
    /// specifications, and is allowed to reference a MedicinalProductDefinition,
    /// SubstanceDefinition, AdministrableProductDefinition, ManufacturedItemDefinition,
    /// or PackagedProductDefinition resource.
    pub fn subject_reference(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("subjectReference") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A text equivalent of the action to be performed. This provides a human-
    /// interpretable description of the action when the definition is consumed by a
    /// system that might not be capable of interpreting it dynamically.
    pub fn text_equivalent(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("textEquivalent") {
            return Some(string);
        }
        return None;
    }

    /// An optional value describing when the action should be performed.
    pub fn timing_age(&self) -> Option<Age> {
        if let Some(val) = self.value.get("timingAge") {
            return Some(Age {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// An optional value describing when the action should be performed.
    pub fn timing_duration(&self) -> Option<Duration> {
        if let Some(val) = self.value.get("timingDuration") {
            return Some(Duration {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// An optional value describing when the action should be performed.
    pub fn timing_range(&self) -> Option<Range> {
        if let Some(val) = self.value.get("timingRange") {
            return Some(Range {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// An optional value describing when the action should be performed.
    pub fn timing_timing(&self) -> Option<Timing> {
        if let Some(val) = self.value.get("timingTiming") {
            return Some(Timing {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The textual description of the action displayed to a user. For example, when the
    /// action is a test to be performed, the title would be the title of the test such as
    /// Assay by HPLC.
    pub fn title(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("title") {
            return Some(string);
        }
        return None;
    }

    /// A reference to a StructureMap resource that defines a transform that can be
    /// executed to produce the intent resource using the ActivityDefinition instance as
    /// the input.
    pub fn transform(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("transform") {
            return Some(string);
        }
        return None;
    }

    /// A description of when the action should be triggered.
    pub fn trigger(&self) -> Option<Vec<TriggerDefinition>> {
        if let Some(Value::Array(val)) = self.value.get("trigger") {
            return Some(
                val.into_iter()
                    .map(|e| TriggerDefinition {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The type of action to perform (create, update, remove).
    pub fn fhir_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("type") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._cardinality_behavior() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._definition_canonical() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._definition_uri() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._description() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._goal_id() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self._grouping_behavior() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._link_id() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._precheck_behavior() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._prefix() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._priority() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._required_behavior() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._selection_behavior() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._subject_canonical() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._text_equivalent() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._title() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.action() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.cardinality_behavior() {}
        if let Some(_val) = self.code() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.condition() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.definition_canonical() {}
        if let Some(_val) = self.definition_uri() {}
        if let Some(_val) = self.description() {}
        if let Some(_val) = self.documentation() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.dynamic_value() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.goal_id() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.grouping_behavior() {}
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.input() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.link_id() {}
        if let Some(_val) = self.location() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.output() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.participant() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.precheck_behavior() {}
        if let Some(_val) = self.prefix() {}
        if let Some(_val) = self.priority() {}
        if let Some(_val) = self.reason() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.related_action() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.required_behavior() {}
        if let Some(_val) = self.selection_behavior() {}
        if let Some(_val) = self.subject_canonical() {}
        if let Some(_val) = self.subject_codeable_concept() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.subject_reference() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.text_equivalent() {}
        if let Some(_val) = self.timing_age() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.timing_duration() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.timing_range() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.timing_timing() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.title() {}
        if let Some(_val) = self.transform() {}
        if let Some(_val) = self.trigger() {
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
pub struct PlanDefinition_ActionBuilder {
    pub(crate) value: Value,
}

impl PlanDefinition_ActionBuilder {
    pub fn build(&self) -> PlanDefinition_Action {
        PlanDefinition_Action {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: PlanDefinition_Action) -> PlanDefinition_ActionBuilder {
        PlanDefinition_ActionBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> PlanDefinition_ActionBuilder {
        let mut __value: Value = json!({});
        return PlanDefinition_ActionBuilder { value: __value };
    }

    pub fn _cardinality_behavior<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut PlanDefinition_ActionBuilder {
        self.value["_cardinalityBehavior"] = json!(val.value);
        return self;
    }

    pub fn _definition_canonical<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut PlanDefinition_ActionBuilder {
        self.value["_definitionCanonical"] = json!(val.value);
        return self;
    }

    pub fn _definition_uri<'a>(&'a mut self, val: Element) -> &'a mut PlanDefinition_ActionBuilder {
        self.value["_definitionUri"] = json!(val.value);
        return self;
    }

    pub fn _description<'a>(&'a mut self, val: Element) -> &'a mut PlanDefinition_ActionBuilder {
        self.value["_description"] = json!(val.value);
        return self;
    }

    pub fn _goal_id<'a>(&'a mut self, val: Vec<Element>) -> &'a mut PlanDefinition_ActionBuilder {
        self.value["_goalId"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn _grouping_behavior<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut PlanDefinition_ActionBuilder {
        self.value["_groupingBehavior"] = json!(val.value);
        return self;
    }

    pub fn _link_id<'a>(&'a mut self, val: Element) -> &'a mut PlanDefinition_ActionBuilder {
        self.value["_linkId"] = json!(val.value);
        return self;
    }

    pub fn _precheck_behavior<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut PlanDefinition_ActionBuilder {
        self.value["_precheckBehavior"] = json!(val.value);
        return self;
    }

    pub fn _prefix<'a>(&'a mut self, val: Element) -> &'a mut PlanDefinition_ActionBuilder {
        self.value["_prefix"] = json!(val.value);
        return self;
    }

    pub fn _priority<'a>(&'a mut self, val: Element) -> &'a mut PlanDefinition_ActionBuilder {
        self.value["_priority"] = json!(val.value);
        return self;
    }

    pub fn _required_behavior<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut PlanDefinition_ActionBuilder {
        self.value["_requiredBehavior"] = json!(val.value);
        return self;
    }

    pub fn _selection_behavior<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut PlanDefinition_ActionBuilder {
        self.value["_selectionBehavior"] = json!(val.value);
        return self;
    }

    pub fn _subject_canonical<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut PlanDefinition_ActionBuilder {
        self.value["_subjectCanonical"] = json!(val.value);
        return self;
    }

    pub fn _text_equivalent<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut PlanDefinition_ActionBuilder {
        self.value["_textEquivalent"] = json!(val.value);
        return self;
    }

    pub fn _title<'a>(&'a mut self, val: Element) -> &'a mut PlanDefinition_ActionBuilder {
        self.value["_title"] = json!(val.value);
        return self;
    }

    pub fn action<'a>(
        &'a mut self,
        val: Vec<PlanDefinition_Action>,
    ) -> &'a mut PlanDefinition_ActionBuilder {
        self.value["action"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn cardinality_behavior<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut PlanDefinition_ActionBuilder {
        self.value["cardinalityBehavior"] = json!(val);
        return self;
    }

    pub fn code<'a>(&'a mut self, val: CodeableConcept) -> &'a mut PlanDefinition_ActionBuilder {
        self.value["code"] = json!(val.value);
        return self;
    }

    pub fn condition<'a>(
        &'a mut self,
        val: Vec<PlanDefinition_Condition>,
    ) -> &'a mut PlanDefinition_ActionBuilder {
        self.value["condition"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn definition_canonical<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut PlanDefinition_ActionBuilder {
        self.value["definitionCanonical"] = json!(val);
        return self;
    }

    pub fn definition_uri<'a>(&'a mut self, val: &str) -> &'a mut PlanDefinition_ActionBuilder {
        self.value["definitionUri"] = json!(val);
        return self;
    }

    pub fn description<'a>(&'a mut self, val: &str) -> &'a mut PlanDefinition_ActionBuilder {
        self.value["description"] = json!(val);
        return self;
    }

    pub fn documentation<'a>(
        &'a mut self,
        val: Vec<RelatedArtifact>,
    ) -> &'a mut PlanDefinition_ActionBuilder {
        self.value["documentation"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn dynamic_value<'a>(
        &'a mut self,
        val: Vec<PlanDefinition_DynamicValue>,
    ) -> &'a mut PlanDefinition_ActionBuilder {
        self.value["dynamicValue"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut PlanDefinition_ActionBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn goal_id<'a>(&'a mut self, val: Vec<&str>) -> &'a mut PlanDefinition_ActionBuilder {
        self.value["goalId"] = json!(val);
        return self;
    }

    pub fn grouping_behavior<'a>(&'a mut self, val: &str) -> &'a mut PlanDefinition_ActionBuilder {
        self.value["groupingBehavior"] = json!(val);
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut PlanDefinition_ActionBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn input<'a>(
        &'a mut self,
        val: Vec<PlanDefinition_Input>,
    ) -> &'a mut PlanDefinition_ActionBuilder {
        self.value["input"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn link_id<'a>(&'a mut self, val: &str) -> &'a mut PlanDefinition_ActionBuilder {
        self.value["linkId"] = json!(val);
        return self;
    }

    pub fn location<'a>(
        &'a mut self,
        val: CodeableReference,
    ) -> &'a mut PlanDefinition_ActionBuilder {
        self.value["location"] = json!(val.value);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut PlanDefinition_ActionBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn output<'a>(
        &'a mut self,
        val: Vec<PlanDefinition_Output>,
    ) -> &'a mut PlanDefinition_ActionBuilder {
        self.value["output"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn participant<'a>(
        &'a mut self,
        val: Vec<PlanDefinition_Participant>,
    ) -> &'a mut PlanDefinition_ActionBuilder {
        self.value["participant"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn precheck_behavior<'a>(&'a mut self, val: &str) -> &'a mut PlanDefinition_ActionBuilder {
        self.value["precheckBehavior"] = json!(val);
        return self;
    }

    pub fn prefix<'a>(&'a mut self, val: &str) -> &'a mut PlanDefinition_ActionBuilder {
        self.value["prefix"] = json!(val);
        return self;
    }

    pub fn priority<'a>(&'a mut self, val: &str) -> &'a mut PlanDefinition_ActionBuilder {
        self.value["priority"] = json!(val);
        return self;
    }

    pub fn reason<'a>(
        &'a mut self,
        val: Vec<CodeableConcept>,
    ) -> &'a mut PlanDefinition_ActionBuilder {
        self.value["reason"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn related_action<'a>(
        &'a mut self,
        val: Vec<PlanDefinition_RelatedAction>,
    ) -> &'a mut PlanDefinition_ActionBuilder {
        self.value["relatedAction"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn required_behavior<'a>(&'a mut self, val: &str) -> &'a mut PlanDefinition_ActionBuilder {
        self.value["requiredBehavior"] = json!(val);
        return self;
    }

    pub fn selection_behavior<'a>(&'a mut self, val: &str) -> &'a mut PlanDefinition_ActionBuilder {
        self.value["selectionBehavior"] = json!(val);
        return self;
    }

    pub fn subject_canonical<'a>(&'a mut self, val: &str) -> &'a mut PlanDefinition_ActionBuilder {
        self.value["subjectCanonical"] = json!(val);
        return self;
    }

    pub fn subject_codeable_concept<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut PlanDefinition_ActionBuilder {
        self.value["subjectCodeableConcept"] = json!(val.value);
        return self;
    }

    pub fn subject_reference<'a>(
        &'a mut self,
        val: Reference,
    ) -> &'a mut PlanDefinition_ActionBuilder {
        self.value["subjectReference"] = json!(val.value);
        return self;
    }

    pub fn text_equivalent<'a>(&'a mut self, val: &str) -> &'a mut PlanDefinition_ActionBuilder {
        self.value["textEquivalent"] = json!(val);
        return self;
    }

    pub fn timing_age<'a>(&'a mut self, val: Age) -> &'a mut PlanDefinition_ActionBuilder {
        self.value["timingAge"] = json!(val.value);
        return self;
    }

    pub fn timing_duration<'a>(
        &'a mut self,
        val: Duration,
    ) -> &'a mut PlanDefinition_ActionBuilder {
        self.value["timingDuration"] = json!(val.value);
        return self;
    }

    pub fn timing_range<'a>(&'a mut self, val: Range) -> &'a mut PlanDefinition_ActionBuilder {
        self.value["timingRange"] = json!(val.value);
        return self;
    }

    pub fn timing_timing<'a>(&'a mut self, val: Timing) -> &'a mut PlanDefinition_ActionBuilder {
        self.value["timingTiming"] = json!(val.value);
        return self;
    }

    pub fn title<'a>(&'a mut self, val: &str) -> &'a mut PlanDefinition_ActionBuilder {
        self.value["title"] = json!(val);
        return self;
    }

    pub fn transform<'a>(&'a mut self, val: &str) -> &'a mut PlanDefinition_ActionBuilder {
        self.value["transform"] = json!(val);
        return self;
    }

    pub fn trigger<'a>(
        &'a mut self,
        val: Vec<TriggerDefinition>,
    ) -> &'a mut PlanDefinition_ActionBuilder {
        self.value["trigger"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn fhir_type<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut PlanDefinition_ActionBuilder {
        self.value["type"] = json!(val.value);
        return self;
    }
}
