#![allow(unused_imports, non_camel_case_types)]

use crate::models::r4b::Element::Element;
use crate::models::r4b::Extension::Extension;
use crate::models::r4b::SubscriptionTopic_QueryCriteria::SubscriptionTopic_QueryCriteria;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Describes a stream of resource state changes or events and annotated with labels
/// useful to filter projections from this topic.

#[derive(Debug)]
pub struct SubscriptionTopic_ResourceTrigger<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl SubscriptionTopic_ResourceTrigger<'_> {
    pub fn new(value: &Value) -> SubscriptionTopic_ResourceTrigger {
        SubscriptionTopic_ResourceTrigger {
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

    /// Extensions for fhirPathCriteria
    pub fn _fhir_path_criteria(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_fhirPathCriteria") {
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

    /// Extensions for supportedInteraction
    pub fn _supported_interaction(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_supportedInteraction") {
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

    /// The human readable description of this resource trigger for the
    /// SubscriptionTopic -  for example, "An Encounter enters the 'in-progress' state".
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

    /// The FHIRPath based rules that the server should use to determine when to trigger
    /// a notification for this topic.
    pub fn fhir_path_criteria(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("fhirPathCriteria") {
            return Some(string);
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

    /// The FHIR query based rules that the server should use to determine when to
    /// trigger a notification for this subscription topic.
    pub fn query_criteria(&self) -> Option<SubscriptionTopic_QueryCriteria> {
        if let Some(val) = self.value.get("queryCriteria") {
            return Some(SubscriptionTopic_QueryCriteria {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// URL of the Resource that is the type used in this resource trigger.  Relative
    /// URLs are relative to the StructureDefinition root of the implemented FHIR
    /// version (e.g., http://hl7.org/fhir/StructureDefinition). For example, "Patient"
    /// maps to http://hl7.org/fhir/StructureDefinition/Patient.  For more information,
    /// see <a href="elementdefinition-
    /// definitions.html#ElementDefinition.type.code">ElementDef
    /// inition.type.code</a>.
    pub fn resource(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("resource") {
            return Some(string);
        }
        return None;
    }

    /// The FHIR RESTful interaction which can be used to trigger a notification for the
    /// SubscriptionTopic. Multiple values are considered OR joined (e.g., CREATE or
    /// UPDATE).
    pub fn supported_interaction(&self) -> Option<Vec<&str>> {
        if let Some(Value::Array(val)) = self.value.get("supportedInteraction") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_str().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._description() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._fhir_path_criteria() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._resource() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._supported_interaction() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.description() {}
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.fhir_path_criteria() {}
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.query_criteria() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.resource() {}
        if let Some(_val) = self.supported_interaction() {
            _val.into_iter().for_each(|_e| {});
        }
        return true;
    }
}

#[derive(Debug)]
pub struct SubscriptionTopic_ResourceTriggerBuilder {
    pub(crate) value: Value,
}

impl SubscriptionTopic_ResourceTriggerBuilder {
    pub fn build(&self) -> SubscriptionTopic_ResourceTrigger {
        SubscriptionTopic_ResourceTrigger {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(
        existing: SubscriptionTopic_ResourceTrigger,
    ) -> SubscriptionTopic_ResourceTriggerBuilder {
        SubscriptionTopic_ResourceTriggerBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> SubscriptionTopic_ResourceTriggerBuilder {
        let mut __value: Value = json!({});
        return SubscriptionTopic_ResourceTriggerBuilder { value: __value };
    }

    pub fn _description<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut SubscriptionTopic_ResourceTriggerBuilder {
        self.value["_description"] = json!(val.value);
        return self;
    }

    pub fn _fhir_path_criteria<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut SubscriptionTopic_ResourceTriggerBuilder {
        self.value["_fhirPathCriteria"] = json!(val.value);
        return self;
    }

    pub fn _resource<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut SubscriptionTopic_ResourceTriggerBuilder {
        self.value["_resource"] = json!(val.value);
        return self;
    }

    pub fn _supported_interaction<'a>(
        &'a mut self,
        val: Vec<Element>,
    ) -> &'a mut SubscriptionTopic_ResourceTriggerBuilder {
        self.value["_supportedInteraction"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn description<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut SubscriptionTopic_ResourceTriggerBuilder {
        self.value["description"] = json!(val);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut SubscriptionTopic_ResourceTriggerBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn fhir_path_criteria<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut SubscriptionTopic_ResourceTriggerBuilder {
        self.value["fhirPathCriteria"] = json!(val);
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut SubscriptionTopic_ResourceTriggerBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut SubscriptionTopic_ResourceTriggerBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn query_criteria<'a>(
        &'a mut self,
        val: SubscriptionTopic_QueryCriteria,
    ) -> &'a mut SubscriptionTopic_ResourceTriggerBuilder {
        self.value["queryCriteria"] = json!(val.value);
        return self;
    }

    pub fn resource<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut SubscriptionTopic_ResourceTriggerBuilder {
        self.value["resource"] = json!(val);
        return self;
    }

    pub fn supported_interaction<'a>(
        &'a mut self,
        val: Vec<&str>,
    ) -> &'a mut SubscriptionTopic_ResourceTriggerBuilder {
        self.value["supportedInteraction"] = json!(val);
        return self;
    }
}
