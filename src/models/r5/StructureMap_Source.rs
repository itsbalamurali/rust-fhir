#![allow(unused_imports, non_camel_case_types)]

use crate::models::r5::Element::Element;
use crate::models::r5::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A Map of relationships between 2 structures that can be used to transform data.

#[derive(Debug)]
pub struct StructureMap_Source<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl StructureMap_Source<'_> {
    pub fn new(value: &Value) -> StructureMap_Source {
        StructureMap_Source {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for check
    pub fn _check(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_check") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for condition
    pub fn _condition(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_condition") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for context
    pub fn _context(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_context") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for defaultValue
    pub fn _default_value(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_defaultValue") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for element
    pub fn _element(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_element") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for listMode
    pub fn _list_mode(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_listMode") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for logMessage
    pub fn _log_message(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_logMessage") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for max
    pub fn _max(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_max") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for min
    pub fn _min(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_min") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for type
    pub fn _type(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_type") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for variable
    pub fn _variable(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_variable") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// FHIRPath expression  - must be true or the mapping engine throws an error
    /// instead of completing.
    pub fn check(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("check") {
            return Some(string);
        }
        return None;
    }

    /// FHIRPath expression  - must be true or the rule does not apply.
    pub fn condition(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("condition") {
            return Some(string);
        }
        return None;
    }

    /// Type or variable this rule applies to.
    pub fn context(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("context") {
            return Some(string);
        }
        return None;
    }

    /// A value to use if there is no existing value in the source object.
    pub fn default_value(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("defaultValue") {
            return Some(string);
        }
        return None;
    }

    /// Optional field for this source.
    pub fn element(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("element") {
            return Some(string);
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

    /// How to handle the list mode for this element.
    pub fn list_mode(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("listMode") {
            return Some(string);
        }
        return None;
    }

    /// A FHIRPath expression which specifies a message to put in the transform log when
    /// content matching the source rule is found.
    pub fn log_message(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("logMessage") {
            return Some(string);
        }
        return None;
    }

    /// Specified maximum cardinality for the element - a number or a "*". This is
    /// optional; if present, it acts an implicit check on the input content (* just
    /// serves as documentation; it's the default value).
    pub fn max(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("max") {
            return Some(string);
        }
        return None;
    }

    /// Specified minimum cardinality for the element. This is optional; if present, it
    /// acts an implicit check on the input content.
    pub fn min(&self) -> Option<i64> {
        if let Some(val) = self.value.get("min") {
            return Some(val.as_i64().unwrap());
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

    /// Specified type for the element. This works as a condition on the mapping - use
    /// for polymorphic elements.
    pub fn fhir_type(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("type") {
            return Some(string);
        }
        return None;
    }

    /// Named context for field, if a field is specified.
    pub fn variable(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("variable") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._check() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._condition() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._context() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._default_value() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._element() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._list_mode() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._log_message() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._max() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._min() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._type() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._variable() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.check() {}
        if let Some(_val) = self.condition() {}
        if let Some(_val) = self.context() {}
        if let Some(_val) = self.default_value() {}
        if let Some(_val) = self.element() {}
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.list_mode() {}
        if let Some(_val) = self.log_message() {}
        if let Some(_val) = self.max() {}
        if let Some(_val) = self.min() {}
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.fhir_type() {}
        if let Some(_val) = self.variable() {}
        return true;
    }
}

#[derive(Debug)]
pub struct StructureMap_SourceBuilder {
    pub(crate) value: Value,
}

impl StructureMap_SourceBuilder {
    pub fn build(&self) -> StructureMap_Source {
        StructureMap_Source {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: StructureMap_Source) -> StructureMap_SourceBuilder {
        StructureMap_SourceBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> StructureMap_SourceBuilder {
        let mut __value: Value = json!({});
        return StructureMap_SourceBuilder { value: __value };
    }

    pub fn _check<'a>(&'a mut self, val: Element) -> &'a mut StructureMap_SourceBuilder {
        self.value["_check"] = json!(val.value);
        return self;
    }

    pub fn _condition<'a>(&'a mut self, val: Element) -> &'a mut StructureMap_SourceBuilder {
        self.value["_condition"] = json!(val.value);
        return self;
    }

    pub fn _context<'a>(&'a mut self, val: Element) -> &'a mut StructureMap_SourceBuilder {
        self.value["_context"] = json!(val.value);
        return self;
    }

    pub fn _default_value<'a>(&'a mut self, val: Element) -> &'a mut StructureMap_SourceBuilder {
        self.value["_defaultValue"] = json!(val.value);
        return self;
    }

    pub fn _element<'a>(&'a mut self, val: Element) -> &'a mut StructureMap_SourceBuilder {
        self.value["_element"] = json!(val.value);
        return self;
    }

    pub fn _list_mode<'a>(&'a mut self, val: Element) -> &'a mut StructureMap_SourceBuilder {
        self.value["_listMode"] = json!(val.value);
        return self;
    }

    pub fn _log_message<'a>(&'a mut self, val: Element) -> &'a mut StructureMap_SourceBuilder {
        self.value["_logMessage"] = json!(val.value);
        return self;
    }

    pub fn _max<'a>(&'a mut self, val: Element) -> &'a mut StructureMap_SourceBuilder {
        self.value["_max"] = json!(val.value);
        return self;
    }

    pub fn _min<'a>(&'a mut self, val: Element) -> &'a mut StructureMap_SourceBuilder {
        self.value["_min"] = json!(val.value);
        return self;
    }

    pub fn _type<'a>(&'a mut self, val: Element) -> &'a mut StructureMap_SourceBuilder {
        self.value["_type"] = json!(val.value);
        return self;
    }

    pub fn _variable<'a>(&'a mut self, val: Element) -> &'a mut StructureMap_SourceBuilder {
        self.value["_variable"] = json!(val.value);
        return self;
    }

    pub fn check<'a>(&'a mut self, val: &str) -> &'a mut StructureMap_SourceBuilder {
        self.value["check"] = json!(val);
        return self;
    }

    pub fn condition<'a>(&'a mut self, val: &str) -> &'a mut StructureMap_SourceBuilder {
        self.value["condition"] = json!(val);
        return self;
    }

    pub fn context<'a>(&'a mut self, val: &str) -> &'a mut StructureMap_SourceBuilder {
        self.value["context"] = json!(val);
        return self;
    }

    pub fn default_value<'a>(&'a mut self, val: &str) -> &'a mut StructureMap_SourceBuilder {
        self.value["defaultValue"] = json!(val);
        return self;
    }

    pub fn element<'a>(&'a mut self, val: &str) -> &'a mut StructureMap_SourceBuilder {
        self.value["element"] = json!(val);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut StructureMap_SourceBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut StructureMap_SourceBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn list_mode<'a>(&'a mut self, val: &str) -> &'a mut StructureMap_SourceBuilder {
        self.value["listMode"] = json!(val);
        return self;
    }

    pub fn log_message<'a>(&'a mut self, val: &str) -> &'a mut StructureMap_SourceBuilder {
        self.value["logMessage"] = json!(val);
        return self;
    }

    pub fn max<'a>(&'a mut self, val: &str) -> &'a mut StructureMap_SourceBuilder {
        self.value["max"] = json!(val);
        return self;
    }

    pub fn min<'a>(&'a mut self, val: i64) -> &'a mut StructureMap_SourceBuilder {
        self.value["min"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut StructureMap_SourceBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn fhir_type<'a>(&'a mut self, val: &str) -> &'a mut StructureMap_SourceBuilder {
        self.value["type"] = json!(val);
        return self;
    }

    pub fn variable<'a>(&'a mut self, val: &str) -> &'a mut StructureMap_SourceBuilder {
        self.value["variable"] = json!(val);
        return self;
    }
}
