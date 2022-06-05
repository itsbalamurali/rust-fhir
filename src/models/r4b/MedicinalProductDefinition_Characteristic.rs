#![allow(unused_imports, non_camel_case_types)]

use crate::models::r4b::Attachment::Attachment;
use crate::models::r4b::CodeableConcept::CodeableConcept;
use crate::models::r4b::Element::Element;
use crate::models::r4b::Extension::Extension;
use crate::models::r4b::Quantity::Quantity;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A medicinal product, being a substance or combination of substances that is
/// intended to treat, prevent or diagnose a disease, or to restore, correct or modify
/// physiological functions by exerting a pharmacological, immunological or metabolic
/// action. This resource is intended to define and detail such products and their
/// properties, for uses other than direct patient care (e.g. regulatory use, or drug
/// catalogs).

#[derive(Debug)]
pub struct MedicinalProductDefinition_Characteristic<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl MedicinalProductDefinition_Characteristic<'_> {
    pub fn new(value: &Value) -> MedicinalProductDefinition_Characteristic {
        MedicinalProductDefinition_Characteristic {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for valueBoolean
    pub fn _value_boolean(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_valueBoolean") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for valueDate
    pub fn _value_date(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_valueDate") {
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

    /// A code expressing the type of characteristic.
    pub fn fhir_type(&self) -> CodeableConcept {
        CodeableConcept {
            value: Cow::Borrowed(&self.value["type"]),
        }
    }

    /// A value for the characteristic.
    pub fn value_attachment(&self) -> Option<Attachment> {
        if let Some(val) = self.value.get("valueAttachment") {
            return Some(Attachment {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A value for the characteristic.
    pub fn value_boolean(&self) -> Option<bool> {
        if let Some(val) = self.value.get("valueBoolean") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    /// A value for the characteristic.
    pub fn value_codeable_concept(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("valueCodeableConcept") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A value for the characteristic.
    pub fn value_date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("valueDate") {
            return Some(string);
        }
        return None;
    }

    /// A value for the characteristic.
    pub fn value_quantity(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("valueQuantity") {
            return Some(Quantity {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._value_boolean() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._value_date() {
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
        if !self.fhir_type().validate() {
            return false;
        }
        if let Some(_val) = self.value_attachment() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.value_boolean() {}
        if let Some(_val) = self.value_codeable_concept() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.value_date() {}
        if let Some(_val) = self.value_quantity() {
            if !_val.validate() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct MedicinalProductDefinition_CharacteristicBuilder {
    pub(crate) value: Value,
}

impl MedicinalProductDefinition_CharacteristicBuilder {
    pub fn build(&self) -> MedicinalProductDefinition_Characteristic {
        MedicinalProductDefinition_Characteristic {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(
        existing: MedicinalProductDefinition_Characteristic,
    ) -> MedicinalProductDefinition_CharacteristicBuilder {
        MedicinalProductDefinition_CharacteristicBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(fhir_type: CodeableConcept) -> MedicinalProductDefinition_CharacteristicBuilder {
        let mut __value: Value = json!({});
        __value["type"] = json!(fhir_type.value);
        return MedicinalProductDefinition_CharacteristicBuilder { value: __value };
    }

    pub fn _value_boolean<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut MedicinalProductDefinition_CharacteristicBuilder {
        self.value["_valueBoolean"] = json!(val.value);
        return self;
    }

    pub fn _value_date<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut MedicinalProductDefinition_CharacteristicBuilder {
        self.value["_valueDate"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut MedicinalProductDefinition_CharacteristicBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut MedicinalProductDefinition_CharacteristicBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut MedicinalProductDefinition_CharacteristicBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn value_attachment<'a>(
        &'a mut self,
        val: Attachment,
    ) -> &'a mut MedicinalProductDefinition_CharacteristicBuilder {
        self.value["valueAttachment"] = json!(val.value);
        return self;
    }

    pub fn value_boolean<'a>(
        &'a mut self,
        val: bool,
    ) -> &'a mut MedicinalProductDefinition_CharacteristicBuilder {
        self.value["valueBoolean"] = json!(val);
        return self;
    }

    pub fn value_codeable_concept<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut MedicinalProductDefinition_CharacteristicBuilder {
        self.value["valueCodeableConcept"] = json!(val.value);
        return self;
    }

    pub fn value_date<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut MedicinalProductDefinition_CharacteristicBuilder {
        self.value["valueDate"] = json!(val);
        return self;
    }

    pub fn value_quantity<'a>(
        &'a mut self,
        val: Quantity,
    ) -> &'a mut MedicinalProductDefinition_CharacteristicBuilder {
        self.value["valueQuantity"] = json!(val.value);
        return self;
    }
}
