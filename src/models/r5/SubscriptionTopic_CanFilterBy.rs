#![allow(unused_imports, non_camel_case_types)]

use crate::models::r5::Element::Element;
use crate::models::r5::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Describes a stream of resource state changes or events and annotated with labels
/// useful to filter projections from this topic.

#[derive(Debug)]
pub struct SubscriptionTopic_CanFilterBy<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl SubscriptionTopic_CanFilterBy<'_> {
    pub fn new(value: &Value) -> SubscriptionTopic_CanFilterBy {
        SubscriptionTopic_CanFilterBy {
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

    /// Extensions for filterParameter
    pub fn _filter_parameter(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_filterParameter") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for modifier
    pub fn _modifier(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_modifier") {
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

    /// Extensions for resource
    pub fn _resource(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_resource") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Description of how this filtering parameter is intended to be used.
    pub fn description(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("description") {
            return Some(string);
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

    /// Either the canonical URL to a search parameter (like "http://hl7.org/fhir/
    /// SearchParameter/encounter-patient") or topic-defined parameter (like "hub.event")
    /// which is a label for the filter.
    pub fn filter_parameter(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("filterParameter") {
            return Some(string);
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

    /// Allowable operators to apply when determining matches (Search Modifiers).
    pub fn modifier(&self) -> Option<Vec<&str>> {
        if let Some(Value::Array(val)) = self.value.get("modifier") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_str().unwrap())
                    .collect::<Vec<_>>(),
            );
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

    /// URL of the Resource that is the type used in this filter. This is the "focus"
    /// of the topic (or one of them if there are more than one). It will be the same,
    /// a generality, or a specificity of SubscriptionTopic.resourceTrigger.resource or
    /// SubscriptionTopic.eventTrigger.resource when they are present.
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
        if let Some(_val) = self._filter_parameter() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._modifier() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self._resource() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.description() {}
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.filter_parameter() {}
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.modifier() {
            _val.into_iter().for_each(|_e| {});
        }
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
pub struct SubscriptionTopic_CanFilterByBuilder {
    pub(crate) value: Value,
}

impl SubscriptionTopic_CanFilterByBuilder {
    pub fn build(&self) -> SubscriptionTopic_CanFilterBy {
        SubscriptionTopic_CanFilterBy {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: SubscriptionTopic_CanFilterBy) -> SubscriptionTopic_CanFilterByBuilder {
        SubscriptionTopic_CanFilterByBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> SubscriptionTopic_CanFilterByBuilder {
        let mut __value: Value = json!({});
        return SubscriptionTopic_CanFilterByBuilder { value: __value };
    }

    pub fn _description<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut SubscriptionTopic_CanFilterByBuilder {
        self.value["_description"] = json!(val.value);
        return self;
    }

    pub fn _filter_parameter<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut SubscriptionTopic_CanFilterByBuilder {
        self.value["_filterParameter"] = json!(val.value);
        return self;
    }

    pub fn _modifier<'a>(
        &'a mut self,
        val: Vec<Element>,
    ) -> &'a mut SubscriptionTopic_CanFilterByBuilder {
        self.value["_modifier"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn _resource<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut SubscriptionTopic_CanFilterByBuilder {
        self.value["_resource"] = json!(val.value);
        return self;
    }

    pub fn description<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut SubscriptionTopic_CanFilterByBuilder {
        self.value["description"] = json!(val);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut SubscriptionTopic_CanFilterByBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn filter_parameter<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut SubscriptionTopic_CanFilterByBuilder {
        self.value["filterParameter"] = json!(val);
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut SubscriptionTopic_CanFilterByBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier<'a>(
        &'a mut self,
        val: Vec<&str>,
    ) -> &'a mut SubscriptionTopic_CanFilterByBuilder {
        self.value["modifier"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut SubscriptionTopic_CanFilterByBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn resource<'a>(&'a mut self, val: &str) -> &'a mut SubscriptionTopic_CanFilterByBuilder {
        self.value["resource"] = json!(val);
        return self;
    }
}
