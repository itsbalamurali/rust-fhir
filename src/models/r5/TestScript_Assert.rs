#![allow(unused_imports, non_camel_case_types)]

use crate::models::r5::Element::Element;
use crate::models::r5::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A structured set of tests against a FHIR server or client implementation to
/// determine compliance against the FHIR specification.

#[derive(Debug)]
pub struct TestScript_Assert<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl TestScript_Assert<'_> {
    pub fn new(value: &Value) -> TestScript_Assert {
        TestScript_Assert {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for compareToSourceExpression
    pub fn _compare_to_source_expression(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_compareToSourceExpression") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for compareToSourceId
    pub fn _compare_to_source_id(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_compareToSourceId") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for compareToSourcePath
    pub fn _compare_to_source_path(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_compareToSourcePath") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for contentType
    pub fn _content_type(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_contentType") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
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

    /// Extensions for direction
    pub fn _direction(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_direction") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for expression
    pub fn _expression(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_expression") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for headerField
    pub fn _header_field(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_headerField") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for label
    pub fn _label(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_label") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for minimumId
    pub fn _minimum_id(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_minimumId") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for navigationLinks
    pub fn _navigation_links(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_navigationLinks") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for operator
    pub fn _operator(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_operator") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for path
    pub fn _path(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_path") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for requestMethod
    pub fn _request_method(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_requestMethod") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for requestURL
    pub fn _request_u_r_l(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_requestURL") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for resource
    pub fn _resource(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_resource") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for response
    pub fn _response(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_response") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for responseCode
    pub fn _response_code(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_responseCode") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for sourceId
    pub fn _source_id(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_sourceId") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for stopTestOnFail
    pub fn _stop_test_on_fail(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_stopTestOnFail") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for validateProfileId
    pub fn _validate_profile_id(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_validateProfileId") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for value
    pub fn _value(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_value") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for warningOnly
    pub fn _warning_only(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_warningOnly") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The FHIRPath expression to evaluate against the source fixture. When
    /// compareToSourceId is defined, either compareToSourceExpression or
    /// compareToSourcePath must be defined, but not both.
    pub fn compare_to_source_expression(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("compareToSourceExpression") {
            return Some(string);
        }
        return None;
    }

    /// Id of the source fixture used as the contents to be evaluated by either the
    /// "source/expression" or "sourceId/path" definition.
    pub fn compare_to_source_id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("compareToSourceId") {
            return Some(string);
        }
        return None;
    }

    /// XPath or JSONPath expression to evaluate against the source fixture. When
    /// compareToSourceId is defined, either compareToSourceExpression or
    /// compareToSourcePath must be defined, but not both.
    pub fn compare_to_source_path(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("compareToSourcePath") {
            return Some(string);
        }
        return None;
    }

    /// The mime-type contents to compare against the request or response message
    /// 'Content-Type' header.
    pub fn content_type(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("contentType") {
            return Some(string);
        }
        return None;
    }

    /// The description would be used by test engines for tracking and reporting
    /// purposes.
    pub fn description(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("description") {
            return Some(string);
        }
        return None;
    }

    /// The direction to use for the assertion.
    pub fn direction(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("direction") {
            return Some(string);
        }
        return None;
    }

    /// The FHIRPath expression to be evaluated against the request or response message
    /// contents - HTTP headers and payload.
    pub fn expression(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("expression") {
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

    /// The HTTP header field name e.g. 'Location'.
    pub fn header_field(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("headerField") {
            return Some(string);
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

    /// The label would be used for tracking/logging purposes by test engines.
    pub fn label(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("label") {
            return Some(string);
        }
        return None;
    }

    /// The ID of a fixture.  Asserts that the response contains at a minimum the
    /// fixture specified by minimumId.
    pub fn minimum_id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("minimumId") {
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

    /// Whether or not the test execution performs validation on the bundle navigation
    /// links.
    pub fn navigation_links(&self) -> Option<bool> {
        if let Some(val) = self.value.get("navigationLinks") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    /// The operator type defines the conditional behavior of the assert. If not
    /// defined, the default is equals.
    pub fn operator(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("operator") {
            return Some(string);
        }
        return None;
    }

    /// The XPath or JSONPath expression to be evaluated against the fixture
    /// representing the response received from server.
    pub fn path(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("path") {
            return Some(string);
        }
        return None;
    }

    /// The request method or HTTP operation code to compare against that used by the
    /// client system under test.
    pub fn request_method(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("requestMethod") {
            return Some(string);
        }
        return None;
    }

    /// The value to use in a comparison against the request URL path string.
    pub fn request_u_r_l(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("requestURL") {
            return Some(string);
        }
        return None;
    }

    /// The type of the resource.  See http://build.fhir.org/resourcelist.html.
    pub fn resource(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("resource") {
            return Some(string);
        }
        return None;
    }

    /// okay | created | noContent | notModified | bad | forbidden | notFound |
    /// methodNotAllowed | conflict | gone | preconditionFailed | unprocessable.
    pub fn response(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("response") {
            return Some(string);
        }
        return None;
    }

    /// The value of the HTTP response code to be tested.
    pub fn response_code(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("responseCode") {
            return Some(string);
        }
        return None;
    }

    /// Fixture to evaluate the XPath/JSONPath expression or the headerField  against.
    pub fn source_id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("sourceId") {
            return Some(string);
        }
        return None;
    }

    /// Whether or not the current test execution will stop on failure for this assert.
    pub fn stop_test_on_fail(&self) -> Option<bool> {
        if let Some(val) = self.value.get("stopTestOnFail") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    /// The ID of the Profile to validate against.
    pub fn validate_profile_id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("validateProfileId") {
            return Some(string);
        }
        return None;
    }

    /// The value to compare to.
    pub fn value(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("value") {
            return Some(string);
        }
        return None;
    }

    /// Whether or not the test execution will produce a warning only on error for this
    /// assert.
    pub fn warning_only(&self) -> Option<bool> {
        if let Some(val) = self.value.get("warningOnly") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._compare_to_source_expression() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._compare_to_source_id() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._compare_to_source_path() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._content_type() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._description() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._direction() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._expression() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._header_field() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._label() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._minimum_id() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._navigation_links() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._operator() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._path() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._request_method() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._request_u_r_l() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._resource() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._response() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._response_code() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._source_id() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._stop_test_on_fail() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._validate_profile_id() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._value() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._warning_only() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.compare_to_source_expression() {}
        if let Some(_val) = self.compare_to_source_id() {}
        if let Some(_val) = self.compare_to_source_path() {}
        if let Some(_val) = self.content_type() {}
        if let Some(_val) = self.description() {}
        if let Some(_val) = self.direction() {}
        if let Some(_val) = self.expression() {}
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.header_field() {}
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.label() {}
        if let Some(_val) = self.minimum_id() {}
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.navigation_links() {}
        if let Some(_val) = self.operator() {}
        if let Some(_val) = self.path() {}
        if let Some(_val) = self.request_method() {}
        if let Some(_val) = self.request_u_r_l() {}
        if let Some(_val) = self.resource() {}
        if let Some(_val) = self.response() {}
        if let Some(_val) = self.response_code() {}
        if let Some(_val) = self.source_id() {}
        if let Some(_val) = self.stop_test_on_fail() {}
        if let Some(_val) = self.validate_profile_id() {}
        if let Some(_val) = self.value() {}
        if let Some(_val) = self.warning_only() {}
        return true;
    }
}

#[derive(Debug)]
pub struct TestScript_AssertBuilder {
    pub(crate) value: Value,
}

impl TestScript_AssertBuilder {
    pub fn build(&self) -> TestScript_Assert {
        TestScript_Assert {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: TestScript_Assert) -> TestScript_AssertBuilder {
        TestScript_AssertBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> TestScript_AssertBuilder {
        let mut __value: Value = json!({});
        return TestScript_AssertBuilder { value: __value };
    }

    pub fn _compare_to_source_expression<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut TestScript_AssertBuilder {
        self.value["_compareToSourceExpression"] = json!(val.value);
        return self;
    }

    pub fn _compare_to_source_id<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut TestScript_AssertBuilder {
        self.value["_compareToSourceId"] = json!(val.value);
        return self;
    }

    pub fn _compare_to_source_path<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut TestScript_AssertBuilder {
        self.value["_compareToSourcePath"] = json!(val.value);
        return self;
    }

    pub fn _content_type<'a>(&'a mut self, val: Element) -> &'a mut TestScript_AssertBuilder {
        self.value["_contentType"] = json!(val.value);
        return self;
    }

    pub fn _description<'a>(&'a mut self, val: Element) -> &'a mut TestScript_AssertBuilder {
        self.value["_description"] = json!(val.value);
        return self;
    }

    pub fn _direction<'a>(&'a mut self, val: Element) -> &'a mut TestScript_AssertBuilder {
        self.value["_direction"] = json!(val.value);
        return self;
    }

    pub fn _expression<'a>(&'a mut self, val: Element) -> &'a mut TestScript_AssertBuilder {
        self.value["_expression"] = json!(val.value);
        return self;
    }

    pub fn _header_field<'a>(&'a mut self, val: Element) -> &'a mut TestScript_AssertBuilder {
        self.value["_headerField"] = json!(val.value);
        return self;
    }

    pub fn _label<'a>(&'a mut self, val: Element) -> &'a mut TestScript_AssertBuilder {
        self.value["_label"] = json!(val.value);
        return self;
    }

    pub fn _minimum_id<'a>(&'a mut self, val: Element) -> &'a mut TestScript_AssertBuilder {
        self.value["_minimumId"] = json!(val.value);
        return self;
    }

    pub fn _navigation_links<'a>(&'a mut self, val: Element) -> &'a mut TestScript_AssertBuilder {
        self.value["_navigationLinks"] = json!(val.value);
        return self;
    }

    pub fn _operator<'a>(&'a mut self, val: Element) -> &'a mut TestScript_AssertBuilder {
        self.value["_operator"] = json!(val.value);
        return self;
    }

    pub fn _path<'a>(&'a mut self, val: Element) -> &'a mut TestScript_AssertBuilder {
        self.value["_path"] = json!(val.value);
        return self;
    }

    pub fn _request_method<'a>(&'a mut self, val: Element) -> &'a mut TestScript_AssertBuilder {
        self.value["_requestMethod"] = json!(val.value);
        return self;
    }

    pub fn _request_u_r_l<'a>(&'a mut self, val: Element) -> &'a mut TestScript_AssertBuilder {
        self.value["_requestURL"] = json!(val.value);
        return self;
    }

    pub fn _resource<'a>(&'a mut self, val: Element) -> &'a mut TestScript_AssertBuilder {
        self.value["_resource"] = json!(val.value);
        return self;
    }

    pub fn _response<'a>(&'a mut self, val: Element) -> &'a mut TestScript_AssertBuilder {
        self.value["_response"] = json!(val.value);
        return self;
    }

    pub fn _response_code<'a>(&'a mut self, val: Element) -> &'a mut TestScript_AssertBuilder {
        self.value["_responseCode"] = json!(val.value);
        return self;
    }

    pub fn _source_id<'a>(&'a mut self, val: Element) -> &'a mut TestScript_AssertBuilder {
        self.value["_sourceId"] = json!(val.value);
        return self;
    }

    pub fn _stop_test_on_fail<'a>(&'a mut self, val: Element) -> &'a mut TestScript_AssertBuilder {
        self.value["_stopTestOnFail"] = json!(val.value);
        return self;
    }

    pub fn _validate_profile_id<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut TestScript_AssertBuilder {
        self.value["_validateProfileId"] = json!(val.value);
        return self;
    }

    pub fn _value<'a>(&'a mut self, val: Element) -> &'a mut TestScript_AssertBuilder {
        self.value["_value"] = json!(val.value);
        return self;
    }

    pub fn _warning_only<'a>(&'a mut self, val: Element) -> &'a mut TestScript_AssertBuilder {
        self.value["_warningOnly"] = json!(val.value);
        return self;
    }

    pub fn compare_to_source_expression<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut TestScript_AssertBuilder {
        self.value["compareToSourceExpression"] = json!(val);
        return self;
    }

    pub fn compare_to_source_id<'a>(&'a mut self, val: &str) -> &'a mut TestScript_AssertBuilder {
        self.value["compareToSourceId"] = json!(val);
        return self;
    }

    pub fn compare_to_source_path<'a>(&'a mut self, val: &str) -> &'a mut TestScript_AssertBuilder {
        self.value["compareToSourcePath"] = json!(val);
        return self;
    }

    pub fn content_type<'a>(&'a mut self, val: &str) -> &'a mut TestScript_AssertBuilder {
        self.value["contentType"] = json!(val);
        return self;
    }

    pub fn description<'a>(&'a mut self, val: &str) -> &'a mut TestScript_AssertBuilder {
        self.value["description"] = json!(val);
        return self;
    }

    pub fn direction<'a>(&'a mut self, val: &str) -> &'a mut TestScript_AssertBuilder {
        self.value["direction"] = json!(val);
        return self;
    }

    pub fn expression<'a>(&'a mut self, val: &str) -> &'a mut TestScript_AssertBuilder {
        self.value["expression"] = json!(val);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut TestScript_AssertBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn header_field<'a>(&'a mut self, val: &str) -> &'a mut TestScript_AssertBuilder {
        self.value["headerField"] = json!(val);
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut TestScript_AssertBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn label<'a>(&'a mut self, val: &str) -> &'a mut TestScript_AssertBuilder {
        self.value["label"] = json!(val);
        return self;
    }

    pub fn minimum_id<'a>(&'a mut self, val: &str) -> &'a mut TestScript_AssertBuilder {
        self.value["minimumId"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut TestScript_AssertBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn navigation_links<'a>(&'a mut self, val: bool) -> &'a mut TestScript_AssertBuilder {
        self.value["navigationLinks"] = json!(val);
        return self;
    }

    pub fn operator<'a>(&'a mut self, val: &str) -> &'a mut TestScript_AssertBuilder {
        self.value["operator"] = json!(val);
        return self;
    }

    pub fn path<'a>(&'a mut self, val: &str) -> &'a mut TestScript_AssertBuilder {
        self.value["path"] = json!(val);
        return self;
    }

    pub fn request_method<'a>(&'a mut self, val: &str) -> &'a mut TestScript_AssertBuilder {
        self.value["requestMethod"] = json!(val);
        return self;
    }

    pub fn request_u_r_l<'a>(&'a mut self, val: &str) -> &'a mut TestScript_AssertBuilder {
        self.value["requestURL"] = json!(val);
        return self;
    }

    pub fn resource<'a>(&'a mut self, val: &str) -> &'a mut TestScript_AssertBuilder {
        self.value["resource"] = json!(val);
        return self;
    }

    pub fn response<'a>(&'a mut self, val: &str) -> &'a mut TestScript_AssertBuilder {
        self.value["response"] = json!(val);
        return self;
    }

    pub fn response_code<'a>(&'a mut self, val: &str) -> &'a mut TestScript_AssertBuilder {
        self.value["responseCode"] = json!(val);
        return self;
    }

    pub fn source_id<'a>(&'a mut self, val: &str) -> &'a mut TestScript_AssertBuilder {
        self.value["sourceId"] = json!(val);
        return self;
    }

    pub fn stop_test_on_fail<'a>(&'a mut self, val: bool) -> &'a mut TestScript_AssertBuilder {
        self.value["stopTestOnFail"] = json!(val);
        return self;
    }

    pub fn validate_profile_id<'a>(&'a mut self, val: &str) -> &'a mut TestScript_AssertBuilder {
        self.value["validateProfileId"] = json!(val);
        return self;
    }

    pub fn value<'a>(&'a mut self, val: &str) -> &'a mut TestScript_AssertBuilder {
        self.value["value"] = json!(val);
        return self;
    }

    pub fn warning_only<'a>(&'a mut self, val: bool) -> &'a mut TestScript_AssertBuilder {
        self.value["warningOnly"] = json!(val);
        return self;
    }
}
