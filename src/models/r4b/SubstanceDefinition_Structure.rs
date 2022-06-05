#![allow(unused_imports, non_camel_case_types)]

use crate::models::r4b::CodeableConcept::CodeableConcept;
use crate::models::r4b::Element::Element;
use crate::models::r4b::Extension::Extension;
use crate::models::r4b::Reference::Reference;
use crate::models::r4b::SubstanceDefinition_MolecularWeight::SubstanceDefinition_MolecularWeight;
use crate::models::r4b::SubstanceDefinition_Representation::SubstanceDefinition_Representation;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// The detailed description of a substance, typically at a level beyond what is used
/// for prescribing.

#[derive(Debug)]
pub struct SubstanceDefinition_Structure<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl SubstanceDefinition_Structure<'_> {
    pub fn new(value: &Value) -> SubstanceDefinition_Structure {
        SubstanceDefinition_Structure {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for molecularFormula
    pub fn _molecular_formula(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_molecularFormula") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for molecularFormulaByMoiety
    pub fn _molecular_formula_by_moiety(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_molecularFormulaByMoiety") {
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

    /// Molecular formula of this substance, typically using the Hill system.
    pub fn molecular_formula(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("molecularFormula") {
            return Some(string);
        }
        return None;
    }

    /// Specified per moiety according to the Hill system, i.e. first C, then H, then
    /// alphabetical, each moiety separated by a dot.
    pub fn molecular_formula_by_moiety(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("molecularFormulaByMoiety") {
            return Some(string);
        }
        return None;
    }

    /// The molecular weight or weight range (for proteins, polymers or nucleic acids).
    pub fn molecular_weight(&self) -> Option<SubstanceDefinition_MolecularWeight> {
        if let Some(val) = self.value.get("molecularWeight") {
            return Some(SubstanceDefinition_MolecularWeight {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Optical activity type.
    pub fn optical_activity(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("opticalActivity") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A depiction of the structure or characterization of the substance.
    pub fn representation(&self) -> Option<Vec<SubstanceDefinition_Representation>> {
        if let Some(Value::Array(val)) = self.value.get("representation") {
            return Some(
                val.into_iter()
                    .map(|e| SubstanceDefinition_Representation {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The source of information about the structure.
    pub fn source_document(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("sourceDocument") {
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

    /// Stereochemistry type.
    pub fn stereochemistry(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("stereochemistry") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The method used to elucidate the structure or characterization of the drug
    /// substance. Examples: X-ray, HPLC, NMR, Peptide mapping, Ligand binding assay.
    pub fn technique(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("technique") {
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

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._molecular_formula() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._molecular_formula_by_moiety() {
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
        if let Some(_val) = self.molecular_formula() {}
        if let Some(_val) = self.molecular_formula_by_moiety() {}
        if let Some(_val) = self.molecular_weight() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.optical_activity() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.representation() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.source_document() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.stereochemistry() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.technique() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct SubstanceDefinition_StructureBuilder {
    pub(crate) value: Value,
}

impl SubstanceDefinition_StructureBuilder {
    pub fn build(&self) -> SubstanceDefinition_Structure {
        SubstanceDefinition_Structure {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: SubstanceDefinition_Structure) -> SubstanceDefinition_StructureBuilder {
        SubstanceDefinition_StructureBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> SubstanceDefinition_StructureBuilder {
        let mut __value: Value = json!({});
        return SubstanceDefinition_StructureBuilder { value: __value };
    }

    pub fn _molecular_formula<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut SubstanceDefinition_StructureBuilder {
        self.value["_molecularFormula"] = json!(val.value);
        return self;
    }

    pub fn _molecular_formula_by_moiety<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut SubstanceDefinition_StructureBuilder {
        self.value["_molecularFormulaByMoiety"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut SubstanceDefinition_StructureBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut SubstanceDefinition_StructureBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut SubstanceDefinition_StructureBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn molecular_formula<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut SubstanceDefinition_StructureBuilder {
        self.value["molecularFormula"] = json!(val);
        return self;
    }

    pub fn molecular_formula_by_moiety<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut SubstanceDefinition_StructureBuilder {
        self.value["molecularFormulaByMoiety"] = json!(val);
        return self;
    }

    pub fn molecular_weight<'a>(
        &'a mut self,
        val: SubstanceDefinition_MolecularWeight,
    ) -> &'a mut SubstanceDefinition_StructureBuilder {
        self.value["molecularWeight"] = json!(val.value);
        return self;
    }

    pub fn optical_activity<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut SubstanceDefinition_StructureBuilder {
        self.value["opticalActivity"] = json!(val.value);
        return self;
    }

    pub fn representation<'a>(
        &'a mut self,
        val: Vec<SubstanceDefinition_Representation>,
    ) -> &'a mut SubstanceDefinition_StructureBuilder {
        self.value["representation"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn source_document<'a>(
        &'a mut self,
        val: Vec<Reference>,
    ) -> &'a mut SubstanceDefinition_StructureBuilder {
        self.value["sourceDocument"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn stereochemistry<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut SubstanceDefinition_StructureBuilder {
        self.value["stereochemistry"] = json!(val.value);
        return self;
    }

    pub fn technique<'a>(
        &'a mut self,
        val: Vec<CodeableConcept>,
    ) -> &'a mut SubstanceDefinition_StructureBuilder {
        self.value["technique"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }
}
