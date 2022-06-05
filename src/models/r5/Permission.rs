#![allow(unused_imports, non_camel_case_types)]

use crate::models::r5::CodeableConcept::CodeableConcept;
use crate::models::r5::Element::Element;
use crate::models::r5::Expression::Expression;
use crate::models::r5::Extension::Extension;
use crate::models::r5::Meta::Meta;
use crate::models::r5::Narrative::Narrative;
use crate::models::r5::Period::Period;
use crate::models::r5::Permission_Justification::Permission_Justification;
use crate::models::r5::Permission_ProcessingActivity::Permission_ProcessingActivity;
use crate::models::r5::Reference::Reference;
use crate::models::r5::ResourceList::ResourceList;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Permission.

#[derive(Debug)]
pub struct Permission<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Permission<'_> {
    pub fn new(value: &Value) -> Permission {
        Permission {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for assertionDate
    pub fn _assertion_date(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_assertionDate") {
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

    /// The person or entity that asserts the permission.
    pub fn asserter(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("asserter") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The date that permission was asserted.
    pub fn assertion_date(&self) -> Option<Vec<&str>> {
        if let Some(Value::Array(val)) = self.value.get("assertionDate") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_str().unwrap())
                    .collect::<Vec<_>>(),
            );
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

    /// This can be 1) the definition of data elements, or 2) a category or label)
    /// e.g. “sensitive”. It could also be a c) graph-like definition of a set of data
    /// elements.
    pub fn data_scope(&self) -> Option<Vec<Expression>> {
        if let Some(Value::Array(val)) = self.value.get("dataScope") {
            return Some(
                val.into_iter()
                    .map(|e| Expression {
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

    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
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

    /// grant|refuse.
    pub fn intent(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("intent") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The asserted justification for using the data.
    pub fn justification(&self) -> Option<Permission_Justification> {
        if let Some(val) = self.value.get("justification") {
            return Some(Permission_Justification {
                value: Cow::Borrowed(val),
            });
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

    /// A description or definition of which activities are allowed to be done on the
    /// data.
    pub fn processing_activity(&self) -> Option<Vec<Permission_ProcessingActivity>> {
        if let Some(Value::Array(val)) = self.value.get("processingActivity") {
            return Some(
                val.into_iter()
                    .map(|e| Permission_ProcessingActivity {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The purpose for which the permission is given.
    pub fn purpose(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("purpose") {
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

    /// Status.
    pub fn status(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("status") {
            return Some(string);
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

    /// What limits apply to the use of the data.
    pub fn usage_limitations(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("usageLimitations") {
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

    /// The period in which the permission is active.
    pub fn validity(&self) -> Option<Period> {
        if let Some(val) = self.value.get("validity") {
            return Some(Period {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._assertion_date() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
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
        if let Some(_val) = self._status() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.asserter() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.assertion_date() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.contained() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.data_scope() {
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
        if let Some(_val) = self.implicit_rules() {}
        if let Some(_val) = self.intent() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.justification() {
            if !_val.validate() {
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
        if let Some(_val) = self.processing_activity() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.purpose() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.status() {}
        if let Some(_val) = self.text() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.usage_limitations() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.validity() {
            if !_val.validate() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct PermissionBuilder {
    pub(crate) value: Value,
}

impl PermissionBuilder {
    pub fn build(&self) -> Permission {
        Permission {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Permission) -> PermissionBuilder {
        PermissionBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> PermissionBuilder {
        let mut __value: Value = json!({});
        return PermissionBuilder { value: __value };
    }

    pub fn _assertion_date<'a>(&'a mut self, val: Vec<Element>) -> &'a mut PermissionBuilder {
        self.value["_assertionDate"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn _implicit_rules<'a>(&'a mut self, val: Element) -> &'a mut PermissionBuilder {
        self.value["_implicitRules"] = json!(val.value);
        return self;
    }

    pub fn _language<'a>(&'a mut self, val: Element) -> &'a mut PermissionBuilder {
        self.value["_language"] = json!(val.value);
        return self;
    }

    pub fn _status<'a>(&'a mut self, val: Element) -> &'a mut PermissionBuilder {
        self.value["_status"] = json!(val.value);
        return self;
    }

    pub fn asserter<'a>(&'a mut self, val: Reference) -> &'a mut PermissionBuilder {
        self.value["asserter"] = json!(val.value);
        return self;
    }

    pub fn assertion_date<'a>(&'a mut self, val: Vec<&str>) -> &'a mut PermissionBuilder {
        self.value["assertionDate"] = json!(val);
        return self;
    }

    pub fn contained<'a>(&'a mut self, val: Vec<ResourceList>) -> &'a mut PermissionBuilder {
        self.value["contained"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn data_scope<'a>(&'a mut self, val: Vec<Expression>) -> &'a mut PermissionBuilder {
        self.value["dataScope"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut PermissionBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut PermissionBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn implicit_rules<'a>(&'a mut self, val: &str) -> &'a mut PermissionBuilder {
        self.value["implicitRules"] = json!(val);
        return self;
    }

    pub fn intent<'a>(&'a mut self, val: CodeableConcept) -> &'a mut PermissionBuilder {
        self.value["intent"] = json!(val.value);
        return self;
    }

    pub fn justification<'a>(
        &'a mut self,
        val: Permission_Justification,
    ) -> &'a mut PermissionBuilder {
        self.value["justification"] = json!(val.value);
        return self;
    }

    pub fn language<'a>(&'a mut self, val: &str) -> &'a mut PermissionBuilder {
        self.value["language"] = json!(val);
        return self;
    }

    pub fn meta<'a>(&'a mut self, val: Meta) -> &'a mut PermissionBuilder {
        self.value["meta"] = json!(val.value);
        return self;
    }

    pub fn modifier_extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut PermissionBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn processing_activity<'a>(
        &'a mut self,
        val: Vec<Permission_ProcessingActivity>,
    ) -> &'a mut PermissionBuilder {
        self.value["processingActivity"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn purpose<'a>(&'a mut self, val: Vec<CodeableConcept>) -> &'a mut PermissionBuilder {
        self.value["purpose"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn status<'a>(&'a mut self, val: &str) -> &'a mut PermissionBuilder {
        self.value["status"] = json!(val);
        return self;
    }

    pub fn text<'a>(&'a mut self, val: Narrative) -> &'a mut PermissionBuilder {
        self.value["text"] = json!(val.value);
        return self;
    }

    pub fn usage_limitations<'a>(
        &'a mut self,
        val: Vec<CodeableConcept>,
    ) -> &'a mut PermissionBuilder {
        self.value["usageLimitations"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn validity<'a>(&'a mut self, val: Period) -> &'a mut PermissionBuilder {
        self.value["validity"] = json!(val.value);
        return self;
    }
}
