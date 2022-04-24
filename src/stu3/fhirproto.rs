/// Whether or not the primitive is missing a value.
/// See <https://g.co/fhir/StructureDefinition/primitiveHasNoValue>
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrimitiveHasNoValue {
    /// xml:id (or equivalent in JSON)
    #[prost(message, optional, tag="1")]
    pub id: ::core::option::Option<super::proto::String>,
    /// Value of extension
    #[prost(message, optional, tag="3")]
    pub value_boolean: ::core::option::Option<super::proto::Boolean>,
}
/// Base64Binary rendering parameters.
/// See <https://g.co/fhir/StructureDefinition/base64Binary-separatorStride>
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Base64BinarySeparatorStride {
    /// xml:id (or equivalent in JSON)
    #[prost(message, optional, tag="1")]
    pub id: ::core::option::Option<super::proto::String>,
    /// The separator, usually a sequence of spaces.
    #[prost(message, optional, tag="4")]
    pub separator: ::core::option::Option<super::proto::String>,
    /// The stride.
    #[prost(message, optional, tag="5")]
    pub stride: ::core::option::Option<super::proto::PositiveInt>,
}
