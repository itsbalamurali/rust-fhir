#![allow(unused_imports, non_camel_case_types)]

use crate::models::r5::Annotation::Annotation;
use crate::models::r5::CodeableConcept::CodeableConcept;
use crate::models::r5::CodeableReference::CodeableReference;
use crate::models::r5::DeviceDispense_Performer::DeviceDispense_Performer;
use crate::models::r5::Element::Element;
use crate::models::r5::Extension::Extension;
use crate::models::r5::Identifier::Identifier;
use crate::models::r5::Meta::Meta;
use crate::models::r5::Narrative::Narrative;
use crate::models::r5::Quantity::Quantity;
use crate::models::r5::Reference::Reference;
use crate::models::r5::ResourceList::ResourceList;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A record of dispensation of a device.

#[derive(Debug)]
pub struct DeviceDispense<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl DeviceDispense<'_> {
    pub fn new(value: &Value) -> DeviceDispense {
        DeviceDispense {
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

    /// Extensions for language
    pub fn _language(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_language") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for preparedDate
    pub fn _prepared_date(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_preparedDate") {
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

    /// Extensions for usageInstruction
    pub fn _usage_instruction(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_usageInstruction") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for whenHandedOver
    pub fn _when_handed_over(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_whenHandedOver") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The order or request that this dispense is fulfilling.
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

    /// Indicates the type of device dispense.
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

    /// Identification of the facility/location where the device was /should be shipped
    /// to, as part of the dispense process.
    pub fn destination(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("destination") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Identifies the device being dispensed. This is either a link to a resource
    /// representing the details of the device or a simple attribute carrying a code that
    /// identifies the device from a known list of devices.
    pub fn device(&self) -> CodeableReference {
        CodeableReference {
            value: Cow::Borrowed(&self.value["device"]),
        }
    }

    /// The encounter that establishes the context for this event.
    pub fn encounter(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("encounter") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A summary of the events of interest that have occurred, such as when the dispense
    /// was verified.
    pub fn event_history(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("eventHistory") {
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

    /// Business identifier for this dispensation.
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

    /// The principal physical location where the dispense was performed.
    pub fn location(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("location") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
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

    /// Extra information about the dispense that could not be conveyed in the other
    /// attributes.
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

    /// The bigger event that this dispense is a part of.
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

    /// Indicates who or what performed the event.
    pub fn performer(&self) -> Option<Vec<DeviceDispense_Performer>> {
        if let Some(Value::Array(val)) = self.value.get("performer") {
            return Some(
                val.into_iter()
                    .map(|e| DeviceDispense_Performer {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The time when the dispensed product was packaged and reviewed.
    pub fn prepared_date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("preparedDate") {
            return Some(string);
        }
        return None;
    }

    /// The number of devices that have been dispensed.
    pub fn quantity(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("quantity") {
            return Some(Quantity {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A code specifying the state of the set of dispense events.
    pub fn status(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("status") {
            return Some(string);
        }
        return None;
    }

    /// Indicates the reason why a dispense was or was not performed.
    pub fn status_reason(&self) -> Option<CodeableReference> {
        if let Some(val) = self.value.get("statusReason") {
            return Some(CodeableReference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A link to a resource representing the person to whom the device is intended.
    pub fn subject(&self) -> Reference {
        Reference {
            value: Cow::Borrowed(&self.value["subject"]),
        }
    }

    /// Additional information that supports the device being dispensed.
    pub fn supporting_information(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("supportingInformation") {
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

    /// Indicates the type of dispensing event that is performed.
    pub fn fhir_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("type") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The full representation of the instructions.
    pub fn usage_instruction(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("usageInstruction") {
            return Some(string);
        }
        return None;
    }

    /// The time the dispensed product was made available to the patient or their
    /// representative.
    pub fn when_handed_over(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("whenHandedOver") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
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
        if let Some(_val) = self._prepared_date() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._status() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._usage_instruction() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._when_handed_over() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.based_on() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
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
        if let Some(_val) = self.destination() {
            if !_val.validate() {
                return false;
            }
        }
        if !self.device().validate() {
            return false;
        }
        if let Some(_val) = self.encounter() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.event_history() {
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
        if let Some(_val) = self.prepared_date() {}
        if let Some(_val) = self.quantity() {
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
        if let Some(_val) = self.supporting_information() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.text() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.fhir_type() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.usage_instruction() {}
        if let Some(_val) = self.when_handed_over() {}
        return true;
    }
}

#[derive(Debug)]
pub struct DeviceDispenseBuilder {
    pub(crate) value: Value,
}

impl DeviceDispenseBuilder {
    pub fn build(&self) -> DeviceDispense {
        DeviceDispense {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: DeviceDispense) -> DeviceDispenseBuilder {
        DeviceDispenseBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(device: CodeableReference, subject: Reference) -> DeviceDispenseBuilder {
        let mut __value: Value = json!({});
        __value["device"] = json!(device.value);
        __value["subject"] = json!(subject.value);
        return DeviceDispenseBuilder { value: __value };
    }

    pub fn _implicit_rules<'a>(&'a mut self, val: Element) -> &'a mut DeviceDispenseBuilder {
        self.value["_implicitRules"] = json!(val.value);
        return self;
    }

    pub fn _language<'a>(&'a mut self, val: Element) -> &'a mut DeviceDispenseBuilder {
        self.value["_language"] = json!(val.value);
        return self;
    }

    pub fn _prepared_date<'a>(&'a mut self, val: Element) -> &'a mut DeviceDispenseBuilder {
        self.value["_preparedDate"] = json!(val.value);
        return self;
    }

    pub fn _status<'a>(&'a mut self, val: Element) -> &'a mut DeviceDispenseBuilder {
        self.value["_status"] = json!(val.value);
        return self;
    }

    pub fn _usage_instruction<'a>(&'a mut self, val: Element) -> &'a mut DeviceDispenseBuilder {
        self.value["_usageInstruction"] = json!(val.value);
        return self;
    }

    pub fn _when_handed_over<'a>(&'a mut self, val: Element) -> &'a mut DeviceDispenseBuilder {
        self.value["_whenHandedOver"] = json!(val.value);
        return self;
    }

    pub fn based_on<'a>(&'a mut self, val: Vec<Reference>) -> &'a mut DeviceDispenseBuilder {
        self.value["basedOn"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn category<'a>(&'a mut self, val: Vec<CodeableConcept>) -> &'a mut DeviceDispenseBuilder {
        self.value["category"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn contained<'a>(&'a mut self, val: Vec<ResourceList>) -> &'a mut DeviceDispenseBuilder {
        self.value["contained"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn destination<'a>(&'a mut self, val: Reference) -> &'a mut DeviceDispenseBuilder {
        self.value["destination"] = json!(val.value);
        return self;
    }

    pub fn encounter<'a>(&'a mut self, val: Reference) -> &'a mut DeviceDispenseBuilder {
        self.value["encounter"] = json!(val.value);
        return self;
    }

    pub fn event_history<'a>(&'a mut self, val: Vec<Reference>) -> &'a mut DeviceDispenseBuilder {
        self.value["eventHistory"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut DeviceDispenseBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut DeviceDispenseBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn identifier<'a>(&'a mut self, val: Vec<Identifier>) -> &'a mut DeviceDispenseBuilder {
        self.value["identifier"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn implicit_rules<'a>(&'a mut self, val: &str) -> &'a mut DeviceDispenseBuilder {
        self.value["implicitRules"] = json!(val);
        return self;
    }

    pub fn language<'a>(&'a mut self, val: &str) -> &'a mut DeviceDispenseBuilder {
        self.value["language"] = json!(val);
        return self;
    }

    pub fn location<'a>(&'a mut self, val: Reference) -> &'a mut DeviceDispenseBuilder {
        self.value["location"] = json!(val.value);
        return self;
    }

    pub fn meta<'a>(&'a mut self, val: Meta) -> &'a mut DeviceDispenseBuilder {
        self.value["meta"] = json!(val.value);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut DeviceDispenseBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn note<'a>(&'a mut self, val: Vec<Annotation>) -> &'a mut DeviceDispenseBuilder {
        self.value["note"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn part_of<'a>(&'a mut self, val: Vec<Reference>) -> &'a mut DeviceDispenseBuilder {
        self.value["partOf"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn performer<'a>(
        &'a mut self,
        val: Vec<DeviceDispense_Performer>,
    ) -> &'a mut DeviceDispenseBuilder {
        self.value["performer"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn prepared_date<'a>(&'a mut self, val: &str) -> &'a mut DeviceDispenseBuilder {
        self.value["preparedDate"] = json!(val);
        return self;
    }

    pub fn quantity<'a>(&'a mut self, val: Quantity) -> &'a mut DeviceDispenseBuilder {
        self.value["quantity"] = json!(val.value);
        return self;
    }

    pub fn status<'a>(&'a mut self, val: &str) -> &'a mut DeviceDispenseBuilder {
        self.value["status"] = json!(val);
        return self;
    }

    pub fn status_reason<'a>(
        &'a mut self,
        val: CodeableReference,
    ) -> &'a mut DeviceDispenseBuilder {
        self.value["statusReason"] = json!(val.value);
        return self;
    }

    pub fn supporting_information<'a>(
        &'a mut self,
        val: Vec<Reference>,
    ) -> &'a mut DeviceDispenseBuilder {
        self.value["supportingInformation"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn text<'a>(&'a mut self, val: Narrative) -> &'a mut DeviceDispenseBuilder {
        self.value["text"] = json!(val.value);
        return self;
    }

    pub fn fhir_type<'a>(&'a mut self, val: CodeableConcept) -> &'a mut DeviceDispenseBuilder {
        self.value["type"] = json!(val.value);
        return self;
    }

    pub fn usage_instruction<'a>(&'a mut self, val: &str) -> &'a mut DeviceDispenseBuilder {
        self.value["usageInstruction"] = json!(val);
        return self;
    }

    pub fn when_handed_over<'a>(&'a mut self, val: &str) -> &'a mut DeviceDispenseBuilder {
        self.value["whenHandedOver"] = json!(val);
        return self;
    }
}
