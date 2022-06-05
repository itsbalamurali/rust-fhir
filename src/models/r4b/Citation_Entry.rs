#![allow(unused_imports, non_camel_case_types)]

use crate::models::r4b::Address::Address;
use crate::models::r4b::Citation_AffiliationInfo::Citation_AffiliationInfo;
use crate::models::r4b::Citation_ContributionInstance::Citation_ContributionInstance;
use crate::models::r4b::CodeableConcept::CodeableConcept;
use crate::models::r4b::ContactPoint::ContactPoint;
use crate::models::r4b::Element::Element;
use crate::models::r4b::Extension::Extension;
use crate::models::r4b::HumanName::HumanName;
use crate::models::r4b::Identifier::Identifier;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// The Citation Resource enables reference to any knowledge artifact for purposes of
/// identification and attribution. The Citation Resource supports existing reference
/// structures and developing publication practices such as versioning, expressing
/// complex contributorship roles, and referencing computable resources.

#[derive(Debug)]
pub struct Citation_Entry<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Citation_Entry<'_> {
    pub fn new(value: &Value) -> Citation_Entry {
        Citation_Entry {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for collectiveName
    pub fn _collective_name(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_collectiveName") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for correspondingContact
    pub fn _corresponding_contact(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_correspondingContact") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for initials
    pub fn _initials(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_initials") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for listOrder
    pub fn _list_order(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_listOrder") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Physical mailing address for the author or contributor.
    pub fn address(&self) -> Option<Vec<Address>> {
        if let Some(Value::Array(val)) = self.value.get("address") {
            return Some(
                val.into_iter()
                    .map(|e| Address {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Organization affiliated with the entity.
    pub fn affiliation_info(&self) -> Option<Vec<Citation_AffiliationInfo>> {
        if let Some(Value::Array(val)) = self.value.get("affiliationInfo") {
            return Some(
                val.into_iter()
                    .map(|e| Citation_AffiliationInfo {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Used for collective or corporate name as an author.
    pub fn collective_name(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("collectiveName") {
            return Some(string);
        }
        return None;
    }

    /// Contributions with accounting for time or number.
    pub fn contribution_instance(&self) -> Option<Vec<Citation_ContributionInstance>> {
        if let Some(Value::Array(val)) = self.value.get("contributionInstance") {
            return Some(
                val.into_iter()
                    .map(|e| Citation_ContributionInstance {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// This element identifies the specific nature of an individual’s contribution with
    /// respect to the cited work.
    pub fn contribution_type(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("contributionType") {
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

    /// Indication of which contributor is the corresponding contributor for the role.
    pub fn corresponding_contact(&self) -> Option<bool> {
        if let Some(val) = self.value.get("correspondingContact") {
            return Some(val.as_bool().unwrap());
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

    /// Unique person identifier.
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

    /// Initials for forename.
    pub fn initials(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("initials") {
            return Some(string);
        }
        return None;
    }

    /// Used to code order of authors.
    pub fn list_order(&self) -> Option<i64> {
        if let Some(val) = self.value.get("listOrder") {
            return Some(val.as_i64().unwrap());
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

    /// A name associated with the individual.
    pub fn name(&self) -> Option<HumanName> {
        if let Some(val) = self.value.get("name") {
            return Some(HumanName {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The role of the contributor (e.g. author, editor, reviewer).
    pub fn role(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("role") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Email or telephone contact methods for the author or contributor.
    pub fn telecom(&self) -> Option<Vec<ContactPoint>> {
        if let Some(Value::Array(val)) = self.value.get("telecom") {
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

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._collective_name() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._corresponding_contact() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._initials() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._list_order() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.address() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.affiliation_info() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.collective_name() {}
        if let Some(_val) = self.contribution_instance() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.contribution_type() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.corresponding_contact() {}
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
        if let Some(_val) = self.initials() {}
        if let Some(_val) = self.list_order() {}
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.name() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.role() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.telecom() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct Citation_EntryBuilder {
    pub(crate) value: Value,
}

impl Citation_EntryBuilder {
    pub fn build(&self) -> Citation_Entry {
        Citation_Entry {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Citation_Entry) -> Citation_EntryBuilder {
        Citation_EntryBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> Citation_EntryBuilder {
        let mut __value: Value = json!({});
        return Citation_EntryBuilder { value: __value };
    }

    pub fn _collective_name<'a>(&'a mut self, val: Element) -> &'a mut Citation_EntryBuilder {
        self.value["_collectiveName"] = json!(val.value);
        return self;
    }

    pub fn _corresponding_contact<'a>(&'a mut self, val: Element) -> &'a mut Citation_EntryBuilder {
        self.value["_correspondingContact"] = json!(val.value);
        return self;
    }

    pub fn _initials<'a>(&'a mut self, val: Element) -> &'a mut Citation_EntryBuilder {
        self.value["_initials"] = json!(val.value);
        return self;
    }

    pub fn _list_order<'a>(&'a mut self, val: Element) -> &'a mut Citation_EntryBuilder {
        self.value["_listOrder"] = json!(val.value);
        return self;
    }

    pub fn address<'a>(&'a mut self, val: Vec<Address>) -> &'a mut Citation_EntryBuilder {
        self.value["address"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn affiliation_info<'a>(
        &'a mut self,
        val: Vec<Citation_AffiliationInfo>,
    ) -> &'a mut Citation_EntryBuilder {
        self.value["affiliationInfo"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn collective_name<'a>(&'a mut self, val: &str) -> &'a mut Citation_EntryBuilder {
        self.value["collectiveName"] = json!(val);
        return self;
    }

    pub fn contribution_instance<'a>(
        &'a mut self,
        val: Vec<Citation_ContributionInstance>,
    ) -> &'a mut Citation_EntryBuilder {
        self.value["contributionInstance"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn contribution_type<'a>(
        &'a mut self,
        val: Vec<CodeableConcept>,
    ) -> &'a mut Citation_EntryBuilder {
        self.value["contributionType"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn corresponding_contact<'a>(&'a mut self, val: bool) -> &'a mut Citation_EntryBuilder {
        self.value["correspondingContact"] = json!(val);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut Citation_EntryBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut Citation_EntryBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn identifier<'a>(&'a mut self, val: Vec<Identifier>) -> &'a mut Citation_EntryBuilder {
        self.value["identifier"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn initials<'a>(&'a mut self, val: &str) -> &'a mut Citation_EntryBuilder {
        self.value["initials"] = json!(val);
        return self;
    }

    pub fn list_order<'a>(&'a mut self, val: i64) -> &'a mut Citation_EntryBuilder {
        self.value["listOrder"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut Citation_EntryBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn name<'a>(&'a mut self, val: HumanName) -> &'a mut Citation_EntryBuilder {
        self.value["name"] = json!(val.value);
        return self;
    }

    pub fn role<'a>(&'a mut self, val: CodeableConcept) -> &'a mut Citation_EntryBuilder {
        self.value["role"] = json!(val.value);
        return self;
    }

    pub fn telecom<'a>(&'a mut self, val: Vec<ContactPoint>) -> &'a mut Citation_EntryBuilder {
        self.value["telecom"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }
}
