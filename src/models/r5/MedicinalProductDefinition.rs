#![allow(unused_imports, non_camel_case_types)]

use crate::models::r5::CodeableConcept::CodeableConcept;
use crate::models::r5::CodeableReference::CodeableReference;
use crate::models::r5::Coding::Coding;
use crate::models::r5::Element::Element;
use crate::models::r5::Extension::Extension;
use crate::models::r5::Identifier::Identifier;
use crate::models::r5::MarketingStatus::MarketingStatus;
use crate::models::r5::MedicinalProductDefinition_Characteristic::MedicinalProductDefinition_Characteristic;
use crate::models::r5::MedicinalProductDefinition_Contact::MedicinalProductDefinition_Contact;
use crate::models::r5::MedicinalProductDefinition_CrossReference::MedicinalProductDefinition_CrossReference;
use crate::models::r5::MedicinalProductDefinition_Name::MedicinalProductDefinition_Name;
use crate::models::r5::MedicinalProductDefinition_Operation::MedicinalProductDefinition_Operation;
use crate::models::r5::Meta::Meta;
use crate::models::r5::Narrative::Narrative;
use crate::models::r5::Reference::Reference;
use crate::models::r5::ResourceList::ResourceList;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Detailed definition of a medicinal product, typically for uses other than direct
/// patient care (e.g. regulatory use, drug catalogs).

#[derive(Debug)]
pub struct MedicinalProductDefinition<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl MedicinalProductDefinition<'_> {
    pub fn new(value: &Value) -> MedicinalProductDefinition {
        MedicinalProductDefinition {
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

    /// Extensions for indication
    pub fn _indication(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_indication") {
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

    /// Extensions for statusDate
    pub fn _status_date(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_statusDate") {
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

    /// Whether the Medicinal Product is subject to additional monitoring for regulatory
    /// reasons.
    pub fn additional_monitoring_indicator(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("additionalMonitoringIndicator") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Additional information or supporting documentation about the medicinal product.
    pub fn attached_document(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("attachedDocument") {
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

    /// Allows the key product features to be recorded, such as "sugar free", "modified
    /// release", "parallel import".
    pub fn characteristic(&self) -> Option<Vec<MedicinalProductDefinition_Characteristic>> {
        if let Some(Value::Array(val)) = self.value.get("characteristic") {
            return Some(
                val.into_iter()
                    .map(|e| MedicinalProductDefinition_Characteristic {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Allows the product to be classified by various systems.
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

    /// Clinical trials or studies that this product is involved in.
    pub fn clinical_trial(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("clinicalTrial") {
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

    /// A code that this product is known by, usually within some formal terminology.
    /// Products (types of medications) tend to be known by identifiers during
    /// development and within regulatory process. However when they are prescribed they
    /// tend to be identified by codes. The same product may be have multiple codes,
    /// applied to it by multiple organizations.
    pub fn code(&self) -> Option<Vec<Coding>> {
        if let Some(Value::Array(val)) = self.value.get("code") {
            return Some(
                val.into_iter()
                    .map(|e| Coding {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The dose form for a single part product, or combined form of a multiple part
    /// product.
    pub fn combined_pharmaceutical_dose_form(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("combinedPharmaceuticalDoseForm") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A product specific contact, person (in a role), or an organization.
    pub fn contact(&self) -> Option<Vec<MedicinalProductDefinition_Contact>> {
        if let Some(Value::Array(val)) = self.value.get("contact") {
            return Some(
                val.into_iter()
                    .map(|e| MedicinalProductDefinition_Contact {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
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

    /// Reference to another product, e.g. for linking authorised to investigational
    /// product.
    pub fn cross_reference(&self) -> Option<Vec<MedicinalProductDefinition_CrossReference>> {
        if let Some(Value::Array(val)) = self.value.get("crossReference") {
            return Some(
                val.into_iter()
                    .map(|e| MedicinalProductDefinition_CrossReference {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// General description of this product.
    pub fn description(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("description") {
            return Some(string);
        }
        return None;
    }

    /// If this medicine applies to human or veterinary uses.
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

    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
        }
        return None;
    }

    /// Business identifier for this product. Could be an MPID.
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
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub fn implicit_rules(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("implicitRules") {
            return Some(string);
        }
        return None;
    }

    /// Any component of the drug product which is not the chemical entity defined as
    /// the drug substance or an excipient in the drug product. This includes process-
    /// related impurities and contaminants, product-related impurities
    /// including degradation products.
    pub fn impurity(&self) -> Option<Vec<CodeableReference>> {
        if let Some(Value::Array(val)) = self.value.get("impurity") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableReference {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Description of indication(s) for this product, used when structured indications
    /// are not required. In cases where structured indications are required, they are
    /// captured using the ClinicalUseDefinition resource. An indication is a medical
    /// situation for which using the product is appropriate.
    pub fn indication(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("indication") {
            return Some(string);
        }
        return None;
    }

    /// The ingredients of this medicinal product - when not detailed in other
    /// resources. This is only needed if the ingredients are not specified by incoming
    /// references from the Ingredient resource, or indirectly via incoming
    /// AdministrableProductDefinition, PackagedProductDefinition or
    /// ManufacturedItemDefinition references. In cases where those levels of detail are
    /// not used, the ingredients may be specified directly here as codes.
    pub fn ingredient(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("ingredient") {
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

    /// The base language in which the resource is written.
    pub fn language(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("language") {
            return Some(string);
        }
        return None;
    }

    /// The legal status of supply of the medicinal product as classified by the
    /// regulator.
    pub fn legal_status_of_supply(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("legalStatusOfSupply") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Marketing status of the medicinal product, in contrast to marketing
    /// authorization.
    pub fn marketing_status(&self) -> Option<Vec<MarketingStatus>> {
        if let Some(Value::Array(val)) = self.value.get("marketingStatus") {
            return Some(
                val.into_iter()
                    .map(|e| MarketingStatus {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A master file for the medicinal product (e.g. Pharmacovigilance System Master
    /// File). Drug master files (DMFs) are documents submitted to regulatory agencies
    /// to provide confidential detailed information about facilities, processes or
    /// articles used in the manufacturing, processing, packaging and storing of drug
    /// products.
    pub fn master_file(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("masterFile") {
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
    /// infrastructure. Changes to the content might not always be associated with
    /// version changes to the resource.
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
    /// that contains it and/or the understanding of the containing element's
    /// descendants. Usually modifier elements provide negation or qualification. To
    /// make the use of extensions safe and manageable, there is a strict set of
    /// governance applied to the definition and use of extensions. Though any
    /// implementer is allowed to define an extension, there is a set of requirements
    /// that SHALL be met as part of the definition of the extension. Applications
    /// processing a resource are required to check for modifier extensions.    Modifier
    /// extensions SHALL NOT change the meaning of any elements on Resource or
    /// DomainResource (including cannot change the meaning of modifierExtension
    /// itself).
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

    /// The product's name, including full name and possibly coded parts.
    pub fn name(&self) -> Vec<MedicinalProductDefinition_Name> {
        self.value
            .get("name")
            .unwrap()
            .as_array()
            .unwrap()
            .into_iter()
            .map(|e| MedicinalProductDefinition_Name {
                value: Cow::Borrowed(e),
            })
            .collect::<Vec<_>>()
    }

    /// A manufacturing or administrative process or step associated with (or performed
    /// on) the medicinal product.
    pub fn operation(&self) -> Option<Vec<MedicinalProductDefinition_Operation>> {
        if let Some(Value::Array(val)) = self.value.get("operation") {
            return Some(
                val.into_iter()
                    .map(|e| MedicinalProductDefinition_Operation {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Package representation for the product. See also the PackagedProductDefinition
    /// resource.
    pub fn packaged_medicinal_product(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("packagedMedicinalProduct") {
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

    /// If authorised for use in children.
    pub fn pediatric_use_indicator(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("pediatricUseIndicator") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The path by which the product is taken into or makes contact with the body. In
    /// some regions this is referred to as the licenced or approved route. See also
    /// AdministrableProductDefinition resource.
    pub fn route(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("route") {
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

    /// Whether the Medicinal Product is subject to special measures for regulatory
    /// reasons.
    pub fn special_measures(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("specialMeasures") {
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

    /// The status within the lifecycle of this product record. A high-level status,
    /// this is not intended to duplicate details carried elsewhere such as legal
    /// status, or authorization status.
    pub fn status(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("status") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The date at which the given status became applicable.
    pub fn status_date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("statusDate") {
            return Some(string);
        }
        return None;
    }

    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need not
    /// encode all the structured data, but is required to contain sufficient detail to
    /// make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub fn text(&self) -> Option<Narrative> {
        if let Some(val) = self.value.get("text") {
            return Some(Narrative {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Regulatory type, e.g. Investigational or Authorized.
    pub fn fhir_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("type") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A business identifier relating to a specific version of the product, this is
    /// commonly used to support revisions to an existing product.
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
        if let Some(_val) = self._indication() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._language() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._status_date() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._version() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.additional_monitoring_indicator() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.attached_document() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.characteristic() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.classification() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.clinical_trial() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.code() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.combined_pharmaceutical_dose_form() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.contact() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.contained() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.cross_reference() {
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
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.identifier() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.implicit_rules() {}
        if let Some(_val) = self.impurity() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.indication() {}
        if let Some(_val) = self.ingredient() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.language() {}
        if let Some(_val) = self.legal_status_of_supply() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.marketing_status() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.master_file() {
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
        if !self
            .name()
            .into_iter()
            .map(|e| e.validate())
            .all(|x| x == true)
        {
            return false;
        }
        if let Some(_val) = self.operation() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.packaged_medicinal_product() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.pediatric_use_indicator() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.route() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.special_measures() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.status() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.status_date() {}
        if let Some(_val) = self.text() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.fhir_type() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.version() {}
        return true;
    }
}

#[derive(Debug)]
pub struct MedicinalProductDefinitionBuilder {
    pub(crate) value: Value,
}

impl MedicinalProductDefinitionBuilder {
    pub fn build(&self) -> MedicinalProductDefinition {
        MedicinalProductDefinition {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: MedicinalProductDefinition) -> MedicinalProductDefinitionBuilder {
        MedicinalProductDefinitionBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(name: Vec<MedicinalProductDefinition_Name>) -> MedicinalProductDefinitionBuilder {
        let mut __value: Value = json!({});
        __value["name"] = json!(name.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return MedicinalProductDefinitionBuilder { value: __value };
    }

    pub fn _description<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut MedicinalProductDefinitionBuilder {
        self.value["_description"] = json!(val.value);
        return self;
    }

    pub fn _implicit_rules<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut MedicinalProductDefinitionBuilder {
        self.value["_implicitRules"] = json!(val.value);
        return self;
    }

    pub fn _indication<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut MedicinalProductDefinitionBuilder {
        self.value["_indication"] = json!(val.value);
        return self;
    }

    pub fn _language<'a>(&'a mut self, val: Element) -> &'a mut MedicinalProductDefinitionBuilder {
        self.value["_language"] = json!(val.value);
        return self;
    }

    pub fn _status_date<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut MedicinalProductDefinitionBuilder {
        self.value["_statusDate"] = json!(val.value);
        return self;
    }

    pub fn _version<'a>(&'a mut self, val: Element) -> &'a mut MedicinalProductDefinitionBuilder {
        self.value["_version"] = json!(val.value);
        return self;
    }

    pub fn additional_monitoring_indicator<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut MedicinalProductDefinitionBuilder {
        self.value["additionalMonitoringIndicator"] = json!(val.value);
        return self;
    }

    pub fn attached_document<'a>(
        &'a mut self,
        val: Vec<Reference>,
    ) -> &'a mut MedicinalProductDefinitionBuilder {
        self.value["attachedDocument"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn characteristic<'a>(
        &'a mut self,
        val: Vec<MedicinalProductDefinition_Characteristic>,
    ) -> &'a mut MedicinalProductDefinitionBuilder {
        self.value["characteristic"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn classification<'a>(
        &'a mut self,
        val: Vec<CodeableConcept>,
    ) -> &'a mut MedicinalProductDefinitionBuilder {
        self.value["classification"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn clinical_trial<'a>(
        &'a mut self,
        val: Vec<Reference>,
    ) -> &'a mut MedicinalProductDefinitionBuilder {
        self.value["clinicalTrial"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn code<'a>(&'a mut self, val: Vec<Coding>) -> &'a mut MedicinalProductDefinitionBuilder {
        self.value["code"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn combined_pharmaceutical_dose_form<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut MedicinalProductDefinitionBuilder {
        self.value["combinedPharmaceuticalDoseForm"] = json!(val.value);
        return self;
    }

    pub fn contact<'a>(
        &'a mut self,
        val: Vec<MedicinalProductDefinition_Contact>,
    ) -> &'a mut MedicinalProductDefinitionBuilder {
        self.value["contact"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn contained<'a>(
        &'a mut self,
        val: Vec<ResourceList>,
    ) -> &'a mut MedicinalProductDefinitionBuilder {
        self.value["contained"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn cross_reference<'a>(
        &'a mut self,
        val: Vec<MedicinalProductDefinition_CrossReference>,
    ) -> &'a mut MedicinalProductDefinitionBuilder {
        self.value["crossReference"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn description<'a>(&'a mut self, val: &str) -> &'a mut MedicinalProductDefinitionBuilder {
        self.value["description"] = json!(val);
        return self;
    }

    pub fn domain<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut MedicinalProductDefinitionBuilder {
        self.value["domain"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut MedicinalProductDefinitionBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut MedicinalProductDefinitionBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn identifier<'a>(
        &'a mut self,
        val: Vec<Identifier>,
    ) -> &'a mut MedicinalProductDefinitionBuilder {
        self.value["identifier"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn implicit_rules<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut MedicinalProductDefinitionBuilder {
        self.value["implicitRules"] = json!(val);
        return self;
    }

    pub fn impurity<'a>(
        &'a mut self,
        val: Vec<CodeableReference>,
    ) -> &'a mut MedicinalProductDefinitionBuilder {
        self.value["impurity"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn indication<'a>(&'a mut self, val: &str) -> &'a mut MedicinalProductDefinitionBuilder {
        self.value["indication"] = json!(val);
        return self;
    }

    pub fn ingredient<'a>(
        &'a mut self,
        val: Vec<CodeableConcept>,
    ) -> &'a mut MedicinalProductDefinitionBuilder {
        self.value["ingredient"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn language<'a>(&'a mut self, val: &str) -> &'a mut MedicinalProductDefinitionBuilder {
        self.value["language"] = json!(val);
        return self;
    }

    pub fn legal_status_of_supply<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut MedicinalProductDefinitionBuilder {
        self.value["legalStatusOfSupply"] = json!(val.value);
        return self;
    }

    pub fn marketing_status<'a>(
        &'a mut self,
        val: Vec<MarketingStatus>,
    ) -> &'a mut MedicinalProductDefinitionBuilder {
        self.value["marketingStatus"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn master_file<'a>(
        &'a mut self,
        val: Vec<Reference>,
    ) -> &'a mut MedicinalProductDefinitionBuilder {
        self.value["masterFile"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn meta<'a>(&'a mut self, val: Meta) -> &'a mut MedicinalProductDefinitionBuilder {
        self.value["meta"] = json!(val.value);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut MedicinalProductDefinitionBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn operation<'a>(
        &'a mut self,
        val: Vec<MedicinalProductDefinition_Operation>,
    ) -> &'a mut MedicinalProductDefinitionBuilder {
        self.value["operation"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn packaged_medicinal_product<'a>(
        &'a mut self,
        val: Vec<CodeableConcept>,
    ) -> &'a mut MedicinalProductDefinitionBuilder {
        self.value["packagedMedicinalProduct"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn pediatric_use_indicator<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut MedicinalProductDefinitionBuilder {
        self.value["pediatricUseIndicator"] = json!(val.value);
        return self;
    }

    pub fn route<'a>(
        &'a mut self,
        val: Vec<CodeableConcept>,
    ) -> &'a mut MedicinalProductDefinitionBuilder {
        self.value["route"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn special_measures<'a>(
        &'a mut self,
        val: Vec<CodeableConcept>,
    ) -> &'a mut MedicinalProductDefinitionBuilder {
        self.value["specialMeasures"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn status<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut MedicinalProductDefinitionBuilder {
        self.value["status"] = json!(val.value);
        return self;
    }

    pub fn status_date<'a>(&'a mut self, val: &str) -> &'a mut MedicinalProductDefinitionBuilder {
        self.value["statusDate"] = json!(val);
        return self;
    }

    pub fn text<'a>(&'a mut self, val: Narrative) -> &'a mut MedicinalProductDefinitionBuilder {
        self.value["text"] = json!(val.value);
        return self;
    }

    pub fn fhir_type<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut MedicinalProductDefinitionBuilder {
        self.value["type"] = json!(val.value);
        return self;
    }

    pub fn version<'a>(&'a mut self, val: &str) -> &'a mut MedicinalProductDefinitionBuilder {
        self.value["version"] = json!(val);
        return self;
    }
}
