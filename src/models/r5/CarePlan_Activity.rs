#![allow(unused_imports, non_camel_case_types)]

use crate::models::r5::Annotation::Annotation;
use crate::models::r5::CarePlan_PlannedActivityDetail::CarePlan_PlannedActivityDetail;
use crate::models::r5::CodeableReference::CodeableReference;
use crate::models::r5::Extension::Extension;
use crate::models::r5::Reference::Reference;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Describes the intention of how one or more practitioners intend to deliver care
/// for a particular patient, group or community for a period of time, possibly
/// limited to care for a specific condition or set of conditions.

#[derive(Debug)]
pub struct CarePlan_Activity<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl CarePlan_Activity<'_> {
    pub fn new(value: &Value) -> CarePlan_Activity {
        CarePlan_Activity {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
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

    /// Identifies the activity that was performed. For example, an activity could be
    /// patient education, exercise, or a medication administration. The reference to an
    /// "event" resource, such as Procedure or Encounter or Observation, represents the
    /// activity that was performed. The requested activity can be conveyed using
    /// CarePlan.activity.plannedActivityDetail OR using the
    /// CarePlan.activity.plannedActivityReference (a reference to a “request”
    /// resource).
    pub fn performed_activity(&self) -> Option<Vec<CodeableReference>> {
        if let Some(Value::Array(val)) = self.value.get("performedActivity") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableReference {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A simple summary of a planned activity suitable for a general care plan system
    /// (e.g. form driven) that doesn't know about specific resources such as procedure
    /// etc.
    pub fn planned_activity_detail(&self) -> Option<CarePlan_PlannedActivityDetail> {
        if let Some(val) = self.value.get("plannedActivityDetail") {
            return Some(CarePlan_PlannedActivityDetail {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The details of the proposed activity represented in a specific resource.
    pub fn planned_activity_reference(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("plannedActivityReference") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Notes about the adherence/status/progress of the activity.
    pub fn progress(&self) -> Option<Vec<Annotation>> {
        if let Some(Value::Array(val)) = self.value.get("progress") {
            return Some(
                val.into_iter()
                    .map(|e| Annotation {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    pub fn validate(&self) -> bool {
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
        if let Some(_val) = self.performed_activity() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.planned_activity_detail() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.planned_activity_reference() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.progress() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct CarePlan_ActivityBuilder {
    pub(crate) value: Value,
}

impl CarePlan_ActivityBuilder {
    pub fn build(&self) -> CarePlan_Activity {
        CarePlan_Activity {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: CarePlan_Activity) -> CarePlan_ActivityBuilder {
        CarePlan_ActivityBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> CarePlan_ActivityBuilder {
        let mut __value: Value = json!({});
        return CarePlan_ActivityBuilder { value: __value };
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut CarePlan_ActivityBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut CarePlan_ActivityBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut CarePlan_ActivityBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn performed_activity<'a>(
        &'a mut self,
        val: Vec<CodeableReference>,
    ) -> &'a mut CarePlan_ActivityBuilder {
        self.value["performedActivity"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn planned_activity_detail<'a>(
        &'a mut self,
        val: CarePlan_PlannedActivityDetail,
    ) -> &'a mut CarePlan_ActivityBuilder {
        self.value["plannedActivityDetail"] = json!(val.value);
        return self;
    }

    pub fn planned_activity_reference<'a>(
        &'a mut self,
        val: Reference,
    ) -> &'a mut CarePlan_ActivityBuilder {
        self.value["plannedActivityReference"] = json!(val.value);
        return self;
    }

    pub fn progress<'a>(&'a mut self, val: Vec<Annotation>) -> &'a mut CarePlan_ActivityBuilder {
        self.value["progress"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }
}
