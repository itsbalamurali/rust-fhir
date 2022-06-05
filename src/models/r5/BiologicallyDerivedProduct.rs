#![allow(unused_imports, non_camel_case_types)]

use crate::models::r5::BiologicallyDerivedProduct_Collection::BiologicallyDerivedProduct_Collection;
use crate::models::r5::BiologicallyDerivedProduct_Property::BiologicallyDerivedProduct_Property;
use crate::models::r5::CodeableConcept::CodeableConcept;
use crate::models::r5::Element::Element;
use crate::models::r5::Extension::Extension;
use crate::models::r5::Identifier::Identifier;
use crate::models::r5::Meta::Meta;
use crate::models::r5::Narrative::Narrative;
use crate::models::r5::Range::Range;
use crate::models::r5::Reference::Reference;
use crate::models::r5::ResourceList::ResourceList;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// This resource reflects an instance of a biologically derived product. A material
/// substance originating from a biological entity intended to be transplanted or
/// infused
/// into another (possibly the same) biological entity.

#[derive(Debug)]
pub struct BiologicallyDerivedProduct<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl BiologicallyDerivedProduct<'_> {
    pub fn new(value: &Value) -> BiologicallyDerivedProduct {
        BiologicallyDerivedProduct {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for division
    pub fn _division(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_division") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for expirationDate
    pub fn _expiration_date(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_expirationDate") {
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

    /// Extensions for productCategory
    pub fn _product_category(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_productCategory") {
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

    /// An identifier that supports traceability to the biological entity that is the
    /// source of biological material in the product.
    pub fn biological_source(&self) -> Option<Identifier> {
        if let Some(val) = self.value.get("biologicalSource") {
            return Some(Identifier {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// How this product was collected.
    pub fn collection(&self) -> Option<BiologicallyDerivedProduct_Collection> {
        if let Some(val) = self.value.get("collection") {
            return Some(BiologicallyDerivedProduct_Collection {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// These resources do not have an independent existence apart from the resource that
    /// contains them - they cannot be identified independently, nor can they have their
    /// own independent transaction scope.
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

    /// Description of division.
    pub fn division(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("division") {
            return Some(string);
        }
        return None;
    }

    /// Date of expiration.
    pub fn expiration_date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("expirationDate") {
            return Some(string);
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

    /// This records identifiers associated with this biologically derived product
    /// instance that are defined by business processes and/or used to refer to it when
    /// a direct URL reference to the resource itself is not appropriate (e.g. in CDA
    /// documents, or in written / printed documentation).
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

    /// The base language in which the resource is written.
    pub fn language(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("language") {
            return Some(string);
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

    /// Parent product (if any).
    pub fn parent(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("parent") {
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

    /// Processing facilities for this biologically derived product.
    pub fn processing_facility(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("processingFacility") {
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

    /// Broad category of this product.
    pub fn product_category(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("productCategory") {
            return Some(string);
        }
        return None;
    }

    /// A code that identifies the kind of this biologically derived product (SNOMED
    /// Ctcode).
    pub fn product_code(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("productCode") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A property that is specific to this BiologicallyDerviedProduct instance.
    pub fn property(&self) -> Option<Vec<BiologicallyDerivedProduct_Property>> {
        if let Some(Value::Array(val)) = self.value.get("property") {
            return Some(
                val.into_iter()
                    .map(|e| BiologicallyDerivedProduct_Property {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Procedure request to obtain this biologically derived product.
    pub fn request(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("request") {
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

    /// Whether the product is currently available.
    pub fn status(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("status") {
            return Some(string);
        }
        return None;
    }

    /// Product storage temp requirements.
    pub fn storage_temp_requirements(&self) -> Option<Range> {
        if let Some(val) = self.value.get("storageTempRequirements") {
            return Some(Range {
                value: Cow::Borrowed(val),
            });
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

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._division() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._expiration_date() {
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
        if let Some(_val) = self._product_category() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._status() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.biological_source() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.collection() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.contained() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.division() {}
        if let Some(_val) = self.expiration_date() {}
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
        if let Some(_val) = self.parent() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.processing_facility() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.product_category() {}
        if let Some(_val) = self.product_code() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.property() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.request() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.status() {}
        if let Some(_val) = self.storage_temp_requirements() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.text() {
            if !_val.validate() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct BiologicallyDerivedProductBuilder {
    pub(crate) value: Value,
}

impl BiologicallyDerivedProductBuilder {
    pub fn build(&self) -> BiologicallyDerivedProduct {
        BiologicallyDerivedProduct {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: BiologicallyDerivedProduct) -> BiologicallyDerivedProductBuilder {
        BiologicallyDerivedProductBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> BiologicallyDerivedProductBuilder {
        let mut __value: Value = json!({});
        return BiologicallyDerivedProductBuilder { value: __value };
    }

    pub fn _division<'a>(&'a mut self, val: Element) -> &'a mut BiologicallyDerivedProductBuilder {
        self.value["_division"] = json!(val.value);
        return self;
    }

    pub fn _expiration_date<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut BiologicallyDerivedProductBuilder {
        self.value["_expirationDate"] = json!(val.value);
        return self;
    }

    pub fn _implicit_rules<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut BiologicallyDerivedProductBuilder {
        self.value["_implicitRules"] = json!(val.value);
        return self;
    }

    pub fn _language<'a>(&'a mut self, val: Element) -> &'a mut BiologicallyDerivedProductBuilder {
        self.value["_language"] = json!(val.value);
        return self;
    }

    pub fn _product_category<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut BiologicallyDerivedProductBuilder {
        self.value["_productCategory"] = json!(val.value);
        return self;
    }

    pub fn _status<'a>(&'a mut self, val: Element) -> &'a mut BiologicallyDerivedProductBuilder {
        self.value["_status"] = json!(val.value);
        return self;
    }

    pub fn biological_source<'a>(
        &'a mut self,
        val: Identifier,
    ) -> &'a mut BiologicallyDerivedProductBuilder {
        self.value["biologicalSource"] = json!(val.value);
        return self;
    }

    pub fn collection<'a>(
        &'a mut self,
        val: BiologicallyDerivedProduct_Collection,
    ) -> &'a mut BiologicallyDerivedProductBuilder {
        self.value["collection"] = json!(val.value);
        return self;
    }

    pub fn contained<'a>(
        &'a mut self,
        val: Vec<ResourceList>,
    ) -> &'a mut BiologicallyDerivedProductBuilder {
        self.value["contained"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn division<'a>(&'a mut self, val: &str) -> &'a mut BiologicallyDerivedProductBuilder {
        self.value["division"] = json!(val);
        return self;
    }

    pub fn expiration_date<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut BiologicallyDerivedProductBuilder {
        self.value["expirationDate"] = json!(val);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut BiologicallyDerivedProductBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut BiologicallyDerivedProductBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn identifier<'a>(
        &'a mut self,
        val: Vec<Identifier>,
    ) -> &'a mut BiologicallyDerivedProductBuilder {
        self.value["identifier"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn implicit_rules<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut BiologicallyDerivedProductBuilder {
        self.value["implicitRules"] = json!(val);
        return self;
    }

    pub fn language<'a>(&'a mut self, val: &str) -> &'a mut BiologicallyDerivedProductBuilder {
        self.value["language"] = json!(val);
        return self;
    }

    pub fn meta<'a>(&'a mut self, val: Meta) -> &'a mut BiologicallyDerivedProductBuilder {
        self.value["meta"] = json!(val.value);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut BiologicallyDerivedProductBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn parent<'a>(
        &'a mut self,
        val: Vec<Reference>,
    ) -> &'a mut BiologicallyDerivedProductBuilder {
        self.value["parent"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn processing_facility<'a>(
        &'a mut self,
        val: Vec<Reference>,
    ) -> &'a mut BiologicallyDerivedProductBuilder {
        self.value["processingFacility"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn product_category<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut BiologicallyDerivedProductBuilder {
        self.value["productCategory"] = json!(val);
        return self;
    }

    pub fn product_code<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut BiologicallyDerivedProductBuilder {
        self.value["productCode"] = json!(val.value);
        return self;
    }

    pub fn property<'a>(
        &'a mut self,
        val: Vec<BiologicallyDerivedProduct_Property>,
    ) -> &'a mut BiologicallyDerivedProductBuilder {
        self.value["property"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn request<'a>(
        &'a mut self,
        val: Vec<Reference>,
    ) -> &'a mut BiologicallyDerivedProductBuilder {
        self.value["request"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn status<'a>(&'a mut self, val: &str) -> &'a mut BiologicallyDerivedProductBuilder {
        self.value["status"] = json!(val);
        return self;
    }

    pub fn storage_temp_requirements<'a>(
        &'a mut self,
        val: Range,
    ) -> &'a mut BiologicallyDerivedProductBuilder {
        self.value["storageTempRequirements"] = json!(val.value);
        return self;
    }

    pub fn text<'a>(&'a mut self, val: Narrative) -> &'a mut BiologicallyDerivedProductBuilder {
        self.value["text"] = json!(val.value);
        return self;
    }
}
