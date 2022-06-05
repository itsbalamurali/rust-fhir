#![allow(unused_imports, non_camel_case_types)]

use crate::models::r5::Element::Element;
use crate::models::r5::Extension::Extension;
use crate::models::r5::Reference::Reference;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// The SubscriptionStatus resource describes the state of a Subscription during
/// notifications.

#[derive(Debug)]
pub struct SubscriptionStatus_NotificationEvent<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl SubscriptionStatus_NotificationEvent<'_> {
    pub fn new(value: &Value) -> SubscriptionStatus_NotificationEvent {
        SubscriptionStatus_NotificationEvent {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for eventNumber
    pub fn _event_number(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_eventNumber") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for timestamp
    pub fn _timestamp(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_timestamp") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Additional context information for this event. Generally, this will contain
    /// references to additional resources included with the event (e.g., the Patient
    /// relevant to an Encounter), however it MAY refer to non-FHIR objects.
    pub fn additional_context(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("additionalContext") {
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

    /// The sequential number of this event in this subscription context.
    pub fn event_number(&self) -> Option<i64> {
        if let Some(val) = self.value.get("eventNumber") {
            return Some(val.as_i64().unwrap());
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

    /// The focus of this event. While this will usually be a reference to the focus
    /// resource of the event, it MAY contain a reference to a non-FHIR object.
    pub fn focus(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("focus") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
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

    /// The actual time this event occured on the server.
    pub fn timestamp(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("timestamp") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._event_number() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._timestamp() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.additional_context() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.event_number() {}
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.focus() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.timestamp() {}
        return true;
    }
}

#[derive(Debug)]
pub struct SubscriptionStatus_NotificationEventBuilder {
    pub(crate) value: Value,
}

impl SubscriptionStatus_NotificationEventBuilder {
    pub fn build(&self) -> SubscriptionStatus_NotificationEvent {
        SubscriptionStatus_NotificationEvent {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(
        existing: SubscriptionStatus_NotificationEvent,
    ) -> SubscriptionStatus_NotificationEventBuilder {
        SubscriptionStatus_NotificationEventBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> SubscriptionStatus_NotificationEventBuilder {
        let mut __value: Value = json!({});
        return SubscriptionStatus_NotificationEventBuilder { value: __value };
    }

    pub fn _event_number<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut SubscriptionStatus_NotificationEventBuilder {
        self.value["_eventNumber"] = json!(val.value);
        return self;
    }

    pub fn _timestamp<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut SubscriptionStatus_NotificationEventBuilder {
        self.value["_timestamp"] = json!(val.value);
        return self;
    }

    pub fn additional_context<'a>(
        &'a mut self,
        val: Vec<Reference>,
    ) -> &'a mut SubscriptionStatus_NotificationEventBuilder {
        self.value["additionalContext"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn event_number<'a>(
        &'a mut self,
        val: i64,
    ) -> &'a mut SubscriptionStatus_NotificationEventBuilder {
        self.value["eventNumber"] = json!(val);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut SubscriptionStatus_NotificationEventBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn focus<'a>(
        &'a mut self,
        val: Reference,
    ) -> &'a mut SubscriptionStatus_NotificationEventBuilder {
        self.value["focus"] = json!(val.value);
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut SubscriptionStatus_NotificationEventBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut SubscriptionStatus_NotificationEventBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn timestamp<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut SubscriptionStatus_NotificationEventBuilder {
        self.value["timestamp"] = json!(val);
        return self;
    }
}
