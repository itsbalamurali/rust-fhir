#![allow(unused_imports, non_camel_case_types)]

use crate::models::r4::CodeableConcept::CodeableConcept;
use crate::models::r4::Extension::Extension;
use crate::models::r4::Identifier::Identifier;
use crate::models::r4::Reference::Reference;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Todo.

#[derive(Debug)]
pub struct SubstanceReferenceInformation_GeneElement<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl SubstanceReferenceInformation_GeneElement<'_> {
    pub fn new(value: &Value) -> SubstanceReferenceInformation_GeneElement {
        SubstanceReferenceInformation_GeneElement {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Todo.
    pub fn element(&self) -> Option<Identifier> {
        if let Some(val) = self.value.get("element") {
            return Some(Identifier {
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

    /// Todo.
    pub fn source(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("source") {
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

    /// Todo.
    pub fn fhir_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("type") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.element() {
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
        if let Some(_val) = self.source() {
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
pub struct SubstanceReferenceInformation_GeneElementBuilder {
    pub(crate) value: Value,
}

impl SubstanceReferenceInformation_GeneElementBuilder {
    pub fn build(&self) -> SubstanceReferenceInformation_GeneElement {
        SubstanceReferenceInformation_GeneElement {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(
        existing: SubstanceReferenceInformation_GeneElement,
    ) -> SubstanceReferenceInformation_GeneElementBuilder {
        SubstanceReferenceInformation_GeneElementBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> SubstanceReferenceInformation_GeneElementBuilder {
        let mut __value: Value = json!({});
        return SubstanceReferenceInformation_GeneElementBuilder { value: __value };
    }

    pub fn element<'a>(
        &'a mut self,
        val: Identifier,
    ) -> &'a mut SubstanceReferenceInformation_GeneElementBuilder {
        self.value["element"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut SubstanceReferenceInformation_GeneElementBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut SubstanceReferenceInformation_GeneElementBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut SubstanceReferenceInformation_GeneElementBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn source<'a>(
        &'a mut self,
        val: Vec<Reference>,
    ) -> &'a mut SubstanceReferenceInformation_GeneElementBuilder {
        self.value["source"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn fhir_type<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut SubstanceReferenceInformation_GeneElementBuilder {
        self.value["type"] = json!(val.value);
        return self;
    }
}
