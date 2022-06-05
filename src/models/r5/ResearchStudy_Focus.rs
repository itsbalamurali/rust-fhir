#![allow(unused_imports, non_camel_case_types)]

use crate::models::r5::CodeableConcept::CodeableConcept;
use crate::models::r5::Element::Element;
use crate::models::r5::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A process where a researcher or organization plans and then executes a series of
/// steps intended to increase the field of healthcare-related knowledge.  This
/// includes studies of safety, efficacy, comparative effectiveness and other
/// information about medications, devices, therapies and other interventional and
/// investigative techniques. A ResearchStudy involves the gathering of information
/// about human or animal subjects or stability data about drug products or drug
/// substances.

#[derive(Debug)]
pub struct ResearchStudy_Focus<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl ResearchStudy_Focus<'_> {
    pub fn new(value: &Value) -> ResearchStudy_Focus {
        ResearchStudy_Focus {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for factor
    pub fn _factor(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_factor") {
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

    /// A factor corresponds to an independent variable manipulated by the
    /// experimentalist with the intention to affect biological systems in a way that
    /// can be measured by an assay.
    pub fn factor(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("factor") {
            return Some(string);
        }
        return None;
    }

    /// Indicates whether the focus is a medication, a device, a procedure, a specific
    /// factor or some other intervention or characteristic.
    pub fn focus_type(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("focusType") {
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

    /// Identification of product under study.  This may be any combination of code
    /// and/or name.
    pub fn product_code(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("productCode") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._factor() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.factor() {}
        if let Some(_val) = self.focus_type() {
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
        if let Some(_val) = self.product_code() {
            if !_val.validate() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct ResearchStudy_FocusBuilder {
    pub(crate) value: Value,
}

impl ResearchStudy_FocusBuilder {
    pub fn build(&self) -> ResearchStudy_Focus {
        ResearchStudy_Focus {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: ResearchStudy_Focus) -> ResearchStudy_FocusBuilder {
        ResearchStudy_FocusBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> ResearchStudy_FocusBuilder {
        let mut __value: Value = json!({});
        return ResearchStudy_FocusBuilder { value: __value };
    }

    pub fn _factor<'a>(&'a mut self, val: Element) -> &'a mut ResearchStudy_FocusBuilder {
        self.value["_factor"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut ResearchStudy_FocusBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn factor<'a>(&'a mut self, val: &str) -> &'a mut ResearchStudy_FocusBuilder {
        self.value["factor"] = json!(val);
        return self;
    }

    pub fn focus_type<'a>(
        &'a mut self,
        val: Vec<CodeableConcept>,
    ) -> &'a mut ResearchStudy_FocusBuilder {
        self.value["focusType"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut ResearchStudy_FocusBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ResearchStudy_FocusBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn product_code<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut ResearchStudy_FocusBuilder {
        self.value["productCode"] = json!(val.value);
        return self;
    }
}
