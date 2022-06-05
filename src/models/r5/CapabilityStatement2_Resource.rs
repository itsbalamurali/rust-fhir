#![allow(unused_imports, non_camel_case_types)]

use crate::models::r5::CapabilityStatement2_Feature::CapabilityStatement2_Feature;
use crate::models::r5::CapabilityStatement2_Interaction::CapabilityStatement2_Interaction;
use crate::models::r5::CapabilityStatement2_Operation::CapabilityStatement2_Operation;
use crate::models::r5::CapabilityStatement2_SearchParam::CapabilityStatement2_SearchParam;
use crate::models::r5::Element::Element;
use crate::models::r5::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A Capability Statement documents a set of capabilities (behaviors) of a FHIR
/// Server for a particular version of FHIR that may be used as a statement of
/// actual server functionality or a statement of required or desired server
/// implementation.

#[derive(Debug)]
pub struct CapabilityStatement2_Resource<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl CapabilityStatement2_Resource<'_> {
    pub fn new(value: &Value) -> CapabilityStatement2_Resource {
        CapabilityStatement2_Resource {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for documentation
    pub fn _documentation(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_documentation") {
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

    /// Additional information about the resource type used by the system.
    pub fn documentation(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("documentation") {
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

    /// A statement that affirms support for a feature, in this context.
    pub fn feature(&self) -> Option<Vec<CapabilityStatement2_Feature>> {
        if let Some(Value::Array(val)) = self.value.get("feature") {
            return Some(
                val.into_iter()
                    .map(|e| CapabilityStatement2_Feature {
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

    /// Identifies a restful operation supported by the solution.
    pub fn interaction(&self) -> Option<Vec<CapabilityStatement2_Interaction>> {
        if let Some(Value::Array(val)) = self.value.get("interaction") {
            return Some(
                val.into_iter()
                    .map(|e| CapabilityStatement2_Interaction {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
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

    /// Definition of an operation or a named query together with its parameters and
    /// their meaning and type. Consult the definition of the operation for details
    /// about how to invoke the operation, and the parameters.
    pub fn operation(&self) -> Option<Vec<CapabilityStatement2_Operation>> {
        if let Some(Value::Array(val)) = self.value.get("operation") {
            return Some(
                val.into_iter()
                    .map(|e| CapabilityStatement2_Operation {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A specification of the profile that describes the solution's overall support for
    /// the resource, including any constraints on cardinality, bindings, lengths or
    /// other limitations. See further discussion in [Using
    /// Profiles](profiling.html#profile-uses).
    pub fn profile(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("profile") {
            return Some(string);
        }
        return None;
    }

    /// Search parameters for implementations to support and/or make use of - either
    /// references to ones defined in the specification, or additional ones defined
    /// for/by the implementation.
    pub fn search_param(&self) -> Option<Vec<CapabilityStatement2_SearchParam>> {
        if let Some(Value::Array(val)) = self.value.get("searchParam") {
            return Some(
                val.into_iter()
                    .map(|e| CapabilityStatement2_SearchParam {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A list of profiles that represent different use cases supported by the system.
    /// For a server, "supported by the system" means the system hosts/produces a set of
    /// resources that are conformant to a particular profile, and allows clients that
    /// use its services to search using this profile and to find appropriate data. For
    /// a client, it means the system will search by this profile and process data
    /// according to the guidance implicit in the profile. See further discussion in
    /// [Using Profiles](profiling.html#profile-uses).
    pub fn supported_profile(&self) -> Option<Vec<&str>> {
        if let Some(Value::Array(val)) = self.value.get("supportedProfile") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_str().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A type of resource exposed via the restful interface.
    pub fn fhir_type(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("type") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._documentation() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._type() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.documentation() {}
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.feature() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.interaction() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.operation() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.profile() {}
        if let Some(_val) = self.search_param() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.supported_profile() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.fhir_type() {}
        return true;
    }
}

#[derive(Debug)]
pub struct CapabilityStatement2_ResourceBuilder {
    pub(crate) value: Value,
}

impl CapabilityStatement2_ResourceBuilder {
    pub fn build(&self) -> CapabilityStatement2_Resource {
        CapabilityStatement2_Resource {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: CapabilityStatement2_Resource) -> CapabilityStatement2_ResourceBuilder {
        CapabilityStatement2_ResourceBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> CapabilityStatement2_ResourceBuilder {
        let mut __value: Value = json!({});
        return CapabilityStatement2_ResourceBuilder { value: __value };
    }

    pub fn _documentation<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut CapabilityStatement2_ResourceBuilder {
        self.value["_documentation"] = json!(val.value);
        return self;
    }

    pub fn _type<'a>(&'a mut self, val: Element) -> &'a mut CapabilityStatement2_ResourceBuilder {
        self.value["_type"] = json!(val.value);
        return self;
    }

    pub fn documentation<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut CapabilityStatement2_ResourceBuilder {
        self.value["documentation"] = json!(val);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut CapabilityStatement2_ResourceBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn feature<'a>(
        &'a mut self,
        val: Vec<CapabilityStatement2_Feature>,
    ) -> &'a mut CapabilityStatement2_ResourceBuilder {
        self.value["feature"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut CapabilityStatement2_ResourceBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn interaction<'a>(
        &'a mut self,
        val: Vec<CapabilityStatement2_Interaction>,
    ) -> &'a mut CapabilityStatement2_ResourceBuilder {
        self.value["interaction"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut CapabilityStatement2_ResourceBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn operation<'a>(
        &'a mut self,
        val: Vec<CapabilityStatement2_Operation>,
    ) -> &'a mut CapabilityStatement2_ResourceBuilder {
        self.value["operation"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn profile<'a>(&'a mut self, val: &str) -> &'a mut CapabilityStatement2_ResourceBuilder {
        self.value["profile"] = json!(val);
        return self;
    }

    pub fn search_param<'a>(
        &'a mut self,
        val: Vec<CapabilityStatement2_SearchParam>,
    ) -> &'a mut CapabilityStatement2_ResourceBuilder {
        self.value["searchParam"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn supported_profile<'a>(
        &'a mut self,
        val: Vec<&str>,
    ) -> &'a mut CapabilityStatement2_ResourceBuilder {
        self.value["supportedProfile"] = json!(val);
        return self;
    }

    pub fn fhir_type<'a>(&'a mut self, val: &str) -> &'a mut CapabilityStatement2_ResourceBuilder {
        self.value["type"] = json!(val);
        return self;
    }
}
