#![allow(unused_imports, non_camel_case_types)]

use crate::models::r4b::Annotation::Annotation;
use crate::models::r4b::CodeableConcept::CodeableConcept;
use crate::models::r4b::Element::Element;
use crate::models::r4b::Extension::Extension;
use crate::models::r4b::Identifier::Identifier;
use crate::models::r4b::Meta::Meta;
use crate::models::r4b::Narrative::Narrative;
use crate::models::r4b::Reference::Reference;
use crate::models::r4b::ResourceList::ResourceList;
use crate::models::r4b::SubstanceDefinition_Code::SubstanceDefinition_Code;
use crate::models::r4b::SubstanceDefinition_Moiety::SubstanceDefinition_Moiety;
use crate::models::r4b::SubstanceDefinition_MolecularWeight::SubstanceDefinition_MolecularWeight;
use crate::models::r4b::SubstanceDefinition_Name::SubstanceDefinition_Name;
use crate::models::r4b::SubstanceDefinition_Property::SubstanceDefinition_Property;
use crate::models::r4b::SubstanceDefinition_Relationship::SubstanceDefinition_Relationship;
use crate::models::r4b::SubstanceDefinition_SourceMaterial::SubstanceDefinition_SourceMaterial;
use crate::models::r4b::SubstanceDefinition_Structure::SubstanceDefinition_Structure;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// The detailed description of a substance, typically at a level beyond what is used
/// for prescribing.

#[derive(Debug)]
pub struct SubstanceDefinition<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl SubstanceDefinition<'_> {
    pub fn new(value: &Value) -> SubstanceDefinition {
        SubstanceDefinition {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for description
    pub fn _description(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_description") {
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

    /// Extensions for language
    pub fn _language(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_language") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for version
    pub fn _version(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_version") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A high level categorization, e.g. polymer or nucleic acid, or food, chemical,
    /// biological, or a lower level such as the general types of polymer (linear or
    /// branch chain) or type of impurity (process related or contaminant).
    pub fn classification(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("classification") {
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

    /// Codes associated with the substance.
    pub fn code(&self) -> Option<Vec<SubstanceDefinition_Code>> {
        if let Some(Value::Array(val)) = self.value.get("code") {
            return Some(
                val.into_iter()
                    .map(|e| SubstanceDefinition_Code {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// These resources do not have an independent existence apart from the resource that
    /// contains them - they cannot be identified independently, and nor can they have
    /// their own independent transaction scope.
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

    /// Textual description of the substance.
    pub fn description(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("description") {
            return Some(string);
        }
        return None;
    }

    /// If the substance applies to human or veterinary use.
    pub fn domain(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("domain") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
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

    /// The quality standard, established benchmark, to which substance complies (e.g.
    /// USP/NF, Ph. Eur, JP, BP, Company Standard).
    pub fn grade(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("grade") {
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

    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
        }
        return None;
    }

    /// Identifier by which this substance is known.
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

    /// Supporting literature.
    pub fn information_source(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("informationSource") {
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

    /// The base language in which the resource is written.
    pub fn language(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("language") {
            return Some(string);
        }
        return None;
    }

    /// The entity that creates, makes, produces or fabricates the substance. This is a
    /// set of potential manufacturers but is not necessarily comprehensive.
    pub fn manufacturer(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("manufacturer") {
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

    /// Moiety, for structural modifications.
    pub fn moiety(&self) -> Option<Vec<SubstanceDefinition_Moiety>> {
        if let Some(Value::Array(val)) = self.value.get("moiety") {
            return Some(
                val.into_iter()
                    .map(|e| SubstanceDefinition_Moiety {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The molecular weight or weight range (for proteins, polymers or nucleic acids).
    pub fn molecular_weight(&self) -> Option<Vec<SubstanceDefinition_MolecularWeight>> {
        if let Some(Value::Array(val)) = self.value.get("molecularWeight") {
            return Some(
                val.into_iter()
                    .map(|e| SubstanceDefinition_MolecularWeight {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Names applicable to this substance.
    pub fn name(&self) -> Option<Vec<SubstanceDefinition_Name>> {
        if let Some(Value::Array(val)) = self.value.get("name") {
            return Some(
                val.into_iter()
                    .map(|e| SubstanceDefinition_Name {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Textual comment about the substance's catalogue or registry record.
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

    /// General specifications for this substance.
    pub fn property(&self) -> Option<Vec<SubstanceDefinition_Property>> {
        if let Some(Value::Array(val)) = self.value.get("property") {
            return Some(
                val.into_iter()
                    .map(|e| SubstanceDefinition_Property {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A link between this substance and another, with details of the relationship.
    pub fn relationship(&self) -> Option<Vec<SubstanceDefinition_Relationship>> {
        if let Some(Value::Array(val)) = self.value.get("relationship") {
            return Some(
                val.into_iter()
                    .map(|e| SubstanceDefinition_Relationship {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Material or taxonomic/anatomical source for the substance.
    pub fn source_material(&self) -> Option<SubstanceDefinition_SourceMaterial> {
        if let Some(val) = self.value.get("sourceMaterial") {
            return Some(SubstanceDefinition_SourceMaterial {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Status of substance within the catalogue e.g. active, retired.
    pub fn status(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("status") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Structural information.
    pub fn structure(&self) -> Option<SubstanceDefinition_Structure> {
        if let Some(val) = self.value.get("structure") {
            return Some(SubstanceDefinition_Structure {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// An entity that is the source for the substance. It may be different from the
    /// manufacturer. Supplier is synonymous to a distributor.
    pub fn supplier(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("supplier") {
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

    /// A business level version identifier of the substance.
    pub fn version(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("version") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._description() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._implicit_rules() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._language() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._version() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.classification() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.code() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.contained() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.description() {}
        if let Some(_val) = self.domain() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.grade() {
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
        if let Some(_val) = self.implicit_rules() {}
        if let Some(_val) = self.information_source() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.language() {}
        if let Some(_val) = self.manufacturer() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
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
        if let Some(_val) = self.moiety() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.molecular_weight() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.name() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.note() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.property() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.relationship() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.source_material() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.status() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.structure() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.supplier() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.text() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.version() {}
        return true;
    }
}

#[derive(Debug)]
pub struct SubstanceDefinitionBuilder {
    pub(crate) value: Value,
}

impl SubstanceDefinitionBuilder {
    pub fn build(&self) -> SubstanceDefinition {
        SubstanceDefinition {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: SubstanceDefinition) -> SubstanceDefinitionBuilder {
        SubstanceDefinitionBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> SubstanceDefinitionBuilder {
        let mut __value: Value = json!({});
        return SubstanceDefinitionBuilder { value: __value };
    }

    pub fn _description<'a>(&'a mut self, val: Element) -> &'a mut SubstanceDefinitionBuilder {
        self.value["_description"] = json!(val.value);
        return self;
    }

    pub fn _implicit_rules<'a>(&'a mut self, val: Element) -> &'a mut SubstanceDefinitionBuilder {
        self.value["_implicitRules"] = json!(val.value);
        return self;
    }

    pub fn _language<'a>(&'a mut self, val: Element) -> &'a mut SubstanceDefinitionBuilder {
        self.value["_language"] = json!(val.value);
        return self;
    }

    pub fn _version<'a>(&'a mut self, val: Element) -> &'a mut SubstanceDefinitionBuilder {
        self.value["_version"] = json!(val.value);
        return self;
    }

    pub fn classification<'a>(
        &'a mut self,
        val: Vec<CodeableConcept>,
    ) -> &'a mut SubstanceDefinitionBuilder {
        self.value["classification"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn code<'a>(
        &'a mut self,
        val: Vec<SubstanceDefinition_Code>,
    ) -> &'a mut SubstanceDefinitionBuilder {
        self.value["code"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn contained<'a>(
        &'a mut self,
        val: Vec<ResourceList>,
    ) -> &'a mut SubstanceDefinitionBuilder {
        self.value["contained"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn description<'a>(&'a mut self, val: &str) -> &'a mut SubstanceDefinitionBuilder {
        self.value["description"] = json!(val);
        return self;
    }

    pub fn domain<'a>(&'a mut self, val: CodeableConcept) -> &'a mut SubstanceDefinitionBuilder {
        self.value["domain"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut SubstanceDefinitionBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn grade<'a>(
        &'a mut self,
        val: Vec<CodeableConcept>,
    ) -> &'a mut SubstanceDefinitionBuilder {
        self.value["grade"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut SubstanceDefinitionBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn identifier<'a>(
        &'a mut self,
        val: Vec<Identifier>,
    ) -> &'a mut SubstanceDefinitionBuilder {
        self.value["identifier"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn implicit_rules<'a>(&'a mut self, val: &str) -> &'a mut SubstanceDefinitionBuilder {
        self.value["implicitRules"] = json!(val);
        return self;
    }

    pub fn information_source<'a>(
        &'a mut self,
        val: Vec<Reference>,
    ) -> &'a mut SubstanceDefinitionBuilder {
        self.value["informationSource"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn language<'a>(&'a mut self, val: &str) -> &'a mut SubstanceDefinitionBuilder {
        self.value["language"] = json!(val);
        return self;
    }

    pub fn manufacturer<'a>(
        &'a mut self,
        val: Vec<Reference>,
    ) -> &'a mut SubstanceDefinitionBuilder {
        self.value["manufacturer"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn meta<'a>(&'a mut self, val: Meta) -> &'a mut SubstanceDefinitionBuilder {
        self.value["meta"] = json!(val.value);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut SubstanceDefinitionBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn moiety<'a>(
        &'a mut self,
        val: Vec<SubstanceDefinition_Moiety>,
    ) -> &'a mut SubstanceDefinitionBuilder {
        self.value["moiety"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn molecular_weight<'a>(
        &'a mut self,
        val: Vec<SubstanceDefinition_MolecularWeight>,
    ) -> &'a mut SubstanceDefinitionBuilder {
        self.value["molecularWeight"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn name<'a>(
        &'a mut self,
        val: Vec<SubstanceDefinition_Name>,
    ) -> &'a mut SubstanceDefinitionBuilder {
        self.value["name"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn note<'a>(&'a mut self, val: Vec<Annotation>) -> &'a mut SubstanceDefinitionBuilder {
        self.value["note"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn property<'a>(
        &'a mut self,
        val: Vec<SubstanceDefinition_Property>,
    ) -> &'a mut SubstanceDefinitionBuilder {
        self.value["property"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn relationship<'a>(
        &'a mut self,
        val: Vec<SubstanceDefinition_Relationship>,
    ) -> &'a mut SubstanceDefinitionBuilder {
        self.value["relationship"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn source_material<'a>(
        &'a mut self,
        val: SubstanceDefinition_SourceMaterial,
    ) -> &'a mut SubstanceDefinitionBuilder {
        self.value["sourceMaterial"] = json!(val.value);
        return self;
    }

    pub fn status<'a>(&'a mut self, val: CodeableConcept) -> &'a mut SubstanceDefinitionBuilder {
        self.value["status"] = json!(val.value);
        return self;
    }

    pub fn structure<'a>(
        &'a mut self,
        val: SubstanceDefinition_Structure,
    ) -> &'a mut SubstanceDefinitionBuilder {
        self.value["structure"] = json!(val.value);
        return self;
    }

    pub fn supplier<'a>(&'a mut self, val: Vec<Reference>) -> &'a mut SubstanceDefinitionBuilder {
        self.value["supplier"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn text<'a>(&'a mut self, val: Narrative) -> &'a mut SubstanceDefinitionBuilder {
        self.value["text"] = json!(val.value);
        return self;
    }

    pub fn version<'a>(&'a mut self, val: &str) -> &'a mut SubstanceDefinitionBuilder {
        self.value["version"] = json!(val);
        return self;
    }
}
