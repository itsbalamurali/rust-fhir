#![allow(unused_imports, non_camel_case_types)]

use crate::models::r5::CodeableConcept::CodeableConcept;
use crate::models::r5::Element::Element;
use crate::models::r5::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A collection of error, warning, or information messages that result from a
/// system action.

#[derive(Debug)]
pub struct OperationOutcome_Issue<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl OperationOutcome_Issue<'_> {
    pub fn new(value: &Value) -> OperationOutcome_Issue {
        OperationOutcome_Issue {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for code
    pub fn _code(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_code") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for diagnostics
    pub fn _diagnostics(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_diagnostics") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for expression
    pub fn _expression(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_expression") {
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

    /// Extensions for location
    pub fn _location(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_location") {
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

    /// Extensions for severity
    pub fn _severity(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_severity") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Describes the type of the issue. The system that creates an OperationOutcome
    /// SHALL choose the most applicable code from the IssueType value set, and may
    /// additional provide its own code for the error in the details element.
    pub fn code(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("code") {
            return Some(string);
        }
        return None;
    }

    /// Additional details about the error. This may be a text description of the error
    /// or a system code that identifies the error.
    pub fn details(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("details") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Additional diagnostic information about the issue.
    pub fn diagnostics(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("diagnostics") {
            return Some(string);
        }
        return None;
    }

    /// A [simple subset of FHIRPath](fhirpath.html#simple) limited to element names,
    /// repetition indicators and the default child accessor that identifies one of the
    /// elements in the resource that caused this issue to be raised.
    pub fn expression(&self) -> Option<Vec<&str>> {
        if let Some(Value::Array(val)) = self.value.get("expression") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_str().unwrap())
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

    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
        }
        return None;
    }

    /// This element is deprecated because it is XML specific. It is replaced by
    /// issue.expression, which is format independent, and simpler to parse.     For
    /// resource issues, this will be a simple XPath limited to element names,
    /// repetition indicators and the default child accessor that identifies one of the
    /// elements in the resource that caused this issue to be raised.  For HTTP errors,
    /// will be "http." + the parameter name.
    pub fn location(&self) -> Option<Vec<&str>> {
        if let Some(Value::Array(val)) = self.value.get("location") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_str().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// May be used to represent additional information that is not part of the basic
    /// definition of the element and that modifies the understanding of the element in
    /// which it is contained and/or the understanding of the containing element's
    /// descendants. Usually modifier elements provide negation or qualification. To
    /// make the use of extensions safe and manageable, there is a strict set of
    /// governance applied to the definition and use of extensions. Though any
    /// implementer can define an extension, there is a set of requirements that SHALL
    /// be met as part of the definition of the extension. Applications processing a
    /// resource are required to check for modifier extensions.    Modifier extensions
    /// SHALL NOT change the meaning of any elements on Resource or DomainResource
    /// (including cannot change the meaning of modifierExtension itself).
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

    /// Indicates whether the issue indicates a variation from successful processing.
    pub fn severity(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("severity") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._code() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._diagnostics() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._expression() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self._location() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self._severity() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.code() {}
        if let Some(_val) = self.details() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.diagnostics() {}
        if let Some(_val) = self.expression() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.location() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.severity() {}
        return true;
    }
}

#[derive(Debug)]
pub struct OperationOutcome_IssueBuilder {
    pub(crate) value: Value,
}

impl OperationOutcome_IssueBuilder {
    pub fn build(&self) -> OperationOutcome_Issue {
        OperationOutcome_Issue {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: OperationOutcome_Issue) -> OperationOutcome_IssueBuilder {
        OperationOutcome_IssueBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> OperationOutcome_IssueBuilder {
        let mut __value: Value = json!({});
        return OperationOutcome_IssueBuilder { value: __value };
    }

    pub fn _code<'a>(&'a mut self, val: Element) -> &'a mut OperationOutcome_IssueBuilder {
        self.value["_code"] = json!(val.value);
        return self;
    }

    pub fn _diagnostics<'a>(&'a mut self, val: Element) -> &'a mut OperationOutcome_IssueBuilder {
        self.value["_diagnostics"] = json!(val.value);
        return self;
    }

    pub fn _expression<'a>(
        &'a mut self,
        val: Vec<Element>,
    ) -> &'a mut OperationOutcome_IssueBuilder {
        self.value["_expression"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn _location<'a>(&'a mut self, val: Vec<Element>) -> &'a mut OperationOutcome_IssueBuilder {
        self.value["_location"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn _severity<'a>(&'a mut self, val: Element) -> &'a mut OperationOutcome_IssueBuilder {
        self.value["_severity"] = json!(val.value);
        return self;
    }

    pub fn code<'a>(&'a mut self, val: &str) -> &'a mut OperationOutcome_IssueBuilder {
        self.value["code"] = json!(val);
        return self;
    }

    pub fn details<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut OperationOutcome_IssueBuilder {
        self.value["details"] = json!(val.value);
        return self;
    }

    pub fn diagnostics<'a>(&'a mut self, val: &str) -> &'a mut OperationOutcome_IssueBuilder {
        self.value["diagnostics"] = json!(val);
        return self;
    }

    pub fn expression<'a>(&'a mut self, val: Vec<&str>) -> &'a mut OperationOutcome_IssueBuilder {
        self.value["expression"] = json!(val);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut OperationOutcome_IssueBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut OperationOutcome_IssueBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn location<'a>(&'a mut self, val: Vec<&str>) -> &'a mut OperationOutcome_IssueBuilder {
        self.value["location"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut OperationOutcome_IssueBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn severity<'a>(&'a mut self, val: &str) -> &'a mut OperationOutcome_IssueBuilder {
        self.value["severity"] = json!(val);
        return self;
    }
}
