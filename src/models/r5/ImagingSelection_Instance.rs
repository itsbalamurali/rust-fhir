#![allow(unused_imports, non_camel_case_types)]

use crate::models::r5::Coding::Coding;
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
pub struct ImagingSelection_Instance<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl ImagingSelection_Instance<'_> {
    pub fn new(value: &Value) -> ImagingSelection_Instance {
        ImagingSelection_Instance {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for frameList
    pub fn _frame_list(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_frameList") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for observationUid
    pub fn _observation_uid(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_observationUid") {
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

    /// Extensions for roiList
    pub fn _roi_list(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_roiList") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for segmentList
    pub fn _segment_list(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_segmentList") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for uid
    pub fn _uid(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_uid") {
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

    /// The set of frames within a multi-frame SOP Instance that are included in the
    /// imaging selection.       Encoded as a comma separated list of one or more non
    /// duplicate frame numbers.       If this is absent, all frames within the referenced
    /// SOP Instance are included in the selection.
    pub fn frame_list(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("frameList") {
            return Some(string);
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

    /// The unique identifier for the observation Content Item (and its subsidiary Content
    /// Items, if any) that are included in the imaging selection.
    pub fn observation_uid(&self) -> Option<Vec<&str>> {
        if let Some(Value::Array(val)) = self.value.get("observationUid") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_str().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The set of regions of interest (ROI) within a radiotherapy structure set instance
    /// that are included in the imaging selection.       Encoded as a comma separated
    /// list of one or more non duplicate ROI numbers.       If this is absent, all ROIs
    /// within the referenced radiotherapy structure set SOP Instance are included in
    /// the selection.
    pub fn roi_list(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("roiList") {
            return Some(string);
        }
        return None;
    }

    /// The set of segments within a segmentation SOP Instance that are included in
    /// the imaging selection.       Encoded as a comma separated list of one or more
    /// non duplicate segment numbers.       If this is absent, all segments within the
    /// referenced segmentation SOP Instance are included in the selection.
    pub fn segment_list(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("segmentList") {
            return Some(string);
        }
        return None;
    }

    /// The SOP Class UID for the selected DICOM instance.
    pub fn sop_class(&self) -> Option<Coding> {
        if let Some(val) = self.value.get("sopClass") {
            return Some(Coding {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The SOP Instance UID for the selected DICOM instance.
    pub fn uid(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("uid") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._frame_list() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._observation_uid() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self._roi_list() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._segment_list() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._uid() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.frame_list() {}
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.observation_uid() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.roi_list() {}
        if let Some(_val) = self.segment_list() {}
        if let Some(_val) = self.sop_class() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.uid() {}
        return true;
    }
}

#[derive(Debug)]
pub struct ImagingSelection_InstanceBuilder {
    pub(crate) value: Value,
}

impl ImagingSelection_InstanceBuilder {
    pub fn build(&self) -> ImagingSelection_Instance {
        ImagingSelection_Instance {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: ImagingSelection_Instance) -> ImagingSelection_InstanceBuilder {
        ImagingSelection_InstanceBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> ImagingSelection_InstanceBuilder {
        let mut __value: Value = json!({});
        return ImagingSelection_InstanceBuilder { value: __value };
    }

    pub fn _frame_list<'a>(&'a mut self, val: Element) -> &'a mut ImagingSelection_InstanceBuilder {
        self.value["_frameList"] = json!(val.value);
        return self;
    }

    pub fn _observation_uid<'a>(
        &'a mut self,
        val: Vec<Element>,
    ) -> &'a mut ImagingSelection_InstanceBuilder {
        self.value["_observationUid"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn _roi_list<'a>(&'a mut self, val: Element) -> &'a mut ImagingSelection_InstanceBuilder {
        self.value["_roiList"] = json!(val.value);
        return self;
    }

    pub fn _segment_list<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut ImagingSelection_InstanceBuilder {
        self.value["_segmentList"] = json!(val.value);
        return self;
    }

    pub fn _uid<'a>(&'a mut self, val: Element) -> &'a mut ImagingSelection_InstanceBuilder {
        self.value["_uid"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ImagingSelection_InstanceBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn frame_list<'a>(&'a mut self, val: &str) -> &'a mut ImagingSelection_InstanceBuilder {
        self.value["frameList"] = json!(val);
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut ImagingSelection_InstanceBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ImagingSelection_InstanceBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn observation_uid<'a>(
        &'a mut self,
        val: Vec<&str>,
    ) -> &'a mut ImagingSelection_InstanceBuilder {
        self.value["observationUid"] = json!(val);
        return self;
    }

    pub fn roi_list<'a>(&'a mut self, val: &str) -> &'a mut ImagingSelection_InstanceBuilder {
        self.value["roiList"] = json!(val);
        return self;
    }

    pub fn segment_list<'a>(&'a mut self, val: &str) -> &'a mut ImagingSelection_InstanceBuilder {
        self.value["segmentList"] = json!(val);
        return self;
    }

    pub fn sop_class<'a>(&'a mut self, val: Coding) -> &'a mut ImagingSelection_InstanceBuilder {
        self.value["sopClass"] = json!(val.value);
        return self;
    }

    pub fn uid<'a>(&'a mut self, val: &str) -> &'a mut ImagingSelection_InstanceBuilder {
        self.value["uid"] = json!(val);
        return self;
    }
}
