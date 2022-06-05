#![allow(unused_imports, non_camel_case_types)]

use crate::models::r5::Element::Element;
use crate::models::r5::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Describes a stream of resource state changes or events and annotated with labels
/// useful to filter projections from this topic.

#[derive(Debug)]
pub struct SubscriptionTopic_QueryCriteria<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl SubscriptionTopic_QueryCriteria<'_> {
    pub fn new(value: &Value) -> SubscriptionTopic_QueryCriteria {
        SubscriptionTopic_QueryCriteria {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for current
    pub fn _current(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_current") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for previous
    pub fn _previous(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_previous") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for requireBoth
    pub fn _require_both(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_requireBoth") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for resultForCreate
    pub fn _result_for_create(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_resultForCreate") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for resultForDelete
    pub fn _result_for_delete(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_resultForDelete") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The FHIR query based rules are applied to the current resource state (e.g.,
    /// state after an update).
    pub fn current(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("current") {
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

    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
        }
        return None;
    }

    /// May be used to represent additional information that is not part of the basic
    /// definition of the element and that modifies the understanding of the element in
    /// which it is contained and/or the understanding of the containing element's
    /// descendants. Usually modifier elements provide negation or qualification. To
    /// make the use of extensions safe and manageable, there is a strict set of
    /// governance applied to the definition and use of extensions. Though any
    /// implementer can define an extension, there is a set of requirements that SHALL
    /// be met as part of the definition of the extension. Applications processing a
    /// resource are required to check for modifier extensions.    Modifier extensions
    /// SHALL NOT change the meaning of any elements on Resource or DomainResource
    /// (including cannot change the meaning of modifierExtension itself).
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

    /// The FHIR query based rules are applied to the previous resource state (e.g.,
    /// state before an update).
    pub fn previous(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("previous") {
            return Some(string);
        }
        return None;
    }

    /// If set to true, both current and previous criteria must evaluate true to
    /// trigger a notification for this topic.  Otherwise a notification for this topic
    /// will be triggered if either one evaluates to true.
    pub fn require_both(&self) -> Option<bool> {
        if let Some(val) = self.value.get("requireBoth") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    /// What behavior a server will exhibit if the previous state of a resource does NOT
    /// exist (e.g., prior to a create).
    pub fn result_for_create(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("resultForCreate") {
            return Some(string);
        }
        return None;
    }

    /// What behavior a server will exhibit if the current state of a resource does NOT
    /// exist (e.g., after a DELETE).
    pub fn result_for_delete(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("resultForDelete") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._current() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._previous() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._require_both() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._result_for_create() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._result_for_delete() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.current() {}
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
        if let Some(_val) = self.previous() {}
        if let Some(_val) = self.require_both() {}
        if let Some(_val) = self.result_for_create() {}
        if let Some(_val) = self.result_for_delete() {}
        return true;
    }
}

#[derive(Debug)]
pub struct SubscriptionTopic_QueryCriteriaBuilder {
    pub(crate) value: Value,
}

impl SubscriptionTopic_QueryCriteriaBuilder {
    pub fn build(&self) -> SubscriptionTopic_QueryCriteria {
        SubscriptionTopic_QueryCriteria {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(
        existing: SubscriptionTopic_QueryCriteria,
    ) -> SubscriptionTopic_QueryCriteriaBuilder {
        SubscriptionTopic_QueryCriteriaBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> SubscriptionTopic_QueryCriteriaBuilder {
        let mut __value: Value = json!({});
        return SubscriptionTopic_QueryCriteriaBuilder { value: __value };
    }

    pub fn _current<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut SubscriptionTopic_QueryCriteriaBuilder {
        self.value["_current"] = json!(val.value);
        return self;
    }

    pub fn _previous<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut SubscriptionTopic_QueryCriteriaBuilder {
        self.value["_previous"] = json!(val.value);
        return self;
    }

    pub fn _require_both<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut SubscriptionTopic_QueryCriteriaBuilder {
        self.value["_requireBoth"] = json!(val.value);
        return self;
    }

    pub fn _result_for_create<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut SubscriptionTopic_QueryCriteriaBuilder {
        self.value["_resultForCreate"] = json!(val.value);
        return self;
    }

    pub fn _result_for_delete<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut SubscriptionTopic_QueryCriteriaBuilder {
        self.value["_resultForDelete"] = json!(val.value);
        return self;
    }

    pub fn current<'a>(&'a mut self, val: &str) -> &'a mut SubscriptionTopic_QueryCriteriaBuilder {
        self.value["current"] = json!(val);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut SubscriptionTopic_QueryCriteriaBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut SubscriptionTopic_QueryCriteriaBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut SubscriptionTopic_QueryCriteriaBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn previous<'a>(&'a mut self, val: &str) -> &'a mut SubscriptionTopic_QueryCriteriaBuilder {
        self.value["previous"] = json!(val);
        return self;
    }

    pub fn require_both<'a>(
        &'a mut self,
        val: bool,
    ) -> &'a mut SubscriptionTopic_QueryCriteriaBuilder {
        self.value["requireBoth"] = json!(val);
        return self;
    }

    pub fn result_for_create<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut SubscriptionTopic_QueryCriteriaBuilder {
        self.value["resultForCreate"] = json!(val);
        return self;
    }

    pub fn result_for_delete<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut SubscriptionTopic_QueryCriteriaBuilder {
        self.value["resultForDelete"] = json!(val);
        return self;
    }
}
