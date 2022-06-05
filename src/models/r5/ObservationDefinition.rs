#![allow(unused_imports, non_camel_case_types)]

use crate::models::r5::CodeableConcept::CodeableConcept;
use crate::models::r5::ContactDetail::ContactDetail;
use crate::models::r5::Element::Element;
use crate::models::r5::Extension::Extension;
use crate::models::r5::Identifier::Identifier;
use crate::models::r5::Meta::Meta;
use crate::models::r5::Narrative::Narrative;
use crate::models::r5::ObservationDefinition_Component::ObservationDefinition_Component;
use crate::models::r5::ObservationDefinition_QualifiedValue::ObservationDefinition_QualifiedValue;
use crate::models::r5::ObservationDefinition_QuantitativeDetails::ObservationDefinition_QuantitativeDetails;
use crate::models::r5::Period::Period;
use crate::models::r5::Reference::Reference;
use crate::models::r5::ResourceList::ResourceList;
use crate::models::r5::UsageContext::UsageContext;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Set of definitional characteristics for a kind of observation or measurement
/// produced or consumed by an orderable health care service.

#[derive(Debug)]
pub struct ObservationDefinition<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl ObservationDefinition<'_> {
    pub fn new(value: &Value) -> ObservationDefinition {
        ObservationDefinition {
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

    /// Extensions for multipleResultsAllowed
    pub fn _multiple_results_allowed(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_multipleResultsAllowed") {
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

    /// Extensions for permittedDataType
    pub fn _permitted_data_type(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_permittedDataType") {
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

    /// Extensions for preferredReportName
    pub fn _preferred_report_name(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_preferredReportName") {
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

    /// The site on the subject's body where the  observation is to be made.
    pub fn body_site(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("bodySite") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A code that classifies the general type of observation.
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

    /// Describes what will be observed. Sometimes this is called the observation "name".
    pub fn code(&self) -> CodeableConcept {
        CodeableConcept {
            value: Cow::Borrowed(&self.value["code"]),
        }
    }

    /// Some observations have multiple component observations, expressed as separate code
    /// value pairs.
    pub fn component(&self) -> Option<Vec<ObservationDefinition_Component>> {
        if let Some(Value::Array(val)) = self.value.get("component") {
            return Some(
                val.into_iter()
                    .map(|e| ObservationDefinition_Component {
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

    /// Copyright statement relating to the ObservationDefinition and/or its contents.
    /// Copyright statements are generally legal restrictions on the use and publishing of
    /// the ObservationDefinition.
    pub fn copyright(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("copyright") {
            return Some(string);
        }
        return None;
    }

    /// The date (and optionally time) when the ObservationDefinition was published.
    /// The date must change when the business version changes and it must change if the
    /// status code changes. In addition, it should change when the substantive content of
    /// the ObservationDefinition changes.
    pub fn date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("date") {
            return Some(string);
        }
        return None;
    }

    /// The canonical URL pointing to another FHIR-defined ObservationDefinition that is
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

    /// The URL pointing to an externally-defined observation definition, guideline or
    /// other definition that is adhered to in whole or in part by this definition.
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

    /// A free text natural language description of the ObservationDefinition from the
    /// consumer's perspective.
    pub fn description(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("description") {
            return Some(string);
        }
        return None;
    }

    /// The measurement model of device or actual device used to produce observations of
    /// this type.
    pub fn device(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("device") {
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

    /// The period during which the ObservationDefinition content was or is planned to
    /// be effective.
    pub fn effective_period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("effectivePeriod") {
            return Some(Period {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A flag to indicate that this ObservationDefinition is authored for testing
    /// purposes (or education/evaluation/marketing), and is not intended to be used for
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

    /// This ObservationDefinition defines a group  observation (e.g. a battery, a panel
    /// of tests, a set of vital sign measurements) that includes the target as a member
    /// of the group.
    pub fn has_member(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("hasMember") {
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

    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
        }
        return None;
    }

    /// Business identifiers assigned to this ObservationDefinition. by the performer and/
    /// or other systems. These identifiers remain constant as the resource is updated and
    /// propagates from server to server.
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

    /// A jurisdiction in which the ObservationDefinition is intended to be used.
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

    /// The method or technique used to perform the observation.
    pub fn method(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("method") {
            return Some(CodeableConcept {
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

    /// Multiple results allowed for observations conforming to this
    /// ObservationDefinition.
    pub fn multiple_results_allowed(&self) -> Option<bool> {
        if let Some(val) = self.value.get("multipleResultsAllowed") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    /// A natural language name identifying the ObservationDefinition. This name should be
    /// usable as an identifier for the module by machine processing applications such as
    /// code generation.
    pub fn name(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("name") {
            return Some(string);
        }
        return None;
    }

    /// The type of individual/organization/device that is expected to act upon instances
    /// of this definition.
    pub fn performer_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("performerType") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The data types allowed for the value element of the instance observations
    /// conforming to this ObservationDefinition.
    pub fn permitted_data_type(&self) -> Option<Vec<&str>> {
        if let Some(Value::Array(val)) = self.value.get("permittedDataType") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_str().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The preferred name to be used when reporting the results of observations
    /// conforming to this ObservationDefinition.
    pub fn preferred_report_name(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("preferredReportName") {
            return Some(string);
        }
        return None;
    }

    /// Helps establish the "authority/credibility" of the ObservationDefinition. May also
    /// allow for contact.
    pub fn publisher(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("publisher") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Explains why this ObservationDefinition is needed and why it has been designed as
    /// it has.
    pub fn purpose(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("purpose") {
            return Some(string);
        }
        return None;
    }

    /// A set of qualified values associated with a context and a set of conditions -
    /// provides a range for quantitative and ordinal observations and a collection of
    /// value sets for qualitative observations.
    pub fn qualified_value(&self) -> Option<Vec<ObservationDefinition_QualifiedValue>> {
        if let Some(Value::Array(val)) = self.value.get("qualifiedValue") {
            return Some(
                val.into_iter()
                    .map(|e| ObservationDefinition_QualifiedValue {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Characteristics for quantitative results of observations conforming to this
    /// ObservationDefinition.
    pub fn quantitative_details(&self) -> Option<ObservationDefinition_QuantitativeDetails> {
        if let Some(val) = self.value.get("quantitativeDetails") {
            return Some(ObservationDefinition_QuantitativeDetails {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The kind of specimen that this type of observation is produced on.
    pub fn specimen(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("specimen") {
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

    /// The current state of the ObservationDefinition.
    pub fn status(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("status") {
            return Some(string);
        }
        return None;
    }

    /// A code that describes the intended kind of subject of Observation instances
    /// conforming to this ObservationDefinition.
    pub fn subject(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("subject") {
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

    /// A short, descriptive, user-friendly title for the ObservationDefinition.
    pub fn title(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("title") {
            return Some(string);
        }
        return None;
    }

    /// An absolute URL that is used to identify this ObservationDefinition when it
    /// is referenced in a specification, model, design or an instance. This SHALL
    /// be a URL, SHOULD be globally unique, and SHOULD be an address at which this
    /// ObservationDefinition is (or will be) published. The URL SHOULD include the major
    /// version of the ObservationDefinition. For more information see Technical and
    /// Business Versions.
    pub fn url(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("url") {
            return Some(string);
        }
        return None;
    }

    /// The content was developed with a focus and intent of supporting the contexts that
    /// are listed. These contexts may be general categories (gender, age, ...) or may
    /// be references to specific programs (insurance plans, studies, ...) and may be
    /// used to assist with indexing and searching for appropriate ObservationDefinition
    /// instances.
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

    /// The identifier that is used to identify this version of the ObservationDefinition
    /// when it is referenced in a specification, model, design or instance. This is an
    /// arbitrary value managed by the ObservationDefinition author and is not expected
    /// to be globally unique. For example, it might be a timestamp (e.g. yyyymmdd) if
    /// a managed version is not available. There is also no expectation that versions
    /// are orderable.
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
        if let Some(_val) = self._multiple_results_allowed() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._name() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._permitted_data_type() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self._preferred_report_name() {
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
        if let Some(_val) = self.body_site() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.category() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if !self.code().validate() {
            return false;
        }
        if let Some(_val) = self.component() {
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
        if let Some(_val) = self.device() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
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
        if let Some(_val) = self.has_member() {
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
        if let Some(_val) = self.method() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.multiple_results_allowed() {}
        if let Some(_val) = self.name() {}
        if let Some(_val) = self.performer_type() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.permitted_data_type() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.preferred_report_name() {}
        if let Some(_val) = self.publisher() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.purpose() {}
        if let Some(_val) = self.qualified_value() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.quantitative_details() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.specimen() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.status() {}
        if let Some(_val) = self.subject() {
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
pub struct ObservationDefinitionBuilder {
    pub(crate) value: Value,
}

impl ObservationDefinitionBuilder {
    pub fn build(&self) -> ObservationDefinition {
        ObservationDefinition {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: ObservationDefinition) -> ObservationDefinitionBuilder {
        ObservationDefinitionBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(code: CodeableConcept) -> ObservationDefinitionBuilder {
        let mut __value: Value = json!({});
        __value["code"] = json!(code.value);
        return ObservationDefinitionBuilder { value: __value };
    }

    pub fn _approval_date<'a>(&'a mut self, val: Element) -> &'a mut ObservationDefinitionBuilder {
        self.value["_approvalDate"] = json!(val.value);
        return self;
    }

    pub fn _copyright<'a>(&'a mut self, val: Element) -> &'a mut ObservationDefinitionBuilder {
        self.value["_copyright"] = json!(val.value);
        return self;
    }

    pub fn _date<'a>(&'a mut self, val: Element) -> &'a mut ObservationDefinitionBuilder {
        self.value["_date"] = json!(val.value);
        return self;
    }

    pub fn _derived_from_uri<'a>(
        &'a mut self,
        val: Vec<Element>,
    ) -> &'a mut ObservationDefinitionBuilder {
        self.value["_derivedFromUri"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn _description<'a>(&'a mut self, val: Element) -> &'a mut ObservationDefinitionBuilder {
        self.value["_description"] = json!(val.value);
        return self;
    }

    pub fn _experimental<'a>(&'a mut self, val: Element) -> &'a mut ObservationDefinitionBuilder {
        self.value["_experimental"] = json!(val.value);
        return self;
    }

    pub fn _implicit_rules<'a>(&'a mut self, val: Element) -> &'a mut ObservationDefinitionBuilder {
        self.value["_implicitRules"] = json!(val.value);
        return self;
    }

    pub fn _language<'a>(&'a mut self, val: Element) -> &'a mut ObservationDefinitionBuilder {
        self.value["_language"] = json!(val.value);
        return self;
    }

    pub fn _last_review_date<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut ObservationDefinitionBuilder {
        self.value["_lastReviewDate"] = json!(val.value);
        return self;
    }

    pub fn _multiple_results_allowed<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut ObservationDefinitionBuilder {
        self.value["_multipleResultsAllowed"] = json!(val.value);
        return self;
    }

    pub fn _name<'a>(&'a mut self, val: Element) -> &'a mut ObservationDefinitionBuilder {
        self.value["_name"] = json!(val.value);
        return self;
    }

    pub fn _permitted_data_type<'a>(
        &'a mut self,
        val: Vec<Element>,
    ) -> &'a mut ObservationDefinitionBuilder {
        self.value["_permittedDataType"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn _preferred_report_name<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut ObservationDefinitionBuilder {
        self.value["_preferredReportName"] = json!(val.value);
        return self;
    }

    pub fn _purpose<'a>(&'a mut self, val: Element) -> &'a mut ObservationDefinitionBuilder {
        self.value["_purpose"] = json!(val.value);
        return self;
    }

    pub fn _status<'a>(&'a mut self, val: Element) -> &'a mut ObservationDefinitionBuilder {
        self.value["_status"] = json!(val.value);
        return self;
    }

    pub fn _title<'a>(&'a mut self, val: Element) -> &'a mut ObservationDefinitionBuilder {
        self.value["_title"] = json!(val.value);
        return self;
    }

    pub fn _url<'a>(&'a mut self, val: Element) -> &'a mut ObservationDefinitionBuilder {
        self.value["_url"] = json!(val.value);
        return self;
    }

    pub fn _version<'a>(&'a mut self, val: Element) -> &'a mut ObservationDefinitionBuilder {
        self.value["_version"] = json!(val.value);
        return self;
    }

    pub fn approval_date<'a>(&'a mut self, val: &str) -> &'a mut ObservationDefinitionBuilder {
        self.value["approvalDate"] = json!(val);
        return self;
    }

    pub fn body_site<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut ObservationDefinitionBuilder {
        self.value["bodySite"] = json!(val.value);
        return self;
    }

    pub fn category<'a>(
        &'a mut self,
        val: Vec<CodeableConcept>,
    ) -> &'a mut ObservationDefinitionBuilder {
        self.value["category"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn component<'a>(
        &'a mut self,
        val: Vec<ObservationDefinition_Component>,
    ) -> &'a mut ObservationDefinitionBuilder {
        self.value["component"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn contact<'a>(
        &'a mut self,
        val: Vec<ContactDetail>,
    ) -> &'a mut ObservationDefinitionBuilder {
        self.value["contact"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn contained<'a>(
        &'a mut self,
        val: Vec<ResourceList>,
    ) -> &'a mut ObservationDefinitionBuilder {
        self.value["contained"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn copyright<'a>(&'a mut self, val: &str) -> &'a mut ObservationDefinitionBuilder {
        self.value["copyright"] = json!(val);
        return self;
    }

    pub fn date<'a>(&'a mut self, val: &str) -> &'a mut ObservationDefinitionBuilder {
        self.value["date"] = json!(val);
        return self;
    }

    pub fn derived_from_canonical<'a>(
        &'a mut self,
        val: Vec<&str>,
    ) -> &'a mut ObservationDefinitionBuilder {
        self.value["derivedFromCanonical"] = json!(val);
        return self;
    }

    pub fn derived_from_uri<'a>(
        &'a mut self,
        val: Vec<&str>,
    ) -> &'a mut ObservationDefinitionBuilder {
        self.value["derivedFromUri"] = json!(val);
        return self;
    }

    pub fn description<'a>(&'a mut self, val: &str) -> &'a mut ObservationDefinitionBuilder {
        self.value["description"] = json!(val);
        return self;
    }

    pub fn device<'a>(&'a mut self, val: Vec<Reference>) -> &'a mut ObservationDefinitionBuilder {
        self.value["device"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn effective_period<'a>(&'a mut self, val: Period) -> &'a mut ObservationDefinitionBuilder {
        self.value["effectivePeriod"] = json!(val.value);
        return self;
    }

    pub fn experimental<'a>(&'a mut self, val: bool) -> &'a mut ObservationDefinitionBuilder {
        self.value["experimental"] = json!(val);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ObservationDefinitionBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn has_member<'a>(
        &'a mut self,
        val: Vec<Reference>,
    ) -> &'a mut ObservationDefinitionBuilder {
        self.value["hasMember"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut ObservationDefinitionBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn identifier<'a>(&'a mut self, val: Identifier) -> &'a mut ObservationDefinitionBuilder {
        self.value["identifier"] = json!(val.value);
        return self;
    }

    pub fn implicit_rules<'a>(&'a mut self, val: &str) -> &'a mut ObservationDefinitionBuilder {
        self.value["implicitRules"] = json!(val);
        return self;
    }

    pub fn jurisdiction<'a>(
        &'a mut self,
        val: Vec<CodeableConcept>,
    ) -> &'a mut ObservationDefinitionBuilder {
        self.value["jurisdiction"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn language<'a>(&'a mut self, val: &str) -> &'a mut ObservationDefinitionBuilder {
        self.value["language"] = json!(val);
        return self;
    }

    pub fn last_review_date<'a>(&'a mut self, val: &str) -> &'a mut ObservationDefinitionBuilder {
        self.value["lastReviewDate"] = json!(val);
        return self;
    }

    pub fn meta<'a>(&'a mut self, val: Meta) -> &'a mut ObservationDefinitionBuilder {
        self.value["meta"] = json!(val.value);
        return self;
    }

    pub fn method<'a>(&'a mut self, val: CodeableConcept) -> &'a mut ObservationDefinitionBuilder {
        self.value["method"] = json!(val.value);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ObservationDefinitionBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn multiple_results_allowed<'a>(
        &'a mut self,
        val: bool,
    ) -> &'a mut ObservationDefinitionBuilder {
        self.value["multipleResultsAllowed"] = json!(val);
        return self;
    }

    pub fn name<'a>(&'a mut self, val: &str) -> &'a mut ObservationDefinitionBuilder {
        self.value["name"] = json!(val);
        return self;
    }

    pub fn performer_type<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut ObservationDefinitionBuilder {
        self.value["performerType"] = json!(val.value);
        return self;
    }

    pub fn permitted_data_type<'a>(
        &'a mut self,
        val: Vec<&str>,
    ) -> &'a mut ObservationDefinitionBuilder {
        self.value["permittedDataType"] = json!(val);
        return self;
    }

    pub fn preferred_report_name<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut ObservationDefinitionBuilder {
        self.value["preferredReportName"] = json!(val);
        return self;
    }

    pub fn publisher<'a>(&'a mut self, val: Reference) -> &'a mut ObservationDefinitionBuilder {
        self.value["publisher"] = json!(val.value);
        return self;
    }

    pub fn purpose<'a>(&'a mut self, val: &str) -> &'a mut ObservationDefinitionBuilder {
        self.value["purpose"] = json!(val);
        return self;
    }

    pub fn qualified_value<'a>(
        &'a mut self,
        val: Vec<ObservationDefinition_QualifiedValue>,
    ) -> &'a mut ObservationDefinitionBuilder {
        self.value["qualifiedValue"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn quantitative_details<'a>(
        &'a mut self,
        val: ObservationDefinition_QuantitativeDetails,
    ) -> &'a mut ObservationDefinitionBuilder {
        self.value["quantitativeDetails"] = json!(val.value);
        return self;
    }

    pub fn specimen<'a>(&'a mut self, val: Vec<Reference>) -> &'a mut ObservationDefinitionBuilder {
        self.value["specimen"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn status<'a>(&'a mut self, val: &str) -> &'a mut ObservationDefinitionBuilder {
        self.value["status"] = json!(val);
        return self;
    }

    pub fn subject<'a>(
        &'a mut self,
        val: Vec<CodeableConcept>,
    ) -> &'a mut ObservationDefinitionBuilder {
        self.value["subject"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn text<'a>(&'a mut self, val: Narrative) -> &'a mut ObservationDefinitionBuilder {
        self.value["text"] = json!(val.value);
        return self;
    }

    pub fn title<'a>(&'a mut self, val: &str) -> &'a mut ObservationDefinitionBuilder {
        self.value["title"] = json!(val);
        return self;
    }

    pub fn url<'a>(&'a mut self, val: &str) -> &'a mut ObservationDefinitionBuilder {
        self.value["url"] = json!(val);
        return self;
    }

    pub fn use_context<'a>(
        &'a mut self,
        val: Vec<UsageContext>,
    ) -> &'a mut ObservationDefinitionBuilder {
        self.value["useContext"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn version<'a>(&'a mut self, val: &str) -> &'a mut ObservationDefinitionBuilder {
        self.value["version"] = json!(val);
        return self;
    }
}
