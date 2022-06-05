#![allow(unused_imports, non_camel_case_types)]

use crate::models::r5::CodeableConcept::CodeableConcept;
use crate::models::r5::ContactDetail::ContactDetail;
use crate::models::r5::Element::Element;
use crate::models::r5::Extension::Extension;
use crate::models::r5::Identifier::Identifier;
use crate::models::r5::Meta::Meta;
use crate::models::r5::Narrative::Narrative;
use crate::models::r5::Period::Period;
use crate::models::r5::Reference::Reference;
use crate::models::r5::ResourceList::ResourceList;
use crate::models::r5::SpecimenDefinition_TypeTested::SpecimenDefinition_TypeTested;
use crate::models::r5::UsageContext::UsageContext;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A kind of specimen with associated set of requirements.

#[derive(Debug)]
pub struct SpecimenDefinition<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl SpecimenDefinition<'_> {
    pub fn new(value: &Value) -> SpecimenDefinition {
        SpecimenDefinition {
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

    /// Extensions for derivedFromUri
    pub fn _derived_from_uri(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_derivedFromUri") {
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

    /// Extensions for timeAspect
    pub fn _time_aspect(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_timeAspect") {
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

    /// The date on which the asset content was approved by the publisher. Approval
    /// happens once when the content is officially approved for usage.
    pub fn approval_date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("approvalDate") {
            return Some(string);
        }
        return None;
    }

    /// The action to be performed for collecting the specimen.
    pub fn collection(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("collection") {
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

    /// Copyright statement relating to the SpecimenDefinition and/or its contents.
    /// Copyright statements are generally legal restrictions on the use and publishing of
    /// the SpecimenDefinition.
    pub fn copyright(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("copyright") {
            return Some(string);
        }
        return None;
    }

    /// For draft definitions, indicates the date of initial creation. For active
    /// definitions, represents the date of activation. For withdrawn definitions,
    /// indicates the date of withdrawal.
    pub fn date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("date") {
            return Some(string);
        }
        return None;
    }

    /// The canonical URL pointing to another FHIR-defined SpecimenDefinition that is
    /// adhered to in whole or in part by this definition.
    pub fn derived_from_canonical(&self) -> Option<Vec<&str>> {
        if let Some(Value::Array(val)) = self.value.get("derivedFromCanonical") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_str().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The URL pointing to an externally-defined type of specimen, guideline or other
    /// definition that is adhered to in whole or in part by this definition.
    pub fn derived_from_uri(&self) -> Option<Vec<&str>> {
        if let Some(Value::Array(val)) = self.value.get("derivedFromUri") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_str().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A free text natural language description of the SpecimenDefinition from the
    /// consumer's perspective.
    pub fn description(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("description") {
            return Some(string);
        }
        return None;
    }

    /// The period during which the SpecimenDefinition content was or is planned to be
    /// effective.
    pub fn effective_period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("effectivePeriod") {
            return Some(Period {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A flag to indicate that this SpecimenDefinition is not authored for  genuine
    /// usage.
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

    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
        }
        return None;
    }

    /// A business identifier assigned to this SpecimenDefinition.
    pub fn identifier(&self) -> Option<Identifier> {
        if let Some(val) = self.value.get("identifier") {
            return Some(Identifier {
                value: Cow::Borrowed(val),
            });
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

    /// A jurisdiction in which the SpecimenDefinition is intended to be used.
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

    /// The date on which the asset content was last reviewed. Review happens periodically
    /// after that, but doesn't change the original approval date.
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

    /// Preparation of the patient for specimen collection.
    pub fn patient_preparation(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("patientPreparation") {
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

    /// Helps establish the "authority/credibility" of the SpecimenDefinition. May also
    /// allow for contact.
    pub fn publisher(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("publisher") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Explains why this SpecimeDefinition is needed and why it has been designed as
    /// it has.
    pub fn purpose(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("purpose") {
            return Some(string);
        }
        return None;
    }

    /// The current state of theSpecimenDefinition.
    pub fn status(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("status") {
            return Some(string);
        }
        return None;
    }

    /// A code or group definition that describes the intended subject  from which this
    /// kind of specimen is to be collected.
    pub fn subject_codeable_concept(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("subjectCodeableConcept") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A code or group definition that describes the intended subject  from which this
    /// kind of specimen is to be collected.
    pub fn subject_reference(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("subjectReference") {
            return Some(Reference {
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

    /// Time aspect of specimen collection (duration or offset).
    pub fn time_aspect(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("timeAspect") {
            return Some(string);
        }
        return None;
    }

    /// A short, descriptive, user-friendly title for the SpecimenDefinition.
    pub fn title(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("title") {
            return Some(string);
        }
        return None;
    }

    /// The kind of material to be collected.
    pub fn type_collected(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("typeCollected") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Specimen conditioned in a container as expected by the testing laboratory.
    pub fn type_tested(&self) -> Option<Vec<SpecimenDefinition_TypeTested>> {
        if let Some(Value::Array(val)) = self.value.get("typeTested") {
            return Some(
                val.into_iter()
                    .map(|e| SpecimenDefinition_TypeTested {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// An absolute URL that is used to identify this SpecimenDefinition when it
    /// is referenced in a specification, model, design or an instance. This SHALL
    /// be a URL, SHOULD be globally unique, and SHOULD be an address at which this
    /// SpecimenDefinition is (or will be) published. The URL SHOULD include the major
    /// version of the SpecimenDefinition. For more information see Technical and Business
    /// Versions.
    pub fn url(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("url") {
            return Some(string);
        }
        return None;
    }

    /// The content was developed with a focus and intent of supporting the contexts
    /// that are listed. These terms may be used to assist with indexing and searching of
    /// specimen definitions.
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

    /// The identifier that is used to identify this version of the SpecimenDefinition
    /// when it is referenced in a specification, model, design or instance. This is an
    /// arbitrary value managed by the SpecimenDefinition author and is not expected to be
    /// globally unique.
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
        if let Some(_val) = self._derived_from_uri() {
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
        if let Some(_val) = self._time_aspect() {
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
        if let Some(_val) = self.collection() {
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
        if let Some(_val) = self.copyright() {}
        if let Some(_val) = self.date() {}
        if let Some(_val) = self.derived_from_canonical() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.derived_from_uri() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.description() {}
        if let Some(_val) = self.effective_period() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.experimental() {}
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.identifier() {
            if !_val.validate() {
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
        if let Some(_val) = self.patient_preparation() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.publisher() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.purpose() {}
        if let Some(_val) = self.status() {}
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
        if let Some(_val) = self.text() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.time_aspect() {}
        if let Some(_val) = self.title() {}
        if let Some(_val) = self.type_collected() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.type_tested() {
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
pub struct SpecimenDefinitionBuilder {
    pub(crate) value: Value,
}

impl SpecimenDefinitionBuilder {
    pub fn build(&self) -> SpecimenDefinition {
        SpecimenDefinition {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: SpecimenDefinition) -> SpecimenDefinitionBuilder {
        SpecimenDefinitionBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> SpecimenDefinitionBuilder {
        let mut __value: Value = json!({});
        return SpecimenDefinitionBuilder { value: __value };
    }

    pub fn _approval_date<'a>(&'a mut self, val: Element) -> &'a mut SpecimenDefinitionBuilder {
        self.value["_approvalDate"] = json!(val.value);
        return self;
    }

    pub fn _copyright<'a>(&'a mut self, val: Element) -> &'a mut SpecimenDefinitionBuilder {
        self.value["_copyright"] = json!(val.value);
        return self;
    }

    pub fn _date<'a>(&'a mut self, val: Element) -> &'a mut SpecimenDefinitionBuilder {
        self.value["_date"] = json!(val.value);
        return self;
    }

    pub fn _derived_from_uri<'a>(
        &'a mut self,
        val: Vec<Element>,
    ) -> &'a mut SpecimenDefinitionBuilder {
        self.value["_derivedFromUri"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn _description<'a>(&'a mut self, val: Element) -> &'a mut SpecimenDefinitionBuilder {
        self.value["_description"] = json!(val.value);
        return self;
    }

    pub fn _experimental<'a>(&'a mut self, val: Element) -> &'a mut SpecimenDefinitionBuilder {
        self.value["_experimental"] = json!(val.value);
        return self;
    }

    pub fn _implicit_rules<'a>(&'a mut self, val: Element) -> &'a mut SpecimenDefinitionBuilder {
        self.value["_implicitRules"] = json!(val.value);
        return self;
    }

    pub fn _language<'a>(&'a mut self, val: Element) -> &'a mut SpecimenDefinitionBuilder {
        self.value["_language"] = json!(val.value);
        return self;
    }

    pub fn _last_review_date<'a>(&'a mut self, val: Element) -> &'a mut SpecimenDefinitionBuilder {
        self.value["_lastReviewDate"] = json!(val.value);
        return self;
    }

    pub fn _purpose<'a>(&'a mut self, val: Element) -> &'a mut SpecimenDefinitionBuilder {
        self.value["_purpose"] = json!(val.value);
        return self;
    }

    pub fn _status<'a>(&'a mut self, val: Element) -> &'a mut SpecimenDefinitionBuilder {
        self.value["_status"] = json!(val.value);
        return self;
    }

    pub fn _time_aspect<'a>(&'a mut self, val: Element) -> &'a mut SpecimenDefinitionBuilder {
        self.value["_timeAspect"] = json!(val.value);
        return self;
    }

    pub fn _title<'a>(&'a mut self, val: Element) -> &'a mut SpecimenDefinitionBuilder {
        self.value["_title"] = json!(val.value);
        return self;
    }

    pub fn _url<'a>(&'a mut self, val: Element) -> &'a mut SpecimenDefinitionBuilder {
        self.value["_url"] = json!(val.value);
        return self;
    }

    pub fn _version<'a>(&'a mut self, val: Element) -> &'a mut SpecimenDefinitionBuilder {
        self.value["_version"] = json!(val.value);
        return self;
    }

    pub fn approval_date<'a>(&'a mut self, val: &str) -> &'a mut SpecimenDefinitionBuilder {
        self.value["approvalDate"] = json!(val);
        return self;
    }

    pub fn collection<'a>(
        &'a mut self,
        val: Vec<CodeableConcept>,
    ) -> &'a mut SpecimenDefinitionBuilder {
        self.value["collection"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn contact<'a>(&'a mut self, val: Vec<ContactDetail>) -> &'a mut SpecimenDefinitionBuilder {
        self.value["contact"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn contained<'a>(
        &'a mut self,
        val: Vec<ResourceList>,
    ) -> &'a mut SpecimenDefinitionBuilder {
        self.value["contained"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn copyright<'a>(&'a mut self, val: &str) -> &'a mut SpecimenDefinitionBuilder {
        self.value["copyright"] = json!(val);
        return self;
    }

    pub fn date<'a>(&'a mut self, val: &str) -> &'a mut SpecimenDefinitionBuilder {
        self.value["date"] = json!(val);
        return self;
    }

    pub fn derived_from_canonical<'a>(
        &'a mut self,
        val: Vec<&str>,
    ) -> &'a mut SpecimenDefinitionBuilder {
        self.value["derivedFromCanonical"] = json!(val);
        return self;
    }

    pub fn derived_from_uri<'a>(&'a mut self, val: Vec<&str>) -> &'a mut SpecimenDefinitionBuilder {
        self.value["derivedFromUri"] = json!(val);
        return self;
    }

    pub fn description<'a>(&'a mut self, val: &str) -> &'a mut SpecimenDefinitionBuilder {
        self.value["description"] = json!(val);
        return self;
    }

    pub fn effective_period<'a>(&'a mut self, val: Period) -> &'a mut SpecimenDefinitionBuilder {
        self.value["effectivePeriod"] = json!(val.value);
        return self;
    }

    pub fn experimental<'a>(&'a mut self, val: bool) -> &'a mut SpecimenDefinitionBuilder {
        self.value["experimental"] = json!(val);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut SpecimenDefinitionBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut SpecimenDefinitionBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn identifier<'a>(&'a mut self, val: Identifier) -> &'a mut SpecimenDefinitionBuilder {
        self.value["identifier"] = json!(val.value);
        return self;
    }

    pub fn implicit_rules<'a>(&'a mut self, val: &str) -> &'a mut SpecimenDefinitionBuilder {
        self.value["implicitRules"] = json!(val);
        return self;
    }

    pub fn jurisdiction<'a>(
        &'a mut self,
        val: Vec<CodeableConcept>,
    ) -> &'a mut SpecimenDefinitionBuilder {
        self.value["jurisdiction"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn language<'a>(&'a mut self, val: &str) -> &'a mut SpecimenDefinitionBuilder {
        self.value["language"] = json!(val);
        return self;
    }

    pub fn last_review_date<'a>(&'a mut self, val: &str) -> &'a mut SpecimenDefinitionBuilder {
        self.value["lastReviewDate"] = json!(val);
        return self;
    }

    pub fn meta<'a>(&'a mut self, val: Meta) -> &'a mut SpecimenDefinitionBuilder {
        self.value["meta"] = json!(val.value);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut SpecimenDefinitionBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn patient_preparation<'a>(
        &'a mut self,
        val: Vec<CodeableConcept>,
    ) -> &'a mut SpecimenDefinitionBuilder {
        self.value["patientPreparation"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn publisher<'a>(&'a mut self, val: Reference) -> &'a mut SpecimenDefinitionBuilder {
        self.value["publisher"] = json!(val.value);
        return self;
    }

    pub fn purpose<'a>(&'a mut self, val: &str) -> &'a mut SpecimenDefinitionBuilder {
        self.value["purpose"] = json!(val);
        return self;
    }

    pub fn status<'a>(&'a mut self, val: &str) -> &'a mut SpecimenDefinitionBuilder {
        self.value["status"] = json!(val);
        return self;
    }

    pub fn subject_codeable_concept<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut SpecimenDefinitionBuilder {
        self.value["subjectCodeableConcept"] = json!(val.value);
        return self;
    }

    pub fn subject_reference<'a>(
        &'a mut self,
        val: Reference,
    ) -> &'a mut SpecimenDefinitionBuilder {
        self.value["subjectReference"] = json!(val.value);
        return self;
    }

    pub fn text<'a>(&'a mut self, val: Narrative) -> &'a mut SpecimenDefinitionBuilder {
        self.value["text"] = json!(val.value);
        return self;
    }

    pub fn time_aspect<'a>(&'a mut self, val: &str) -> &'a mut SpecimenDefinitionBuilder {
        self.value["timeAspect"] = json!(val);
        return self;
    }

    pub fn title<'a>(&'a mut self, val: &str) -> &'a mut SpecimenDefinitionBuilder {
        self.value["title"] = json!(val);
        return self;
    }

    pub fn type_collected<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut SpecimenDefinitionBuilder {
        self.value["typeCollected"] = json!(val.value);
        return self;
    }

    pub fn type_tested<'a>(
        &'a mut self,
        val: Vec<SpecimenDefinition_TypeTested>,
    ) -> &'a mut SpecimenDefinitionBuilder {
        self.value["typeTested"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn url<'a>(&'a mut self, val: &str) -> &'a mut SpecimenDefinitionBuilder {
        self.value["url"] = json!(val);
        return self;
    }

    pub fn use_context<'a>(
        &'a mut self,
        val: Vec<UsageContext>,
    ) -> &'a mut SpecimenDefinitionBuilder {
        self.value["useContext"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn version<'a>(&'a mut self, val: &str) -> &'a mut SpecimenDefinitionBuilder {
        self.value["version"] = json!(val);
        return self;
    }
}
