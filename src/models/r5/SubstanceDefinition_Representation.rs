#![allow(unused_imports, non_camel_case_types)]

use crate::models::r5::CodeableConcept::CodeableConcept;
use crate::models::r5::Element::Element;
use crate::models::r5::Extension::Extension;
use crate::models::r5::Reference::Reference;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// The detailed description of a substance, typically at a level beyond what is used
/// for prescribing.

#[derive(Debug)]
pub struct SubstanceDefinition_Representation<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl SubstanceDefinition_Representation<'_> {
    pub fn new(value: &Value) -> SubstanceDefinition_Representation {
        SubstanceDefinition_Representation {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for representation
    pub fn _representation(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_representation") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// An attached file with the structural representation or characterization e.g. a
    /// molecular structure graphic of the substance, a JCAMP or AnIML file.
    pub fn document(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("document") {
            return Some(Reference {
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

    /// The format of the representation e.g. InChI, SMILES, MOLFILE, CDX, SDF, PDB,
    /// mmCIF. The logical content type rather than the physical file format of a
    /// document.
    pub fn format(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("format") {
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

    /// The structural representation or characterization as a text string in a standard
    /// format.
    pub fn representation(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("representation") {
            return Some(string);
        }
        return None;
    }

    /// The kind of structural representation (e.g. full, partial).
    pub fn fhir_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("type") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._representation() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.document() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.format() {
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
        if let Some(_val) = self.representation() {}
        if let Some(_val) = self.fhir_type() {
            if !_val.validate() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct SubstanceDefinition_RepresentationBuilder {
    pub(crate) value: Value,
}

impl SubstanceDefinition_RepresentationBuilder {
    pub fn build(&self) -> SubstanceDefinition_Representation {
        SubstanceDefinition_Representation {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(
        existing: SubstanceDefinition_Representation,
    ) -> SubstanceDefinition_RepresentationBuilder {
        SubstanceDefinition_RepresentationBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> SubstanceDefinition_RepresentationBuilder {
        let mut __value: Value = json!({});
        return SubstanceDefinition_RepresentationBuilder { value: __value };
    }

    pub fn _representation<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut SubstanceDefinition_RepresentationBuilder {
        self.value["_representation"] = json!(val.value);
        return self;
    }

    pub fn document<'a>(
        &'a mut self,
        val: Reference,
    ) -> &'a mut SubstanceDefinition_RepresentationBuilder {
        self.value["document"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut SubstanceDefinition_RepresentationBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn format<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut SubstanceDefinition_RepresentationBuilder {
        self.value["format"] = json!(val.value);
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut SubstanceDefinition_RepresentationBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut SubstanceDefinition_RepresentationBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn representation<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut SubstanceDefinition_RepresentationBuilder {
        self.value["representation"] = json!(val);
        return self;
    }

    pub fn fhir_type<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut SubstanceDefinition_RepresentationBuilder {
        self.value["type"] = json!(val.value);
        return self;
    }
}
