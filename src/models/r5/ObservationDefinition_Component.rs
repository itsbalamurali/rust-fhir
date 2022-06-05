#![allow(unused_imports, non_camel_case_types)]

use crate::models::r5::CodeableConcept::CodeableConcept;
use crate::models::r5::Element::Element;
use crate::models::r5::Extension::Extension;
use crate::models::r5::ObservationDefinition_QualifiedValue::ObservationDefinition_QualifiedValue;
use crate::models::r5::ObservationDefinition_QuantitativeDetails::ObservationDefinition_QuantitativeDetails;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Set of definitional characteristics for a kind of observation or measurement
/// produced or consumed by an orderable health care service.

#[derive(Debug)]
pub struct ObservationDefinition_Component<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl ObservationDefinition_Component<'_> {
    pub fn new(value: &Value) -> ObservationDefinition_Component {
        ObservationDefinition_Component {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for permittedDataType
    pub fn _permitted_data_type(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_permittedDataType") {
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

    /// Describes what will be observed.
    pub fn code(&self) -> CodeableConcept {
        CodeableConcept {
            value: Cow::Borrowed(&self.value["code"]),
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

    /// The data types allowed for the value element of the instance of this component
    /// observations.
    pub fn permitted_data_type(&self) -> Option<Vec<&str>> {
        if let Some(Value::Array(val)) = self.value.get("permittedDataType") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_str().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A set of qualified values associated with a context and a set of conditions -
    /// provides a range for quantitative and ordinal observations and a collection of
    /// value sets for qualitative observations.
    pub fn qualified_value(&self) -> Option<Vec<ObservationDefinition_QualifiedValue>> {
        if let Some(Value::Array(val)) = self.value.get("qualifiedValue") {
            return Some(
                val.into_iter()
                    .map(|e| ObservationDefinition_QualifiedValue {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Characteristics for quantitative results of this observation.
    pub fn quantitative_details(&self) -> Option<ObservationDefinition_QuantitativeDetails> {
        if let Some(val) = self.value.get("quantitativeDetails") {
            return Some(ObservationDefinition_QuantitativeDetails {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._permitted_data_type() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if !self.code().validate() {
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
        if let Some(_val) = self.permitted_data_type() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.qualified_value() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.quantitative_details() {
            if !_val.validate() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct ObservationDefinition_ComponentBuilder {
    pub(crate) value: Value,
}

impl ObservationDefinition_ComponentBuilder {
    pub fn build(&self) -> ObservationDefinition_Component {
        ObservationDefinition_Component {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(
        existing: ObservationDefinition_Component,
    ) -> ObservationDefinition_ComponentBuilder {
        ObservationDefinition_ComponentBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(code: CodeableConcept) -> ObservationDefinition_ComponentBuilder {
        let mut __value: Value = json!({});
        __value["code"] = json!(code.value);
        return ObservationDefinition_ComponentBuilder { value: __value };
    }

    pub fn _permitted_data_type<'a>(
        &'a mut self,
        val: Vec<Element>,
    ) -> &'a mut ObservationDefinition_ComponentBuilder {
        self.value["_permittedDataType"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ObservationDefinition_ComponentBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut ObservationDefinition_ComponentBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ObservationDefinition_ComponentBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn permitted_data_type<'a>(
        &'a mut self,
        val: Vec<&str>,
    ) -> &'a mut ObservationDefinition_ComponentBuilder {
        self.value["permittedDataType"] = json!(val);
        return self;
    }

    pub fn qualified_value<'a>(
        &'a mut self,
        val: Vec<ObservationDefinition_QualifiedValue>,
    ) -> &'a mut ObservationDefinition_ComponentBuilder {
        self.value["qualifiedValue"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn quantitative_details<'a>(
        &'a mut self,
        val: ObservationDefinition_QuantitativeDetails,
    ) -> &'a mut ObservationDefinition_ComponentBuilder {
        self.value["quantitativeDetails"] = json!(val.value);
        return self;
    }
}
