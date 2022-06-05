#![allow(unused_imports, non_camel_case_types)]

use crate::models::r5::CodeableConcept::CodeableConcept;
use crate::models::r5::Element::Element;
use crate::models::r5::Extension::Extension;
use crate::models::r5::Reference::Reference;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A record of an event relevant for purposes such as operations, privacy,
/// security, maintenance, and performance analysis.

#[derive(Debug)]
pub struct AuditEvent_Agent<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl AuditEvent_Agent<'_> {
    pub fn new(value: &Value) -> AuditEvent_Agent {
        AuditEvent_Agent {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for networkString
    pub fn _network_string(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_networkString") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for networkUri
    pub fn _network_uri(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_networkUri") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for policy
    pub fn _policy(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_policy") {
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

    /// Extensions for requestor
    pub fn _requestor(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_requestor") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The authorization (e.g., PurposeOfUse) that was used during the event being
    /// recorded.
    pub fn authorization(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("authorization") {
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

    /// Where the agent location is known, the agent location when the event occurred.
    pub fn location(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("location") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
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

    /// When the event utilizes a network there should be an agent describing the local
    /// system, and an agent describing remote system, with the network interface
    /// details.
    pub fn network_reference(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("networkReference") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// When the event utilizes a network there should be an agent describing the local
    /// system, and an agent describing remote system, with the network interface
    /// details.
    pub fn network_string(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("networkString") {
            return Some(string);
        }
        return None;
    }

    /// When the event utilizes a network there should be an agent describing the local
    /// system, and an agent describing remote system, with the network interface
    /// details.
    pub fn network_uri(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("networkUri") {
            return Some(string);
        }
        return None;
    }

    /// Where the policy(ies) are known that authorized the agent participation in the
    /// event. Typically, a single activity may have multiple applicable policies, such
    /// as patient consent, guarantor funding, etc. The policy would also indicate the
    /// security token used.
    pub fn policy(&self) -> Option<Vec<&str>> {
        if let Some(Value::Array(val)) = self.value.get("policy") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_str().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Indicator that the user is or is not the requestor, or initiator, for the event
    /// being audited.
    pub fn requestor(&self) -> Option<bool> {
        if let Some(val) = self.value.get("requestor") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    /// The structural roles of the agent indicating the agent's competency. The
    /// security role enabling the agent with respect to the activity.
    pub fn role(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("role") {
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

    /// The Functional Role of the user when performing the event.
    pub fn fhir_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("type") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Reference to who this agent is that was involved in the event.
    pub fn who(&self) -> Reference {
        Reference {
            value: Cow::Borrowed(&self.value["who"]),
        }
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._network_string() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._network_uri() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._policy() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self._requestor() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.authorization() {
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
        if let Some(_val) = self.location() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.network_reference() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.network_string() {}
        if let Some(_val) = self.network_uri() {}
        if let Some(_val) = self.policy() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.requestor() {}
        if let Some(_val) = self.role() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.fhir_type() {
            if !_val.validate() {
                return false;
            }
        }
        if !self.who().validate() {
            return false;
        }
        return true;
    }
}

#[derive(Debug)]
pub struct AuditEvent_AgentBuilder {
    pub(crate) value: Value,
}

impl AuditEvent_AgentBuilder {
    pub fn build(&self) -> AuditEvent_Agent {
        AuditEvent_Agent {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: AuditEvent_Agent) -> AuditEvent_AgentBuilder {
        AuditEvent_AgentBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(who: Reference) -> AuditEvent_AgentBuilder {
        let mut __value: Value = json!({});
        __value["who"] = json!(who.value);
        return AuditEvent_AgentBuilder { value: __value };
    }

    pub fn _network_string<'a>(&'a mut self, val: Element) -> &'a mut AuditEvent_AgentBuilder {
        self.value["_networkString"] = json!(val.value);
        return self;
    }

    pub fn _network_uri<'a>(&'a mut self, val: Element) -> &'a mut AuditEvent_AgentBuilder {
        self.value["_networkUri"] = json!(val.value);
        return self;
    }

    pub fn _policy<'a>(&'a mut self, val: Vec<Element>) -> &'a mut AuditEvent_AgentBuilder {
        self.value["_policy"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn _requestor<'a>(&'a mut self, val: Element) -> &'a mut AuditEvent_AgentBuilder {
        self.value["_requestor"] = json!(val.value);
        return self;
    }

    pub fn authorization<'a>(
        &'a mut self,
        val: Vec<CodeableConcept>,
    ) -> &'a mut AuditEvent_AgentBuilder {
        self.value["authorization"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut AuditEvent_AgentBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut AuditEvent_AgentBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn location<'a>(&'a mut self, val: Reference) -> &'a mut AuditEvent_AgentBuilder {
        self.value["location"] = json!(val.value);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut AuditEvent_AgentBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn network_reference<'a>(&'a mut self, val: Reference) -> &'a mut AuditEvent_AgentBuilder {
        self.value["networkReference"] = json!(val.value);
        return self;
    }

    pub fn network_string<'a>(&'a mut self, val: &str) -> &'a mut AuditEvent_AgentBuilder {
        self.value["networkString"] = json!(val);
        return self;
    }

    pub fn network_uri<'a>(&'a mut self, val: &str) -> &'a mut AuditEvent_AgentBuilder {
        self.value["networkUri"] = json!(val);
        return self;
    }

    pub fn policy<'a>(&'a mut self, val: Vec<&str>) -> &'a mut AuditEvent_AgentBuilder {
        self.value["policy"] = json!(val);
        return self;
    }

    pub fn requestor<'a>(&'a mut self, val: bool) -> &'a mut AuditEvent_AgentBuilder {
        self.value["requestor"] = json!(val);
        return self;
    }

    pub fn role<'a>(&'a mut self, val: Vec<CodeableConcept>) -> &'a mut AuditEvent_AgentBuilder {
        self.value["role"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn fhir_type<'a>(&'a mut self, val: CodeableConcept) -> &'a mut AuditEvent_AgentBuilder {
        self.value["type"] = json!(val.value);
        return self;
    }
}
