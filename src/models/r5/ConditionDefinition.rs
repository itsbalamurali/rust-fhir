#![allow(unused_imports, non_camel_case_types)]

use crate::models::r5::CodeableConcept::CodeableConcept;
use crate::models::r5::ConditionDefinition_Medication::ConditionDefinition_Medication;
use crate::models::r5::ConditionDefinition_Observation::ConditionDefinition_Observation;
use crate::models::r5::ConditionDefinition_Plan::ConditionDefinition_Plan;
use crate::models::r5::ConditionDefinition_Precondition::ConditionDefinition_Precondition;
use crate::models::r5::ConditionDefinition_Questionnaire::ConditionDefinition_Questionnaire;
use crate::models::r5::ContactDetail::ContactDetail;
use crate::models::r5::Element::Element;
use crate::models::r5::Extension::Extension;
use crate::models::r5::Identifier::Identifier;
use crate::models::r5::Meta::Meta;
use crate::models::r5::Narrative::Narrative;
use crate::models::r5::Period::Period;
use crate::models::r5::Reference::Reference;
use crate::models::r5::RelatedArtifact::RelatedArtifact;
use crate::models::r5::ResourceList::ResourceList;
use crate::models::r5::UsageContext::UsageContext;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A definition of a condition and information relevant to managing it.

#[derive(Debug)]
pub struct ConditionDefinition<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl ConditionDefinition<'_> {
    pub fn new(value: &Value) -> ConditionDefinition {
        ConditionDefinition {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for approvalDate
    pub fn _approval_date(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_approvalDate") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for copyright
    pub fn _copyright(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_copyright") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
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

    /// Extensions for definition
    pub fn _definition(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_definition") {
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

    /// Extensions for description
    pub fn _description(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_description") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for experimental
    pub fn _experimental(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_experimental") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for hasBodySite
    pub fn _has_body_site(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_hasBodySite") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for hasSeverity
    pub fn _has_severity(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_hasSeverity") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for hasStage
    pub fn _has_stage(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_hasStage") {
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

    /// Extensions for lastReviewDate
    pub fn _last_review_date(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_lastReviewDate") {
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

    /// Extensions for publisher
    pub fn _publisher(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_publisher") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for purpose
    pub fn _purpose(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_purpose") {
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

    /// Extensions for subtitle
    pub fn _subtitle(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_subtitle") {
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

    /// Extensions for url
    pub fn _url(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_url") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for version
    pub fn _version(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_version") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The date on which the resource content was approved by the publisher. Approval
    /// happens once when the content is officially approved for usage.
    pub fn approval_date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("approvalDate") {
            return Some(string);
        }
        return None;
    }

    /// An individiual or organization primarily involved in the creation and maintenance
    /// of the {{title}}.
    pub fn author(&self) -> Option<Vec<ContactDetail>> {
        if let Some(Value::Array(val)) = self.value.get("author") {
            return Some(
                val.into_iter()
                    .map(|e| ContactDetail {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The anatomical location where this condition manifests itself.
    pub fn body_site(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("bodySite") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Identification of the condition, problem or diagnosis.
    pub fn code(&self) -> CodeableConcept {
        CodeableConcept {
            value: Cow::Borrowed(&self.value["code"]),
        }
    }

    /// Contact details to assist a user in finding and communicating with the publisher.
    pub fn contact(&self) -> Option<Vec<ContactDetail>> {
        if let Some(Value::Array(val)) = self.value.get("contact") {
            return Some(
                val.into_iter()
                    .map(|e| ContactDetail {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
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

    /// A copyright statement relating to the {{title}} and/or its contents. Copyright
    /// statements are generally legal restrictions on the use and publishing of the
    /// {{title}}.
    pub fn copyright(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("copyright") {
            return Some(string);
        }
        return None;
    }

    /// The date  (and optionally time) when the condition definition was published.
    /// The date must change when the business version changes and it must change if the
    /// status code changes. In addition, it should change when the substantive content of
    /// the condition definition changes.
    pub fn date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("date") {
            return Some(string);
        }
        return None;
    }

    /// Formal definitions of the condition. These may be references to ontologies,
    /// published clinical protocols or research papers.
    pub fn definition(&self) -> Option<Vec<&str>> {
        if let Some(Value::Array(val)) = self.value.get("definition") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_str().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A free text natural language description of the condition definition from a
    /// consumer's perspective.
    pub fn description(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("description") {
            return Some(string);
        }
        return None;
    }

    /// An individual or organization primarily responsible for internal coherence of
    /// the {{title}}.
    pub fn editor(&self) -> Option<Vec<ContactDetail>> {
        if let Some(Value::Array(val)) = self.value.get("editor") {
            return Some(
                val.into_iter()
                    .map(|e| ContactDetail {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The period during which the {{title}} content was or is planned to be in active
    /// use.
    pub fn effective_period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("effectivePeriod") {
            return Some(Period {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// An individual or organization responsible for officially endorsing the {{title}}
    /// for use in some setting.
    pub fn endorser(&self) -> Option<Vec<ContactDetail>> {
        if let Some(Value::Array(val)) = self.value.get("endorser") {
            return Some(
                val.into_iter()
                    .map(|e| ContactDetail {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A Boolean value to indicate that this condition definition is authored for testing
    /// purposes (or education/evaluation/marketing) and is not intended to be used for
    /// genuine usage.
    pub fn experimental(&self) -> Option<bool> {
        if let Some(val) = self.value.get("experimental") {
            return Some(val.as_bool().unwrap());
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

    /// Whether bodySite is appropriate to collect for this condition.
    pub fn has_body_site(&self) -> Option<bool> {
        if let Some(val) = self.value.get("hasBodySite") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    /// Whether Severity is appropriate to collect for this condition.
    pub fn has_severity(&self) -> Option<bool> {
        if let Some(val) = self.value.get("hasSeverity") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    /// Whether stage is appropriate to collect for this condition.
    pub fn has_stage(&self) -> Option<bool> {
        if let Some(val) = self.value.get("hasStage") {
            return Some(val.as_bool().unwrap());
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

    /// A formal identifier that is used to identify this condition definition when it is
    /// represented in other formats, or referenced in a specification, model, design or
    /// an instance.
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
    /// constructed, and which must be understood when processing the content. Often, this
    /// is a reference to an implementation guide that defines the special rules along
    /// with other profiles etc.
    pub fn implicit_rules(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("implicitRules") {
            return Some(string);
        }
        return None;
    }

    /// A legal or geographic region in which the condition definition is intended to
    /// be used.
    pub fn jurisdiction(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("jurisdiction") {
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

    /// The base language in which the resource is written.
    pub fn language(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("language") {
            return Some(string);
        }
        return None;
    }

    /// The date on which the resource content was last reviewed. Review happens
    /// periodically after approval but does not change the original approval date.
    pub fn last_review_date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("lastReviewDate") {
            return Some(string);
        }
        return None;
    }

    /// Medications particularly relevant for this condition.
    pub fn medication(&self) -> Option<Vec<ConditionDefinition_Medication>> {
        if let Some(Value::Array(val)) = self.value.get("medication") {
            return Some(
                val.into_iter()
                    .map(|e| ConditionDefinition_Medication {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
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

    /// A natural language name identifying the condition definition. This name should be
    /// usable as an identifier for the module by machine processing applications such as
    /// code generation.
    pub fn name(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("name") {
            return Some(string);
        }
        return None;
    }

    /// Observations particularly relevant to this condition.
    pub fn observation(&self) -> Option<Vec<ConditionDefinition_Observation>> {
        if let Some(Value::Array(val)) = self.value.get("observation") {
            return Some(
                val.into_iter()
                    .map(|e| ConditionDefinition_Observation {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Plan that is appropriate.
    pub fn plan(&self) -> Option<Vec<ConditionDefinition_Plan>> {
        if let Some(Value::Array(val)) = self.value.get("plan") {
            return Some(
                val.into_iter()
                    .map(|e| ConditionDefinition_Plan {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// An observation that suggests that this condition applies.
    pub fn precondition(&self) -> Option<Vec<ConditionDefinition_Precondition>> {
        if let Some(Value::Array(val)) = self.value.get("precondition") {
            return Some(
                val.into_iter()
                    .map(|e| ConditionDefinition_Precondition {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The name of the organization or individual that published the condition
    /// definition.
    pub fn publisher(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("publisher") {
            return Some(string);
        }
        return None;
    }

    /// Explanation of why this {{title}} is needed and why it has been designed as it
    /// has.
    pub fn purpose(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("purpose") {
            return Some(string);
        }
        return None;
    }

    /// Questionnaire for this condition.
    pub fn questionnaire(&self) -> Option<Vec<ConditionDefinition_Questionnaire>> {
        if let Some(Value::Array(val)) = self.value.get("questionnaire") {
            return Some(
                val.into_iter()
                    .map(|e| ConditionDefinition_Questionnaire {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Related artifacts such as additional documentation, justification, dependencies,
    /// bibliographic references, and predecessor and successor artifacts.
    pub fn related_artifact(&self) -> Option<Vec<RelatedArtifact>> {
        if let Some(Value::Array(val)) = self.value.get("relatedArtifact") {
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

    /// An individual or organization primarily responsible for review of some aspect of
    /// the {{title}}.
    pub fn reviewer(&self) -> Option<Vec<ContactDetail>> {
        if let Some(Value::Array(val)) = self.value.get("reviewer") {
            return Some(
                val.into_iter()
                    .map(|e| ContactDetail {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A subjective assessment of the severity of the condition as evaluated by the
    /// clinician.
    pub fn severity(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("severity") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Clinical stage or grade of a condition. May include formal severity assessments.
    pub fn stage(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("stage") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The status of this condition definition. Enables tracking the life-cycle of the
    /// content.
    pub fn status(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("status") {
            return Some(string);
        }
        return None;
    }

    /// An explanatory or alternate title for the event definition giving additional
    /// information about its content.
    pub fn subtitle(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("subtitle") {
            return Some(string);
        }
        return None;
    }

    /// Appropriate team for this condition.
    pub fn team(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("team") {
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

    /// A short, descriptive, user-friendly title for the condition definition.
    pub fn title(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("title") {
            return Some(string);
        }
        return None;
    }

    /// Descriptive topics related to the content of the library. Topics provide a
    /// high-level categorization of the library that can be useful for filtering and
    /// searching.
    pub fn topic(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("topic") {
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

    /// An absolute URI that is used to identify this condition definition when it is
    /// referenced in a specification, model, design or an instance; also called its
    /// canonical identifier. This SHOULD be globally unique and SHOULD be a literal
    /// address at which at which an authoritative instance of this condition definition
    /// is (or will be) published. This URL can be the target of a canonical reference.
    /// It SHALL remain the same when the condition definition is stored on different
    /// servers.
    pub fn url(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("url") {
            return Some(string);
        }
        return None;
    }

    /// The content was developed with a focus and intent of supporting the contexts that
    /// are listed. These contexts may be general categories (gender, age, ...) or may be
    /// references to specific programs (insurance plans, studies, ...) and may be used to
    /// assist with indexing and searching for appropriate condition definition instances.
    pub fn use_context(&self) -> Option<Vec<UsageContext>> {
        if let Some(Value::Array(val)) = self.value.get("useContext") {
            return Some(
                val.into_iter()
                    .map(|e| UsageContext {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The identifier that is used to identify this version of the condition definition
    /// when it is referenced in a specification, model, design or instance. This is an
    /// arbitrary value managed by the condition definition author and is not expected
    /// to be globally unique. For example, it might be a timestamp (e.g. yyyymmdd) if a
    /// managed version is not available. There is also no expectation that versions can
    /// be placed in a lexicographical sequence.
    pub fn version(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("version") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._approval_date() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._copyright() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._date() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._definition() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self._description() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._experimental() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._has_body_site() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._has_severity() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._has_stage() {
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
        if let Some(_val) = self._last_review_date() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._name() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._publisher() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._purpose() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._status() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._subtitle() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._title() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._url() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._version() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.approval_date() {}
        if let Some(_val) = self.author() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.body_site() {
            if !_val.validate() {
                return false;
            }
        }
        if !self.code().validate() {
            return false;
        }
        if let Some(_val) = self.contact() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.contained() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.copyright() {}
        if let Some(_val) = self.date() {}
        if let Some(_val) = self.definition() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.description() {}
        if let Some(_val) = self.editor() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.effective_period() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.endorser() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.experimental() {}
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.has_body_site() {}
        if let Some(_val) = self.has_severity() {}
        if let Some(_val) = self.has_stage() {}
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.identifier() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.implicit_rules() {}
        if let Some(_val) = self.jurisdiction() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.language() {}
        if let Some(_val) = self.last_review_date() {}
        if let Some(_val) = self.medication() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
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
        if let Some(_val) = self.name() {}
        if let Some(_val) = self.observation() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.plan() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.precondition() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.publisher() {}
        if let Some(_val) = self.purpose() {}
        if let Some(_val) = self.questionnaire() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.related_artifact() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.reviewer() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.severity() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.stage() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.status() {}
        if let Some(_val) = self.subtitle() {}
        if let Some(_val) = self.team() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.text() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.title() {}
        if let Some(_val) = self.topic() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.url() {}
        if let Some(_val) = self.use_context() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.version() {}
        return true;
    }
}

#[derive(Debug)]
pub struct ConditionDefinitionBuilder {
    pub(crate) value: Value,
}

impl ConditionDefinitionBuilder {
    pub fn build(&self) -> ConditionDefinition {
        ConditionDefinition {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: ConditionDefinition) -> ConditionDefinitionBuilder {
        ConditionDefinitionBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(code: CodeableConcept) -> ConditionDefinitionBuilder {
        let mut __value: Value = json!({});
        __value["code"] = json!(code.value);
        return ConditionDefinitionBuilder { value: __value };
    }

    pub fn _approval_date<'a>(&'a mut self, val: Element) -> &'a mut ConditionDefinitionBuilder {
        self.value["_approvalDate"] = json!(val.value);
        return self;
    }

    pub fn _copyright<'a>(&'a mut self, val: Element) -> &'a mut ConditionDefinitionBuilder {
        self.value["_copyright"] = json!(val.value);
        return self;
    }

    pub fn _date<'a>(&'a mut self, val: Element) -> &'a mut ConditionDefinitionBuilder {
        self.value["_date"] = json!(val.value);
        return self;
    }

    pub fn _definition<'a>(&'a mut self, val: Vec<Element>) -> &'a mut ConditionDefinitionBuilder {
        self.value["_definition"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn _description<'a>(&'a mut self, val: Element) -> &'a mut ConditionDefinitionBuilder {
        self.value["_description"] = json!(val.value);
        return self;
    }

    pub fn _experimental<'a>(&'a mut self, val: Element) -> &'a mut ConditionDefinitionBuilder {
        self.value["_experimental"] = json!(val.value);
        return self;
    }

    pub fn _has_body_site<'a>(&'a mut self, val: Element) -> &'a mut ConditionDefinitionBuilder {
        self.value["_hasBodySite"] = json!(val.value);
        return self;
    }

    pub fn _has_severity<'a>(&'a mut self, val: Element) -> &'a mut ConditionDefinitionBuilder {
        self.value["_hasSeverity"] = json!(val.value);
        return self;
    }

    pub fn _has_stage<'a>(&'a mut self, val: Element) -> &'a mut ConditionDefinitionBuilder {
        self.value["_hasStage"] = json!(val.value);
        return self;
    }

    pub fn _implicit_rules<'a>(&'a mut self, val: Element) -> &'a mut ConditionDefinitionBuilder {
        self.value["_implicitRules"] = json!(val.value);
        return self;
    }

    pub fn _language<'a>(&'a mut self, val: Element) -> &'a mut ConditionDefinitionBuilder {
        self.value["_language"] = json!(val.value);
        return self;
    }

    pub fn _last_review_date<'a>(&'a mut self, val: Element) -> &'a mut ConditionDefinitionBuilder {
        self.value["_lastReviewDate"] = json!(val.value);
        return self;
    }

    pub fn _name<'a>(&'a mut self, val: Element) -> &'a mut ConditionDefinitionBuilder {
        self.value["_name"] = json!(val.value);
        return self;
    }

    pub fn _publisher<'a>(&'a mut self, val: Element) -> &'a mut ConditionDefinitionBuilder {
        self.value["_publisher"] = json!(val.value);
        return self;
    }

    pub fn _purpose<'a>(&'a mut self, val: Element) -> &'a mut ConditionDefinitionBuilder {
        self.value["_purpose"] = json!(val.value);
        return self;
    }

    pub fn _status<'a>(&'a mut self, val: Element) -> &'a mut ConditionDefinitionBuilder {
        self.value["_status"] = json!(val.value);
        return self;
    }

    pub fn _subtitle<'a>(&'a mut self, val: Element) -> &'a mut ConditionDefinitionBuilder {
        self.value["_subtitle"] = json!(val.value);
        return self;
    }

    pub fn _title<'a>(&'a mut self, val: Element) -> &'a mut ConditionDefinitionBuilder {
        self.value["_title"] = json!(val.value);
        return self;
    }

    pub fn _url<'a>(&'a mut self, val: Element) -> &'a mut ConditionDefinitionBuilder {
        self.value["_url"] = json!(val.value);
        return self;
    }

    pub fn _version<'a>(&'a mut self, val: Element) -> &'a mut ConditionDefinitionBuilder {
        self.value["_version"] = json!(val.value);
        return self;
    }

    pub fn approval_date<'a>(&'a mut self, val: &str) -> &'a mut ConditionDefinitionBuilder {
        self.value["approvalDate"] = json!(val);
        return self;
    }

    pub fn author<'a>(&'a mut self, val: Vec<ContactDetail>) -> &'a mut ConditionDefinitionBuilder {
        self.value["author"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn body_site<'a>(&'a mut self, val: CodeableConcept) -> &'a mut ConditionDefinitionBuilder {
        self.value["bodySite"] = json!(val.value);
        return self;
    }

    pub fn contact<'a>(
        &'a mut self,
        val: Vec<ContactDetail>,
    ) -> &'a mut ConditionDefinitionBuilder {
        self.value["contact"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn contained<'a>(
        &'a mut self,
        val: Vec<ResourceList>,
    ) -> &'a mut ConditionDefinitionBuilder {
        self.value["contained"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn copyright<'a>(&'a mut self, val: &str) -> &'a mut ConditionDefinitionBuilder {
        self.value["copyright"] = json!(val);
        return self;
    }

    pub fn date<'a>(&'a mut self, val: &str) -> &'a mut ConditionDefinitionBuilder {
        self.value["date"] = json!(val);
        return self;
    }

    pub fn definition<'a>(&'a mut self, val: Vec<&str>) -> &'a mut ConditionDefinitionBuilder {
        self.value["definition"] = json!(val);
        return self;
    }

    pub fn description<'a>(&'a mut self, val: &str) -> &'a mut ConditionDefinitionBuilder {
        self.value["description"] = json!(val);
        return self;
    }

    pub fn editor<'a>(&'a mut self, val: Vec<ContactDetail>) -> &'a mut ConditionDefinitionBuilder {
        self.value["editor"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn effective_period<'a>(&'a mut self, val: Period) -> &'a mut ConditionDefinitionBuilder {
        self.value["effectivePeriod"] = json!(val.value);
        return self;
    }

    pub fn endorser<'a>(
        &'a mut self,
        val: Vec<ContactDetail>,
    ) -> &'a mut ConditionDefinitionBuilder {
        self.value["endorser"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn experimental<'a>(&'a mut self, val: bool) -> &'a mut ConditionDefinitionBuilder {
        self.value["experimental"] = json!(val);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut ConditionDefinitionBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn has_body_site<'a>(&'a mut self, val: bool) -> &'a mut ConditionDefinitionBuilder {
        self.value["hasBodySite"] = json!(val);
        return self;
    }

    pub fn has_severity<'a>(&'a mut self, val: bool) -> &'a mut ConditionDefinitionBuilder {
        self.value["hasSeverity"] = json!(val);
        return self;
    }

    pub fn has_stage<'a>(&'a mut self, val: bool) -> &'a mut ConditionDefinitionBuilder {
        self.value["hasStage"] = json!(val);
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut ConditionDefinitionBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn identifier<'a>(
        &'a mut self,
        val: Vec<Identifier>,
    ) -> &'a mut ConditionDefinitionBuilder {
        self.value["identifier"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn implicit_rules<'a>(&'a mut self, val: &str) -> &'a mut ConditionDefinitionBuilder {
        self.value["implicitRules"] = json!(val);
        return self;
    }

    pub fn jurisdiction<'a>(
        &'a mut self,
        val: Vec<CodeableConcept>,
    ) -> &'a mut ConditionDefinitionBuilder {
        self.value["jurisdiction"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn language<'a>(&'a mut self, val: &str) -> &'a mut ConditionDefinitionBuilder {
        self.value["language"] = json!(val);
        return self;
    }

    pub fn last_review_date<'a>(&'a mut self, val: &str) -> &'a mut ConditionDefinitionBuilder {
        self.value["lastReviewDate"] = json!(val);
        return self;
    }

    pub fn medication<'a>(
        &'a mut self,
        val: Vec<ConditionDefinition_Medication>,
    ) -> &'a mut ConditionDefinitionBuilder {
        self.value["medication"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn meta<'a>(&'a mut self, val: Meta) -> &'a mut ConditionDefinitionBuilder {
        self.value["meta"] = json!(val.value);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ConditionDefinitionBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn name<'a>(&'a mut self, val: &str) -> &'a mut ConditionDefinitionBuilder {
        self.value["name"] = json!(val);
        return self;
    }

    pub fn observation<'a>(
        &'a mut self,
        val: Vec<ConditionDefinition_Observation>,
    ) -> &'a mut ConditionDefinitionBuilder {
        self.value["observation"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn plan<'a>(
        &'a mut self,
        val: Vec<ConditionDefinition_Plan>,
    ) -> &'a mut ConditionDefinitionBuilder {
        self.value["plan"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn precondition<'a>(
        &'a mut self,
        val: Vec<ConditionDefinition_Precondition>,
    ) -> &'a mut ConditionDefinitionBuilder {
        self.value["precondition"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn publisher<'a>(&'a mut self, val: &str) -> &'a mut ConditionDefinitionBuilder {
        self.value["publisher"] = json!(val);
        return self;
    }

    pub fn purpose<'a>(&'a mut self, val: &str) -> &'a mut ConditionDefinitionBuilder {
        self.value["purpose"] = json!(val);
        return self;
    }

    pub fn questionnaire<'a>(
        &'a mut self,
        val: Vec<ConditionDefinition_Questionnaire>,
    ) -> &'a mut ConditionDefinitionBuilder {
        self.value["questionnaire"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn related_artifact<'a>(
        &'a mut self,
        val: Vec<RelatedArtifact>,
    ) -> &'a mut ConditionDefinitionBuilder {
        self.value["relatedArtifact"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn reviewer<'a>(
        &'a mut self,
        val: Vec<ContactDetail>,
    ) -> &'a mut ConditionDefinitionBuilder {
        self.value["reviewer"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn severity<'a>(&'a mut self, val: CodeableConcept) -> &'a mut ConditionDefinitionBuilder {
        self.value["severity"] = json!(val.value);
        return self;
    }

    pub fn stage<'a>(&'a mut self, val: CodeableConcept) -> &'a mut ConditionDefinitionBuilder {
        self.value["stage"] = json!(val.value);
        return self;
    }

    pub fn status<'a>(&'a mut self, val: &str) -> &'a mut ConditionDefinitionBuilder {
        self.value["status"] = json!(val);
        return self;
    }

    pub fn subtitle<'a>(&'a mut self, val: &str) -> &'a mut ConditionDefinitionBuilder {
        self.value["subtitle"] = json!(val);
        return self;
    }

    pub fn team<'a>(&'a mut self, val: Vec<Reference>) -> &'a mut ConditionDefinitionBuilder {
        self.value["team"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn text<'a>(&'a mut self, val: Narrative) -> &'a mut ConditionDefinitionBuilder {
        self.value["text"] = json!(val.value);
        return self;
    }

    pub fn title<'a>(&'a mut self, val: &str) -> &'a mut ConditionDefinitionBuilder {
        self.value["title"] = json!(val);
        return self;
    }

    pub fn topic<'a>(
        &'a mut self,
        val: Vec<CodeableConcept>,
    ) -> &'a mut ConditionDefinitionBuilder {
        self.value["topic"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn url<'a>(&'a mut self, val: &str) -> &'a mut ConditionDefinitionBuilder {
        self.value["url"] = json!(val);
        return self;
    }

    pub fn use_context<'a>(
        &'a mut self,
        val: Vec<UsageContext>,
    ) -> &'a mut ConditionDefinitionBuilder {
        self.value["useContext"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn version<'a>(&'a mut self, val: &str) -> &'a mut ConditionDefinitionBuilder {
        self.value["version"] = json!(val);
        return self;
    }
}
