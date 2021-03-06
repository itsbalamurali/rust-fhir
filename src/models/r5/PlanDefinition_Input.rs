#![allow(unused_imports, non_camel_case_types)]

use crate::models::r5::DataRequirement::DataRequirement;
use crate::models::r5::Element::Element;
use crate::models::r5::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// This resource allows for the definition of various types of plans as a sharable,
/// consumable, and executable artifact. The resource is general enough to support
/// the description of a broad range of clinical and non-clinical artifacts such
/// as clinical decision support rules, order sets, protocols, and drug quality
/// specifications.

#[derive(Debug)]
pub struct PlanDefinition_Input<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl PlanDefinition_Input<'_> {
    pub fn new(value: &Value) -> PlanDefinition_Input {
        PlanDefinition_Input {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for relatedData
    pub fn _related_data(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_relatedData") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for title
    pub fn _title(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_title") {
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

    /// Points to an existing input or output element that provides data to this input.
    pub fn related_data(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("relatedData") {
            return Some(string);
        }
        return None;
    }

    /// Defines the data that is to be provided as input to the action.
    pub fn requirement(&self) -> Option<DataRequirement> {
        if let Some(val) = self.value.get("requirement") {
            return Some(DataRequirement {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A human-readable label for the data requirement used to label data flows in BPMN
    /// or similar diagrams. Also provides a human readable label when rendering the data
    /// requirement that conveys its purpose to human readers.
    pub fn title(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("title") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._related_data() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._title() {
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
        if let Some(_val) = self.related_data() {}
        if let Some(_val) = self.requirement() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.title() {}
        return true;
    }
}

#[derive(Debug)]
pub struct PlanDefinition_InputBuilder {
    pub(crate) value: Value,
}

impl PlanDefinition_InputBuilder {
    pub fn build(&self) -> PlanDefinition_Input {
        PlanDefinition_Input {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: PlanDefinition_Input) -> PlanDefinition_InputBuilder {
        PlanDefinition_InputBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> PlanDefinition_InputBuilder {
        let mut __value: Value = json!({});
        return PlanDefinition_InputBuilder { value: __value };
    }

    pub fn _related_data<'a>(&'a mut self, val: Element) -> &'a mut PlanDefinition_InputBuilder {
        self.value["_relatedData"] = json!(val.value);
        return self;
    }

    pub fn _title<'a>(&'a mut self, val: Element) -> &'a mut PlanDefinition_InputBuilder {
        self.value["_title"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut PlanDefinition_InputBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut PlanDefinition_InputBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut PlanDefinition_InputBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn related_data<'a>(&'a mut self, val: &str) -> &'a mut PlanDefinition_InputBuilder {
        self.value["relatedData"] = json!(val);
        return self;
    }

    pub fn requirement<'a>(
        &'a mut self,
        val: DataRequirement,
    ) -> &'a mut PlanDefinition_InputBuilder {
        self.value["requirement"] = json!(val.value);
        return self;
    }

    pub fn title<'a>(&'a mut self, val: &str) -> &'a mut PlanDefinition_InputBuilder {
        self.value["title"] = json!(val);
        return self;
    }
}
