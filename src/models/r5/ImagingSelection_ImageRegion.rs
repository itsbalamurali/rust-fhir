#![allow(unused_imports, non_camel_case_types)]

use crate::models::r5::Element::Element;
use crate::models::r5::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A selection of DICOM SOP instances and/or frames within a single Study and Series.
/// This might include additional specifics such as an image region, an Observation
/// UID or a Segmentation Number, allowing linkage to an Observation Resource or
/// transferring this information along with the ImagingStudy Resource.

#[derive(Debug)]
pub struct ImagingSelection_ImageRegion<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl ImagingSelection_ImageRegion<'_> {
    pub fn new(value: &Value) -> ImagingSelection_ImageRegion {
        ImagingSelection_ImageRegion {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for coordinateType
    pub fn _coordinate_type(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_coordinateType") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for coordinates
    pub fn _coordinates(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_coordinates") {
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

    /// Extensions for regionType
    pub fn _region_type(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_regionType") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Specifies the type of coordinate system that define the image region.
    pub fn coordinate_type(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("coordinateType") {
            return Some(string);
        }
        return None;
    }

    /// The coordinates describing the image region.       If coordinateType is 2D this
    /// specifies sequence of (x,y) coordinates in the coordinate system of the image
    /// specified by the instance.uid element that contains this image region.       If
    /// coordinateType is 3D this specifies sequence of (x,y,z) coordinates in the
    /// coordinate system specified by the frameOfReferenceUid element.
    pub fn coordinates(&self) -> Option<Vec<f64>> {
        if let Some(Value::Array(val)) = self.value.get("coordinates") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_f64().unwrap())
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

    /// Specifies the type of image region.
    pub fn region_type(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("regionType") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._coordinate_type() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._coordinates() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self._region_type() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.coordinate_type() {}
        if let Some(_val) = self.coordinates() {
            _val.into_iter().for_each(|_e| {});
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
        if let Some(_val) = self.region_type() {}
        return true;
    }
}

#[derive(Debug)]
pub struct ImagingSelection_ImageRegionBuilder {
    pub(crate) value: Value,
}

impl ImagingSelection_ImageRegionBuilder {
    pub fn build(&self) -> ImagingSelection_ImageRegion {
        ImagingSelection_ImageRegion {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: ImagingSelection_ImageRegion) -> ImagingSelection_ImageRegionBuilder {
        ImagingSelection_ImageRegionBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> ImagingSelection_ImageRegionBuilder {
        let mut __value: Value = json!({});
        return ImagingSelection_ImageRegionBuilder { value: __value };
    }

    pub fn _coordinate_type<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut ImagingSelection_ImageRegionBuilder {
        self.value["_coordinateType"] = json!(val.value);
        return self;
    }

    pub fn _coordinates<'a>(
        &'a mut self,
        val: Vec<Element>,
    ) -> &'a mut ImagingSelection_ImageRegionBuilder {
        self.value["_coordinates"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn _region_type<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut ImagingSelection_ImageRegionBuilder {
        self.value["_regionType"] = json!(val.value);
        return self;
    }

    pub fn coordinate_type<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut ImagingSelection_ImageRegionBuilder {
        self.value["coordinateType"] = json!(val);
        return self;
    }

    pub fn coordinates<'a>(
        &'a mut self,
        val: Vec<f64>,
    ) -> &'a mut ImagingSelection_ImageRegionBuilder {
        self.value["coordinates"] = json!(val);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ImagingSelection_ImageRegionBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut ImagingSelection_ImageRegionBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ImagingSelection_ImageRegionBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn region_type<'a>(&'a mut self, val: &str) -> &'a mut ImagingSelection_ImageRegionBuilder {
        self.value["regionType"] = json!(val);
        return self;
    }
}
