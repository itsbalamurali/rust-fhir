#![allow(unused_imports, non_camel_case_types)]

use crate::models::r4b::CodeableConcept::CodeableConcept;
use crate::models::r4b::Element::Element;
use crate::models::r4b::Extension::Extension;
use crate::models::r4b::Narrative::Narrative;
use crate::models::r4b::Quantity::Quantity;
use crate::models::r4b::Reference::Reference;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// The EvidenceReport Resource is a specialized container for a collection of
/// resources and codable concepts, adapted to support compositions of Evidence,
/// EvidenceVariable, and Citation resources and related concepts.

#[derive(Debug)]
pub struct EvidenceReport_Section<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl EvidenceReport_Section<'_> {
    pub fn new(value: &Value) -> EvidenceReport_Section {
        EvidenceReport_Section {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for mode
    pub fn _mode(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_mode") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for title
    pub fn _title(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_title") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Identifies who is responsible for the information in this section, not
    /// necessarily who typed it in.
    pub fn author(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("author") {
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

    /// If the section is empty, why the list is empty. An empty section typically has
    /// some text explaining the empty reason.
    pub fn empty_reason(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("emptyReason") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Specifies any type of classification of the evidence report.
    pub fn entry_classifier(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("entryClassifier") {
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

    /// Quantity as content.
    pub fn entry_quantity(&self) -> Option<Vec<Quantity>> {
        if let Some(Value::Array(val)) = self.value.get("entryQuantity") {
            return Some(
                val.into_iter()
                    .map(|e| Quantity {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A reference to the actual resource from which the narrative in the section is
    /// derived.
    pub fn entry_reference(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("entryReference") {
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

    /// A code identifying the kind of content contained within the section. This should
    /// be consistent with the section title.
    pub fn focus(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("focus") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A definitional Resource identifying the kind of content contained within the
    /// section. This should be consistent with the section title.
    pub fn focus_reference(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("focusReference") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
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

    /// How the entry list was prepared - whether it is a working list that is suitable
    /// for being maintained on an ongoing basis, or if it represents a snapshot of a
    /// list of items from another source, or whether it is a prepared list where items
    /// may be marked as added, modified or deleted.
    pub fn mode(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("mode") {
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

    /// Specifies the order applied to the items in the section entries.
    pub fn ordered_by(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("orderedBy") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A nested sub-section within this section.
    pub fn section(&self) -> Option<Vec<EvidenceReport_Section>> {
        if let Some(Value::Array(val)) = self.value.get("section") {
            return Some(
                val.into_iter()
                    .map(|e| EvidenceReport_Section {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A human-readable narrative that contains the attested content of the section,
    /// used to represent the content of the resource to a human. The narrative need not
    /// encode all the structured data, but is peferred to contain sufficient detail to
    /// make it acceptable for a human to just read the narrative.
    pub fn text(&self) -> Option<Narrative> {
        if let Some(val) = self.value.get("text") {
            return Some(Narrative {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The label for this particular section.  This will be part of the rendered
    /// content for the document, and is often used to build a table of contents.
    pub fn title(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("title") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._mode() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._title() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.author() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.empty_reason() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.entry_classifier() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.entry_quantity() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.entry_reference() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.focus() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.focus_reference() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.mode() {}
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.ordered_by() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.section() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.text() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.title() {}
        return true;
    }
}

#[derive(Debug)]
pub struct EvidenceReport_SectionBuilder {
    pub(crate) value: Value,
}

impl EvidenceReport_SectionBuilder {
    pub fn build(&self) -> EvidenceReport_Section {
        EvidenceReport_Section {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: EvidenceReport_Section) -> EvidenceReport_SectionBuilder {
        EvidenceReport_SectionBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> EvidenceReport_SectionBuilder {
        let mut __value: Value = json!({});
        return EvidenceReport_SectionBuilder { value: __value };
    }

    pub fn _mode<'a>(&'a mut self, val: Element) -> &'a mut EvidenceReport_SectionBuilder {
        self.value["_mode"] = json!(val.value);
        return self;
    }

    pub fn _title<'a>(&'a mut self, val: Element) -> &'a mut EvidenceReport_SectionBuilder {
        self.value["_title"] = json!(val.value);
        return self;
    }

    pub fn author<'a>(&'a mut self, val: Vec<Reference>) -> &'a mut EvidenceReport_SectionBuilder {
        self.value["author"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn empty_reason<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut EvidenceReport_SectionBuilder {
        self.value["emptyReason"] = json!(val.value);
        return self;
    }

    pub fn entry_classifier<'a>(
        &'a mut self,
        val: Vec<CodeableConcept>,
    ) -> &'a mut EvidenceReport_SectionBuilder {
        self.value["entryClassifier"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn entry_quantity<'a>(
        &'a mut self,
        val: Vec<Quantity>,
    ) -> &'a mut EvidenceReport_SectionBuilder {
        self.value["entryQuantity"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn entry_reference<'a>(
        &'a mut self,
        val: Vec<Reference>,
    ) -> &'a mut EvidenceReport_SectionBuilder {
        self.value["entryReference"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut EvidenceReport_SectionBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn focus<'a>(&'a mut self, val: CodeableConcept) -> &'a mut EvidenceReport_SectionBuilder {
        self.value["focus"] = json!(val.value);
        return self;
    }

    pub fn focus_reference<'a>(
        &'a mut self,
        val: Reference,
    ) -> &'a mut EvidenceReport_SectionBuilder {
        self.value["focusReference"] = json!(val.value);
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut EvidenceReport_SectionBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn mode<'a>(&'a mut self, val: &str) -> &'a mut EvidenceReport_SectionBuilder {
        self.value["mode"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut EvidenceReport_SectionBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn ordered_by<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut EvidenceReport_SectionBuilder {
        self.value["orderedBy"] = json!(val.value);
        return self;
    }

    pub fn section<'a>(
        &'a mut self,
        val: Vec<EvidenceReport_Section>,
    ) -> &'a mut EvidenceReport_SectionBuilder {
        self.value["section"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn text<'a>(&'a mut self, val: Narrative) -> &'a mut EvidenceReport_SectionBuilder {
        self.value["text"] = json!(val.value);
        return self;
    }

    pub fn title<'a>(&'a mut self, val: &str) -> &'a mut EvidenceReport_SectionBuilder {
        self.value["title"] = json!(val);
        return self;
    }
}
