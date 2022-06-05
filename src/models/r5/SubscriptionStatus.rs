#![allow(unused_imports, non_camel_case_types)]

use crate::models::r5::CodeableConcept::CodeableConcept;
use crate::models::r5::Element::Element;
use crate::models::r5::Extension::Extension;
use crate::models::r5::Meta::Meta;
use crate::models::r5::Narrative::Narrative;
use crate::models::r5::Reference::Reference;
use crate::models::r5::ResourceList::ResourceList;
use crate::models::r5::SubscriptionStatus_NotificationEvent::SubscriptionStatus_NotificationEvent;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// The SubscriptionStatus resource describes the state of a Subscription during
/// notifications.

#[derive(Debug)]
pub struct SubscriptionStatus<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl SubscriptionStatus<'_> {
    pub fn new(value: &Value) -> SubscriptionStatus {
        SubscriptionStatus {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for eventsInNotification
    pub fn _events_in_notification(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_eventsInNotification") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for eventsSinceSubscriptionStart
    pub fn _events_since_subscription_start(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_eventsSinceSubscriptionStart") {
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

    /// Extensions for status
    pub fn _status(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_status") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for type
    pub fn _type(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_type") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
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

    /// A record of errors that occurred when the server processed a notification.
    pub fn error(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("error") {
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

    /// The total number of actual events represented within this notification.  For
    /// handshake and heartbeat notifications, this will be zero or not present.  For
    /// event-notifications, this number may be one or more, depending on server
    /// batching.
    pub fn events_in_notification(&self) -> Option<i64> {
        if let Some(val) = self.value.get("eventsInNotification") {
            return Some(val.as_i64().unwrap());
        }
        return None;
    }

    /// The total number of actual events which have been generated since the
    /// Subscription was created (inclusive of this notification) - regardless of how
    /// many have been successfully communicated.  This number is NOT incremented for
    /// handshake and heartbeat notifications.
    pub fn events_since_subscription_start(&self) -> Option<i64> {
        if let Some(val) = self.value.get("eventsSinceSubscriptionStart") {
            return Some(val.as_i64().unwrap());
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

    /// The base language in which the resource is written.
    pub fn language(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("language") {
            return Some(string);
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

    /// Detailed information about events relevant to this subscription notification.
    pub fn notification_event(&self) -> Option<Vec<SubscriptionStatus_NotificationEvent>> {
        if let Some(Value::Array(val)) = self.value.get("notificationEvent") {
            return Some(
                val.into_iter()
                    .map(|e| SubscriptionStatus_NotificationEvent {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The status of the subscription, which marks the server state for managing the
    /// subscription.
    pub fn status(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("status") {
            return Some(string);
        }
        return None;
    }

    /// The reference to the Subscription which generated this notification.
    pub fn subscription(&self) -> Reference {
        Reference {
            value: Cow::Borrowed(&self.value["subscription"]),
        }
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

    /// The reference to the SubscriptionTopic for the Subscription which generated this
    /// notification.
    pub fn topic(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("topic") {
            return Some(string);
        }
        return None;
    }

    /// The type of event being conveyed with this notificaiton.
    pub fn fhir_type(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("type") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._events_in_notification() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._events_since_subscription_start() {
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
        if let Some(_val) = self._status() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._type() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.contained() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.error() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.events_in_notification() {}
        if let Some(_val) = self.events_since_subscription_start() {}
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.implicit_rules() {}
        if let Some(_val) = self.language() {}
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
        if let Some(_val) = self.notification_event() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.status() {}
        if !self.subscription().validate() {
            return false;
        }
        if let Some(_val) = self.text() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.topic() {}
        if let Some(_val) = self.fhir_type() {}
        return true;
    }
}

#[derive(Debug)]
pub struct SubscriptionStatusBuilder {
    pub(crate) value: Value,
}

impl SubscriptionStatusBuilder {
    pub fn build(&self) -> SubscriptionStatus {
        SubscriptionStatus {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: SubscriptionStatus) -> SubscriptionStatusBuilder {
        SubscriptionStatusBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(subscription: Reference) -> SubscriptionStatusBuilder {
        let mut __value: Value = json!({});
        __value["subscription"] = json!(subscription.value);
        return SubscriptionStatusBuilder { value: __value };
    }

    pub fn _events_in_notification<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut SubscriptionStatusBuilder {
        self.value["_eventsInNotification"] = json!(val.value);
        return self;
    }

    pub fn _events_since_subscription_start<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut SubscriptionStatusBuilder {
        self.value["_eventsSinceSubscriptionStart"] = json!(val.value);
        return self;
    }

    pub fn _implicit_rules<'a>(&'a mut self, val: Element) -> &'a mut SubscriptionStatusBuilder {
        self.value["_implicitRules"] = json!(val.value);
        return self;
    }

    pub fn _language<'a>(&'a mut self, val: Element) -> &'a mut SubscriptionStatusBuilder {
        self.value["_language"] = json!(val.value);
        return self;
    }

    pub fn _status<'a>(&'a mut self, val: Element) -> &'a mut SubscriptionStatusBuilder {
        self.value["_status"] = json!(val.value);
        return self;
    }

    pub fn _type<'a>(&'a mut self, val: Element) -> &'a mut SubscriptionStatusBuilder {
        self.value["_type"] = json!(val.value);
        return self;
    }

    pub fn contained<'a>(
        &'a mut self,
        val: Vec<ResourceList>,
    ) -> &'a mut SubscriptionStatusBuilder {
        self.value["contained"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn error<'a>(&'a mut self, val: Vec<CodeableConcept>) -> &'a mut SubscriptionStatusBuilder {
        self.value["error"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn events_in_notification<'a>(&'a mut self, val: i64) -> &'a mut SubscriptionStatusBuilder {
        self.value["eventsInNotification"] = json!(val);
        return self;
    }

    pub fn events_since_subscription_start<'a>(
        &'a mut self,
        val: i64,
    ) -> &'a mut SubscriptionStatusBuilder {
        self.value["eventsSinceSubscriptionStart"] = json!(val);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut SubscriptionStatusBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut SubscriptionStatusBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn implicit_rules<'a>(&'a mut self, val: &str) -> &'a mut SubscriptionStatusBuilder {
        self.value["implicitRules"] = json!(val);
        return self;
    }

    pub fn language<'a>(&'a mut self, val: &str) -> &'a mut SubscriptionStatusBuilder {
        self.value["language"] = json!(val);
        return self;
    }

    pub fn meta<'a>(&'a mut self, val: Meta) -> &'a mut SubscriptionStatusBuilder {
        self.value["meta"] = json!(val.value);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut SubscriptionStatusBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn notification_event<'a>(
        &'a mut self,
        val: Vec<SubscriptionStatus_NotificationEvent>,
    ) -> &'a mut SubscriptionStatusBuilder {
        self.value["notificationEvent"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn status<'a>(&'a mut self, val: &str) -> &'a mut SubscriptionStatusBuilder {
        self.value["status"] = json!(val);
        return self;
    }

    pub fn text<'a>(&'a mut self, val: Narrative) -> &'a mut SubscriptionStatusBuilder {
        self.value["text"] = json!(val.value);
        return self;
    }

    pub fn topic<'a>(&'a mut self, val: &str) -> &'a mut SubscriptionStatusBuilder {
        self.value["topic"] = json!(val);
        return self;
    }

    pub fn fhir_type<'a>(&'a mut self, val: &str) -> &'a mut SubscriptionStatusBuilder {
        self.value["type"] = json!(val);
        return self;
    }
}
