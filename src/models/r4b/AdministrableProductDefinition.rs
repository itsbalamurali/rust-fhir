#![allow(unused_imports, non_camel_case_types)]

use crate::models::r4b::AdministrableProductDefinition_Property::AdministrableProductDefinition_Property;
use crate::models::r4b::AdministrableProductDefinition_RouteOfAdministration::AdministrableProductDefinition_RouteOfAdministration;
use crate::models::r4b::CodeableConcept::CodeableConcept;
use crate::models::r4b::Element::Element;
use crate::models::r4b::Extension::Extension;
use crate::models::r4b::Identifier::Identifier;
use crate::models::r4b::Meta::Meta;
use crate::models::r4b::Narrative::Narrative;
use crate::models::r4b::Reference::Reference;
use crate::models::r4b::ResourceList::ResourceList;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A medicinal product in the final form which is suitable for administering to a
/// patient (after any mixing of multiple components, dissolution etc. has been
/// performed).

#[derive(Debug)]
pub struct AdministrableProductDefinition<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl AdministrableProductDefinition<'_> {
    pub fn new(value: &Value) -> AdministrableProductDefinition {
        AdministrableProductDefinition {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
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

    /// Extensions for status
    pub fn _status(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_status") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The dose form of the final product after necessary reconstitution or processing.
    /// Contrasts to the manufactured dose form (see ManufacturedItemDefinition). If the
    /// manufactured form was 'powder for solution for injection', the administrable
    /// dose form could be 'solution for injection' (once mixed with another item having
    /// manufactured form 'solvent for solution for injection').
    pub fn administrable_dose_form(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("administrableDoseForm") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, and nor can they
    /// have their own independent transaction scope.
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

    /// A device that is integral to the medicinal product, in effect being considered
    /// as an "ingredient" of the medicinal product. This is not intended for devices
    /// that are just co-packaged.
    pub fn device(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("device") {
            return Some(Reference {
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

    /// References a product from which one or more of the constituent parts of that
    /// product can be prepared and used as described by this administrable product.  If
    /// this administrable product describes the administration of a crushed tablet, the
    /// 'formOf' would be the product representing a distribution containing tablets and
    /// possibly also a cream.  This is distinct from the 'producedFrom' which refers to
    /// the specific components of the product that are used in this preparation, rather
    /// than the product as a whole.
    pub fn form_of(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("formOf") {
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

    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
        }
        return None;
    }

    /// An identifier for the administrable product.
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

    /// The ingredients of this administrable medicinal product. This is only needed if
    /// the ingredients are not specified either using ManufacturedItemDefiniton (via
    /// AdministrableProductDefinition.producedFrom) to state which component items are
    /// used to make this, or using by incoming references from the Ingredient resource,
    /// to state in detail which substances exist within this. This element allows a
    /// basic coded ingredient to be used.
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

    /// Indicates the specific manufactured items that are part of the 'formOf' product
    /// that are used in the preparation of this specific administrable form.  In some
    /// cases, an administrable form might use all of the items from the overall product
    /// (or there might only be one item), while in other cases, an administrable form
    /// might use only a subset of the items available in the overall product.  For
    /// example, an administrable form might involve combining a liquid and a powder
    /// available as part of an overall product, but not involve applying the also
    /// supplied cream.
    pub fn produced_from(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("producedFrom") {
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

    /// Characteristics e.g. a product's onset of action.
    pub fn property(&self) -> Option<Vec<AdministrableProductDefinition_Property>> {
        if let Some(Value::Array(val)) = self.value.get("property") {
            return Some(
                val.into_iter()
                    .map(|e| AdministrableProductDefinition_Property {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The path by which the product is taken into or makes contact with the body. In
    /// some regions this is referred to as the licenced or approved route.
    /// RouteOfAdministration cannot be used when the 'formOf' product already uses
    /// MedicinalProductDefinition.route (and vice versa).
    pub fn route_of_administration(
        &self,
    ) -> Vec<AdministrableProductDefinition_RouteOfAdministration> {
        self.value
            .get("routeOfAdministration")
            .unwrap()
            .as_array()
            .unwrap()
            .into_iter()
            .map(|e| AdministrableProductDefinition_RouteOfAdministration {
                value: Cow::Borrowed(e),
            })
            .collect::<Vec<_>>()
    }

    /// The status of this administrable product. Enables tracking the life-cycle of the
    /// content.
    pub fn status(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("status") {
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

    /// The presentation type in which this item is given to a patient. e.g. for a spray
    /// - 'puff' (as in 'contains 100 mcg per puff'), or for a liquid - 'vial' (as in
    /// 'contains 5 ml per vial').
    pub fn unit_of_presentation(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("unitOfPresentation") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
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
        if let Some(_val) = self._status() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.administrable_dose_form() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.contained() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.device() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.form_of() {
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
        if let Some(_val) = self.ingredient() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
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
        if let Some(_val) = self.produced_from() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.property() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if !self
            .route_of_administration()
            .into_iter()
            .map(|e| e.validate())
            .all(|x| x == true)
        {
            return false;
        }
        if let Some(_val) = self.status() {}
        if let Some(_val) = self.text() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.unit_of_presentation() {
            if !_val.validate() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct AdministrableProductDefinitionBuilder {
    pub(crate) value: Value,
}

impl AdministrableProductDefinitionBuilder {
    pub fn build(&self) -> AdministrableProductDefinition {
        AdministrableProductDefinition {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: AdministrableProductDefinition) -> AdministrableProductDefinitionBuilder {
        AdministrableProductDefinitionBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(
        route_of_administration: Vec<AdministrableProductDefinition_RouteOfAdministration>,
    ) -> AdministrableProductDefinitionBuilder {
        let mut __value: Value = json!({});
        __value["routeOfAdministration"] = json!(route_of_administration
            .into_iter()
            .map(|e| e.value)
            .collect::<Vec<_>>());
        return AdministrableProductDefinitionBuilder { value: __value };
    }

    pub fn _implicit_rules<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut AdministrableProductDefinitionBuilder {
        self.value["_implicitRules"] = json!(val.value);
        return self;
    }

    pub fn _language<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut AdministrableProductDefinitionBuilder {
        self.value["_language"] = json!(val.value);
        return self;
    }

    pub fn _status<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut AdministrableProductDefinitionBuilder {
        self.value["_status"] = json!(val.value);
        return self;
    }

    pub fn administrable_dose_form<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut AdministrableProductDefinitionBuilder {
        self.value["administrableDoseForm"] = json!(val.value);
        return self;
    }

    pub fn contained<'a>(
        &'a mut self,
        val: Vec<ResourceList>,
    ) -> &'a mut AdministrableProductDefinitionBuilder {
        self.value["contained"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn device<'a>(
        &'a mut self,
        val: Reference,
    ) -> &'a mut AdministrableProductDefinitionBuilder {
        self.value["device"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut AdministrableProductDefinitionBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn form_of<'a>(
        &'a mut self,
        val: Vec<Reference>,
    ) -> &'a mut AdministrableProductDefinitionBuilder {
        self.value["formOf"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut AdministrableProductDefinitionBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn identifier<'a>(
        &'a mut self,
        val: Vec<Identifier>,
    ) -> &'a mut AdministrableProductDefinitionBuilder {
        self.value["identifier"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn implicit_rules<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut AdministrableProductDefinitionBuilder {
        self.value["implicitRules"] = json!(val);
        return self;
    }

    pub fn ingredient<'a>(
        &'a mut self,
        val: Vec<CodeableConcept>,
    ) -> &'a mut AdministrableProductDefinitionBuilder {
        self.value["ingredient"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn language<'a>(&'a mut self, val: &str) -> &'a mut AdministrableProductDefinitionBuilder {
        self.value["language"] = json!(val);
        return self;
    }

    pub fn meta<'a>(&'a mut self, val: Meta) -> &'a mut AdministrableProductDefinitionBuilder {
        self.value["meta"] = json!(val.value);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut AdministrableProductDefinitionBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn produced_from<'a>(
        &'a mut self,
        val: Vec<Reference>,
    ) -> &'a mut AdministrableProductDefinitionBuilder {
        self.value["producedFrom"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn property<'a>(
        &'a mut self,
        val: Vec<AdministrableProductDefinition_Property>,
    ) -> &'a mut AdministrableProductDefinitionBuilder {
        self.value["property"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn status<'a>(&'a mut self, val: &str) -> &'a mut AdministrableProductDefinitionBuilder {
        self.value["status"] = json!(val);
        return self;
    }

    pub fn text<'a>(&'a mut self, val: Narrative) -> &'a mut AdministrableProductDefinitionBuilder {
        self.value["text"] = json!(val.value);
        return self;
    }

    pub fn unit_of_presentation<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut AdministrableProductDefinitionBuilder {
        self.value["unitOfPresentation"] = json!(val.value);
        return self;
    }
}
