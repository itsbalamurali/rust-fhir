/// Auto-generated from StructureDefinition for ValidationOutcome.
/// Information about the success/failure of an action.
/// See
/// <https://g.co/fhir/StructureDefinition/StructureDefinition/ValidationOutcome>
#[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct ValidationOutcome {
    /// Logical id of this artifact
    #[prost(message, optional, tag="1")]
    pub id: ::core::option::Option<super::core::Id>,
    /// Metadata about the resource
    #[prost(message, optional, tag="2")]
    pub meta: ::core::option::Option<super::core::Meta>,
    /// A set of rules under which this content was created
    #[prost(message, optional, tag="3")]
    pub implicit_rules: ::core::option::Option<super::core::Uri>,
    /// Language of the resource content
    #[prost(message, optional, tag="4")]
    pub language: ::core::option::Option<super::core::Code>,
    /// Text summary of the resource, for human interpretation
    #[prost(message, optional, tag="5")]
    pub text: ::core::option::Option<super::core::Narrative>,
    /// Contained, inline Resources
    #[prost(message, repeated, tag="6")]
    pub contained: prost::alloc::vec::Vec<::prost_wkt_types::Any>,
    /// Additional content defined by implementations
    #[prost(message, repeated, tag="8")]
    pub extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// Extensions that cannot be ignored
    #[prost(message, repeated, tag="9")]
    pub modifier_extension: prost::alloc::vec::Vec<super::core::Extension>,
    #[prost(message, repeated, tag="10")]
    pub issue: prost::alloc::vec::Vec<validation_outcome::Issue>,
    /// The FHIR resource this Validation is for.
    #[prost(message, optional, tag="11")]
    pub subject: ::core::option::Option<super::core::Reference>,
}
/// Nested message and enum types in `ValidationOutcome`.
pub mod validation_outcome {
    /// A single issue associated with the action
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct Issue {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag="1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag="2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag="3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        #[prost(message, optional, tag="4")]
        pub severity: ::core::option::Option<issue::SeverityCode>,
        #[prost(message, optional, tag="5")]
        pub code: ::core::option::Option<issue::CodeType>,
        /// Additional details about the error
        #[prost(message, optional, tag="6")]
        pub details: ::core::option::Option<super::super::core::CodeableConcept>,
        /// Additional diagnostic information about the issue
        #[prost(message, optional, tag="7")]
        pub diagnostics: ::core::option::Option<super::super::core::String>,
        /// Deprecated: Path of element(s) related to issue
        #[prost(message, repeated, tag="8")]
        pub location: prost::alloc::vec::Vec<super::super::core::String>,
        /// FHIRPath of element(s) related to issue
        #[prost(message, repeated, tag="9")]
        pub expression: prost::alloc::vec::Vec<super::super::core::String>,
    }
    /// Nested message and enum types in `Issue`.
    pub mod issue {
        /// fatal | error | warning | information
        #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Message)]
        pub struct SeverityCode {
            #[prost(enumeration="super::super::super::core::issue_severity_code::Value", tag="1")]
            pub value: i32,
            #[prost(message, optional, tag="2")]
            pub id: ::core::option::Option<super::super::super::core::String>,
            #[prost(message, repeated, tag="3")]
            pub extension: prost::alloc::vec::Vec<super::super::super::core::Extension>,
        }
        /// Error or warning code
        #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Message)]
        pub struct CodeType {
            #[prost(enumeration="super::super::super::core::issue_type_code::Value", tag="1")]
            pub value: i32,
            #[prost(message, optional, tag="2")]
            pub id: ::core::option::Option<super::super::super::core::String>,
            #[prost(message, repeated, tag="3")]
            pub extension: prost::alloc::vec::Vec<super::super::super::core::Extension>,
        }
    }
}
/// Auto-generated from StructureDefinition for PrimitiveHasNoValue.
/// Whether or not the primitive is missing a value.
/// See <https://g.co/fhir/StructureDefinition/primitiveHasNoValue>
#[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct PrimitiveHasNoValue {
    /// Unique id for inter-element referencing
    #[prost(message, optional, tag="1")]
    pub id: ::core::option::Option<super::core::String>,
    /// Value of extension
    #[prost(message, optional, tag="3")]
    pub value_boolean: ::core::option::Option<super::core::Boolean>,
}
/// Auto-generated from StructureDefinition for OperationOutcomeSubject.
/// Indicates the FHIR resource subject of this OperationOutcome.
/// See <https://g.co/fhir/medicalrecords/OperationOutcomeSubject>
#[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct OperationOutcomeSubject {
    /// Unique id for inter-element referencing
    #[prost(message, optional, tag="1")]
    pub id: ::core::option::Option<super::core::String>,
    /// Value of extension
    #[prost(message, optional, tag="3")]
    pub value_reference: ::core::option::Option<super::core::Reference>,
}
/// Auto-generated from StructureDefinition for Base64BinarySeparatorStride.
/// Base64Binary rendering parameters.
/// See <https://g.co/fhir/StructureDefinition/base64Binary-separatorStride>
#[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct Base64BinarySeparatorStride {
    /// Unique id for inter-element referencing
    #[prost(message, optional, tag="1")]
    pub id: ::core::option::Option<super::core::String>,
    /// The separator, usually a sequence of spaces.
    #[prost(message, optional, tag="4")]
    pub separator: ::core::option::Option<super::core::String>,
    /// The stride.
    #[prost(message, optional, tag="5")]
    pub stride: ::core::option::Option<super::core::PositiveInt>,
}
