#![allow(unused_imports, non_camel_case_types)]

use crate::models::r5::CodeableConcept::CodeableConcept;
use crate::models::r5::Element::Element;
use crate::models::r5::Extension::Extension;
use crate::models::r5::Reference::Reference;
use crate::models::r5::RelatedArtifact::RelatedArtifact;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// This Resource provides one or more comments, classifiers or ratings about a
/// Resource and supports attribution and rights management metadata for the added
/// content.

#[derive(Debug)]
pub struct ArtifactAssessment_Content<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl ArtifactAssessment_Content<'_> {
    pub fn new(value: &Value) -> ArtifactAssessment_Content {
        ArtifactAssessment_Content {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for freeToShare
    pub fn _free_to_share(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_freeToShare") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for informationType
    pub fn _information_type(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_informationType") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for path
    pub fn _path(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_path") {
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

    /// Extensions for summary
    pub fn _summary(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_summary") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Indicates who or what authored the content.
    pub fn author(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("author") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Represents a rating, classifier, or assessment of the artifact.
    pub fn classifier(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("classifier") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// If the informationType is container, the components of the content.
    pub fn component(&self) -> Option<Vec<ArtifactAssessment_Content>> {
        if let Some(Value::Array(val)) = self.value.get("component") {
            return Some(
                val.into_iter()
                    .map(|e| ArtifactAssessment_Content {
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

    /// Acceptable to publicly share the comment, classifier or rating.
    pub fn free_to_share(&self) -> Option<bool> {
        if let Some(val) = self.value.get("freeToShare") {
            return Some(val.as_bool().unwrap());
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

    /// The type of information this component of the content represents.
    pub fn information_type(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("informationType") {
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

    /// A URI that points to what the comment is about, such as a line of text in the
    /// CQL, or a specific element in a resource.
    pub fn path(&self) -> Option<Vec<&str>> {
        if let Some(Value::Array(val)) = self.value.get("path") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_str().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Additional related artifacts that provide supporting documentation, additional
    /// evidence, or further information related to the content.
    pub fn related_artifact(&self) -> Option<Vec<RelatedArtifact>> {
        if let Some(Value::Array(val)) = self.value.get("relatedArtifact") {
            return Some(
                val.into_iter()
                    .map(|e| RelatedArtifact {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A brief summary of the content of this component.
    pub fn summary(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("summary") {
            return Some(string);
        }
        return None;
    }

    /// Indicates what type of content this component represents.
    pub fn fhir_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("type") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._free_to_share() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._information_type() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._path() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self._summary() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.author() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.classifier() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.component() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.free_to_share() {}
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.information_type() {}
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.path() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.related_artifact() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.summary() {}
        if let Some(_val) = self.fhir_type() {
            if !_val.validate() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct ArtifactAssessment_ContentBuilder {
    pub(crate) value: Value,
}

impl ArtifactAssessment_ContentBuilder {
    pub fn build(&self) -> ArtifactAssessment_Content {
        ArtifactAssessment_Content {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: ArtifactAssessment_Content) -> ArtifactAssessment_ContentBuilder {
        ArtifactAssessment_ContentBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> ArtifactAssessment_ContentBuilder {
        let mut __value: Value = json!({});
        return ArtifactAssessment_ContentBuilder { value: __value };
    }

    pub fn _free_to_share<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut ArtifactAssessment_ContentBuilder {
        self.value["_freeToShare"] = json!(val.value);
        return self;
    }

    pub fn _information_type<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut ArtifactAssessment_ContentBuilder {
        self.value["_informationType"] = json!(val.value);
        return self;
    }

    pub fn _path<'a>(&'a mut self, val: Vec<Element>) -> &'a mut ArtifactAssessment_ContentBuilder {
        self.value["_path"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn _summary<'a>(&'a mut self, val: Element) -> &'a mut ArtifactAssessment_ContentBuilder {
        self.value["_summary"] = json!(val.value);
        return self;
    }

    pub fn author<'a>(&'a mut self, val: Reference) -> &'a mut ArtifactAssessment_ContentBuilder {
        self.value["author"] = json!(val.value);
        return self;
    }

    pub fn classifier<'a>(
        &'a mut self,
        val: Vec<CodeableConcept>,
    ) -> &'a mut ArtifactAssessment_ContentBuilder {
        self.value["classifier"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn component<'a>(
        &'a mut self,
        val: Vec<ArtifactAssessment_Content>,
    ) -> &'a mut ArtifactAssessment_ContentBuilder {
        self.value["component"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ArtifactAssessment_ContentBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn free_to_share<'a>(&'a mut self, val: bool) -> &'a mut ArtifactAssessment_ContentBuilder {
        self.value["freeToShare"] = json!(val);
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut ArtifactAssessment_ContentBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn information_type<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut ArtifactAssessment_ContentBuilder {
        self.value["informationType"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ArtifactAssessment_ContentBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn path<'a>(&'a mut self, val: Vec<&str>) -> &'a mut ArtifactAssessment_ContentBuilder {
        self.value["path"] = json!(val);
        return self;
    }

    pub fn related_artifact<'a>(
        &'a mut self,
        val: Vec<RelatedArtifact>,
    ) -> &'a mut ArtifactAssessment_ContentBuilder {
        self.value["relatedArtifact"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn summary<'a>(&'a mut self, val: &str) -> &'a mut ArtifactAssessment_ContentBuilder {
        self.value["summary"] = json!(val);
        return self;
    }

    pub fn fhir_type<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut ArtifactAssessment_ContentBuilder {
        self.value["type"] = json!(val.value);
        return self;
    }
}
