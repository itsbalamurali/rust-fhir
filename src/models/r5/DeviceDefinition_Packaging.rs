#![allow(unused_imports, non_camel_case_types)]

use crate::models::r5::CodeableConcept::CodeableConcept;
use crate::models::r5::DeviceDefinition_Distributor::DeviceDefinition_Distributor;
use crate::models::r5::DeviceDefinition_UdiDeviceIdentifier1::DeviceDefinition_UdiDeviceIdentifier1;
use crate::models::r5::Element::Element;
use crate::models::r5::Extension::Extension;
use crate::models::r5::Identifier::Identifier;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// The characteristics, operational status and capabilities of a medical-related
/// component of a medical device.

#[derive(Debug)]
pub struct DeviceDefinition_Packaging<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl DeviceDefinition_Packaging<'_> {
    pub fn new(value: &Value) -> DeviceDefinition_Packaging {
        DeviceDefinition_Packaging {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for count
    pub fn _count(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_count") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The number of items contained in the package (devices or sub-packages).
    pub fn count(&self) -> Option<i64> {
        if let Some(val) = self.value.get("count") {
            return Some(val.as_i64().unwrap());
        }
        return None;
    }

    /// An organization that distributes the packaged device.
    pub fn distributor(&self) -> Option<Vec<DeviceDefinition_Distributor>> {
        if let Some(Value::Array(val)) = self.value.get("distributor") {
            return Some(
                val.into_iter()
                    .map(|e| DeviceDefinition_Distributor {
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

    /// The business identifier of the packaged medication.
    pub fn identifier(&self) -> Option<Identifier> {
        if let Some(val) = self.value.get("identifier") {
            return Some(Identifier {
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

    /// Allows packages within packages.
    pub fn packaging(&self) -> Option<Vec<DeviceDefinition_Packaging>> {
        if let Some(Value::Array(val)) = self.value.get("packaging") {
            return Some(
                val.into_iter()
                    .map(|e| DeviceDefinition_Packaging {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A code that defines the specific type of packaging.
    pub fn fhir_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("type") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Unique Device Identifier (UDI) Barcode string on the packaging.
    pub fn udi_device_identifier(&self) -> Option<Vec<DeviceDefinition_UdiDeviceIdentifier1>> {
        if let Some(Value::Array(val)) = self.value.get("udiDeviceIdentifier") {
            return Some(
                val.into_iter()
                    .map(|e| DeviceDefinition_UdiDeviceIdentifier1 {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._count() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.count() {}
        if let Some(_val) = self.distributor() {
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
        if let Some(_val) = self.identifier() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.packaging() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.fhir_type() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.udi_device_identifier() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct DeviceDefinition_PackagingBuilder {
    pub(crate) value: Value,
}

impl DeviceDefinition_PackagingBuilder {
    pub fn build(&self) -> DeviceDefinition_Packaging {
        DeviceDefinition_Packaging {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: DeviceDefinition_Packaging) -> DeviceDefinition_PackagingBuilder {
        DeviceDefinition_PackagingBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> DeviceDefinition_PackagingBuilder {
        let mut __value: Value = json!({});
        return DeviceDefinition_PackagingBuilder { value: __value };
    }

    pub fn _count<'a>(&'a mut self, val: Element) -> &'a mut DeviceDefinition_PackagingBuilder {
        self.value["_count"] = json!(val.value);
        return self;
    }

    pub fn count<'a>(&'a mut self, val: i64) -> &'a mut DeviceDefinition_PackagingBuilder {
        self.value["count"] = json!(val);
        return self;
    }

    pub fn distributor<'a>(
        &'a mut self,
        val: Vec<DeviceDefinition_Distributor>,
    ) -> &'a mut DeviceDefinition_PackagingBuilder {
        self.value["distributor"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut DeviceDefinition_PackagingBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut DeviceDefinition_PackagingBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn identifier<'a>(
        &'a mut self,
        val: Identifier,
    ) -> &'a mut DeviceDefinition_PackagingBuilder {
        self.value["identifier"] = json!(val.value);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut DeviceDefinition_PackagingBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn packaging<'a>(
        &'a mut self,
        val: Vec<DeviceDefinition_Packaging>,
    ) -> &'a mut DeviceDefinition_PackagingBuilder {
        self.value["packaging"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn fhir_type<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut DeviceDefinition_PackagingBuilder {
        self.value["type"] = json!(val.value);
        return self;
    }

    pub fn udi_device_identifier<'a>(
        &'a mut self,
        val: Vec<DeviceDefinition_UdiDeviceIdentifier1>,
    ) -> &'a mut DeviceDefinition_PackagingBuilder {
        self.value["udiDeviceIdentifier"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }
}
