#![allow(unused_imports, non_camel_case_types)]

use crate::models::r4b::Annotation::Annotation;
use crate::models::r4b::CodeableConcept::CodeableConcept;
use crate::models::r4b::ContactDetail::ContactDetail;
use crate::models::r4b::Element::Element;
use crate::models::r4b::Evidence_Certainty::Evidence_Certainty;
use crate::models::r4b::Evidence_Statistic::Evidence_Statistic;
use crate::models::r4b::Evidence_VariableDefinition::Evidence_VariableDefinition;
use crate::models::r4b::Extension::Extension;
use crate::models::r4b::Identifier::Identifier;
use crate::models::r4b::Meta::Meta;
use crate::models::r4b::Narrative::Narrative;
use crate::models::r4b::Reference::Reference;
use crate::models::r4b::RelatedArtifact::RelatedArtifact;
use crate::models::r4b::ResourceList::ResourceList;
use crate::models::r4b::UsageContext::UsageContext;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// The Evidence Resource provides a machine-interpretable expression of an evidence
/// concept including the evidence variables (eg population, exposures/interventions,
/// comparators, outcomes, measured variables, confounding variables), the statistics,
/// and the certainty of this evidence.

#[derive(Debug)]
pub struct Evidence<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Evidence<'_> {
    pub fn new(value: &Value) -> Evidence {
        Evidence {
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

    /// Extensions for assertion
    pub fn _assertion(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_assertion") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for citeAsMarkdown
    pub fn _cite_as_markdown(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_citeAsMarkdown") {
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

    /// Extensions for description
    pub fn _description(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_description") {
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

    /// Extensions for publisher
    pub fn _publisher(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_publisher") {
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

    /// The date on which the resource content was approved by the publisher. Approval
    /// happens once when the content is officially approved for usage.
    pub fn approval_date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("approvalDate") {
            return Some(string);
        }
        return None;
    }

    /// Declarative description of the Evidence.
    pub fn assertion(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("assertion") {
            return Some(string);
        }
        return None;
    }

    /// An individiual, organization, or device primarily involved in the creation and
    /// maintenance of the content.
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

    /// Assessment of certainty, confidence in the estimates, or quality of the evidence.
    pub fn certainty(&self) -> Option<Vec<Evidence_Certainty>> {
        if let Some(Value::Array(val)) = self.value.get("certainty") {
            return Some(
                val.into_iter()
                    .map(|e| Evidence_Certainty {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Citation Resource or display of suggested citation for this evidence.
    pub fn cite_as_markdown(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("citeAsMarkdown") {
            return Some(string);
        }
        return None;
    }

    /// Citation Resource or display of suggested citation for this evidence.
    pub fn cite_as_reference(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("citeAsReference") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
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
    /// contains them - they cannot be identified independently, and nor can they have
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

    /// The date  (and optionally time) when the summary was published. The date must
    /// change when the business version changes and it must change if the status code
    /// changes. In addition, it should change when the substantive content of the summary
    /// changes.
    pub fn date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("date") {
            return Some(string);
        }
        return None;
    }

    /// A free text natural language description of the evidence from a consumer's
    /// perspective.
    pub fn description(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("description") {
            return Some(string);
        }
        return None;
    }

    /// An individiual, organization, or device primarily responsible for internal
    /// coherence of the content.
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

    /// An individiual, organization, or device responsible for officially endorsing the
    /// content for use in some setting.
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

    /// A formal identifier that is used to identify this summary when it is represented
    /// in other formats, or referenced in a specification, model, design or an instance.
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

    /// Footnotes and/or explanatory notes.
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

    /// The name of the organization or individual that published the evidence.
    pub fn publisher(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("publisher") {
            return Some(string);
        }
        return None;
    }

    /// Link or citation to artifact associated with the summary.
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

    /// An individiual, organization, or device primarily responsible for review of some
    /// aspect of the content.
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

    /// Values and parameters for a single statistic.
    pub fn statistic(&self) -> Option<Vec<Evidence_Statistic>> {
        if let Some(Value::Array(val)) = self.value.get("statistic") {
            return Some(
                val.into_iter()
                    .map(|e| Evidence_Statistic {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The status of this summary. Enables tracking the life-cycle of the content.
    pub fn status(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("status") {
            return Some(string);
        }
        return None;
    }

    /// The type of study that produced this evidence.
    pub fn study_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("studyType") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The method to combine studies.
    pub fn synthesis_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("synthesisType") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
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

    /// A short, descriptive, user-friendly title for the summary.
    pub fn title(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("title") {
            return Some(string);
        }
        return None;
    }

    /// An absolute URI that is used to identify this evidence when it is referenced in a
    /// specification, model, design or an instance; also called its canonical identifier.
    /// This SHOULD be globally unique and SHOULD be a literal address at which at which
    /// an authoritative instance of this summary is (or will be) published. This URL can
    /// be the target of a canonical reference. It SHALL remain the same when the summary
    /// is stored on different servers.
    pub fn url(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("url") {
            return Some(string);
        }
        return None;
    }

    /// The content was developed with a focus and intent of supporting the contexts that
    /// are listed. These contexts may be general categories (gender, age, ...) or may be
    /// references to specific programs (insurance plans, studies, ...) and may be used to
    /// assist with indexing and searching for appropriate evidence instances.
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

    /// Evidence variable such as population, exposure, or outcome.
    pub fn variable_definition(&self) -> Vec<Evidence_VariableDefinition> {
        self.value
            .get("variableDefinition")
            .unwrap()
            .as_array()
            .unwrap()
            .into_iter()
            .map(|e| Evidence_VariableDefinition {
                value: Cow::Borrowed(e),
            })
            .collect::<Vec<_>>()
    }

    /// The identifier that is used to identify this version of the summary when it is
    /// referenced in a specification, model, design or instance. This is an arbitrary
    /// value managed by the summary author and is not expected to be globally unique.
    /// For example, it might be a timestamp (e.g. yyyymmdd) if a managed version is
    /// not available. There is also no expectation that versions can be placed in a
    /// lexicographical sequence.
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
        if let Some(_val) = self._assertion() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._cite_as_markdown() {
            if !_val.validate() {
                return false;
            }
        }
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
        if let Some(_val) = self._publisher() {
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
        if let Some(_val) = self.approval_date() {}
        if let Some(_val) = self.assertion() {}
        if let Some(_val) = self.author() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.certainty() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.cite_as_markdown() {}
        if let Some(_val) = self.cite_as_reference() {
            if !_val.validate() {
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
        if let Some(_val) = self.date() {}
        if let Some(_val) = self.description() {}
        if let Some(_val) = self.editor() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.endorser() {
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
        if let Some(_val) = self.implicit_rules() {}
        if let Some(_val) = self.language() {}
        if let Some(_val) = self.last_review_date() {}
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
        if let Some(_val) = self.note() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.publisher() {}
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
        if let Some(_val) = self.statistic() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.status() {}
        if let Some(_val) = self.study_type() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.synthesis_type() {
            if !_val.validate() {
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
        if let Some(_val) = self.use_context() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if !self
            .variable_definition()
            .into_iter()
            .map(|e| e.validate())
            .all(|x| x == true)
        {
            return false;
        }
        if let Some(_val) = self.version() {}
        return true;
    }
}

#[derive(Debug)]
pub struct EvidenceBuilder {
    pub(crate) value: Value,
}

impl EvidenceBuilder {
    pub fn build(&self) -> Evidence {
        Evidence {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Evidence) -> EvidenceBuilder {
        EvidenceBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(variable_definition: Vec<Evidence_VariableDefinition>) -> EvidenceBuilder {
        let mut __value: Value = json!({});
        __value["variableDefinition"] = json!(variable_definition
            .into_iter()
            .map(|e| e.value)
            .collect::<Vec<_>>());
        return EvidenceBuilder { value: __value };
    }

    pub fn _approval_date<'a>(&'a mut self, val: Element) -> &'a mut EvidenceBuilder {
        self.value["_approvalDate"] = json!(val.value);
        return self;
    }

    pub fn _assertion<'a>(&'a mut self, val: Element) -> &'a mut EvidenceBuilder {
        self.value["_assertion"] = json!(val.value);
        return self;
    }

    pub fn _cite_as_markdown<'a>(&'a mut self, val: Element) -> &'a mut EvidenceBuilder {
        self.value["_citeAsMarkdown"] = json!(val.value);
        return self;
    }

    pub fn _date<'a>(&'a mut self, val: Element) -> &'a mut EvidenceBuilder {
        self.value["_date"] = json!(val.value);
        return self;
    }

    pub fn _description<'a>(&'a mut self, val: Element) -> &'a mut EvidenceBuilder {
        self.value["_description"] = json!(val.value);
        return self;
    }

    pub fn _implicit_rules<'a>(&'a mut self, val: Element) -> &'a mut EvidenceBuilder {
        self.value["_implicitRules"] = json!(val.value);
        return self;
    }

    pub fn _language<'a>(&'a mut self, val: Element) -> &'a mut EvidenceBuilder {
        self.value["_language"] = json!(val.value);
        return self;
    }

    pub fn _last_review_date<'a>(&'a mut self, val: Element) -> &'a mut EvidenceBuilder {
        self.value["_lastReviewDate"] = json!(val.value);
        return self;
    }

    pub fn _publisher<'a>(&'a mut self, val: Element) -> &'a mut EvidenceBuilder {
        self.value["_publisher"] = json!(val.value);
        return self;
    }

    pub fn _status<'a>(&'a mut self, val: Element) -> &'a mut EvidenceBuilder {
        self.value["_status"] = json!(val.value);
        return self;
    }

    pub fn _title<'a>(&'a mut self, val: Element) -> &'a mut EvidenceBuilder {
        self.value["_title"] = json!(val.value);
        return self;
    }

    pub fn _url<'a>(&'a mut self, val: Element) -> &'a mut EvidenceBuilder {
        self.value["_url"] = json!(val.value);
        return self;
    }

    pub fn _version<'a>(&'a mut self, val: Element) -> &'a mut EvidenceBuilder {
        self.value["_version"] = json!(val.value);
        return self;
    }

    pub fn approval_date<'a>(&'a mut self, val: &str) -> &'a mut EvidenceBuilder {
        self.value["approvalDate"] = json!(val);
        return self;
    }

    pub fn assertion<'a>(&'a mut self, val: &str) -> &'a mut EvidenceBuilder {
        self.value["assertion"] = json!(val);
        return self;
    }

    pub fn author<'a>(&'a mut self, val: Vec<ContactDetail>) -> &'a mut EvidenceBuilder {
        self.value["author"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn certainty<'a>(&'a mut self, val: Vec<Evidence_Certainty>) -> &'a mut EvidenceBuilder {
        self.value["certainty"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn cite_as_markdown<'a>(&'a mut self, val: &str) -> &'a mut EvidenceBuilder {
        self.value["citeAsMarkdown"] = json!(val);
        return self;
    }

    pub fn cite_as_reference<'a>(&'a mut self, val: Reference) -> &'a mut EvidenceBuilder {
        self.value["citeAsReference"] = json!(val.value);
        return self;
    }

    pub fn contact<'a>(&'a mut self, val: Vec<ContactDetail>) -> &'a mut EvidenceBuilder {
        self.value["contact"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn contained<'a>(&'a mut self, val: Vec<ResourceList>) -> &'a mut EvidenceBuilder {
        self.value["contained"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn date<'a>(&'a mut self, val: &str) -> &'a mut EvidenceBuilder {
        self.value["date"] = json!(val);
        return self;
    }

    pub fn description<'a>(&'a mut self, val: &str) -> &'a mut EvidenceBuilder {
        self.value["description"] = json!(val);
        return self;
    }

    pub fn editor<'a>(&'a mut self, val: Vec<ContactDetail>) -> &'a mut EvidenceBuilder {
        self.value["editor"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn endorser<'a>(&'a mut self, val: Vec<ContactDetail>) -> &'a mut EvidenceBuilder {
        self.value["endorser"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut EvidenceBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut EvidenceBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn identifier<'a>(&'a mut self, val: Vec<Identifier>) -> &'a mut EvidenceBuilder {
        self.value["identifier"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn implicit_rules<'a>(&'a mut self, val: &str) -> &'a mut EvidenceBuilder {
        self.value["implicitRules"] = json!(val);
        return self;
    }

    pub fn language<'a>(&'a mut self, val: &str) -> &'a mut EvidenceBuilder {
        self.value["language"] = json!(val);
        return self;
    }

    pub fn last_review_date<'a>(&'a mut self, val: &str) -> &'a mut EvidenceBuilder {
        self.value["lastReviewDate"] = json!(val);
        return self;
    }

    pub fn meta<'a>(&'a mut self, val: Meta) -> &'a mut EvidenceBuilder {
        self.value["meta"] = json!(val.value);
        return self;
    }

    pub fn modifier_extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut EvidenceBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn note<'a>(&'a mut self, val: Vec<Annotation>) -> &'a mut EvidenceBuilder {
        self.value["note"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn publisher<'a>(&'a mut self, val: &str) -> &'a mut EvidenceBuilder {
        self.value["publisher"] = json!(val);
        return self;
    }

    pub fn related_artifact<'a>(
        &'a mut self,
        val: Vec<RelatedArtifact>,
    ) -> &'a mut EvidenceBuilder {
        self.value["relatedArtifact"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn reviewer<'a>(&'a mut self, val: Vec<ContactDetail>) -> &'a mut EvidenceBuilder {
        self.value["reviewer"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn statistic<'a>(&'a mut self, val: Vec<Evidence_Statistic>) -> &'a mut EvidenceBuilder {
        self.value["statistic"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn status<'a>(&'a mut self, val: &str) -> &'a mut EvidenceBuilder {
        self.value["status"] = json!(val);
        return self;
    }

    pub fn study_type<'a>(&'a mut self, val: CodeableConcept) -> &'a mut EvidenceBuilder {
        self.value["studyType"] = json!(val.value);
        return self;
    }

    pub fn synthesis_type<'a>(&'a mut self, val: CodeableConcept) -> &'a mut EvidenceBuilder {
        self.value["synthesisType"] = json!(val.value);
        return self;
    }

    pub fn text<'a>(&'a mut self, val: Narrative) -> &'a mut EvidenceBuilder {
        self.value["text"] = json!(val.value);
        return self;
    }

    pub fn title<'a>(&'a mut self, val: &str) -> &'a mut EvidenceBuilder {
        self.value["title"] = json!(val);
        return self;
    }

    pub fn url<'a>(&'a mut self, val: &str) -> &'a mut EvidenceBuilder {
        self.value["url"] = json!(val);
        return self;
    }

    pub fn use_context<'a>(&'a mut self, val: Vec<UsageContext>) -> &'a mut EvidenceBuilder {
        self.value["useContext"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn version<'a>(&'a mut self, val: &str) -> &'a mut EvidenceBuilder {
        self.value["version"] = json!(val);
        return self;
    }
}
