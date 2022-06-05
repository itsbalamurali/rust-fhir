#![allow(unused_imports, non_camel_case_types)]

use crate::models::r5::Annotation::Annotation;
use crate::models::r5::CodeableConcept::CodeableConcept;
use crate::models::r5::ContactPoint::ContactPoint;
use crate::models::r5::DeviceDefinition_ChargeItem::DeviceDefinition_ChargeItem;
use crate::models::r5::DeviceDefinition_Classification::DeviceDefinition_Classification;
use crate::models::r5::DeviceDefinition_CorrectiveAction::DeviceDefinition_CorrectiveAction;
use crate::models::r5::DeviceDefinition_DeviceName::DeviceDefinition_DeviceName;
use crate::models::r5::DeviceDefinition_Guideline::DeviceDefinition_Guideline;
use crate::models::r5::DeviceDefinition_HasPart::DeviceDefinition_HasPart;
use crate::models::r5::DeviceDefinition_Link::DeviceDefinition_Link;
use crate::models::r5::DeviceDefinition_Material::DeviceDefinition_Material;
use crate::models::r5::DeviceDefinition_Packaging::DeviceDefinition_Packaging;
use crate::models::r5::DeviceDefinition_Property::DeviceDefinition_Property;
use crate::models::r5::DeviceDefinition_UdiDeviceIdentifier::DeviceDefinition_UdiDeviceIdentifier;
use crate::models::r5::DeviceDefinition_Version::DeviceDefinition_Version;
use crate::models::r5::Element::Element;
use crate::models::r5::Extension::Extension;
use crate::models::r5::Identifier::Identifier;
use crate::models::r5::Meta::Meta;
use crate::models::r5::Narrative::Narrative;
use crate::models::r5::ProductShelfLife::ProductShelfLife;
use crate::models::r5::Reference::Reference;
use crate::models::r5::RelatedArtifact::RelatedArtifact;
use crate::models::r5::ResourceList::ResourceList;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// The characteristics, operational status and capabilities of a medical-related
/// component of a medical device.

#[derive(Debug)]
pub struct DeviceDefinition<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl DeviceDefinition<'_> {
    pub fn new(value: &Value) -> DeviceDefinition {
        DeviceDefinition {
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

    /// Extensions for manufacturerString
    pub fn _manufacturer_string(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_manufacturerString") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for modelNumber
    pub fn _model_number(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_modelNumber") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for partNumber
    pub fn _part_number(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_partNumber") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for productionIdentifierInUDI
    pub fn _production_identifier_in_u_d_i(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_productionIdentifierInUDI") {
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

    /// Billing code or reference associated with the device.
    pub fn charge_item(&self) -> Option<Vec<DeviceDefinition_ChargeItem>> {
        if let Some(Value::Array(val)) = self.value.get("chargeItem") {
            return Some(
                val.into_iter()
                    .map(|e| DeviceDefinition_ChargeItem {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// What kind of device or device system this is.
    pub fn classification(&self) -> Option<Vec<DeviceDefinition_Classification>> {
        if let Some(Value::Array(val)) = self.value.get("classification") {
            return Some(
                val.into_iter()
                    .map(|e| DeviceDefinition_Classification {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Contact details for an organization or a particular human that is responsible
    /// for the device.
    pub fn contact(&self) -> Option<Vec<ContactPoint>> {
        if let Some(Value::Array(val)) = self.value.get("contact") {
            return Some(
                val.into_iter()
                    .map(|e| ContactPoint {
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

    /// Tracking of latest field safety corrective action.
    pub fn corrective_action(&self) -> Option<DeviceDefinition_CorrectiveAction> {
        if let Some(val) = self.value.get("correctiveAction") {
            return Some(DeviceDefinition_CorrectiveAction {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Additional information to describe the device.
    pub fn description(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("description") {
            return Some(string);
        }
        return None;
    }

    /// The name or names of the device as given by the manufacturer.
    pub fn device_name(&self) -> Option<Vec<DeviceDefinition_DeviceName>> {
        if let Some(Value::Array(val)) = self.value.get("deviceName") {
            return Some(
                val.into_iter()
                    .map(|e| DeviceDefinition_DeviceName {
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

    /// Information aimed at providing directions for the usage of this model of device.
    pub fn guideline(&self) -> Option<DeviceDefinition_Guideline> {
        if let Some(val) = self.value.get("guideline") {
            return Some(DeviceDefinition_Guideline {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A device that is part (for example a component) of the present device.
    pub fn has_part(&self) -> Option<Vec<DeviceDefinition_HasPart>> {
        if let Some(Value::Array(val)) = self.value.get("hasPart") {
            return Some(
                val.into_iter()
                    .map(|e| DeviceDefinition_HasPart {
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

    /// Unique instance identifiers assigned to a device by the software, manufacturers,
    /// other organizations or owners. For example: handle ID. The identifier is
    /// typically valued if the udiDeviceIdentifier, partNumber or modelNumber is not
    /// valued and represents a different type of identifier.  However, it is
    /// permissible to still include those identifiers in DeviceDefinition.identifier
    /// with the appropriate identifier.type.
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

    /// The base language in which the resource is written.
    pub fn language(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("language") {
            return Some(string);
        }
        return None;
    }

    /// Language code for the human-readable text strings produced by the device (all
    /// supported).
    pub fn language_code(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("languageCode") {
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

    /// An associated device, attached to, used with, communicating with or linking a
    /// previous or new device model to the focal device.
    pub fn link(&self) -> Option<Vec<DeviceDefinition_Link>> {
        if let Some(Value::Array(val)) = self.value.get("link") {
            return Some(
                val.into_iter()
                    .map(|e| DeviceDefinition_Link {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A name of the manufacturer  or legal representative e.g. labeler. Whether this
    /// is the actual manufacturer or the labeler or responsible depends on
    /// implementation and jurisdiction.
    pub fn manufacturer_reference(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("manufacturerReference") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A name of the manufacturer  or legal representative e.g. labeler. Whether this
    /// is the actual manufacturer or the labeler or responsible depends on
    /// implementation and jurisdiction.
    pub fn manufacturer_string(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("manufacturerString") {
            return Some(string);
        }
        return None;
    }

    /// A substance used to create the material(s) of which the device is made.
    pub fn material(&self) -> Option<Vec<DeviceDefinition_Material>> {
        if let Some(Value::Array(val)) = self.value.get("material") {
            return Some(
                val.into_iter()
                    .map(|e| DeviceDefinition_Material {
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

    /// The model number for the device for example as defined by the manufacturer or
    /// labeler, or other agency.
    pub fn model_number(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("modelNumber") {
            return Some(string);
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

    /// Descriptive information, usage information or implantation information that is
    /// not captured in an existing element.
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

    /// An organization that is responsible for the provision and ongoing maintenance of
    /// the device.
    pub fn owner(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("owner") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Information about the packaging of the device, i.e. how the device is packaged.
    pub fn packaging(&self) -> Option<Vec<DeviceDefinition_Packaging>> {
        if let Some(Value::Array(val)) = self.value.get("packaging") {
            return Some(
                val.into_iter()
                    .map(|e| DeviceDefinition_Packaging {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The parent device it can be part of.
    pub fn parent_device(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("parentDevice") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The part number or catalog number of the device.
    pub fn part_number(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("partNumber") {
            return Some(string);
        }
        return None;
    }

    /// Indicates the production identifier(s) that are expected to appear in the UDI
    /// carrier on the device label.
    pub fn production_identifier_in_u_d_i(&self) -> Option<Vec<&str>> {
        if let Some(Value::Array(val)) = self.value.get("productionIdentifierInUDI") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_str().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The potential, valid configuration settings of a device, e.g., regulation
    /// status, time properties.
    pub fn property(&self) -> Option<Vec<DeviceDefinition_Property>> {
        if let Some(Value::Array(val)) = self.value.get("property") {
            return Some(
                val.into_iter()
                    .map(|e| DeviceDefinition_Property {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Safety characteristics of the device.
    pub fn safety(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("safety") {
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

    /// Shelf Life and storage information.
    pub fn shelf_life_storage(&self) -> Option<Vec<ProductShelfLife>> {
        if let Some(Value::Array(val)) = self.value.get("shelfLifeStorage") {
            return Some(
                val.into_iter()
                    .map(|e| ProductShelfLife {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The capabilities supported on a  device, the standards to which the device
    /// conforms for a particular purpose, and used for the communication.
    pub fn specialization(&self) -> Option<Vec<RelatedArtifact>> {
        if let Some(Value::Array(val)) = self.value.get("specialization") {
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

    /// Unique device identifier (UDI) assigned to device label or package.  Note that
    /// the Device may include multiple udiCarriers as it either may include just the
    /// udiCarrier for the jurisdiction it is sold, or for multiple jurisdictions it
    /// could have been sold.
    pub fn udi_device_identifier(&self) -> Option<Vec<DeviceDefinition_UdiDeviceIdentifier>> {
        if let Some(Value::Array(val)) = self.value.get("udiDeviceIdentifier") {
            return Some(
                val.into_iter()
                    .map(|e| DeviceDefinition_UdiDeviceIdentifier {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The version of the device or software.
    pub fn version(&self) -> Option<Vec<DeviceDefinition_Version>> {
        if let Some(Value::Array(val)) = self.value.get("version") {
            return Some(
                val.into_iter()
                    .map(|e| DeviceDefinition_Version {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
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
        if let Some(_val) = self._manufacturer_string() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._model_number() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._part_number() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._production_identifier_in_u_d_i() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.charge_item() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.classification() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
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
        if let Some(_val) = self.corrective_action() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.description() {}
        if let Some(_val) = self.device_name() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.guideline() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.has_part() {
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
        if let Some(_val) = self.language() {}
        if let Some(_val) = self.language_code() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.link() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.manufacturer_reference() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.manufacturer_string() {}
        if let Some(_val) = self.material() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.meta() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.model_number() {}
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
        if let Some(_val) = self.owner() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.packaging() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.parent_device() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.part_number() {}
        if let Some(_val) = self.production_identifier_in_u_d_i() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.property() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.safety() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.shelf_life_storage() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.specialization() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.text() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.udi_device_identifier() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.version() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct DeviceDefinitionBuilder {
    pub(crate) value: Value,
}

impl DeviceDefinitionBuilder {
    pub fn build(&self) -> DeviceDefinition {
        DeviceDefinition {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: DeviceDefinition) -> DeviceDefinitionBuilder {
        DeviceDefinitionBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> DeviceDefinitionBuilder {
        let mut __value: Value = json!({});
        return DeviceDefinitionBuilder { value: __value };
    }

    pub fn _description<'a>(&'a mut self, val: Element) -> &'a mut DeviceDefinitionBuilder {
        self.value["_description"] = json!(val.value);
        return self;
    }

    pub fn _implicit_rules<'a>(&'a mut self, val: Element) -> &'a mut DeviceDefinitionBuilder {
        self.value["_implicitRules"] = json!(val.value);
        return self;
    }

    pub fn _language<'a>(&'a mut self, val: Element) -> &'a mut DeviceDefinitionBuilder {
        self.value["_language"] = json!(val.value);
        return self;
    }

    pub fn _manufacturer_string<'a>(&'a mut self, val: Element) -> &'a mut DeviceDefinitionBuilder {
        self.value["_manufacturerString"] = json!(val.value);
        return self;
    }

    pub fn _model_number<'a>(&'a mut self, val: Element) -> &'a mut DeviceDefinitionBuilder {
        self.value["_modelNumber"] = json!(val.value);
        return self;
    }

    pub fn _part_number<'a>(&'a mut self, val: Element) -> &'a mut DeviceDefinitionBuilder {
        self.value["_partNumber"] = json!(val.value);
        return self;
    }

    pub fn _production_identifier_in_u_d_i<'a>(
        &'a mut self,
        val: Vec<Element>,
    ) -> &'a mut DeviceDefinitionBuilder {
        self.value["_productionIdentifierInUDI"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn charge_item<'a>(
        &'a mut self,
        val: Vec<DeviceDefinition_ChargeItem>,
    ) -> &'a mut DeviceDefinitionBuilder {
        self.value["chargeItem"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn classification<'a>(
        &'a mut self,
        val: Vec<DeviceDefinition_Classification>,
    ) -> &'a mut DeviceDefinitionBuilder {
        self.value["classification"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn contact<'a>(&'a mut self, val: Vec<ContactPoint>) -> &'a mut DeviceDefinitionBuilder {
        self.value["contact"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn contained<'a>(&'a mut self, val: Vec<ResourceList>) -> &'a mut DeviceDefinitionBuilder {
        self.value["contained"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn corrective_action<'a>(
        &'a mut self,
        val: DeviceDefinition_CorrectiveAction,
    ) -> &'a mut DeviceDefinitionBuilder {
        self.value["correctiveAction"] = json!(val.value);
        return self;
    }

    pub fn description<'a>(&'a mut self, val: &str) -> &'a mut DeviceDefinitionBuilder {
        self.value["description"] = json!(val);
        return self;
    }

    pub fn device_name<'a>(
        &'a mut self,
        val: Vec<DeviceDefinition_DeviceName>,
    ) -> &'a mut DeviceDefinitionBuilder {
        self.value["deviceName"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut DeviceDefinitionBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn guideline<'a>(
        &'a mut self,
        val: DeviceDefinition_Guideline,
    ) -> &'a mut DeviceDefinitionBuilder {
        self.value["guideline"] = json!(val.value);
        return self;
    }

    pub fn has_part<'a>(
        &'a mut self,
        val: Vec<DeviceDefinition_HasPart>,
    ) -> &'a mut DeviceDefinitionBuilder {
        self.value["hasPart"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut DeviceDefinitionBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn identifier<'a>(&'a mut self, val: Vec<Identifier>) -> &'a mut DeviceDefinitionBuilder {
        self.value["identifier"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn implicit_rules<'a>(&'a mut self, val: &str) -> &'a mut DeviceDefinitionBuilder {
        self.value["implicitRules"] = json!(val);
        return self;
    }

    pub fn language<'a>(&'a mut self, val: &str) -> &'a mut DeviceDefinitionBuilder {
        self.value["language"] = json!(val);
        return self;
    }

    pub fn language_code<'a>(
        &'a mut self,
        val: Vec<CodeableConcept>,
    ) -> &'a mut DeviceDefinitionBuilder {
        self.value["languageCode"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn link<'a>(
        &'a mut self,
        val: Vec<DeviceDefinition_Link>,
    ) -> &'a mut DeviceDefinitionBuilder {
        self.value["link"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn manufacturer_reference<'a>(
        &'a mut self,
        val: Reference,
    ) -> &'a mut DeviceDefinitionBuilder {
        self.value["manufacturerReference"] = json!(val.value);
        return self;
    }

    pub fn manufacturer_string<'a>(&'a mut self, val: &str) -> &'a mut DeviceDefinitionBuilder {
        self.value["manufacturerString"] = json!(val);
        return self;
    }

    pub fn material<'a>(
        &'a mut self,
        val: Vec<DeviceDefinition_Material>,
    ) -> &'a mut DeviceDefinitionBuilder {
        self.value["material"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn meta<'a>(&'a mut self, val: Meta) -> &'a mut DeviceDefinitionBuilder {
        self.value["meta"] = json!(val.value);
        return self;
    }

    pub fn model_number<'a>(&'a mut self, val: &str) -> &'a mut DeviceDefinitionBuilder {
        self.value["modelNumber"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut DeviceDefinitionBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn note<'a>(&'a mut self, val: Vec<Annotation>) -> &'a mut DeviceDefinitionBuilder {
        self.value["note"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn owner<'a>(&'a mut self, val: Reference) -> &'a mut DeviceDefinitionBuilder {
        self.value["owner"] = json!(val.value);
        return self;
    }

    pub fn packaging<'a>(
        &'a mut self,
        val: Vec<DeviceDefinition_Packaging>,
    ) -> &'a mut DeviceDefinitionBuilder {
        self.value["packaging"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn parent_device<'a>(&'a mut self, val: Reference) -> &'a mut DeviceDefinitionBuilder {
        self.value["parentDevice"] = json!(val.value);
        return self;
    }

    pub fn part_number<'a>(&'a mut self, val: &str) -> &'a mut DeviceDefinitionBuilder {
        self.value["partNumber"] = json!(val);
        return self;
    }

    pub fn production_identifier_in_u_d_i<'a>(
        &'a mut self,
        val: Vec<&str>,
    ) -> &'a mut DeviceDefinitionBuilder {
        self.value["productionIdentifierInUDI"] = json!(val);
        return self;
    }

    pub fn property<'a>(
        &'a mut self,
        val: Vec<DeviceDefinition_Property>,
    ) -> &'a mut DeviceDefinitionBuilder {
        self.value["property"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn safety<'a>(&'a mut self, val: Vec<CodeableConcept>) -> &'a mut DeviceDefinitionBuilder {
        self.value["safety"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn shelf_life_storage<'a>(
        &'a mut self,
        val: Vec<ProductShelfLife>,
    ) -> &'a mut DeviceDefinitionBuilder {
        self.value["shelfLifeStorage"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn specialization<'a>(
        &'a mut self,
        val: Vec<RelatedArtifact>,
    ) -> &'a mut DeviceDefinitionBuilder {
        self.value["specialization"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn text<'a>(&'a mut self, val: Narrative) -> &'a mut DeviceDefinitionBuilder {
        self.value["text"] = json!(val.value);
        return self;
    }

    pub fn udi_device_identifier<'a>(
        &'a mut self,
        val: Vec<DeviceDefinition_UdiDeviceIdentifier>,
    ) -> &'a mut DeviceDefinitionBuilder {
        self.value["udiDeviceIdentifier"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn version<'a>(
        &'a mut self,
        val: Vec<DeviceDefinition_Version>,
    ) -> &'a mut DeviceDefinitionBuilder {
        self.value["version"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }
}
