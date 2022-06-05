#![allow(unused_imports, non_camel_case_types)]

use crate::models::r4b::CodeableConcept::CodeableConcept;
use crate::models::r4b::Element::Element;
use crate::models::r4b::Extension::Extension;
use crate::models::r4b::Identifier::Identifier;
use crate::models::r4b::PackagedProductDefinition_ContainedItem::PackagedProductDefinition_ContainedItem;
use crate::models::r4b::PackagedProductDefinition_Property::PackagedProductDefinition_Property;
use crate::models::r4b::PackagedProductDefinition_ShelfLifeStorage::PackagedProductDefinition_ShelfLifeStorage;
use crate::models::r4b::Reference::Reference;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A medically related item or items, in a container or package.

#[derive(Debug)]
pub struct PackagedProductDefinition_Package<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl PackagedProductDefinition_Package<'_> {
    pub fn new(value: &Value) -> PackagedProductDefinition_Package {
        PackagedProductDefinition_Package {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for quantity
    pub fn _quantity(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_quantity") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A possible alternate material for this part of the packaging, that is allowed
    /// to be used instead of the usual material (e.g. different types of plastic for a
    /// blister sleeve).
    pub fn alternate_material(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("alternateMaterial") {
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

    /// The item(s) within the packaging.
    pub fn contained_item(&self) -> Option<Vec<PackagedProductDefinition_ContainedItem>> {
        if let Some(Value::Array(val)) = self.value.get("containedItem") {
            return Some(
                val.into_iter()
                    .map(|e| PackagedProductDefinition_ContainedItem {
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

    /// Unique id for the element within a resource (for internal references). This may be
    /// any string value that does not contain spaces.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
        }
        return None;
    }

    /// An identifier that is specific to this particular part of the packaging. Including
    /// possibly Data Carrier Identifier (a GS1 barcode).
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

    /// Manufacturer of this package Item. When there are multiple it means these are all
    /// possible manufacturers.
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

    /// Material type of the package item.
    pub fn material(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("material") {
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

    /// Allows containers (and parts of containers) parwithin
    /// containers, still a single packaged product.  See also
    /// PackagedProductDefinition.package.containedItem.item(PackagedProductDefinition).
    pub fn package(&self) -> Option<Vec<PackagedProductDefinition_Package>> {
        if let Some(Value::Array(val)) = self.value.get("package") {
            return Some(
                val.into_iter()
                    .map(|e| PackagedProductDefinition_Package {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// General characteristics of this item.
    pub fn property(&self) -> Option<Vec<PackagedProductDefinition_Property>> {
        if let Some(Value::Array(val)) = self.value.get("property") {
            return Some(
                val.into_iter()
                    .map(|e| PackagedProductDefinition_Property {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The quantity of this level of packaging in the package that contains it. If
    /// specified, the outermost level is always 1.
    pub fn quantity(&self) -> Option<i64> {
        if let Some(val) = self.value.get("quantity") {
            return Some(val.as_i64().unwrap());
        }
        return None;
    }

    /// Shelf Life and storage information.
    pub fn shelf_life_storage(&self) -> Option<Vec<PackagedProductDefinition_ShelfLifeStorage>> {
        if let Some(Value::Array(val)) = self.value.get("shelfLifeStorage") {
            return Some(
                val.into_iter()
                    .map(|e| PackagedProductDefinition_ShelfLifeStorage {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The physical type of the container of the items.
    pub fn fhir_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("type") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._quantity() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.alternate_material() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.contained_item() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
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
        if let Some(_val) = self.manufacturer() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.material() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.package() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.property() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.quantity() {}
        if let Some(_val) = self.shelf_life_storage() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.fhir_type() {
            if !_val.validate() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct PackagedProductDefinition_PackageBuilder {
    pub(crate) value: Value,
}

impl PackagedProductDefinition_PackageBuilder {
    pub fn build(&self) -> PackagedProductDefinition_Package {
        PackagedProductDefinition_Package {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(
        existing: PackagedProductDefinition_Package,
    ) -> PackagedProductDefinition_PackageBuilder {
        PackagedProductDefinition_PackageBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> PackagedProductDefinition_PackageBuilder {
        let mut __value: Value = json!({});
        return PackagedProductDefinition_PackageBuilder { value: __value };
    }

    pub fn _quantity<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut PackagedProductDefinition_PackageBuilder {
        self.value["_quantity"] = json!(val.value);
        return self;
    }

    pub fn alternate_material<'a>(
        &'a mut self,
        val: Vec<CodeableConcept>,
    ) -> &'a mut PackagedProductDefinition_PackageBuilder {
        self.value["alternateMaterial"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn contained_item<'a>(
        &'a mut self,
        val: Vec<PackagedProductDefinition_ContainedItem>,
    ) -> &'a mut PackagedProductDefinition_PackageBuilder {
        self.value["containedItem"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut PackagedProductDefinition_PackageBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut PackagedProductDefinition_PackageBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn identifier<'a>(
        &'a mut self,
        val: Vec<Identifier>,
    ) -> &'a mut PackagedProductDefinition_PackageBuilder {
        self.value["identifier"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn manufacturer<'a>(
        &'a mut self,
        val: Vec<Reference>,
    ) -> &'a mut PackagedProductDefinition_PackageBuilder {
        self.value["manufacturer"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn material<'a>(
        &'a mut self,
        val: Vec<CodeableConcept>,
    ) -> &'a mut PackagedProductDefinition_PackageBuilder {
        self.value["material"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut PackagedProductDefinition_PackageBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn package<'a>(
        &'a mut self,
        val: Vec<PackagedProductDefinition_Package>,
    ) -> &'a mut PackagedProductDefinition_PackageBuilder {
        self.value["package"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn property<'a>(
        &'a mut self,
        val: Vec<PackagedProductDefinition_Property>,
    ) -> &'a mut PackagedProductDefinition_PackageBuilder {
        self.value["property"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn quantity<'a>(
        &'a mut self,
        val: i64,
    ) -> &'a mut PackagedProductDefinition_PackageBuilder {
        self.value["quantity"] = json!(val);
        return self;
    }

    pub fn shelf_life_storage<'a>(
        &'a mut self,
        val: Vec<PackagedProductDefinition_ShelfLifeStorage>,
    ) -> &'a mut PackagedProductDefinition_PackageBuilder {
        self.value["shelfLifeStorage"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn fhir_type<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut PackagedProductDefinition_PackageBuilder {
        self.value["type"] = json!(val.value);
        return self;
    }
}
