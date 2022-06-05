#![allow(unused_imports, non_camel_case_types)]

use crate::models::r5::Extension::Extension;
use crate::models::r5::MedicationKnowledge_Cost::MedicationKnowledge_Cost;
use crate::models::r5::Reference::Reference;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Information about a medication that is used to support knowledge.

#[derive(Debug)]
pub struct MedicationKnowledge_Packaging<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl MedicationKnowledge_Packaging<'_> {
    pub fn new(value: &Value) -> MedicationKnowledge_Packaging {
        MedicationKnowledge_Packaging {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// The cost of the packaged medication.
    pub fn cost(&self) -> Option<Vec<MedicationKnowledge_Cost>> {
        if let Some(Value::Array(val)) = self.value.get("cost") {
            return Some(
                val.into_iter()
                    .map(|e| MedicationKnowledge_Cost {
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

    /// A reference to a PackagedProductDefinition that provides the details of the
    /// product that is in the packaging and is being priced.
    pub fn packaged_product(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("packagedProduct") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.cost() {
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
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.packaged_product() {
            if !_val.validate() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct MedicationKnowledge_PackagingBuilder {
    pub(crate) value: Value,
}

impl MedicationKnowledge_PackagingBuilder {
    pub fn build(&self) -> MedicationKnowledge_Packaging {
        MedicationKnowledge_Packaging {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: MedicationKnowledge_Packaging) -> MedicationKnowledge_PackagingBuilder {
        MedicationKnowledge_PackagingBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> MedicationKnowledge_PackagingBuilder {
        let mut __value: Value = json!({});
        return MedicationKnowledge_PackagingBuilder { value: __value };
    }

    pub fn cost<'a>(
        &'a mut self,
        val: Vec<MedicationKnowledge_Cost>,
    ) -> &'a mut MedicationKnowledge_PackagingBuilder {
        self.value["cost"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut MedicationKnowledge_PackagingBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut MedicationKnowledge_PackagingBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut MedicationKnowledge_PackagingBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn packaged_product<'a>(
        &'a mut self,
        val: Reference,
    ) -> &'a mut MedicationKnowledge_PackagingBuilder {
        self.value["packagedProduct"] = json!(val.value);
        return self;
    }
}
