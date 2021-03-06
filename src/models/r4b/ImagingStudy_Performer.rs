#![allow(unused_imports, non_camel_case_types)]

use crate::models::r4b::CodeableConcept::CodeableConcept;
use crate::models::r4b::Extension::Extension;
use crate::models::r4b::Reference::Reference;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Representation of the content produced in a DICOM imaging study. A study comprises
/// a set of series, each of which includes a set of Service-Object Pair Instances
/// (SOP Instances - images or other data) acquired or produced in a common context.
/// A series is of only one modality (e.g. X-ray, CT, MR, ultrasound), but a study may
/// have multiple series of different modalities.

#[derive(Debug)]
pub struct ImagingStudy_Performer<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl ImagingStudy_Performer<'_> {
    pub fn new(value: &Value) -> ImagingStudy_Performer {
        ImagingStudy_Performer {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Indicates who or what performed the series.
    pub fn actor(&self) -> Reference {
        Reference {
            value: Cow::Borrowed(&self.value["actor"]),
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

    /// Distinguishes the type of involvement of the performer in the series.
    pub fn function(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("function") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
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

    pub fn validate(&self) -> bool {
        if !self.actor().validate() {
            return false;
        }
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.function() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct ImagingStudy_PerformerBuilder {
    pub(crate) value: Value,
}

impl ImagingStudy_PerformerBuilder {
    pub fn build(&self) -> ImagingStudy_Performer {
        ImagingStudy_Performer {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: ImagingStudy_Performer) -> ImagingStudy_PerformerBuilder {
        ImagingStudy_PerformerBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(actor: Reference) -> ImagingStudy_PerformerBuilder {
        let mut __value: Value = json!({});
        __value["actor"] = json!(actor.value);
        return ImagingStudy_PerformerBuilder { value: __value };
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ImagingStudy_PerformerBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn function<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut ImagingStudy_PerformerBuilder {
        self.value["function"] = json!(val.value);
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut ImagingStudy_PerformerBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ImagingStudy_PerformerBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }
}
