#![allow(unused_imports, non_camel_case_types)]

use crate::models::r5::Element::Element;
use crate::models::r5::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// The subscription resource describes a particular client's request to be notified
/// about a SubscriptionTopic.

#[derive(Debug)]
pub struct Subscription_FilterBy<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Subscription_FilterBy<'_> {
    pub fn new(value: &Value) -> Subscription_FilterBy {
        Subscription_FilterBy {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for resourceType
    pub fn _resource_type(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_resourceType") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for searchModifier
    pub fn _search_modifier(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_searchModifier") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for searchParamName
    pub fn _search_param_name(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_searchParamName") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for value
    pub fn _value(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_value") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
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

    /// If the element is a reference to another resource, this element contains
    /// "Reference", and the targetProfile element defines what resources can be
    /// referenced. The targetProfile may be a reference to the general definition of a
    /// resource (e.g. http://hl7.org/fhir/StructureDefinition/Patient).
    pub fn resource_type(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("resourceType") {
            return Some(string);
        }
        return None;
    }

    /// The operator to apply to the filter value when determining matches (Search
    /// modifiers).
    pub fn search_modifier(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("searchModifier") {
            return Some(string);
        }
        return None;
    }

    /// The filter label (=key) as defined in the
    /// `SubscriptionTopic.canfilterBy.searchParamName`  element.
    pub fn search_param_name(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("searchParamName") {
            return Some(string);
        }
        return None;
    }

    /// The literal value or resource path as is legal in search - for example,
    /// "Patient/123" or "le1950".
    pub fn value(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("value") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._resource_type() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._search_modifier() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._search_param_name() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._value() {
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
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.resource_type() {}
        if let Some(_val) = self.search_modifier() {}
        if let Some(_val) = self.search_param_name() {}
        if let Some(_val) = self.value() {}
        return true;
    }
}

#[derive(Debug)]
pub struct Subscription_FilterByBuilder {
    pub(crate) value: Value,
}

impl Subscription_FilterByBuilder {
    pub fn build(&self) -> Subscription_FilterBy {
        Subscription_FilterBy {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Subscription_FilterBy) -> Subscription_FilterByBuilder {
        Subscription_FilterByBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> Subscription_FilterByBuilder {
        let mut __value: Value = json!({});
        return Subscription_FilterByBuilder { value: __value };
    }

    pub fn _resource_type<'a>(&'a mut self, val: Element) -> &'a mut Subscription_FilterByBuilder {
        self.value["_resourceType"] = json!(val.value);
        return self;
    }

    pub fn _search_modifier<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut Subscription_FilterByBuilder {
        self.value["_searchModifier"] = json!(val.value);
        return self;
    }

    pub fn _search_param_name<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut Subscription_FilterByBuilder {
        self.value["_searchParamName"] = json!(val.value);
        return self;
    }

    pub fn _value<'a>(&'a mut self, val: Element) -> &'a mut Subscription_FilterByBuilder {
        self.value["_value"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut Subscription_FilterByBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut Subscription_FilterByBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut Subscription_FilterByBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn resource_type<'a>(&'a mut self, val: &str) -> &'a mut Subscription_FilterByBuilder {
        self.value["resourceType"] = json!(val);
        return self;
    }

    pub fn search_modifier<'a>(&'a mut self, val: &str) -> &'a mut Subscription_FilterByBuilder {
        self.value["searchModifier"] = json!(val);
        return self;
    }

    pub fn search_param_name<'a>(&'a mut self, val: &str) -> &'a mut Subscription_FilterByBuilder {
        self.value["searchParamName"] = json!(val);
        return self;
    }

    pub fn value<'a>(&'a mut self, val: &str) -> &'a mut Subscription_FilterByBuilder {
        self.value["value"] = json!(val);
        return self;
    }
}
