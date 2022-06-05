#![allow(unused_imports, non_camel_case_types)]

use crate::models::r4b::Citation_PeriodicRelease::Citation_PeriodicRelease;
use crate::models::r4b::Citation_PublishedIn::Citation_PublishedIn;
use crate::models::r4b::CodeableConcept::CodeableConcept;
use crate::models::r4b::Element::Element;
use crate::models::r4b::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// The Citation Resource enables reference to any knowledge artifact for purposes of
/// identification and attribution. The Citation Resource supports existing reference
/// structures and developing publication practices such as versioning, expressing
/// complex contributorship roles, and referencing computable resources.

#[derive(Debug)]
pub struct Citation_PublicationForm<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Citation_PublicationForm<'_> {
    pub fn new(value: &Value) -> Citation_PublicationForm {
        Citation_PublicationForm {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for accessionNumber
    pub fn _accession_number(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_accessionNumber") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for articleDate
    pub fn _article_date(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_articleDate") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for copyright
    pub fn _copyright(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_copyright") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for firstPage
    pub fn _first_page(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_firstPage") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for lastPage
    pub fn _last_page(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_lastPage") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for lastRevisionDate
    pub fn _last_revision_date(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_lastRevisionDate") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for pageCount
    pub fn _page_count(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_pageCount") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for pageString
    pub fn _page_string(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_pageString") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Entry number or identifier for inclusion in a database.
    pub fn accession_number(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("accessionNumber") {
            return Some(string);
        }
        return None;
    }

    /// The date the article was added to the database, or the date the article was
    /// released (which may differ from the journal issue publication date).
    pub fn article_date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("articleDate") {
            return Some(string);
        }
        return None;
    }

    /// Copyright notice for the full article or artifact.
    pub fn copyright(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("copyright") {
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

    /// Used for isolated representation of first page.
    pub fn first_page(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("firstPage") {
            return Some(string);
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

    /// Language in which this form of the article is published.
    pub fn language(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("language") {
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

    /// Used for isolated representation of last page.
    pub fn last_page(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("lastPage") {
            return Some(string);
        }
        return None;
    }

    /// The date the article was last revised or updated in the database.
    pub fn last_revision_date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("lastRevisionDate") {
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

    /// Actual or approximate number of pages or screens.
    pub fn page_count(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("pageCount") {
            return Some(string);
        }
        return None;
    }

    /// Used for full display of pagination.
    pub fn page_string(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("pageString") {
            return Some(string);
        }
        return None;
    }

    /// The specific issue in which the cited article resides.
    pub fn periodic_release(&self) -> Option<Citation_PeriodicRelease> {
        if let Some(val) = self.value.get("periodicRelease") {
            return Some(Citation_PeriodicRelease {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The collection the cited article or artifact is published in.
    pub fn published_in(&self) -> Option<Citation_PublishedIn> {
        if let Some(val) = self.value.get("publishedIn") {
            return Some(Citation_PublishedIn {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._accession_number() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._article_date() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._copyright() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._first_page() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._last_page() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._last_revision_date() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._page_count() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._page_string() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.accession_number() {}
        if let Some(_val) = self.article_date() {}
        if let Some(_val) = self.copyright() {}
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.first_page() {}
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.language() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.last_page() {}
        if let Some(_val) = self.last_revision_date() {}
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.page_count() {}
        if let Some(_val) = self.page_string() {}
        if let Some(_val) = self.periodic_release() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.published_in() {
            if !_val.validate() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct Citation_PublicationFormBuilder {
    pub(crate) value: Value,
}

impl Citation_PublicationFormBuilder {
    pub fn build(&self) -> Citation_PublicationForm {
        Citation_PublicationForm {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Citation_PublicationForm) -> Citation_PublicationFormBuilder {
        Citation_PublicationFormBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> Citation_PublicationFormBuilder {
        let mut __value: Value = json!({});
        return Citation_PublicationFormBuilder { value: __value };
    }

    pub fn _accession_number<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut Citation_PublicationFormBuilder {
        self.value["_accessionNumber"] = json!(val.value);
        return self;
    }

    pub fn _article_date<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut Citation_PublicationFormBuilder {
        self.value["_articleDate"] = json!(val.value);
        return self;
    }

    pub fn _copyright<'a>(&'a mut self, val: Element) -> &'a mut Citation_PublicationFormBuilder {
        self.value["_copyright"] = json!(val.value);
        return self;
    }

    pub fn _first_page<'a>(&'a mut self, val: Element) -> &'a mut Citation_PublicationFormBuilder {
        self.value["_firstPage"] = json!(val.value);
        return self;
    }

    pub fn _last_page<'a>(&'a mut self, val: Element) -> &'a mut Citation_PublicationFormBuilder {
        self.value["_lastPage"] = json!(val.value);
        return self;
    }

    pub fn _last_revision_date<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut Citation_PublicationFormBuilder {
        self.value["_lastRevisionDate"] = json!(val.value);
        return self;
    }

    pub fn _page_count<'a>(&'a mut self, val: Element) -> &'a mut Citation_PublicationFormBuilder {
        self.value["_pageCount"] = json!(val.value);
        return self;
    }

    pub fn _page_string<'a>(&'a mut self, val: Element) -> &'a mut Citation_PublicationFormBuilder {
        self.value["_pageString"] = json!(val.value);
        return self;
    }

    pub fn accession_number<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut Citation_PublicationFormBuilder {
        self.value["accessionNumber"] = json!(val);
        return self;
    }

    pub fn article_date<'a>(&'a mut self, val: &str) -> &'a mut Citation_PublicationFormBuilder {
        self.value["articleDate"] = json!(val);
        return self;
    }

    pub fn copyright<'a>(&'a mut self, val: &str) -> &'a mut Citation_PublicationFormBuilder {
        self.value["copyright"] = json!(val);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut Citation_PublicationFormBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn first_page<'a>(&'a mut self, val: &str) -> &'a mut Citation_PublicationFormBuilder {
        self.value["firstPage"] = json!(val);
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut Citation_PublicationFormBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn language<'a>(
        &'a mut self,
        val: Vec<CodeableConcept>,
    ) -> &'a mut Citation_PublicationFormBuilder {
        self.value["language"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn last_page<'a>(&'a mut self, val: &str) -> &'a mut Citation_PublicationFormBuilder {
        self.value["lastPage"] = json!(val);
        return self;
    }

    pub fn last_revision_date<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut Citation_PublicationFormBuilder {
        self.value["lastRevisionDate"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut Citation_PublicationFormBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn page_count<'a>(&'a mut self, val: &str) -> &'a mut Citation_PublicationFormBuilder {
        self.value["pageCount"] = json!(val);
        return self;
    }

    pub fn page_string<'a>(&'a mut self, val: &str) -> &'a mut Citation_PublicationFormBuilder {
        self.value["pageString"] = json!(val);
        return self;
    }

    pub fn periodic_release<'a>(
        &'a mut self,
        val: Citation_PeriodicRelease,
    ) -> &'a mut Citation_PublicationFormBuilder {
        self.value["periodicRelease"] = json!(val.value);
        return self;
    }

    pub fn published_in<'a>(
        &'a mut self,
        val: Citation_PublishedIn,
    ) -> &'a mut Citation_PublicationFormBuilder {
        self.value["publishedIn"] = json!(val.value);
        return self;
    }
}
