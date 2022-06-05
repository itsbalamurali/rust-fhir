#![allow(unused_imports, non_camel_case_types)]

use crate::models::r5::CodeableConcept::CodeableConcept;
use crate::models::r5::Extension::Extension;
use crate::models::r5::Period::Period;
use crate::models::r5::Reference::Reference;
use crate::models::r5::Timing::Timing;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// The Care Team includes all the people and organizations who plan to participate
/// in the coordination and delivery of care.

#[derive(Debug)]
pub struct CareTeam_Participant<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl CareTeam_Participant<'_> {
    pub fn new(value: &Value) -> CareTeam_Participant {
        CareTeam_Participant {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// When the member is generally available within this care team.
    pub fn coverage_period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("coveragePeriod") {
            return Some(Period {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// When the member is generally available within this care team.
    pub fn coverage_timing(&self) -> Option<Timing> {
        if let Some(val) = self.value.get("coverageTiming") {
            return Some(Timing {
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

    /// The specific person or organization who is participating/expected to participate
    /// in the care team.
    pub fn member(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("member") {
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

    /// The organization of the practitioner.
    pub fn on_behalf_of(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("onBehalfOf") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Indicates specific responsibility of an individual within the care team, such as
    /// "Primary care physician", "Trained social worker counselor", "Caregiver", etc.
    pub fn role(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("role") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.coverage_period() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.coverage_timing() {
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
        if let Some(_val) = self.member() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.on_behalf_of() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.role() {
            if !_val.validate() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct CareTeam_ParticipantBuilder {
    pub(crate) value: Value,
}

impl CareTeam_ParticipantBuilder {
    pub fn build(&self) -> CareTeam_Participant {
        CareTeam_Participant {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: CareTeam_Participant) -> CareTeam_ParticipantBuilder {
        CareTeam_ParticipantBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> CareTeam_ParticipantBuilder {
        let mut __value: Value = json!({});
        return CareTeam_ParticipantBuilder { value: __value };
    }

    pub fn coverage_period<'a>(&'a mut self, val: Period) -> &'a mut CareTeam_ParticipantBuilder {
        self.value["coveragePeriod"] = json!(val.value);
        return self;
    }

    pub fn coverage_timing<'a>(&'a mut self, val: Timing) -> &'a mut CareTeam_ParticipantBuilder {
        self.value["coverageTiming"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut CareTeam_ParticipantBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut CareTeam_ParticipantBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn member<'a>(&'a mut self, val: Reference) -> &'a mut CareTeam_ParticipantBuilder {
        self.value["member"] = json!(val.value);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut CareTeam_ParticipantBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn on_behalf_of<'a>(&'a mut self, val: Reference) -> &'a mut CareTeam_ParticipantBuilder {
        self.value["onBehalfOf"] = json!(val.value);
        return self;
    }

    pub fn role<'a>(&'a mut self, val: CodeableConcept) -> &'a mut CareTeam_ParticipantBuilder {
        self.value["role"] = json!(val.value);
        return self;
    }
}
