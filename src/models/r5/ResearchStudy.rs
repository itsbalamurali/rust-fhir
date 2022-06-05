#![allow(unused_imports, non_camel_case_types)]

use crate::models::r5::Annotation::Annotation;
use crate::models::r5::CodeableConcept::CodeableConcept;
use crate::models::r5::ContactDetail::ContactDetail;
use crate::models::r5::Element::Element;
use crate::models::r5::Extension::Extension;
use crate::models::r5::Identifier::Identifier;
use crate::models::r5::Meta::Meta;
use crate::models::r5::Narrative::Narrative;
use crate::models::r5::Period::Period;
use crate::models::r5::Reference::Reference;
use crate::models::r5::RelatedArtifact::RelatedArtifact;
use crate::models::r5::ResearchStudy_AssociatedParty::ResearchStudy_AssociatedParty;
use crate::models::r5::ResearchStudy_Classification::ResearchStudy_Classification;
use crate::models::r5::ResearchStudy_ComparisonGroup::ResearchStudy_ComparisonGroup;
use crate::models::r5::ResearchStudy_Focus::ResearchStudy_Focus;
use crate::models::r5::ResearchStudy_Label::ResearchStudy_Label;
use crate::models::r5::ResearchStudy_Objective::ResearchStudy_Objective;
use crate::models::r5::ResearchStudy_OutcomeMeasure::ResearchStudy_OutcomeMeasure;
use crate::models::r5::ResearchStudy_Recruitment::ResearchStudy_Recruitment;
use crate::models::r5::ResearchStudy_StatusDate::ResearchStudy_StatusDate;
use crate::models::r5::ResearchStudy_WebLocation::ResearchStudy_WebLocation;
use crate::models::r5::ResourceList::ResourceList;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A process where a researcher or organization plans and then executes a series
/// of steps intended to increase the field of healthcare-related knowledge.  This
/// includes studies of safety, efficacy, comparative effectiveness and other
/// information about medications, devices, therapies and other interventional and
/// investigative techniques. A ResearchStudy involves the gathering of information
/// about human or animal subjects or stability data about drug products or drug
/// substances.

#[derive(Debug)]
pub struct ResearchStudy<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl ResearchStudy<'_> {
    pub fn new(value: &Value) -> ResearchStudy {
        ResearchStudy {
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

    /// Extensions for descriptionSummary
    pub fn _description_summary(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_descriptionSummary") {
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

    /// Extensions for name
    pub fn _name(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_name") {
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

    /// Sponsors, collaborators, and other parties.
    pub fn associated_party(&self) -> Option<Vec<ResearchStudy_AssociatedParty>> {
        if let Some(Value::Array(val)) = self.value.get("associatedParty") {
            return Some(
                val.into_iter()
                    .map(|e| ResearchStudy_AssociatedParty {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Codes categorizing the type of study such as investigational vs. observational,
    /// type of blinding, type of randomization, safety vs. efficacy, etc.
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

    /// Classification for the study.
    pub fn classification(&self) -> Option<Vec<ResearchStudy_Classification>> {
        if let Some(Value::Array(val)) = self.value.get("classification") {
            return Some(
                val.into_iter()
                    .map(|e| ResearchStudy_Classification {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Describes an expected sequence of events for one of the participants of a study.
    /// E.g. Exposure to drug A, wash-out, exposure to drug B, wash-out, follow-up.
    pub fn comparison_group(&self) -> Option<Vec<ResearchStudy_ComparisonGroup>> {
        if let Some(Value::Array(val)) = self.value.get("comparisonGroup") {
            return Some(
                val.into_iter()
                    .map(|e| ResearchStudy_ComparisonGroup {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The condition that is the focus of the study.  For example, In a study to examine
    /// risk factors for Lupus, might have as an inclusion criterion "healthy volunteer",
    /// but the target condition code would be a Lupus SNOMED code.
    pub fn condition(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("condition") {
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

    /// Contact details to assist a user in learning more about or engaging with the
    /// study.
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

    /// Current status of the study.
    pub fn current_state(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("currentState") {
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

    /// Date the resource last changed.
    pub fn date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("date") {
            return Some(string);
        }
        return None;
    }

    /// A full description of how the study is being conducted.  For a description of what
    /// the study objectives are see ResearchStudy.objective.description.
    pub fn description(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("description") {
            return Some(string);
        }
        return None;
    }

    /// A brief summary of the study description.
    pub fn description_summary(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("descriptionSummary") {
            return Some(string);
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

    /// The medication(s), food(s), therapy(ies), device(s) or other concerns or
    /// interventions that the study is seeking to gain more information about.
    pub fn focus(&self) -> Option<Vec<ResearchStudy_Focus>> {
        if let Some(Value::Array(val)) = self.value.get("focus") {
            return Some(
                val.into_iter()
                    .map(|e| ResearchStudy_Focus {
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

    /// Identifiers assigned to this research study by the sponsor or other systems.
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

    /// Key terms to aid in searching for or filtering the study.
    pub fn keyword(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("keyword") {
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

    /// Additional names for the study.
    pub fn label(&self) -> Option<Vec<ResearchStudy_Label>> {
        if let Some(Value::Array(val)) = self.value.get("label") {
            return Some(
                val.into_iter()
                    .map(|e| ResearchStudy_Label {
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

    /// Indicates a country, state or other region where the study is taking place.
    pub fn location(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("location") {
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

    /// Name for this study (computer friendly).
    pub fn name(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("name") {
            return Some(string);
        }
        return None;
    }

    /// Comments made about the study by the performer, subject or other participants.
    pub fn note(&self) -> Option<Vec<Annotation>> {
        if let Some(Value::Array(val)) = self.value.get("note") {
            return Some(
                val.into_iter()
                    .map(|e| Annotation {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A goal that the study is aiming to achieve in terms of a scientific question to be
    /// answered by the analysis of data collected during the study.
    pub fn objective(&self) -> Option<Vec<ResearchStudy_Objective>> {
        if let Some(Value::Array(val)) = self.value.get("objective") {
            return Some(
                val.into_iter()
                    .map(|e| ResearchStudy_Objective {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// An outcome or planned variable to measure during the study.
    pub fn outcome_measure(&self) -> Option<Vec<ResearchStudy_OutcomeMeasure>> {
        if let Some(Value::Array(val)) = self.value.get("outcomeMeasure") {
            return Some(
                val.into_iter()
                    .map(|e| ResearchStudy_OutcomeMeasure {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A larger research study of which this particular study is a component or step.
    pub fn part_of(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("partOf") {
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

    /// Identifies the start date and the expected (or actual, depending on status) end
    /// date for the study.
    pub fn period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("period") {
            return Some(Period {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The stage in the progression of a therapy from initial experimental use in humans
    /// in clinical trials to post-market evaluation.
    pub fn phase(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("phase") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The type of study based upon the intent of the study activities. A classification
    /// of the intent of the study.
    pub fn primary_purpose_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("primaryPurposeType") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A researcher in a study who oversees multiple aspects of the study, such as
    /// concept development, protocol writing, protocol submission for IRB approval,
    /// participant recruitment, informed consent, data collection, analysis,
    /// interpretation and presentation.
    pub fn principal_investigator(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("principalInvestigator") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The set of steps expected to be performed as part of the execution of the study.
    pub fn protocol(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("protocol") {
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

    /// Target or actual group of participants enrolled in study.
    pub fn recruitment(&self) -> Option<ResearchStudy_Recruitment> {
        if let Some(val) = self.value.get("recruitment") {
            return Some(ResearchStudy_Recruitment {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Citations, references and other related documents.
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

    /// Link to one or more sets of results generated by the study.  Could also link to a
    /// research registry holding the results such as ClinicalTrials.gov.
    pub fn result(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("result") {
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

    /// A facility in which study activities are conducted.
    pub fn site(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("site") {
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

    /// An organization that initiates the investigation and is legally responsible for
    /// the study.
    pub fn sponsor(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("sponsor") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The publication state of the resource (not of the study).
    pub fn status(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("status") {
            return Some(string);
        }
        return None;
    }

    /// Status of study with time for that status.
    pub fn status_date(&self) -> Option<Vec<ResearchStudy_StatusDate>> {
        if let Some(Value::Array(val)) = self.value.get("statusDate") {
            return Some(
                val.into_iter()
                    .map(|e| ResearchStudy_StatusDate {
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

    /// A short, descriptive label for the study particularly for compouter use.
    pub fn title(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("title") {
            return Some(string);
        }
        return None;
    }

    /// Canonical identifier for this study resource, represented as a globally unique
    /// URI.
    pub fn url(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("url") {
            return Some(string);
        }
        return None;
    }

    /// Business identifier for the study record.
    pub fn version(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("version") {
            return Some(string);
        }
        return None;
    }

    /// A general storage or archive location for the study.  This may contain an
    /// assortment of content which is not specified in advance.
    pub fn web_location(&self) -> Option<Vec<ResearchStudy_WebLocation>> {
        if let Some(Value::Array(val)) = self.value.get("webLocation") {
            return Some(
                val.into_iter()
                    .map(|e| ResearchStudy_WebLocation {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A description and/or code explaining the premature termination of the study.
    pub fn why_stopped(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("whyStopped") {
            return Some(CodeableConcept {
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
        if let Some(_val) = self._description_summary() {
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
        if let Some(_val) = self._name() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._status() {
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
        if let Some(_val) = self.associated_party() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.category() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.classification() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.comparison_group() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.condition() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
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
        if let Some(_val) = self.current_state() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.date() {}
        if let Some(_val) = self.description() {}
        if let Some(_val) = self.description_summary() {}
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.focus() {
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
        if let Some(_val) = self.keyword() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.label() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.language() {}
        if let Some(_val) = self.location() {
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
        if let Some(_val) = self.note() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.objective() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.outcome_measure() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.part_of() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.period() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.phase() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.primary_purpose_type() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.principal_investigator() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.protocol() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.recruitment() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.related_artifact() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.result() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.site() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.sponsor() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.status() {}
        if let Some(_val) = self.status_date() {
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
        if let Some(_val) = self.url() {}
        if let Some(_val) = self.version() {}
        if let Some(_val) = self.web_location() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.why_stopped() {
            if !_val.validate() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct ResearchStudyBuilder {
    pub(crate) value: Value,
}

impl ResearchStudyBuilder {
    pub fn build(&self) -> ResearchStudy {
        ResearchStudy {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: ResearchStudy) -> ResearchStudyBuilder {
        ResearchStudyBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> ResearchStudyBuilder {
        let mut __value: Value = json!({});
        return ResearchStudyBuilder { value: __value };
    }

    pub fn _date<'a>(&'a mut self, val: Element) -> &'a mut ResearchStudyBuilder {
        self.value["_date"] = json!(val.value);
        return self;
    }

    pub fn _description<'a>(&'a mut self, val: Element) -> &'a mut ResearchStudyBuilder {
        self.value["_description"] = json!(val.value);
        return self;
    }

    pub fn _description_summary<'a>(&'a mut self, val: Element) -> &'a mut ResearchStudyBuilder {
        self.value["_descriptionSummary"] = json!(val.value);
        return self;
    }

    pub fn _implicit_rules<'a>(&'a mut self, val: Element) -> &'a mut ResearchStudyBuilder {
        self.value["_implicitRules"] = json!(val.value);
        return self;
    }

    pub fn _language<'a>(&'a mut self, val: Element) -> &'a mut ResearchStudyBuilder {
        self.value["_language"] = json!(val.value);
        return self;
    }

    pub fn _name<'a>(&'a mut self, val: Element) -> &'a mut ResearchStudyBuilder {
        self.value["_name"] = json!(val.value);
        return self;
    }

    pub fn _status<'a>(&'a mut self, val: Element) -> &'a mut ResearchStudyBuilder {
        self.value["_status"] = json!(val.value);
        return self;
    }

    pub fn _title<'a>(&'a mut self, val: Element) -> &'a mut ResearchStudyBuilder {
        self.value["_title"] = json!(val.value);
        return self;
    }

    pub fn _url<'a>(&'a mut self, val: Element) -> &'a mut ResearchStudyBuilder {
        self.value["_url"] = json!(val.value);
        return self;
    }

    pub fn _version<'a>(&'a mut self, val: Element) -> &'a mut ResearchStudyBuilder {
        self.value["_version"] = json!(val.value);
        return self;
    }

    pub fn associated_party<'a>(
        &'a mut self,
        val: Vec<ResearchStudy_AssociatedParty>,
    ) -> &'a mut ResearchStudyBuilder {
        self.value["associatedParty"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn category<'a>(&'a mut self, val: Vec<CodeableConcept>) -> &'a mut ResearchStudyBuilder {
        self.value["category"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn classification<'a>(
        &'a mut self,
        val: Vec<ResearchStudy_Classification>,
    ) -> &'a mut ResearchStudyBuilder {
        self.value["classification"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn comparison_group<'a>(
        &'a mut self,
        val: Vec<ResearchStudy_ComparisonGroup>,
    ) -> &'a mut ResearchStudyBuilder {
        self.value["comparisonGroup"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn condition<'a>(&'a mut self, val: Vec<CodeableConcept>) -> &'a mut ResearchStudyBuilder {
        self.value["condition"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn contact<'a>(&'a mut self, val: Vec<ContactDetail>) -> &'a mut ResearchStudyBuilder {
        self.value["contact"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn contained<'a>(&'a mut self, val: Vec<ResourceList>) -> &'a mut ResearchStudyBuilder {
        self.value["contained"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn current_state<'a>(
        &'a mut self,
        val: Vec<CodeableConcept>,
    ) -> &'a mut ResearchStudyBuilder {
        self.value["currentState"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn date<'a>(&'a mut self, val: &str) -> &'a mut ResearchStudyBuilder {
        self.value["date"] = json!(val);
        return self;
    }

    pub fn description<'a>(&'a mut self, val: &str) -> &'a mut ResearchStudyBuilder {
        self.value["description"] = json!(val);
        return self;
    }

    pub fn description_summary<'a>(&'a mut self, val: &str) -> &'a mut ResearchStudyBuilder {
        self.value["descriptionSummary"] = json!(val);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut ResearchStudyBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn focus<'a>(&'a mut self, val: Vec<ResearchStudy_Focus>) -> &'a mut ResearchStudyBuilder {
        self.value["focus"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut ResearchStudyBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn identifier<'a>(&'a mut self, val: Vec<Identifier>) -> &'a mut ResearchStudyBuilder {
        self.value["identifier"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn implicit_rules<'a>(&'a mut self, val: &str) -> &'a mut ResearchStudyBuilder {
        self.value["implicitRules"] = json!(val);
        return self;
    }

    pub fn keyword<'a>(&'a mut self, val: Vec<CodeableConcept>) -> &'a mut ResearchStudyBuilder {
        self.value["keyword"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn label<'a>(&'a mut self, val: Vec<ResearchStudy_Label>) -> &'a mut ResearchStudyBuilder {
        self.value["label"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn language<'a>(&'a mut self, val: &str) -> &'a mut ResearchStudyBuilder {
        self.value["language"] = json!(val);
        return self;
    }

    pub fn location<'a>(&'a mut self, val: Vec<CodeableConcept>) -> &'a mut ResearchStudyBuilder {
        self.value["location"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn meta<'a>(&'a mut self, val: Meta) -> &'a mut ResearchStudyBuilder {
        self.value["meta"] = json!(val.value);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ResearchStudyBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn name<'a>(&'a mut self, val: &str) -> &'a mut ResearchStudyBuilder {
        self.value["name"] = json!(val);
        return self;
    }

    pub fn note<'a>(&'a mut self, val: Vec<Annotation>) -> &'a mut ResearchStudyBuilder {
        self.value["note"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn objective<'a>(
        &'a mut self,
        val: Vec<ResearchStudy_Objective>,
    ) -> &'a mut ResearchStudyBuilder {
        self.value["objective"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn outcome_measure<'a>(
        &'a mut self,
        val: Vec<ResearchStudy_OutcomeMeasure>,
    ) -> &'a mut ResearchStudyBuilder {
        self.value["outcomeMeasure"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn part_of<'a>(&'a mut self, val: Vec<Reference>) -> &'a mut ResearchStudyBuilder {
        self.value["partOf"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn period<'a>(&'a mut self, val: Period) -> &'a mut ResearchStudyBuilder {
        self.value["period"] = json!(val.value);
        return self;
    }

    pub fn phase<'a>(&'a mut self, val: CodeableConcept) -> &'a mut ResearchStudyBuilder {
        self.value["phase"] = json!(val.value);
        return self;
    }

    pub fn primary_purpose_type<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut ResearchStudyBuilder {
        self.value["primaryPurposeType"] = json!(val.value);
        return self;
    }

    pub fn principal_investigator<'a>(
        &'a mut self,
        val: Reference,
    ) -> &'a mut ResearchStudyBuilder {
        self.value["principalInvestigator"] = json!(val.value);
        return self;
    }

    pub fn protocol<'a>(&'a mut self, val: Vec<Reference>) -> &'a mut ResearchStudyBuilder {
        self.value["protocol"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn recruitment<'a>(
        &'a mut self,
        val: ResearchStudy_Recruitment,
    ) -> &'a mut ResearchStudyBuilder {
        self.value["recruitment"] = json!(val.value);
        return self;
    }

    pub fn related_artifact<'a>(
        &'a mut self,
        val: Vec<RelatedArtifact>,
    ) -> &'a mut ResearchStudyBuilder {
        self.value["relatedArtifact"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn result<'a>(&'a mut self, val: Vec<Reference>) -> &'a mut ResearchStudyBuilder {
        self.value["result"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn site<'a>(&'a mut self, val: Vec<Reference>) -> &'a mut ResearchStudyBuilder {
        self.value["site"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn sponsor<'a>(&'a mut self, val: Reference) -> &'a mut ResearchStudyBuilder {
        self.value["sponsor"] = json!(val.value);
        return self;
    }

    pub fn status<'a>(&'a mut self, val: &str) -> &'a mut ResearchStudyBuilder {
        self.value["status"] = json!(val);
        return self;
    }

    pub fn status_date<'a>(
        &'a mut self,
        val: Vec<ResearchStudy_StatusDate>,
    ) -> &'a mut ResearchStudyBuilder {
        self.value["statusDate"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn text<'a>(&'a mut self, val: Narrative) -> &'a mut ResearchStudyBuilder {
        self.value["text"] = json!(val.value);
        return self;
    }

    pub fn title<'a>(&'a mut self, val: &str) -> &'a mut ResearchStudyBuilder {
        self.value["title"] = json!(val);
        return self;
    }

    pub fn url<'a>(&'a mut self, val: &str) -> &'a mut ResearchStudyBuilder {
        self.value["url"] = json!(val);
        return self;
    }

    pub fn version<'a>(&'a mut self, val: &str) -> &'a mut ResearchStudyBuilder {
        self.value["version"] = json!(val);
        return self;
    }

    pub fn web_location<'a>(
        &'a mut self,
        val: Vec<ResearchStudy_WebLocation>,
    ) -> &'a mut ResearchStudyBuilder {
        self.value["webLocation"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn why_stopped<'a>(&'a mut self, val: CodeableConcept) -> &'a mut ResearchStudyBuilder {
        self.value["whyStopped"] = json!(val.value);
        return self;
    }
}
