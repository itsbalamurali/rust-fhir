#![allow(unused_imports, non_camel_case_types)]

use crate::models::r5::Age::Age;
use crate::models::r5::Annotation::Annotation;
use crate::models::r5::CodeableConcept::CodeableConcept;
use crate::models::r5::CodeableReference::CodeableReference;
use crate::models::r5::Element::Element;
use crate::models::r5::Extension::Extension;
use crate::models::r5::Identifier::Identifier;
use crate::models::r5::Meta::Meta;
use crate::models::r5::Narrative::Narrative;
use crate::models::r5::Period::Period;
use crate::models::r5::Procedure_FocalDevice::Procedure_FocalDevice;
use crate::models::r5::Procedure_Performer::Procedure_Performer;
use crate::models::r5::Range::Range;
use crate::models::r5::Reference::Reference;
use crate::models::r5::ResourceList::ResourceList;
use crate::models::r5::Timing::Timing;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// An action that is or was performed on or for a patient, practitioner, device,
/// organization, or location. For example, this can be a physical intervention on a
/// patient like an operation, or less invasive like long term services, counseling,
/// or hypnotherapy.  This can be a quality or safety inspection for a location,
/// organization, or device.  This can be an accreditation procedure on a
/// practitioner for licensing.

#[derive(Debug)]
pub struct Procedure<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Procedure<'_> {
    pub fn new(value: &Value) -> Procedure {
        Procedure {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
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

    /// Extensions for instantiatesUri
    pub fn _instantiates_uri(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_instantiatesUri") {
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

    /// Extensions for occurrenceString
    pub fn _occurrence_string(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_occurrenceString") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for recorded
    pub fn _recorded(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_recorded") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for reportedBoolean
    pub fn _reported_boolean(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_reportedBoolean") {
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

    /// A reference to a resource that contains details of the request for this
    /// procedure.
    pub fn based_on(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("basedOn") {
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

    /// Detailed and structured anatomical location information. Multiple locations are
    /// allowed - e.g. multiple punch biopsies of a lesion.
    pub fn body_site(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("bodySite") {
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

    /// A code that classifies the procedure for searching, sorting and display purposes
    /// (e.g. "Surgical Procedure").
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

    /// The specific procedure that is performed. Use text if the exact nature of the
    /// procedure cannot be coded (e.g. "Laparoscopic Appendectomy").
    pub fn code(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("code") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Any complications that occurred during the procedure, or in the immediate post-
    /// performance period. These are generally tracked separately from the notes,
    /// which will typically describe the procedure itself rather than any 'post
    /// procedure' issues.
    pub fn complication(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("complication") {
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

    /// Any complications that occurred during the procedure, or in the immediate post-
    /// performance period.
    pub fn complication_detail(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("complicationDetail") {
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

    /// The Encounter during which this Procedure was created or performed or to which
    /// the creation of this record is tightly associated.
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

    /// A device that is implanted, removed or otherwise manipulated (calibration,
    /// battery replacement, fitting a prosthesis, attaching a wound-vac, etc.) as a
    /// focal portion of the Procedure.
    pub fn focal_device(&self) -> Option<Vec<Procedure_FocalDevice>> {
        if let Some(Value::Array(val)) = self.value.get("focalDevice") {
            return Some(
                val.into_iter()
                    .map(|e| Procedure_FocalDevice {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// If the procedure required specific follow up - e.g. removal of sutures. The
    /// follow up may be represented as a simple note or could potentially be more
    /// complex, in which case the CarePlan resource can be used.
    pub fn follow_up(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("followUp") {
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

    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
        }
        return None;
    }

    /// Business identifiers assigned to this procedure by the performer or other
    /// systems which remain constant as the resource is updated and is propagated from
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

    /// The URL pointing to a FHIR-defined protocol, guideline, order set or other
    /// definition that is adhered to in whole or in part by this Procedure.
    pub fn instantiates_canonical(&self) -> Option<Vec<&str>> {
        if let Some(Value::Array(val)) = self.value.get("instantiatesCanonical") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_str().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The URL pointing to an externally maintained protocol, guideline, order set or
    /// other definition that is adhered to in whole or in part by this Procedure.
    pub fn instantiates_uri(&self) -> Option<Vec<&str>> {
        if let Some(Value::Array(val)) = self.value.get("instantiatesUri") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_str().unwrap())
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

    /// The location where the procedure actually happened.  E.g. a newborn at home, a
    /// tracheostomy at a restaurant.
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

    /// Any other notes and comments about the procedure.
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

    /// Estimated or actual date, date-time, period, or age when the procedure did occur
    /// or is occurring.  Allows a period to support complex procedures that span more
    /// than one date, and also allows for the length of the procedure to be captured.
    pub fn occurrence_age(&self) -> Option<Age> {
        if let Some(val) = self.value.get("occurrenceAge") {
            return Some(Age {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Estimated or actual date, date-time, period, or age when the procedure did occur
    /// or is occurring.  Allows a period to support complex procedures that span more
    /// than one date, and also allows for the length of the procedure to be captured.
    pub fn occurrence_date_time(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("occurrenceDateTime") {
            return Some(string);
        }
        return None;
    }

    /// Estimated or actual date, date-time, period, or age when the procedure did occur
    /// or is occurring.  Allows a period to support complex procedures that span more
    /// than one date, and also allows for the length of the procedure to be captured.
    pub fn occurrence_period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("occurrencePeriod") {
            return Some(Period {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Estimated or actual date, date-time, period, or age when the procedure did occur
    /// or is occurring.  Allows a period to support complex procedures that span more
    /// than one date, and also allows for the length of the procedure to be captured.
    pub fn occurrence_range(&self) -> Option<Range> {
        if let Some(val) = self.value.get("occurrenceRange") {
            return Some(Range {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Estimated or actual date, date-time, period, or age when the procedure did occur
    /// or is occurring.  Allows a period to support complex procedures that span more
    /// than one date, and also allows for the length of the procedure to be captured.
    pub fn occurrence_string(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("occurrenceString") {
            return Some(string);
        }
        return None;
    }

    /// Estimated or actual date, date-time, period, or age when the procedure did occur
    /// or is occurring.  Allows a period to support complex procedures that span more
    /// than one date, and also allows for the length of the procedure to be captured.
    pub fn occurrence_timing(&self) -> Option<Timing> {
        if let Some(val) = self.value.get("occurrenceTiming") {
            return Some(Timing {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The outcome of the procedure - did it resolve the reasons for the procedure
    /// being performed?
    pub fn outcome(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("outcome") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A larger event of which this particular procedure is a component or step.
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

    /// Limited to "real" people rather than equipment.
    pub fn performer(&self) -> Option<Vec<Procedure_Performer>> {
        if let Some(Value::Array(val)) = self.value.get("performer") {
            return Some(
                val.into_iter()
                    .map(|e| Procedure_Performer {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The coded reason or reference why the procedure was performed. This may be a
    /// coded entity of some type, or may simply be present as text, or may be a
    /// reference to one of several resources that justify the procedure.
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

    /// The date the occurrence of the procedure was first captured in the record
    /// regardless of Procedure.status (potentially after the occurrence of the event).
    pub fn recorded(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("recorded") {
            return Some(string);
        }
        return None;
    }

    /// Individual who recorded the record and takes responsibility for its content.
    pub fn recorder(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("recorder") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// This could be a histology result, pathology report, surgical report, etc.
    pub fn report(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("report") {
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

    /// Indicates if this record was captured as a secondary 'reported' record rather
    /// than as an original primary source-of-truth record.  It may also indicate the
    /// source of the report.
    pub fn reported_boolean(&self) -> Option<bool> {
        if let Some(val) = self.value.get("reportedBoolean") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    /// Indicates if this record was captured as a secondary 'reported' record rather
    /// than as an original primary source-of-truth record.  It may also indicate the
    /// source of the report.
    pub fn reported_reference(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("reportedReference") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A code specifying the state of the procedure. Generally, this will be the in-
    /// progress or completed state.
    pub fn status(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("status") {
            return Some(string);
        }
        return None;
    }

    /// Captures the reason for the current state of the procedure.
    pub fn status_reason(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("statusReason") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// On whom or what the procedure was performed. This is usually an individual
    /// human, but can also be performed on animals, groups of humans or animals,
    /// organizations or practitioners (for licensing), locations or devices (for safety
    /// inspections or regulatory authorizations).
    pub fn subject(&self) -> Reference {
        Reference {
            value: Cow::Borrowed(&self.value["subject"]),
        }
    }

    /// Other resources from the patient record that may be relevant to the procedure.
    /// The information from these resources was either used to create the instance or
    /// is provided to help with its interpretation. This extension should not be used
    /// if more specific inline elements or extensions are available.
    pub fn supporting_info(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("supportingInfo") {
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

    /// Identifies medications, devices and any other substance used as part of the
    /// procedure.
    pub fn used(&self) -> Option<Vec<CodeableReference>> {
        if let Some(Value::Array(val)) = self.value.get("used") {
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

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._implicit_rules() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._instantiates_uri() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
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
        if let Some(_val) = self._occurrence_string() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._recorded() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._reported_boolean() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._status() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.based_on() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.body_site() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
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
        if let Some(_val) = self.complication() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.complication_detail() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.contained() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
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
        if let Some(_val) = self.focal_device() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.follow_up() {
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
        if let Some(_val) = self.instantiates_canonical() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.instantiates_uri() {
            _val.into_iter().for_each(|_e| {});
        }
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
        if let Some(_val) = self.occurrence_age() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.occurrence_date_time() {}
        if let Some(_val) = self.occurrence_period() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.occurrence_range() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.occurrence_string() {}
        if let Some(_val) = self.occurrence_timing() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.outcome() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.part_of() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.performer() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.reason() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.recorded() {}
        if let Some(_val) = self.recorder() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.report() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.reported_boolean() {}
        if let Some(_val) = self.reported_reference() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.status() {}
        if let Some(_val) = self.status_reason() {
            if !_val.validate() {
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
        if let Some(_val) = self.text() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.used() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct ProcedureBuilder {
    pub(crate) value: Value,
}

impl ProcedureBuilder {
    pub fn build(&self) -> Procedure {
        Procedure {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Procedure) -> ProcedureBuilder {
        ProcedureBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(subject: Reference) -> ProcedureBuilder {
        let mut __value: Value = json!({});
        __value["subject"] = json!(subject.value);
        return ProcedureBuilder { value: __value };
    }

    pub fn _implicit_rules<'a>(&'a mut self, val: Element) -> &'a mut ProcedureBuilder {
        self.value["_implicitRules"] = json!(val.value);
        return self;
    }

    pub fn _instantiates_uri<'a>(&'a mut self, val: Vec<Element>) -> &'a mut ProcedureBuilder {
        self.value["_instantiatesUri"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn _language<'a>(&'a mut self, val: Element) -> &'a mut ProcedureBuilder {
        self.value["_language"] = json!(val.value);
        return self;
    }

    pub fn _occurrence_date_time<'a>(&'a mut self, val: Element) -> &'a mut ProcedureBuilder {
        self.value["_occurrenceDateTime"] = json!(val.value);
        return self;
    }

    pub fn _occurrence_string<'a>(&'a mut self, val: Element) -> &'a mut ProcedureBuilder {
        self.value["_occurrenceString"] = json!(val.value);
        return self;
    }

    pub fn _recorded<'a>(&'a mut self, val: Element) -> &'a mut ProcedureBuilder {
        self.value["_recorded"] = json!(val.value);
        return self;
    }

    pub fn _reported_boolean<'a>(&'a mut self, val: Element) -> &'a mut ProcedureBuilder {
        self.value["_reportedBoolean"] = json!(val.value);
        return self;
    }

    pub fn _status<'a>(&'a mut self, val: Element) -> &'a mut ProcedureBuilder {
        self.value["_status"] = json!(val.value);
        return self;
    }

    pub fn based_on<'a>(&'a mut self, val: Vec<Reference>) -> &'a mut ProcedureBuilder {
        self.value["basedOn"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn body_site<'a>(&'a mut self, val: Vec<CodeableConcept>) -> &'a mut ProcedureBuilder {
        self.value["bodySite"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn category<'a>(&'a mut self, val: Vec<CodeableConcept>) -> &'a mut ProcedureBuilder {
        self.value["category"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn code<'a>(&'a mut self, val: CodeableConcept) -> &'a mut ProcedureBuilder {
        self.value["code"] = json!(val.value);
        return self;
    }

    pub fn complication<'a>(&'a mut self, val: Vec<CodeableConcept>) -> &'a mut ProcedureBuilder {
        self.value["complication"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn complication_detail<'a>(&'a mut self, val: Vec<Reference>) -> &'a mut ProcedureBuilder {
        self.value["complicationDetail"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn contained<'a>(&'a mut self, val: Vec<ResourceList>) -> &'a mut ProcedureBuilder {
        self.value["contained"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn encounter<'a>(&'a mut self, val: Reference) -> &'a mut ProcedureBuilder {
        self.value["encounter"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut ProcedureBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn focal_device<'a>(
        &'a mut self,
        val: Vec<Procedure_FocalDevice>,
    ) -> &'a mut ProcedureBuilder {
        self.value["focalDevice"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn follow_up<'a>(&'a mut self, val: Vec<CodeableConcept>) -> &'a mut ProcedureBuilder {
        self.value["followUp"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut ProcedureBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn identifier<'a>(&'a mut self, val: Vec<Identifier>) -> &'a mut ProcedureBuilder {
        self.value["identifier"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn implicit_rules<'a>(&'a mut self, val: &str) -> &'a mut ProcedureBuilder {
        self.value["implicitRules"] = json!(val);
        return self;
    }

    pub fn instantiates_canonical<'a>(&'a mut self, val: Vec<&str>) -> &'a mut ProcedureBuilder {
        self.value["instantiatesCanonical"] = json!(val);
        return self;
    }

    pub fn instantiates_uri<'a>(&'a mut self, val: Vec<&str>) -> &'a mut ProcedureBuilder {
        self.value["instantiatesUri"] = json!(val);
        return self;
    }

    pub fn language<'a>(&'a mut self, val: &str) -> &'a mut ProcedureBuilder {
        self.value["language"] = json!(val);
        return self;
    }

    pub fn location<'a>(&'a mut self, val: Reference) -> &'a mut ProcedureBuilder {
        self.value["location"] = json!(val.value);
        return self;
    }

    pub fn meta<'a>(&'a mut self, val: Meta) -> &'a mut ProcedureBuilder {
        self.value["meta"] = json!(val.value);
        return self;
    }

    pub fn modifier_extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut ProcedureBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn note<'a>(&'a mut self, val: Vec<Annotation>) -> &'a mut ProcedureBuilder {
        self.value["note"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn occurrence_age<'a>(&'a mut self, val: Age) -> &'a mut ProcedureBuilder {
        self.value["occurrenceAge"] = json!(val.value);
        return self;
    }

    pub fn occurrence_date_time<'a>(&'a mut self, val: &str) -> &'a mut ProcedureBuilder {
        self.value["occurrenceDateTime"] = json!(val);
        return self;
    }

    pub fn occurrence_period<'a>(&'a mut self, val: Period) -> &'a mut ProcedureBuilder {
        self.value["occurrencePeriod"] = json!(val.value);
        return self;
    }

    pub fn occurrence_range<'a>(&'a mut self, val: Range) -> &'a mut ProcedureBuilder {
        self.value["occurrenceRange"] = json!(val.value);
        return self;
    }

    pub fn occurrence_string<'a>(&'a mut self, val: &str) -> &'a mut ProcedureBuilder {
        self.value["occurrenceString"] = json!(val);
        return self;
    }

    pub fn occurrence_timing<'a>(&'a mut self, val: Timing) -> &'a mut ProcedureBuilder {
        self.value["occurrenceTiming"] = json!(val.value);
        return self;
    }

    pub fn outcome<'a>(&'a mut self, val: CodeableConcept) -> &'a mut ProcedureBuilder {
        self.value["outcome"] = json!(val.value);
        return self;
    }

    pub fn part_of<'a>(&'a mut self, val: Vec<Reference>) -> &'a mut ProcedureBuilder {
        self.value["partOf"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn performer<'a>(&'a mut self, val: Vec<Procedure_Performer>) -> &'a mut ProcedureBuilder {
        self.value["performer"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn reason<'a>(&'a mut self, val: Vec<CodeableReference>) -> &'a mut ProcedureBuilder {
        self.value["reason"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn recorded<'a>(&'a mut self, val: &str) -> &'a mut ProcedureBuilder {
        self.value["recorded"] = json!(val);
        return self;
    }

    pub fn recorder<'a>(&'a mut self, val: Reference) -> &'a mut ProcedureBuilder {
        self.value["recorder"] = json!(val.value);
        return self;
    }

    pub fn report<'a>(&'a mut self, val: Vec<Reference>) -> &'a mut ProcedureBuilder {
        self.value["report"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn reported_boolean<'a>(&'a mut self, val: bool) -> &'a mut ProcedureBuilder {
        self.value["reportedBoolean"] = json!(val);
        return self;
    }

    pub fn reported_reference<'a>(&'a mut self, val: Reference) -> &'a mut ProcedureBuilder {
        self.value["reportedReference"] = json!(val.value);
        return self;
    }

    pub fn status<'a>(&'a mut self, val: &str) -> &'a mut ProcedureBuilder {
        self.value["status"] = json!(val);
        return self;
    }

    pub fn status_reason<'a>(&'a mut self, val: CodeableConcept) -> &'a mut ProcedureBuilder {
        self.value["statusReason"] = json!(val.value);
        return self;
    }

    pub fn supporting_info<'a>(&'a mut self, val: Vec<Reference>) -> &'a mut ProcedureBuilder {
        self.value["supportingInfo"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn text<'a>(&'a mut self, val: Narrative) -> &'a mut ProcedureBuilder {
        self.value["text"] = json!(val.value);
        return self;
    }

    pub fn used<'a>(&'a mut self, val: Vec<CodeableReference>) -> &'a mut ProcedureBuilder {
        self.value["used"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }
}
