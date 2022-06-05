#![allow(unused_imports, non_camel_case_types)]

use crate::models::r5::CodeableConcept::CodeableConcept;
use crate::models::r5::Element::Element;
use crate::models::r5::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Describes a stream of resource state changes or events and annotated with labels
/// useful to filter projections from this topic.

#[derive(Debug)]
pub struct SubscriptionTopic_EventTrigger<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl SubscriptionTopic_EventTrigger<'_> {
    pub fn new(value: &Value) -> SubscriptionTopic_EventTrigger {
        SubscriptionTopic_EventTrigger {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
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

    /// Extensions for resource
    pub fn _resource(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_resource") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The human readable description of an event to trigger a notification for the
    /// SubscriptionTopic - for example, "Patient Admission, as defined in HL7v2 via
    /// message ADT^A01". Multiple values are considered OR joined (e.g., matching any
    /// single event listed).
    pub fn description(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("description") {
            return Some(string);
        }
        return None;
    }

    /// A well-defined event which can be used to trigger notifications from the
    /// SubscriptionTopic.
    pub fn event(&self) -> CodeableConcept {
        CodeableConcept {
            value: Cow::Borrowed(&self.value["event"]),
        }
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

    /// Unique id for the element within a resource (for internal references). This may be
    /// any string value that does not contain spaces.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
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

    /// URL of the Resource that is the focus type used in this event trigger.
    /// Relative URLs are relative to the StructureDefinition root of the
    /// implemented FHIR version (e.g., http://hl7.org/fhir/StructureDefinition).
    /// For example, "Patient" maps to http://hl7.org/fhir/StructureDefinition/
    /// Patient.  For more information, see <a href="elementdefinition-
    /// definitions.html#ElementDefinition.type.code">ElementDefinition.type.code</a>.
    pub fn resource(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("resource") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._description() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._resource() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.description() {}
        if !self.event().validate() {
            return false;
        }
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.resource() {}
        return true;
    }
}

#[derive(Debug)]
pub struct SubscriptionTopic_EventTriggerBuilder {
    pub(crate) value: Value,
}

impl SubscriptionTopic_EventTriggerBuilder {
    pub fn build(&self) -> SubscriptionTopic_EventTrigger {
        SubscriptionTopic_EventTrigger {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: SubscriptionTopic_EventTrigger) -> SubscriptionTopic_EventTriggerBuilder {
        SubscriptionTopic_EventTriggerBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(event: CodeableConcept) -> SubscriptionTopic_EventTriggerBuilder {
        let mut __value: Value = json!({});
        __value["event"] = json!(event.value);
        return SubscriptionTopic_EventTriggerBuilder { value: __value };
    }

    pub fn _description<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut SubscriptionTopic_EventTriggerBuilder {
        self.value["_description"] = json!(val.value);
        return self;
    }

    pub fn _resource<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut SubscriptionTopic_EventTriggerBuilder {
        self.value["_resource"] = json!(val.value);
        return self;
    }

    pub fn description<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut SubscriptionTopic_EventTriggerBuilder {
        self.value["description"] = json!(val);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut SubscriptionTopic_EventTriggerBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut SubscriptionTopic_EventTriggerBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut SubscriptionTopic_EventTriggerBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn resource<'a>(&'a mut self, val: &str) -> &'a mut SubscriptionTopic_EventTriggerBuilder {
        self.value["resource"] = json!(val);
        return self;
    }
}
