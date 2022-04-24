/// Auto-generated from StructureDefinition for EventLabel.
/// EventLabels define labels used for TensorFlow model training and evaluation.
/// See <https://g.co/fhir/StructureDefinition/eventLabel>
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventLabel {
    /// Unique id for inter-element referencing
    #[prost(message, optional, tag="1")]
    pub id: ::core::option::Option<super::core::String>,
    /// Additional content defined by implementations
    #[prost(message, repeated, tag="2")]
    pub extension: ::prost::alloc::vec::Vec<super::core::Extension>,
    /// The patient associated with this label
    #[prost(message, optional, tag="4")]
    pub patient: ::core::option::Option<super::core::Reference>,
    /// The type of label, part of the prediction task definition
    #[prost(message, optional, tag="5")]
    pub r#type: ::core::option::Option<super::core::Coding>,
    /// Time associated with the label event, if available
    #[prost(message, optional, tag="6")]
    pub event_time: ::core::option::Option<super::core::DateTime>,
    /// Resource that owns this label
    #[prost(message, optional, tag="7")]
    pub source: ::core::option::Option<super::core::Reference>,
    #[prost(message, repeated, tag="8")]
    pub label: ::prost::alloc::vec::Vec<event_label::Label>,
}
/// Nested message and enum types in `EventLabel`.
pub mod event_label {
    /// List of labels associated with this event
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Label {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag="1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Class name in multi-class prediction tasks, e.g. code "780.60" for icd9
        #[prost(message, optional, tag="4")]
        pub class_name: ::core::option::Option<super::super::core::Coding>,
        #[prost(message, optional, tag="5")]
        pub class_value: ::core::option::Option<label::ClassValueX>,
    }
    /// Nested message and enum types in `Label`.
    pub mod label {
        /// The value of the label
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ClassValueX {
            #[prost(oneof="class_value_x::Choice", tags="1, 2, 3, 4, 5")]
            pub choice: ::core::option::Option<class_value_x::Choice>,
        }
        /// Nested message and enum types in `ClassValueX`.
        pub mod class_value_x {
            #[derive(Clone, PartialEq, ::prost::Oneof)]
            pub enum Choice {
                #[prost(message, tag="1")]
                Boolean(super::super::super::super::core::Boolean),
                #[prost(message, tag="2")]
                Decimal(super::super::super::super::core::Decimal),
                #[prost(message, tag="3")]
                Integer(super::super::super::super::core::Integer),
                #[prost(message, tag="4")]
                StringValue(super::super::super::super::core::String),
                #[prost(message, tag="5")]
                DateTime(super::super::super::super::core::DateTime),
            }
        }
    }
}
/// Auto-generated from StructureDefinition for EventTrigger.
/// EventTriggers specify cutoff times for generated TensorFlow examples.
/// See <https://g.co/fhir/StructureDefinition/eventTrigger>
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventTrigger {
    /// Unique id for inter-element referencing
    #[prost(message, optional, tag="1")]
    pub id: ::core::option::Option<super::core::String>,
    /// The type of trigger, part of the prediction task definition.
    #[prost(message, optional, tag="4")]
    pub r#type: ::core::option::Option<super::core::Coding>,
    /// Prediction event time, more recent data will be elided.
    #[prost(message, optional, tag="5")]
    pub event_time: ::core::option::Option<super::core::DateTime>,
    /// Resource that owns this trigger.
    #[prost(message, optional, tag="6")]
    pub source: ::core::option::Option<super::core::Reference>,
}
