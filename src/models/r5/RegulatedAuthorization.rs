#![allow(unused_imports, non_camel_case_types)]

use crate::models::r5::CodeableConcept::CodeableConcept;
use crate::models::r5::CodeableReference::CodeableReference;
use crate::models::r5::Element::Element;
use crate::models::r5::Extension::Extension;
use crate::models::r5::Identifier::Identifier;
use crate::models::r5::Meta::Meta;
use crate::models::r5::Narrative::Narrative;
use crate::models::r5::Period::Period;
use crate::models::r5::Reference::Reference;
use crate::models::r5::RegulatedAuthorization_Case::RegulatedAuthorization_Case;
use crate::models::r5::ResourceList::ResourceList;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Regulatory approval, clearance or licencing related to a regulated product,
/// treatment, facility or activity that is cited in a guidance, regulation, rule
/// or legislative act. An example is Market Authorization relating to a Medicinal
/// Product.

#[derive(Debug)]
pub struct RegulatedAuthorization<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl RegulatedAuthorization<'_> {
    pub fn new(value: &Value) -> RegulatedAuthorization {
        RegulatedAuthorization {
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

    /// Extensions for statusDate
    pub fn _status_date(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_statusDate") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Additional information or supporting documentation about the authorization.
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

    /// The legal or regulatory framework against which this authorization is granted, or
    /// other reasons for it.
    pub fn basis(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("basis") {
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

    /// The case or regulatory procedure for granting or amending a marketing
    /// authorization. Note: This area is subject to ongoing review and the workgroup is
    /// seeking implementer feedback on its use (see link at bottom of page).
    pub fn case(&self) -> Option<RegulatedAuthorization_Case> {
        if let Some(val) = self.value.get("case") {
            return Some(RegulatedAuthorization_Case {
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

    /// General textual supporting information.
    pub fn description(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("description") {
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

    /// The organization that holds the granted authorization.
    pub fn holder(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("holder") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
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

    /// Business identifier for the authorization, typically assigned by the authorizing
    /// body.
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

    /// Condition for which the use of the regulated product applies.
    pub fn indication(&self) -> Option<CodeableReference> {
        if let Some(val) = self.value.get("indication") {
            return Some(CodeableReference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The intended use of the product, e.g. prevention, treatment.
    pub fn intended_use(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("intendedUse") {
            return Some(CodeableConcept {
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

    /// The territory (e.g., country, jurisdiction etc.) in which the authorization has
    /// been granted.
    pub fn region(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("region") {
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

    /// The regulatory authority or authorizing body granting the authorization. For
    /// example, European Medicines Agency (EMA), Food and Drug Administration (FDA),
    /// Health Canada (HC), etc.
    pub fn regulator(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("regulator") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The status that is authorised e.g. approved. Intermediate states can be tracked
    /// with cases and applications.
    pub fn status(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("status") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The date at which the current status was assigned.
    pub fn status_date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("statusDate") {
            return Some(string);
        }
        return None;
    }

    /// The product type, treatment, facility or activity that is being authorized.
    pub fn subject(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("subject") {
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

    /// Overall type of this authorization, for example drug marketing approval, orphan
    /// drug designation.
    pub fn fhir_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("type") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The time period in which the regulatory approval, clearance or licencing
    /// is in effect. As an example, a Marketing Authorization includes the date of
    /// authorization and/or an expiration date.
    pub fn validity_period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("validityPeriod") {
            return Some(Period {
                value: Cow::Borrowed(val),
            });
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
        if let Some(_val) = self._status_date() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.attached_document() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.basis() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.case() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.contained() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.description() {}
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.holder() {
            if !_val.validate() {
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
        if let Some(_val) = self.indication() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.intended_use() {
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
        if let Some(_val) = self.region() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.regulator() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.status() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.status_date() {}
        if let Some(_val) = self.subject() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
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
        if let Some(_val) = self.validity_period() {
            if !_val.validate() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct RegulatedAuthorizationBuilder {
    pub(crate) value: Value,
}

impl RegulatedAuthorizationBuilder {
    pub fn build(&self) -> RegulatedAuthorization {
        RegulatedAuthorization {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: RegulatedAuthorization) -> RegulatedAuthorizationBuilder {
        RegulatedAuthorizationBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> RegulatedAuthorizationBuilder {
        let mut __value: Value = json!({});
        return RegulatedAuthorizationBuilder { value: __value };
    }

    pub fn _description<'a>(&'a mut self, val: Element) -> &'a mut RegulatedAuthorizationBuilder {
        self.value["_description"] = json!(val.value);
        return self;
    }

    pub fn _implicit_rules<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut RegulatedAuthorizationBuilder {
        self.value["_implicitRules"] = json!(val.value);
        return self;
    }

    pub fn _language<'a>(&'a mut self, val: Element) -> &'a mut RegulatedAuthorizationBuilder {
        self.value["_language"] = json!(val.value);
        return self;
    }

    pub fn _status_date<'a>(&'a mut self, val: Element) -> &'a mut RegulatedAuthorizationBuilder {
        self.value["_statusDate"] = json!(val.value);
        return self;
    }

    pub fn attached_document<'a>(
        &'a mut self,
        val: Vec<Reference>,
    ) -> &'a mut RegulatedAuthorizationBuilder {
        self.value["attachedDocument"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn basis<'a>(
        &'a mut self,
        val: Vec<CodeableConcept>,
    ) -> &'a mut RegulatedAuthorizationBuilder {
        self.value["basis"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn case<'a>(
        &'a mut self,
        val: RegulatedAuthorization_Case,
    ) -> &'a mut RegulatedAuthorizationBuilder {
        self.value["case"] = json!(val.value);
        return self;
    }

    pub fn contained<'a>(
        &'a mut self,
        val: Vec<ResourceList>,
    ) -> &'a mut RegulatedAuthorizationBuilder {
        self.value["contained"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn description<'a>(&'a mut self, val: &str) -> &'a mut RegulatedAuthorizationBuilder {
        self.value["description"] = json!(val);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut RegulatedAuthorizationBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn holder<'a>(&'a mut self, val: Reference) -> &'a mut RegulatedAuthorizationBuilder {
        self.value["holder"] = json!(val.value);
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut RegulatedAuthorizationBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn identifier<'a>(
        &'a mut self,
        val: Vec<Identifier>,
    ) -> &'a mut RegulatedAuthorizationBuilder {
        self.value["identifier"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn implicit_rules<'a>(&'a mut self, val: &str) -> &'a mut RegulatedAuthorizationBuilder {
        self.value["implicitRules"] = json!(val);
        return self;
    }

    pub fn indication<'a>(
        &'a mut self,
        val: CodeableReference,
    ) -> &'a mut RegulatedAuthorizationBuilder {
        self.value["indication"] = json!(val.value);
        return self;
    }

    pub fn intended_use<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut RegulatedAuthorizationBuilder {
        self.value["intendedUse"] = json!(val.value);
        return self;
    }

    pub fn language<'a>(&'a mut self, val: &str) -> &'a mut RegulatedAuthorizationBuilder {
        self.value["language"] = json!(val);
        return self;
    }

    pub fn meta<'a>(&'a mut self, val: Meta) -> &'a mut RegulatedAuthorizationBuilder {
        self.value["meta"] = json!(val.value);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut RegulatedAuthorizationBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn region<'a>(
        &'a mut self,
        val: Vec<CodeableConcept>,
    ) -> &'a mut RegulatedAuthorizationBuilder {
        self.value["region"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn regulator<'a>(&'a mut self, val: Reference) -> &'a mut RegulatedAuthorizationBuilder {
        self.value["regulator"] = json!(val.value);
        return self;
    }

    pub fn status<'a>(&'a mut self, val: CodeableConcept) -> &'a mut RegulatedAuthorizationBuilder {
        self.value["status"] = json!(val.value);
        return self;
    }

    pub fn status_date<'a>(&'a mut self, val: &str) -> &'a mut RegulatedAuthorizationBuilder {
        self.value["statusDate"] = json!(val);
        return self;
    }

    pub fn subject<'a>(&'a mut self, val: Vec<Reference>) -> &'a mut RegulatedAuthorizationBuilder {
        self.value["subject"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn text<'a>(&'a mut self, val: Narrative) -> &'a mut RegulatedAuthorizationBuilder {
        self.value["text"] = json!(val.value);
        return self;
    }

    pub fn fhir_type<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut RegulatedAuthorizationBuilder {
        self.value["type"] = json!(val.value);
        return self;
    }

    pub fn validity_period<'a>(&'a mut self, val: Period) -> &'a mut RegulatedAuthorizationBuilder {
        self.value["validityPeriod"] = json!(val.value);
        return self;
    }
}
