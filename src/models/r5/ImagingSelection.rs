#![allow(unused_imports, non_camel_case_types)]

use crate::models::r5::CodeableConcept::CodeableConcept;
use crate::models::r5::Coding::Coding;
use crate::models::r5::Element::Element;
use crate::models::r5::Extension::Extension;
use crate::models::r5::Identifier::Identifier;
use crate::models::r5::ImagingSelection_ImageRegion::ImagingSelection_ImageRegion;
use crate::models::r5::ImagingSelection_Instance::ImagingSelection_Instance;
use crate::models::r5::ImagingSelection_Performer::ImagingSelection_Performer;
use crate::models::r5::Meta::Meta;
use crate::models::r5::Narrative::Narrative;
use crate::models::r5::Reference::Reference;
use crate::models::r5::ResourceList::ResourceList;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A selection of DICOM SOP instances and/or frames within a single Study and Series.
/// This might include additional specifics such as an image region, an Observation
/// UID or a Segmentation Number, allowing linkage to an Observation Resource or
/// transferring this information along with the ImagingStudy Resource.

#[derive(Debug)]
pub struct ImagingSelection<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl ImagingSelection<'_> {
    pub fn new(value: &Value) -> ImagingSelection {
        ImagingSelection {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for frameOfReferenceUid
    pub fn _frame_of_reference_uid(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_frameOfReferenceUid") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for implicitRules
    pub fn _implicit_rules(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_implicitRules") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for issued
    pub fn _issued(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_issued") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for language
    pub fn _language(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_language") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for seriesUid
    pub fn _series_uid(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_seriesUid") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for studyUid
    pub fn _study_uid(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_studyUid") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A list of the diagnostic requests that resulted in this imaging selection being
    /// performed.
    pub fn based_on(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("basedOn") {
            return Some(
                val.into_iter()
                    .map(|e| Reference {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The anatomic structures examined. See DICOM Part 16 Annex L (http://
    /// dicom.nema.org/medical/dicom/current/output/chtml/part16/chapter_L.html) for DICOM
    /// to SNOMED-CT mappings.
    pub fn body_site(&self) -> Option<Coding> {
        if let Some(val) = self.value.get("bodySite") {
            return Some(Coding {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Describes the imaging selection.
    pub fn code(&self) -> CodeableConcept {
        CodeableConcept {
            value: Cow::Borrowed(&self.value["code"]),
        }
    }

    /// These resources do not have an independent existence apart from the resource that
    /// contains them - they cannot be identified independently, nor can they have their
    /// own independent transaction scope.
    pub fn contained(&self) -> Option<Vec<ResourceList>> {
        if let Some(Value::Array(val)) = self.value.get("contained") {
            return Some(
                val.into_iter()
                    .map(|e| ResourceList {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The imaging study from which the imaging selection is made.
    pub fn derived_from(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("derivedFrom") {
            return Some(
                val.into_iter()
                    .map(|e| Reference {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The network service providing retrieval access to the selected images, frames,
    /// etc. See implementation notes for information about using DICOM endpoints.
    pub fn endpoint(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("endpoint") {
            return Some(
                val.into_iter()
                    .map(|e| Reference {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// May be used to represent additional information that is not part of the basic
    /// definition of the resource. To make the use of extensions safe and manageable,
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

    /// The Frame of Reference UID identifying the coordinate system that conveys spatial
    /// and/or temporal information for the selected images or frames.
    pub fn frame_of_reference_uid(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("frameOfReferenceUid") {
            return Some(string);
        }
        return None;
    }

    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
        }
        return None;
    }

    /// A unique identifier assigned to this imaging selection.
    pub fn identifier(&self) -> Option<Vec<Identifier>> {
        if let Some(Value::Array(val)) = self.value.get("identifier") {
            return Some(
                val.into_iter()
                    .map(|e| Identifier {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Each imaging selection might includes one or more image regions. Image regions are
    /// specified by a region type and a set of 2D or 3D coordinates.
    pub fn image_region(&self) -> Option<ImagingSelection_ImageRegion> {
        if let Some(val) = self.value.get("imageRegion") {
            return Some(ImagingSelection_ImageRegion {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often, this
    /// is a reference to an implementation guide that defines the special rules along
    /// with other profiles etc.
    pub fn implicit_rules(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("implicitRules") {
            return Some(string);
        }
        return None;
    }

    /// Each imaging selection includes one or more selected DICOM SOP instances.
    pub fn instance(&self) -> Option<Vec<ImagingSelection_Instance>> {
        if let Some(Value::Array(val)) = self.value.get("instance") {
            return Some(
                val.into_iter()
                    .map(|e| ImagingSelection_Instance {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The date and time this imaging selection was created.
    pub fn issued(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("issued") {
            return Some(string);
        }
        return None;
    }

    /// The base language in which the resource is written.
    pub fn language(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("language") {
            return Some(string);
        }
        return None;
    }

    /// The metadata about the resource. This is content that is maintained by the
    /// infrastructure. Changes to the content might not always be associated with version
    /// changes to the resource.
    pub fn meta(&self) -> Option<Meta> {
        if let Some(val) = self.value.get("meta") {
            return Some(Meta {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// May be used to represent additional information that is not part of the basic
    /// definition of the resource and that modifies the understanding of the element
    /// that contains it and/or the understanding of the containing element's descendants.
    /// Usually modifier elements provide negation or qualification. To make the use of
    /// extensions safe and manageable, there is a strict set of governance applied to
    /// the definition and use of extensions. Though any implementer is allowed to define
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

    /// Author – human or machine.
    pub fn performer(&self) -> Option<Vec<ImagingSelection_Performer>> {
        if let Some(Value::Array(val)) = self.value.get("performer") {
            return Some(
                val.into_iter()
                    .map(|e| ImagingSelection_Performer {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The Series Instance UID for the DICOM Series from which the images were selected.
    pub fn series_uid(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("seriesUid") {
            return Some(string);
        }
        return None;
    }

    /// The Study Instance UID for the DICOM Study from which the images were selected.
    pub fn study_uid(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("studyUid") {
            return Some(string);
        }
        return None;
    }

    /// The patient, or group of patients, location, device, organization, procedure or
    /// practitioner this imaging selection is about and into whose or what record the
    /// imaging selection is placed.
    pub fn subject(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("subject") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A human-readable narrative that contains a summary of the resource and can be used
    /// to represent the content of the resource to a human. The narrative need not encode
    /// all the structured data, but is required to contain sufficient detail to make it
    /// "clinically safe" for a human to just read the narrative. Resource definitions
    /// may define what content should be represented in the narrative to ensure clinical
    /// safety.
    pub fn text(&self) -> Option<Narrative> {
        if let Some(val) = self.value.get("text") {
            return Some(Narrative {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._frame_of_reference_uid() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._implicit_rules() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._issued() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._language() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._series_uid() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._study_uid() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.based_on() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.body_site() {
            if !_val.validate() {
                return false;
            }
        }
        if !self.code().validate() {
            return false;
        }
        if let Some(_val) = self.contained() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.derived_from() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.endpoint() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.frame_of_reference_uid() {}
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.identifier() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.image_region() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.implicit_rules() {}
        if let Some(_val) = self.instance() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.issued() {}
        if let Some(_val) = self.language() {}
        if let Some(_val) = self.meta() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.performer() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.series_uid() {}
        if let Some(_val) = self.study_uid() {}
        if let Some(_val) = self.subject() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.text() {
            if !_val.validate() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct ImagingSelectionBuilder {
    pub(crate) value: Value,
}

impl ImagingSelectionBuilder {
    pub fn build(&self) -> ImagingSelection {
        ImagingSelection {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: ImagingSelection) -> ImagingSelectionBuilder {
        ImagingSelectionBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(code: CodeableConcept) -> ImagingSelectionBuilder {
        let mut __value: Value = json!({});
        __value["code"] = json!(code.value);
        return ImagingSelectionBuilder { value: __value };
    }

    pub fn _frame_of_reference_uid<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut ImagingSelectionBuilder {
        self.value["_frameOfReferenceUid"] = json!(val.value);
        return self;
    }

    pub fn _implicit_rules<'a>(&'a mut self, val: Element) -> &'a mut ImagingSelectionBuilder {
        self.value["_implicitRules"] = json!(val.value);
        return self;
    }

    pub fn _issued<'a>(&'a mut self, val: Element) -> &'a mut ImagingSelectionBuilder {
        self.value["_issued"] = json!(val.value);
        return self;
    }

    pub fn _language<'a>(&'a mut self, val: Element) -> &'a mut ImagingSelectionBuilder {
        self.value["_language"] = json!(val.value);
        return self;
    }

    pub fn _series_uid<'a>(&'a mut self, val: Element) -> &'a mut ImagingSelectionBuilder {
        self.value["_seriesUid"] = json!(val.value);
        return self;
    }

    pub fn _study_uid<'a>(&'a mut self, val: Element) -> &'a mut ImagingSelectionBuilder {
        self.value["_studyUid"] = json!(val.value);
        return self;
    }

    pub fn based_on<'a>(&'a mut self, val: Vec<Reference>) -> &'a mut ImagingSelectionBuilder {
        self.value["basedOn"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn body_site<'a>(&'a mut self, val: Coding) -> &'a mut ImagingSelectionBuilder {
        self.value["bodySite"] = json!(val.value);
        return self;
    }

    pub fn contained<'a>(&'a mut self, val: Vec<ResourceList>) -> &'a mut ImagingSelectionBuilder {
        self.value["contained"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn derived_from<'a>(&'a mut self, val: Vec<Reference>) -> &'a mut ImagingSelectionBuilder {
        self.value["derivedFrom"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn endpoint<'a>(&'a mut self, val: Vec<Reference>) -> &'a mut ImagingSelectionBuilder {
        self.value["endpoint"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut ImagingSelectionBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn frame_of_reference_uid<'a>(&'a mut self, val: &str) -> &'a mut ImagingSelectionBuilder {
        self.value["frameOfReferenceUid"] = json!(val);
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut ImagingSelectionBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn identifier<'a>(&'a mut self, val: Vec<Identifier>) -> &'a mut ImagingSelectionBuilder {
        self.value["identifier"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn image_region<'a>(
        &'a mut self,
        val: ImagingSelection_ImageRegion,
    ) -> &'a mut ImagingSelectionBuilder {
        self.value["imageRegion"] = json!(val.value);
        return self;
    }

    pub fn implicit_rules<'a>(&'a mut self, val: &str) -> &'a mut ImagingSelectionBuilder {
        self.value["implicitRules"] = json!(val);
        return self;
    }

    pub fn instance<'a>(
        &'a mut self,
        val: Vec<ImagingSelection_Instance>,
    ) -> &'a mut ImagingSelectionBuilder {
        self.value["instance"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn issued<'a>(&'a mut self, val: &str) -> &'a mut ImagingSelectionBuilder {
        self.value["issued"] = json!(val);
        return self;
    }

    pub fn language<'a>(&'a mut self, val: &str) -> &'a mut ImagingSelectionBuilder {
        self.value["language"] = json!(val);
        return self;
    }

    pub fn meta<'a>(&'a mut self, val: Meta) -> &'a mut ImagingSelectionBuilder {
        self.value["meta"] = json!(val.value);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ImagingSelectionBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn performer<'a>(
        &'a mut self,
        val: Vec<ImagingSelection_Performer>,
    ) -> &'a mut ImagingSelectionBuilder {
        self.value["performer"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn series_uid<'a>(&'a mut self, val: &str) -> &'a mut ImagingSelectionBuilder {
        self.value["seriesUid"] = json!(val);
        return self;
    }

    pub fn study_uid<'a>(&'a mut self, val: &str) -> &'a mut ImagingSelectionBuilder {
        self.value["studyUid"] = json!(val);
        return self;
    }

    pub fn subject<'a>(&'a mut self, val: Reference) -> &'a mut ImagingSelectionBuilder {
        self.value["subject"] = json!(val.value);
        return self;
    }

    pub fn text<'a>(&'a mut self, val: Narrative) -> &'a mut ImagingSelectionBuilder {
        self.value["text"] = json!(val.value);
        return self;
    }
}
