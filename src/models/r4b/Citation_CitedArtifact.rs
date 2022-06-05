#![allow(unused_imports, non_camel_case_types)]

use crate::models::r4b::Annotation::Annotation;
use crate::models::r4b::Citation_Abstract::Citation_Abstract;
use crate::models::r4b::Citation_Classification1::Citation_Classification1;
use crate::models::r4b::Citation_Contributorship::Citation_Contributorship;
use crate::models::r4b::Citation_Part::Citation_Part;
use crate::models::r4b::Citation_PublicationForm::Citation_PublicationForm;
use crate::models::r4b::Citation_RelatesTo1::Citation_RelatesTo1;
use crate::models::r4b::Citation_StatusDate1::Citation_StatusDate1;
use crate::models::r4b::Citation_Title::Citation_Title;
use crate::models::r4b::Citation_Version::Citation_Version;
use crate::models::r4b::Citation_WebLocation::Citation_WebLocation;
use crate::models::r4b::CodeableConcept::CodeableConcept;
use crate::models::r4b::Element::Element;
use crate::models::r4b::Extension::Extension;
use crate::models::r4b::Identifier::Identifier;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// The Citation Resource enables reference to any knowledge artifact for purposes
/// of identification and attribution. The Citation Resource supports existing
/// reference structures and developing publication practices such as versioning,
/// expressing complex contributorship roles, and referencing computable resources.

#[derive(Debug)]
pub struct Citation_CitedArtifact<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Citation_CitedArtifact<'_> {
    pub fn new(value: &Value) -> Citation_CitedArtifact {
        Citation_CitedArtifact {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for dateAccessed
    pub fn _date_accessed(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_dateAccessed") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Summary of the article or artifact.
    pub fn fhir_abstract(&self) -> Option<Vec<Citation_Abstract>> {
        if let Some(Value::Array(val)) = self.value.get("abstract") {
            return Some(
                val.into_iter()
                    .map(|e| Citation_Abstract {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The assignment to an organizing scheme.
    pub fn classification(&self) -> Option<Vec<Citation_Classification1>> {
        if let Some(Value::Array(val)) = self.value.get("classification") {
            return Some(
                val.into_iter()
                    .map(|e| Citation_Classification1 {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// This element is used to list authors and other contributors, their contact
    /// information, specific contributions, and summary statements.
    pub fn contributorship(&self) -> Option<Citation_Contributorship> {
        if let Some(val) = self.value.get("contributorship") {
            return Some(Citation_Contributorship {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The status of the cited artifact.
    pub fn current_state(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("currentState") {
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

    /// When the cited artifact was accessed.
    pub fn date_accessed(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("dateAccessed") {
            return Some(string);
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

    /// A formal identifier that is used to identify this citation when it is
    /// represented in other formats, or referenced in a specification, model, design or
    /// an instance.
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

    /// Any additional information or content for the article or artifact.
    pub fn note(&self) -> Option<Vec<Annotation>> {
        if let Some(Value::Array(val)) = self.value.get("note") {
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

    /// The component of the article or artifact.
    pub fn part(&self) -> Option<Citation_Part> {
        if let Some(val) = self.value.get("part") {
            return Some(Citation_Part {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// If multiple, used to represent alternative forms of the article that are not
    /// separate citations.
    pub fn publication_form(&self) -> Option<Vec<Citation_PublicationForm>> {
        if let Some(Value::Array(val)) = self.value.get("publicationForm") {
            return Some(
                val.into_iter()
                    .map(|e| Citation_PublicationForm {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A formal identifier that is used to identify things closely related to this
    /// citation.
    pub fn related_identifier(&self) -> Option<Vec<Identifier>> {
        if let Some(Value::Array(val)) = self.value.get("relatedIdentifier") {
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

    /// The artifact related to the cited artifact.
    pub fn relates_to(&self) -> Option<Vec<Citation_RelatesTo1>> {
        if let Some(Value::Array(val)) = self.value.get("relatesTo") {
            return Some(
                val.into_iter()
                    .map(|e| Citation_RelatesTo1 {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// An effective date or period for a status of the cited artifact.
    pub fn status_date(&self) -> Option<Vec<Citation_StatusDate1>> {
        if let Some(Value::Array(val)) = self.value.get("statusDate") {
            return Some(
                val.into_iter()
                    .map(|e| Citation_StatusDate1 {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The title details of the article or artifact.
    pub fn title(&self) -> Option<Vec<Citation_Title>> {
        if let Some(Value::Array(val)) = self.value.get("title") {
            return Some(
                val.into_iter()
                    .map(|e| Citation_Title {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The defined version of the cited artifact.
    pub fn version(&self) -> Option<Citation_Version> {
        if let Some(val) = self.value.get("version") {
            return Some(Citation_Version {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Used for any URL for the article or artifact cited.
    pub fn web_location(&self) -> Option<Vec<Citation_WebLocation>> {
        if let Some(Value::Array(val)) = self.value.get("webLocation") {
            return Some(
                val.into_iter()
                    .map(|e| Citation_WebLocation {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._date_accessed() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.fhir_abstract() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.classification() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.contributorship() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.current_state() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.date_accessed() {}
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.identifier() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.note() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.part() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.publication_form() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.related_identifier() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.relates_to() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.status_date() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.title() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.version() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.web_location() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct Citation_CitedArtifactBuilder {
    pub(crate) value: Value,
}

impl Citation_CitedArtifactBuilder {
    pub fn build(&self) -> Citation_CitedArtifact {
        Citation_CitedArtifact {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Citation_CitedArtifact) -> Citation_CitedArtifactBuilder {
        Citation_CitedArtifactBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> Citation_CitedArtifactBuilder {
        let mut __value: Value = json!({});
        return Citation_CitedArtifactBuilder { value: __value };
    }

    pub fn _date_accessed<'a>(&'a mut self, val: Element) -> &'a mut Citation_CitedArtifactBuilder {
        self.value["_dateAccessed"] = json!(val.value);
        return self;
    }

    pub fn fhir_abstract<'a>(
        &'a mut self,
        val: Vec<Citation_Abstract>,
    ) -> &'a mut Citation_CitedArtifactBuilder {
        self.value["abstract"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn classification<'a>(
        &'a mut self,
        val: Vec<Citation_Classification1>,
    ) -> &'a mut Citation_CitedArtifactBuilder {
        self.value["classification"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn contributorship<'a>(
        &'a mut self,
        val: Citation_Contributorship,
    ) -> &'a mut Citation_CitedArtifactBuilder {
        self.value["contributorship"] = json!(val.value);
        return self;
    }

    pub fn current_state<'a>(
        &'a mut self,
        val: Vec<CodeableConcept>,
    ) -> &'a mut Citation_CitedArtifactBuilder {
        self.value["currentState"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn date_accessed<'a>(&'a mut self, val: &str) -> &'a mut Citation_CitedArtifactBuilder {
        self.value["dateAccessed"] = json!(val);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut Citation_CitedArtifactBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut Citation_CitedArtifactBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn identifier<'a>(
        &'a mut self,
        val: Vec<Identifier>,
    ) -> &'a mut Citation_CitedArtifactBuilder {
        self.value["identifier"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut Citation_CitedArtifactBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn note<'a>(&'a mut self, val: Vec<Annotation>) -> &'a mut Citation_CitedArtifactBuilder {
        self.value["note"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn part<'a>(&'a mut self, val: Citation_Part) -> &'a mut Citation_CitedArtifactBuilder {
        self.value["part"] = json!(val.value);
        return self;
    }

    pub fn publication_form<'a>(
        &'a mut self,
        val: Vec<Citation_PublicationForm>,
    ) -> &'a mut Citation_CitedArtifactBuilder {
        self.value["publicationForm"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn related_identifier<'a>(
        &'a mut self,
        val: Vec<Identifier>,
    ) -> &'a mut Citation_CitedArtifactBuilder {
        self.value["relatedIdentifier"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn relates_to<'a>(
        &'a mut self,
        val: Vec<Citation_RelatesTo1>,
    ) -> &'a mut Citation_CitedArtifactBuilder {
        self.value["relatesTo"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn status_date<'a>(
        &'a mut self,
        val: Vec<Citation_StatusDate1>,
    ) -> &'a mut Citation_CitedArtifactBuilder {
        self.value["statusDate"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn title<'a>(
        &'a mut self,
        val: Vec<Citation_Title>,
    ) -> &'a mut Citation_CitedArtifactBuilder {
        self.value["title"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn version<'a>(
        &'a mut self,
        val: Citation_Version,
    ) -> &'a mut Citation_CitedArtifactBuilder {
        self.value["version"] = json!(val.value);
        return self;
    }

    pub fn web_location<'a>(
        &'a mut self,
        val: Vec<Citation_WebLocation>,
    ) -> &'a mut Citation_CitedArtifactBuilder {
        self.value["webLocation"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }
}
