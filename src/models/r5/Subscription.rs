#![allow(unused_imports, non_camel_case_types)]

use crate::models::r5::Coding::Coding;
use crate::models::r5::ContactPoint::ContactPoint;
use crate::models::r5::Element::Element;
use crate::models::r5::Extension::Extension;
use crate::models::r5::Identifier::Identifier;
use crate::models::r5::Meta::Meta;
use crate::models::r5::Narrative::Narrative;
use crate::models::r5::ResourceList::ResourceList;
use crate::models::r5::Subscription_FilterBy::Subscription_FilterBy;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// The subscription resource describes a particular client's request to be notified
/// about a SubscriptionTopic.

#[derive(Debug)]
pub struct Subscription<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Subscription<'_> {
    pub fn new(value: &Value) -> Subscription {
        Subscription {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for content
    pub fn _content(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_content") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for contentType
    pub fn _content_type(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_contentType") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for end
    pub fn _end(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_end") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for endpoint
    pub fn _endpoint(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_endpoint") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for header
    pub fn _header(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_header") {
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

    /// Extensions for heartbeatPeriod
    pub fn _heartbeat_period(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_heartbeatPeriod") {
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

    /// Extensions for maxCount
    pub fn _max_count(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_maxCount") {
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

    /// Extensions for notificationUrlLocation
    pub fn _notification_url_location(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_notificationUrlLocation") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for reason
    pub fn _reason(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_reason") {
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

    /// Extensions for timeout
    pub fn _timeout(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_timeout") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The type of channel to send notifications on.
    pub fn channel_type(&self) -> Coding {
        Coding {
            value: Cow::Borrowed(&self.value["channelType"]),
        }
    }

    /// Contact details for a human to contact about the subscription. The primary use of
    /// this for system administrator troubleshooting.
    pub fn contact(&self) -> Option<Vec<ContactPoint>> {
        if let Some(Value::Array(val)) = self.value.get("contact") {
            return Some(
                val.into_iter()
                    .map(|e| ContactPoint {
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

    /// How much of the resource content to deliver in the notification payload. The
    /// choices are an empty payload, only the resource id, or the full resource content.
    pub fn content(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("content") {
            return Some(string);
        }
        return None;
    }

    /// The mime type to send the payload in - either application/fhir+xml, or
    /// application/fhir+json. The MIME types "text/plain" and "text/html" may also be
    /// used for Email subscriptions.
    pub fn content_type(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("contentType") {
            return Some(string);
        }
        return None;
    }

    /// The time for the server to turn the subscription off.
    pub fn end(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("end") {
            return Some(string);
        }
        return None;
    }

    /// The url that describes the actual end-point to send messages to.
    pub fn endpoint(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("endpoint") {
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

    /// The filter properties to be applied to narrow the subscription topic stream.  When
    /// multiple filters are applied, evaluates to true if all the conditions are met;
    /// otherwise it returns false.   (i.e., logical AND).
    pub fn filter_by(&self) -> Option<Vec<Subscription_FilterBy>> {
        if let Some(Value::Array(val)) = self.value.get("filterBy") {
            return Some(
                val.into_iter()
                    .map(|e| Subscription_FilterBy {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Additional headers / information to send as part of the notification.
    pub fn header(&self) -> Option<Vec<&str>> {
        if let Some(Value::Array(val)) = self.value.get("header") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_str().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// If present,  a 'hearbeat" notification (keepalive) is sent via this channel with
    /// an the interval period equal to this elements integer value in seconds.    If not
    /// present, a heartbeat notification is not sent.
    pub fn heartbeat_period(&self) -> Option<u64> {
        if let Some(val) = self.value.get("heartbeatPeriod") {
            return Some(val.as_u64().unwrap());
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

    /// A formal identifier that is used to identify this code system when it is
    /// represented in other formats, or referenced in a specification, model, design or
    /// an instance.
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

    /// If present, the maximum number of triggering resources that will be included in
    /// a notification bundle (e.g., a server will not include more than this number of
    /// trigger resources in a single notification).  Note that this is not a strict limit
    /// on the number of entries in a bundle, as dependent resources can be included.
    pub fn max_count(&self) -> Option<i64> {
        if let Some(val) = self.value.get("maxCount") {
            return Some(val.as_i64().unwrap());
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

    /// A natural language name identifying the subscription.
    pub fn name(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("name") {
            return Some(string);
        }
        return None;
    }

    /// If present, where to place URLs of resources in notifications.
    pub fn notification_url_location(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("notificationUrlLocation") {
            return Some(string);
        }
        return None;
    }

    /// A description of why this subscription is defined.
    pub fn reason(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("reason") {
            return Some(string);
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

    /// If present, the maximum amount of time a server will allow before failing a
    /// notification attempt.
    pub fn timeout(&self) -> Option<u64> {
        if let Some(val) = self.value.get("timeout") {
            return Some(val.as_u64().unwrap());
        }
        return None;
    }

    /// The reference to the subscription topic to be notified about.
    pub fn topic(&self) -> &str {
        self.value.get("topic").unwrap().as_str().unwrap()
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._content() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._content_type() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._end() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._endpoint() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._header() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self._heartbeat_period() {
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
        if let Some(_val) = self._max_count() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._name() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._notification_url_location() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._reason() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._status() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._timeout() {
            if !_val.validate() {
                return false;
            }
        }
        if !self.channel_type().validate() {
            return false;
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
        if let Some(_val) = self.content() {}
        if let Some(_val) = self.content_type() {}
        if let Some(_val) = self.end() {}
        if let Some(_val) = self.endpoint() {}
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.filter_by() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.header() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.heartbeat_period() {}
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.identifier() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.implicit_rules() {}
        if let Some(_val) = self.language() {}
        if let Some(_val) = self.max_count() {}
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
        if let Some(_val) = self.notification_url_location() {}
        if let Some(_val) = self.reason() {}
        if let Some(_val) = self.status() {}
        if let Some(_val) = self.text() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.timeout() {}
        return true;
    }
}

#[derive(Debug)]
pub struct SubscriptionBuilder {
    pub(crate) value: Value,
}

impl SubscriptionBuilder {
    pub fn build(&self) -> Subscription {
        Subscription {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Subscription) -> SubscriptionBuilder {
        SubscriptionBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(channel_type: Coding, topic: &str) -> SubscriptionBuilder {
        let mut __value: Value = json!({});
        __value["channelType"] = json!(channel_type.value);
        __value["topic"] = json!(topic);
        return SubscriptionBuilder { value: __value };
    }

    pub fn _content<'a>(&'a mut self, val: Element) -> &'a mut SubscriptionBuilder {
        self.value["_content"] = json!(val.value);
        return self;
    }

    pub fn _content_type<'a>(&'a mut self, val: Element) -> &'a mut SubscriptionBuilder {
        self.value["_contentType"] = json!(val.value);
        return self;
    }

    pub fn _end<'a>(&'a mut self, val: Element) -> &'a mut SubscriptionBuilder {
        self.value["_end"] = json!(val.value);
        return self;
    }

    pub fn _endpoint<'a>(&'a mut self, val: Element) -> &'a mut SubscriptionBuilder {
        self.value["_endpoint"] = json!(val.value);
        return self;
    }

    pub fn _header<'a>(&'a mut self, val: Vec<Element>) -> &'a mut SubscriptionBuilder {
        self.value["_header"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn _heartbeat_period<'a>(&'a mut self, val: Element) -> &'a mut SubscriptionBuilder {
        self.value["_heartbeatPeriod"] = json!(val.value);
        return self;
    }

    pub fn _implicit_rules<'a>(&'a mut self, val: Element) -> &'a mut SubscriptionBuilder {
        self.value["_implicitRules"] = json!(val.value);
        return self;
    }

    pub fn _language<'a>(&'a mut self, val: Element) -> &'a mut SubscriptionBuilder {
        self.value["_language"] = json!(val.value);
        return self;
    }

    pub fn _max_count<'a>(&'a mut self, val: Element) -> &'a mut SubscriptionBuilder {
        self.value["_maxCount"] = json!(val.value);
        return self;
    }

    pub fn _name<'a>(&'a mut self, val: Element) -> &'a mut SubscriptionBuilder {
        self.value["_name"] = json!(val.value);
        return self;
    }

    pub fn _notification_url_location<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut SubscriptionBuilder {
        self.value["_notificationUrlLocation"] = json!(val.value);
        return self;
    }

    pub fn _reason<'a>(&'a mut self, val: Element) -> &'a mut SubscriptionBuilder {
        self.value["_reason"] = json!(val.value);
        return self;
    }

    pub fn _status<'a>(&'a mut self, val: Element) -> &'a mut SubscriptionBuilder {
        self.value["_status"] = json!(val.value);
        return self;
    }

    pub fn _timeout<'a>(&'a mut self, val: Element) -> &'a mut SubscriptionBuilder {
        self.value["_timeout"] = json!(val.value);
        return self;
    }

    pub fn contact<'a>(&'a mut self, val: Vec<ContactPoint>) -> &'a mut SubscriptionBuilder {
        self.value["contact"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn contained<'a>(&'a mut self, val: Vec<ResourceList>) -> &'a mut SubscriptionBuilder {
        self.value["contained"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn content<'a>(&'a mut self, val: &str) -> &'a mut SubscriptionBuilder {
        self.value["content"] = json!(val);
        return self;
    }

    pub fn content_type<'a>(&'a mut self, val: &str) -> &'a mut SubscriptionBuilder {
        self.value["contentType"] = json!(val);
        return self;
    }

    pub fn end<'a>(&'a mut self, val: &str) -> &'a mut SubscriptionBuilder {
        self.value["end"] = json!(val);
        return self;
    }

    pub fn endpoint<'a>(&'a mut self, val: &str) -> &'a mut SubscriptionBuilder {
        self.value["endpoint"] = json!(val);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut SubscriptionBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn filter_by<'a>(
        &'a mut self,
        val: Vec<Subscription_FilterBy>,
    ) -> &'a mut SubscriptionBuilder {
        self.value["filterBy"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn header<'a>(&'a mut self, val: Vec<&str>) -> &'a mut SubscriptionBuilder {
        self.value["header"] = json!(val);
        return self;
    }

    pub fn heartbeat_period<'a>(&'a mut self, val: u64) -> &'a mut SubscriptionBuilder {
        self.value["heartbeatPeriod"] = json!(val);
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut SubscriptionBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn identifier<'a>(&'a mut self, val: Vec<Identifier>) -> &'a mut SubscriptionBuilder {
        self.value["identifier"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn implicit_rules<'a>(&'a mut self, val: &str) -> &'a mut SubscriptionBuilder {
        self.value["implicitRules"] = json!(val);
        return self;
    }

    pub fn language<'a>(&'a mut self, val: &str) -> &'a mut SubscriptionBuilder {
        self.value["language"] = json!(val);
        return self;
    }

    pub fn max_count<'a>(&'a mut self, val: i64) -> &'a mut SubscriptionBuilder {
        self.value["maxCount"] = json!(val);
        return self;
    }

    pub fn meta<'a>(&'a mut self, val: Meta) -> &'a mut SubscriptionBuilder {
        self.value["meta"] = json!(val.value);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut SubscriptionBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn name<'a>(&'a mut self, val: &str) -> &'a mut SubscriptionBuilder {
        self.value["name"] = json!(val);
        return self;
    }

    pub fn notification_url_location<'a>(&'a mut self, val: &str) -> &'a mut SubscriptionBuilder {
        self.value["notificationUrlLocation"] = json!(val);
        return self;
    }

    pub fn reason<'a>(&'a mut self, val: &str) -> &'a mut SubscriptionBuilder {
        self.value["reason"] = json!(val);
        return self;
    }

    pub fn status<'a>(&'a mut self, val: &str) -> &'a mut SubscriptionBuilder {
        self.value["status"] = json!(val);
        return self;
    }

    pub fn text<'a>(&'a mut self, val: Narrative) -> &'a mut SubscriptionBuilder {
        self.value["text"] = json!(val.value);
        return self;
    }

    pub fn timeout<'a>(&'a mut self, val: u64) -> &'a mut SubscriptionBuilder {
        self.value["timeout"] = json!(val);
        return self;
    }
}
