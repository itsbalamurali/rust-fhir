#![allow(unused_imports, non_camel_case_types)]

use crate::models::r5::Element::Element;
use crate::models::r5::Extension::Extension;
use crate::models::r5::Reference::Reference;
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
pub struct ResearchStudy_Recruitment<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl ResearchStudy_Recruitment<'_> {
    pub fn new(value: &Value) -> ResearchStudy_Recruitment {
        ResearchStudy_Recruitment {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for actualNumber
    pub fn _actual_number(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_actualNumber") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for targetNumber
    pub fn _target_number(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_targetNumber") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Group of participants who were enrolled in study.
    pub fn actual_group(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("actualGroup") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Actual total number of participants enrolled in study.
    pub fn actual_number(&self) -> Option<u64> {
        if let Some(val) = self.value.get("actualNumber") {
            return Some(val.as_u64().unwrap());
        }
        return None;
    }

    /// Inclusion and exclusion criteria.
    pub fn eligibility(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("eligibility") {
            return Some(Reference {
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

    /// Estimated total number of participants to be enrolled.
    pub fn target_number(&self) -> Option<u64> {
        if let Some(val) = self.value.get("targetNumber") {
            return Some(val.as_u64().unwrap());
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._actual_number() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._target_number() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.actual_group() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.actual_number() {}
        if let Some(_val) = self.eligibility() {
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
        if let Some(_val) = self.target_number() {}
        return true;
    }
}

#[derive(Debug)]
pub struct ResearchStudy_RecruitmentBuilder {
    pub(crate) value: Value,
}

impl ResearchStudy_RecruitmentBuilder {
    pub fn build(&self) -> ResearchStudy_Recruitment {
        ResearchStudy_Recruitment {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: ResearchStudy_Recruitment) -> ResearchStudy_RecruitmentBuilder {
        ResearchStudy_RecruitmentBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> ResearchStudy_RecruitmentBuilder {
        let mut __value: Value = json!({});
        return ResearchStudy_RecruitmentBuilder { value: __value };
    }

    pub fn _actual_number<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut ResearchStudy_RecruitmentBuilder {
        self.value["_actualNumber"] = json!(val.value);
        return self;
    }

    pub fn _target_number<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut ResearchStudy_RecruitmentBuilder {
        self.value["_targetNumber"] = json!(val.value);
        return self;
    }

    pub fn actual_group<'a>(
        &'a mut self,
        val: Reference,
    ) -> &'a mut ResearchStudy_RecruitmentBuilder {
        self.value["actualGroup"] = json!(val.value);
        return self;
    }

    pub fn actual_number<'a>(&'a mut self, val: u64) -> &'a mut ResearchStudy_RecruitmentBuilder {
        self.value["actualNumber"] = json!(val);
        return self;
    }

    pub fn eligibility<'a>(
        &'a mut self,
        val: Reference,
    ) -> &'a mut ResearchStudy_RecruitmentBuilder {
        self.value["eligibility"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ResearchStudy_RecruitmentBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut ResearchStudy_RecruitmentBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ResearchStudy_RecruitmentBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn target_number<'a>(&'a mut self, val: u64) -> &'a mut ResearchStudy_RecruitmentBuilder {
        self.value["targetNumber"] = json!(val);
        return self;
    }
}
