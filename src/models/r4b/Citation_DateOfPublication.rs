#![allow(unused_imports, non_camel_case_types)]

use crate::models::r4b::Element::Element;
use crate::models::r4b::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// The Citation Resource enables reference to any knowledge artifact for purposes
/// of identification and attribution. The Citation Resource supports existing
/// reference structures and developing publication practices such as versioning,
/// expressing complex contributorship roles, and referencing computable resources.

#[derive(Debug)]
pub struct Citation_DateOfPublication<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Citation_DateOfPublication<'_> {
    pub fn new(value: &Value) -> Citation_DateOfPublication {
        Citation_DateOfPublication {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for date
    pub fn _date(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_date") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for day
    pub fn _day(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_day") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for month
    pub fn _month(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_month") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for season
    pub fn _season(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_season") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for text
    pub fn _text(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_text") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for year
    pub fn _year(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_year") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Date on which the issue of the journal was published.
    pub fn date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("date") {
            return Some(string);
        }
        return None;
    }

    /// Day on which the issue of the journal was published.
    pub fn day(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("day") {
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

    /// Month on which the issue of the journal was published.
    pub fn month(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("month") {
            return Some(string);
        }
        return None;
    }

    /// Spring, Summer, Fall/Autumn, Winter.
    pub fn season(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("season") {
            return Some(string);
        }
        return None;
    }

    /// Text representation of the date of which the issue of the journal was published.
    pub fn text(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("text") {
            return Some(string);
        }
        return None;
    }

    /// Year on which the issue of the journal was published.
    pub fn year(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("year") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._date() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._day() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._month() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._season() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._text() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._year() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.date() {}
        if let Some(_val) = self.day() {}
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
        if let Some(_val) = self.month() {}
        if let Some(_val) = self.season() {}
        if let Some(_val) = self.text() {}
        if let Some(_val) = self.year() {}
        return true;
    }
}

#[derive(Debug)]
pub struct Citation_DateOfPublicationBuilder {
    pub(crate) value: Value,
}

impl Citation_DateOfPublicationBuilder {
    pub fn build(&self) -> Citation_DateOfPublication {
        Citation_DateOfPublication {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Citation_DateOfPublication) -> Citation_DateOfPublicationBuilder {
        Citation_DateOfPublicationBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> Citation_DateOfPublicationBuilder {
        let mut __value: Value = json!({});
        return Citation_DateOfPublicationBuilder { value: __value };
    }

    pub fn _date<'a>(&'a mut self, val: Element) -> &'a mut Citation_DateOfPublicationBuilder {
        self.value["_date"] = json!(val.value);
        return self;
    }

    pub fn _day<'a>(&'a mut self, val: Element) -> &'a mut Citation_DateOfPublicationBuilder {
        self.value["_day"] = json!(val.value);
        return self;
    }

    pub fn _month<'a>(&'a mut self, val: Element) -> &'a mut Citation_DateOfPublicationBuilder {
        self.value["_month"] = json!(val.value);
        return self;
    }

    pub fn _season<'a>(&'a mut self, val: Element) -> &'a mut Citation_DateOfPublicationBuilder {
        self.value["_season"] = json!(val.value);
        return self;
    }

    pub fn _text<'a>(&'a mut self, val: Element) -> &'a mut Citation_DateOfPublicationBuilder {
        self.value["_text"] = json!(val.value);
        return self;
    }

    pub fn _year<'a>(&'a mut self, val: Element) -> &'a mut Citation_DateOfPublicationBuilder {
        self.value["_year"] = json!(val.value);
        return self;
    }

    pub fn date<'a>(&'a mut self, val: &str) -> &'a mut Citation_DateOfPublicationBuilder {
        self.value["date"] = json!(val);
        return self;
    }

    pub fn day<'a>(&'a mut self, val: &str) -> &'a mut Citation_DateOfPublicationBuilder {
        self.value["day"] = json!(val);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut Citation_DateOfPublicationBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut Citation_DateOfPublicationBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut Citation_DateOfPublicationBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn month<'a>(&'a mut self, val: &str) -> &'a mut Citation_DateOfPublicationBuilder {
        self.value["month"] = json!(val);
        return self;
    }

    pub fn season<'a>(&'a mut self, val: &str) -> &'a mut Citation_DateOfPublicationBuilder {
        self.value["season"] = json!(val);
        return self;
    }

    pub fn text<'a>(&'a mut self, val: &str) -> &'a mut Citation_DateOfPublicationBuilder {
        self.value["text"] = json!(val);
        return self;
    }

    pub fn year<'a>(&'a mut self, val: &str) -> &'a mut Citation_DateOfPublicationBuilder {
        self.value["year"] = json!(val);
        return self;
    }
}
