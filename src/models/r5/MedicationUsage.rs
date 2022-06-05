#![allow(unused_imports, non_camel_case_types)]

use crate::models::r5::Annotation::Annotation;
use crate::models::r5::CodeableConcept::CodeableConcept;
use crate::models::r5::CodeableReference::CodeableReference;
use crate::models::r5::Dosage::Dosage;
use crate::models::r5::Element::Element;
use crate::models::r5::Extension::Extension;
use crate::models::r5::Identifier::Identifier;
use crate::models::r5::MedicationUsage_Adherence::MedicationUsage_Adherence;
use crate::models::r5::Meta::Meta;
use crate::models::r5::Narrative::Narrative;
use crate::models::r5::Period::Period;
use crate::models::r5::Reference::Reference;
use crate::models::r5::ResourceList::ResourceList;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A record of a medication that is being consumed by a patient.   A MedicationUsage
/// may indicate that the patient may be taking the medication now or has taken the
/// medication in the past or will be taking the medication in the future.  The source
/// of this information can be the patient, significant other (such as a family member
/// or spouse), or a clinician.  A common scenario where this information is captured
/// is during the history taking process during a patient visit or stay.   The
/// medication information may come from sources such as the patient's memory, from
/// a prescription bottle,  or from a list of medications the patient, clinician or
/// other party maintains.
///
/// The primary difference between a medicationusage and a medicationadministration
/// is that the medication administration has complete administration information and
/// is based on actual administration information from the person who administered the
/// medication.  A medicationusage is often, if not always, less specific.  There is
/// no required date/time when the medication was administered, in fact we only know
/// that a source has reported the patient is taking this medication, where details
/// such as time, quantity, or rate or even medication product may be incomplete or
/// missing or less precise.  As stated earlier, the Medication Usage information
/// may come from the patient's memory, from a prescription bottle or from a list
/// of medications the patient, clinician or other party maintains.  Medication
/// administration is more formal and is not missing detailed information.
///
/// The MedicationUsage resource was previously called MedicationStatement.

#[derive(Debug)]
pub struct MedicationUsage<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl MedicationUsage<'_> {
    pub fn new(value: &Value) -> MedicationUsage {
        MedicationUsage {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for dateAsserted
    pub fn _date_asserted(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_dateAsserted") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for effectiveDateTime
    pub fn _effective_date_time(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_effectiveDateTime") {
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

    /// Extensions for renderedDosageInstruction
    pub fn _rendered_dosage_instruction(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_renderedDosageInstruction") {
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

    /// Indicates if the medication is being consumed or administered as instructed.
    pub fn adherence(&self) -> Option<MedicationUsage_Adherence> {
        if let Some(val) = self.value.get("adherence") {
            return Some(MedicationUsage_Adherence {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Type of medication usage (for example, drug classification like ATC, where meds
    /// would be administered, legal category of the medication.).
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

    /// The date when the Medication Usage was asserted by the information source.
    pub fn date_asserted(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("dateAsserted") {
            return Some(string);
        }
        return None;
    }

    /// Allows linking the MedicationUsage to the underlying MedicationRequest, or to
    /// other information that supports or is used to derive the MedicationUsage.
    pub fn derived_from(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("derivedFrom") {
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

    /// Indicates how the medication is/was or should be taken by the patient.
    pub fn dosage(&self) -> Option<Vec<Dosage>> {
        if let Some(Value::Array(val)) = self.value.get("dosage") {
            return Some(
                val.into_iter()
                    .map(|e| Dosage {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The interval of time during which it is being asserted that the patient
    /// is/was/will be taking the medication (or was not taking, when the
    /// MedicationUsage.adherence element is Not Taking).
    pub fn effective_date_time(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("effectiveDateTime") {
            return Some(string);
        }
        return None;
    }

    /// The interval of time during which it is being asserted that the patient
    /// is/was/will be taking the medication (or was not taking, when the
    /// MedicationUsage.adherence element is Not Taking).
    pub fn effective_period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("effectivePeriod") {
            return Some(Period {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The encounter that establishes the context for this MedicationUsage.
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

    /// Identifiers associated with this Medication Usage that are defined by business
    /// processes and/or used to refer to it when a direct URL reference to the resource
    /// itself is not appropriate. They are business identifiers assigned to this resource
    /// by the performer or other systems and remain constant as the resource is updated
    /// and propagates from server to server.
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

    /// The person or organization that provided the information about the taking of this
    /// medication. Note: Use derivedFrom when a MedicationUsage is derived from other
    /// resources, e.g. Claim or MedicationRequest.
    pub fn information_source(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("informationSource") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
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

    /// Identifies the medication being administered. This is either a link to a resource
    /// representing the details of the medication or a simple attribute carrying a code
    /// that identifies the medication from a known list of medications.
    pub fn medication(&self) -> CodeableReference {
        CodeableReference {
            value: Cow::Borrowed(&self.value["medication"]),
        }
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

    /// Provides extra information about the Medication Usage that is not conveyed by the
    /// other attributes.
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

    /// A concept, Condition or observation that supports why the medication is being/
    /// was taken.
    pub fn reason(&self) -> Option<Vec<CodeableReference>> {
        if let Some(Value::Array(val)) = self.value.get("reason") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableReference {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The full representation of the dose of the medication included in all dosage
    /// instructions.  To be used when multiple dosage instructions are included to
    /// represent complex dosing such as increasing or tapering doses.
    pub fn rendered_dosage_instruction(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("renderedDosageInstruction") {
            return Some(string);
        }
        return None;
    }

    /// A code representing the status of recording the medication usage.
    pub fn status(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("status") {
            return Some(string);
        }
        return None;
    }

    /// The person, animal or group who is/was taking the medication.
    pub fn subject(&self) -> Reference {
        Reference {
            value: Cow::Borrowed(&self.value["subject"]),
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
        if let Some(_val) = self._date_asserted() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._effective_date_time() {
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
        if let Some(_val) = self._rendered_dosage_instruction() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._status() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.adherence() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.category() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.contained() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.date_asserted() {}
        if let Some(_val) = self.derived_from() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.dosage() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.effective_date_time() {}
        if let Some(_val) = self.effective_period() {
            if !_val.validate() {
                return false;
            }
        }
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
        if let Some(_val) = self.information_source() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.language() {}
        if !self.medication().validate() {
            return false;
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
        if let Some(_val) = self.note() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.reason() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.rendered_dosage_instruction() {}
        if let Some(_val) = self.status() {}
        if !self.subject().validate() {
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
pub struct MedicationUsageBuilder {
    pub(crate) value: Value,
}

impl MedicationUsageBuilder {
    pub fn build(&self) -> MedicationUsage {
        MedicationUsage {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: MedicationUsage) -> MedicationUsageBuilder {
        MedicationUsageBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(medication: CodeableReference, subject: Reference) -> MedicationUsageBuilder {
        let mut __value: Value = json!({});
        __value["medication"] = json!(medication.value);
        __value["subject"] = json!(subject.value);
        return MedicationUsageBuilder { value: __value };
    }

    pub fn _date_asserted<'a>(&'a mut self, val: Element) -> &'a mut MedicationUsageBuilder {
        self.value["_dateAsserted"] = json!(val.value);
        return self;
    }

    pub fn _effective_date_time<'a>(&'a mut self, val: Element) -> &'a mut MedicationUsageBuilder {
        self.value["_effectiveDateTime"] = json!(val.value);
        return self;
    }

    pub fn _implicit_rules<'a>(&'a mut self, val: Element) -> &'a mut MedicationUsageBuilder {
        self.value["_implicitRules"] = json!(val.value);
        return self;
    }

    pub fn _language<'a>(&'a mut self, val: Element) -> &'a mut MedicationUsageBuilder {
        self.value["_language"] = json!(val.value);
        return self;
    }

    pub fn _rendered_dosage_instruction<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut MedicationUsageBuilder {
        self.value["_renderedDosageInstruction"] = json!(val.value);
        return self;
    }

    pub fn _status<'a>(&'a mut self, val: Element) -> &'a mut MedicationUsageBuilder {
        self.value["_status"] = json!(val.value);
        return self;
    }

    pub fn adherence<'a>(
        &'a mut self,
        val: MedicationUsage_Adherence,
    ) -> &'a mut MedicationUsageBuilder {
        self.value["adherence"] = json!(val.value);
        return self;
    }

    pub fn category<'a>(&'a mut self, val: Vec<CodeableConcept>) -> &'a mut MedicationUsageBuilder {
        self.value["category"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn contained<'a>(&'a mut self, val: Vec<ResourceList>) -> &'a mut MedicationUsageBuilder {
        self.value["contained"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn date_asserted<'a>(&'a mut self, val: &str) -> &'a mut MedicationUsageBuilder {
        self.value["dateAsserted"] = json!(val);
        return self;
    }

    pub fn derived_from<'a>(&'a mut self, val: Vec<Reference>) -> &'a mut MedicationUsageBuilder {
        self.value["derivedFrom"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn dosage<'a>(&'a mut self, val: Vec<Dosage>) -> &'a mut MedicationUsageBuilder {
        self.value["dosage"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn effective_date_time<'a>(&'a mut self, val: &str) -> &'a mut MedicationUsageBuilder {
        self.value["effectiveDateTime"] = json!(val);
        return self;
    }

    pub fn effective_period<'a>(&'a mut self, val: Period) -> &'a mut MedicationUsageBuilder {
        self.value["effectivePeriod"] = json!(val.value);
        return self;
    }

    pub fn encounter<'a>(&'a mut self, val: Reference) -> &'a mut MedicationUsageBuilder {
        self.value["encounter"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut MedicationUsageBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut MedicationUsageBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn identifier<'a>(&'a mut self, val: Vec<Identifier>) -> &'a mut MedicationUsageBuilder {
        self.value["identifier"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn implicit_rules<'a>(&'a mut self, val: &str) -> &'a mut MedicationUsageBuilder {
        self.value["implicitRules"] = json!(val);
        return self;
    }

    pub fn information_source<'a>(&'a mut self, val: Reference) -> &'a mut MedicationUsageBuilder {
        self.value["informationSource"] = json!(val.value);
        return self;
    }

    pub fn language<'a>(&'a mut self, val: &str) -> &'a mut MedicationUsageBuilder {
        self.value["language"] = json!(val);
        return self;
    }

    pub fn meta<'a>(&'a mut self, val: Meta) -> &'a mut MedicationUsageBuilder {
        self.value["meta"] = json!(val.value);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut MedicationUsageBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn note<'a>(&'a mut self, val: Vec<Annotation>) -> &'a mut MedicationUsageBuilder {
        self.value["note"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn reason<'a>(&'a mut self, val: Vec<CodeableReference>) -> &'a mut MedicationUsageBuilder {
        self.value["reason"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn rendered_dosage_instruction<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut MedicationUsageBuilder {
        self.value["renderedDosageInstruction"] = json!(val);
        return self;
    }

    pub fn status<'a>(&'a mut self, val: &str) -> &'a mut MedicationUsageBuilder {
        self.value["status"] = json!(val);
        return self;
    }

    pub fn text<'a>(&'a mut self, val: Narrative) -> &'a mut MedicationUsageBuilder {
        self.value["text"] = json!(val.value);
        return self;
    }
}
