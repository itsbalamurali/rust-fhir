/// The RAND scoring for appropriateness of the procedure.
/// See <http://hl7.org/fhir/us/qicore/CodeSystem/appropriateness-score>
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct AppropriatenessScoreCode {}
/// Nested message and enum types in `AppropriatenessScoreCode`.
pub mod appropriateness_score_code {
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, prost::Enumeration)]
    #[repr(i32)]
    pub enum Value {
        InvalidUninitialized = 0,
        ExtremelyInappropriate = 1,
        Inappropriate = 2,
        ProbablyInappropriate = 3,
        UncertainInappropriate = 4,
        Uncertain = 5,
        UncertainAppropriate = 6,
        ProbablyAppropriate = 7,
        Appropriate = 8,
        ExtremelyAppropriate = 9,
    }
}
/// Value Set for QICore Communication Medium (Example)
/// See <http://hl7.org/fhir/us/qicore/CodeSystem/communication-medium>
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct CommunicationMediumCode {}
/// Nested message and enum types in `CommunicationMediumCode`.
pub mod communication_medium_code {
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, prost::Enumeration)]
    #[repr(i32)]
    pub enum Value {
        InvalidUninitialized = 0,
        Unspecified = 1,
        Telephone = 2,
        Fax = 3,
        Device = 4,
        Video = 5,
        Voicemail = 6,
        Text = 7,
        SocialMedia = 8,
        InPerson = 9,
        Mail = 10,
        Email = 11,
        Portal = 12,
    }
}
/// Value Set for QICore Condition Criticality (Example)
/// See <http://hl7.org/fhir/us/qicore/CodeSystem/condition-criticality>
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct ConditionCriticalityCode {}
/// Nested message and enum types in `ConditionCriticalityCode`.
pub mod condition_criticality_code {
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, prost::Enumeration)]
    #[repr(i32)]
    pub enum Value {
        InvalidUninitialized = 0,
        Unspecified = 1,
        SelfResolving = 2,
        Controllable = 3,
        FunctionalLoss = 4,
        LifeThreatening = 5,
        RequiresHospitalization = 6,
    }
}
/// The condition, state, or problem that the patient is in or has prior to a
/// therapy or procedure. This captures temporal (temporary circumstances) that
/// have bearing on the data that it qualifies but will not necessarily modify
/// its meaning. Things like 'after activity', 'at rest', or 'post-op'. See
/// <http://hl7.org/fhir/us/qicore/CodeSystem/diagnosticorder-precondition>
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct DiagnosticOrderPreconditionCode {}
/// Nested message and enum types in `DiagnosticOrderPreconditionCode`.
pub mod diagnostic_order_precondition_code {
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, prost::Enumeration)]
    #[repr(i32)]
    pub enum Value {
        InvalidUninitialized = 0,
        AfterActivity = 1,
        AtRest = 2,
        PostOp = 3,
    }
}
/// Military service status codes
/// See <http://hl7.org/fhir/us/qicore/CodeSystem/military-service>
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct MilitaryServiceCode {}
/// Nested message and enum types in `MilitaryServiceCode`.
pub mod military_service_code {
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, prost::Enumeration)]
    #[repr(i32)]
    pub enum Value {
        InvalidUninitialized = 0,
        NotIndicated = 1,
        NoMilitaryService = 2,
        Veteran = 3,
        ActiveDuty = 4,
        ActiveReserve = 5,
        InactiveReserve = 6,
    }
}
/// Code for method by which the observation result was validated, e.g., human
/// review, sliding average. See
/// <http://hl7.org/fhir/us/qicore/CodeSystem/observation-verification>
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct ObservationVerificationCode {}
/// Nested message and enum types in `ObservationVerificationCode`.
pub mod observation_verification_code {
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, prost::Enumeration)]
    #[repr(i32)]
    pub enum Value {
        InvalidUninitialized = 0,
        HumanReview = 1,
        SlidingAverage = 2,
    }
}
/// The value set to instantiate this attribute should be drawn from a
/// terminologically robust code system that consists of or contains concepts to
/// support the encounter process, in particular the process and reasons for
/// canceling or refusing an encounter. This value set is provided as a
/// suggestive example See
/// <http://hl7.org/fhir/us/qicore/ValueSet/qicore-encounter-canceled-reason>
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct QiCoreEncounterCanceledReasonValueSet {}
/// Nested message and enum types in `QICoreEncounterCanceledReasonValueSet`.
pub mod qi_core_encounter_canceled_reason_value_set {
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, prost::Enumeration)]
    #[repr(i32)]
    pub enum Value {
        InvalidUninitialized = 0,
        Blk = 1,
        Dec = 2,
        Fin = 3,
        Med = 4,
        Altd = 5,
    }
}
/// The value set to instantiate this attribute should be drawn from a
/// terminologically robust code system that consists of or contains concepts to
/// support the goal process, in particular the process and reasons for rejecting
/// a goal. This value set is provided as a suggestive example. See
/// <http://hl7.org/fhir/us/qicore/ValueSet/qicore-goal-reason-rejected>
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct QiCoreGoalReasonRejectedValueSet {}
/// Nested message and enum types in `QICoreGoalReasonRejectedValueSet`.
pub mod qi_core_goal_reason_rejected_value_set {
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, prost::Enumeration)]
    #[repr(i32)]
    pub enum Value {
        InvalidUninitialized = 0,
        Blk = 1,
        Dec = 2,
        Fin = 3,
        Med = 4,
        Altd = 5,
    }
}
/// Codes indicating status of current or former military service.
/// See <http://hl7.org/fhir/us/qicore/ValueSet/qicore-military-service>
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct QiCoreMilitaryServiceValueSet {}
/// Nested message and enum types in `QICoreMilitaryServiceValueSet`.
pub mod qi_core_military_service_value_set {
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, prost::Enumeration)]
    #[repr(i32)]
    pub enum Value {
        InvalidUninitialized = 0,
        NotIndicated = 1,
        NoMilitaryService = 2,
        Veteran = 3,
        ActiveDuty = 4,
        ActiveReserve = 5,
        InactiveReserve = 6,
    }
}
/// Code indicating how the current observation compares to previous observations
/// - e.g., no change, rising trend, decreasing trend, etc... See
/// <http://hl7.org/fhir/us/qicore/ValueSet/qicore-observation-delta>
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct QiCoreObservationDeltaValueSet {}
/// Nested message and enum types in `QICoreObservationDeltaValueSet`.
pub mod qi_core_observation_delta_value_set {
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, prost::Enumeration)]
    #[repr(i32)]
    pub enum Value {
        InvalidUninitialized = 0,
        B = 1,
        D = 2,
        U = 3,
        W = 4,
    }
}
/// Auto-generated from StructureDefinition for QICoreAdverseEvent.
/// Medical care, research study or other healthcare event causing physical
/// injury. See
/// <http://hl7.org/fhir/us/qicore/StructureDefinition/qicore-adverseevent>
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct QiCoreAdverseEvent {
    /// Logical id of this artifact
    #[prost(message, optional, tag = "1")]
    pub id: ::core::option::Option<super::core::Id>,
    /// Metadata about the resource
    #[prost(message, optional, tag = "2")]
    pub meta: ::core::option::Option<super::core::Meta>,
    /// A set of rules under which this content was created
    #[prost(message, optional, tag = "3")]
    pub implicit_rules: ::core::option::Option<super::core::Uri>,
    /// Language of the resource content
    #[prost(message, optional, tag = "4")]
    pub language: ::core::option::Option<super::core::Code>,
    /// Text summary of the resource, for human interpretation
    #[prost(message, optional, tag = "5")]
    pub text: ::core::option::Option<super::core::Narrative>,
    /// Contained, inline Resources
    #[prost(message, repeated, tag = "6")]
    pub contained: prost::alloc::vec::Vec<::prost_wkt_types::Any>,
    /// Additional content defined by implementations
    #[prost(message, repeated, tag = "8")]
    pub extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// Extensions that cannot be ignored
    #[prost(message, repeated, tag = "9")]
    pub modifier_extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// Business identifier for the event
    #[prost(message, optional, tag = "10")]
    pub identifier: ::core::option::Option<super::core::Identifier>,
    #[prost(message, optional, tag = "11")]
    pub actuality: ::core::option::Option<qi_core_adverse_event::ActualityCode>,
    /// product-problem | product-quality | product-use-error | wrong-dose |
    /// incorrect-prescribing-information | wrong-technique |
    /// wrong-route-of-administration | wrong-rate | wrong-duration | wrong-time |
    /// expired-drug | medical-device-use-error | problem-different-manufacturer |
    /// unsafe-physical-environment
    #[prost(message, repeated, tag = "12")]
    pub category: prost::alloc::vec::Vec<super::core::CodeableConcept>,
    /// Type of the event itself in relation to the subject
    #[prost(message, optional, tag = "13")]
    pub event: ::core::option::Option<super::core::CodeableConcept>,
    /// Subject impacted by event
    #[prost(message, optional, tag = "14")]
    pub subject: ::core::option::Option<super::core::Reference>,
    /// Encounter created as part of
    #[prost(message, optional, tag = "15")]
    pub encounter: ::core::option::Option<super::core::Reference>,
    /// When the event occurred
    #[prost(message, optional, tag = "16")]
    pub date: ::core::option::Option<super::core::DateTime>,
    /// When the event was detected
    #[prost(message, optional, tag = "17")]
    pub detected: ::core::option::Option<super::core::DateTime>,
    /// When the event was recorded
    #[prost(message, optional, tag = "18")]
    pub recorded_date: ::core::option::Option<super::core::DateTime>,
    /// Effect on the subject due to this event
    #[prost(message, repeated, tag = "19")]
    pub resulting_condition: prost::alloc::vec::Vec<super::core::Reference>,
    /// Location where adverse event occurred
    #[prost(message, optional, tag = "20")]
    pub location: ::core::option::Option<super::core::Reference>,
    /// Seriousness of the event
    #[prost(message, optional, tag = "21")]
    pub seriousness: ::core::option::Option<super::core::CodeableConcept>,
    /// mild | moderate | severe
    #[prost(message, optional, tag = "22")]
    pub severity: ::core::option::Option<super::core::CodeableConcept>,
    /// resolved | recovering | ongoing | resolvedWithSequelae | fatal | unknown
    #[prost(message, optional, tag = "23")]
    pub outcome: ::core::option::Option<super::core::CodeableConcept>,
    /// Who recorded the adverse event
    #[prost(message, optional, tag = "24")]
    pub recorder: ::core::option::Option<super::core::Reference>,
    /// Who  was involved in the adverse event or the potential adverse event
    #[prost(message, repeated, tag = "25")]
    pub contributor: prost::alloc::vec::Vec<super::core::Reference>,
    #[prost(message, repeated, tag = "26")]
    pub suspect_entity: prost::alloc::vec::Vec<qi_core_adverse_event::SuspectEntity>,
    /// AdverseEvent.subjectMedicalHistory
    #[prost(message, repeated, tag = "27")]
    pub subject_medical_history: prost::alloc::vec::Vec<super::core::Reference>,
    /// AdverseEvent.referenceDocument
    #[prost(message, repeated, tag = "28")]
    pub reference_document: prost::alloc::vec::Vec<super::core::Reference>,
    /// AdverseEvent.study
    #[prost(message, repeated, tag = "29")]
    pub study: prost::alloc::vec::Vec<super::core::Reference>,
}
/// Nested message and enum types in `QICoreAdverseEvent`.
pub mod qi_core_adverse_event {
    /// actual | potential
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct ActualityCode {
        #[prost(
            enumeration = "super::super::core::adverse_event_actuality_code::Value",
            tag = "1"
        )]
        pub value: i32,
        #[prost(message, optional, tag = "2")]
        pub id: ::core::option::Option<super::super::core::String>,
        #[prost(message, repeated, tag = "3")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
    }
    /// The suspected agent causing the adverse event
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct SuspectEntity {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag = "1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag = "2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag = "3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Refers to the specific entity that caused the adverse event
        #[prost(message, optional, tag = "4")]
        pub instance: ::core::option::Option<super::super::core::Reference>,
        #[prost(message, optional, tag = "5")]
        pub causality: ::core::option::Option<suspect_entity::Causality>,
    }
    /// Nested message and enum types in `SuspectEntity`.
    pub mod suspect_entity {
        /// Information on the possible cause of the event
        #[derive(Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Message)]
        pub struct Causality {
            /// Unique id for inter-element referencing
            #[prost(message, optional, tag = "1")]
            pub id: ::core::option::Option<super::super::super::core::String>,
            /// Additional content defined by implementations
            #[prost(message, repeated, tag = "2")]
            pub extension: prost::alloc::vec::Vec<super::super::super::core::Extension>,
            /// Extensions that cannot be ignored even if unrecognized
            #[prost(message, repeated, tag = "3")]
            pub modifier_extension: prost::alloc::vec::Vec<super::super::super::core::Extension>,
            /// Assessment of if the entity caused the event
            #[prost(message, optional, tag = "4")]
            pub assessment: ::core::option::Option<super::super::super::core::CodeableConcept>,
            /// AdverseEvent.suspectEntity.causalityProductRelatedness
            #[prost(message, optional, tag = "5")]
            pub product_relatedness: ::core::option::Option<super::super::super::core::String>,
            /// AdverseEvent.suspectEntity.causalityAuthor
            #[prost(message, optional, tag = "6")]
            pub author: ::core::option::Option<super::super::super::core::Reference>,
            /// ProbabilityScale | Bayesian | Checklist
            #[prost(message, optional, tag = "7")]
            pub method: ::core::option::Option<super::super::super::core::CodeableConcept>,
        }
    }
}
/// Auto-generated from StructureDefinition for QICoreAllergyIntolerance.
/// Allergy or Intolerance (generally: Risk of adverse reaction to a substance).
/// See
/// <http://hl7.org/fhir/us/qicore/StructureDefinition/qicore-allergyintolerance>
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct QiCoreAllergyIntolerance {
    /// Logical id of this artifact
    #[prost(message, optional, tag = "1")]
    pub id: ::core::option::Option<super::core::Id>,
    /// Metadata about the resource
    #[prost(message, optional, tag = "2")]
    pub meta: ::core::option::Option<super::core::Meta>,
    /// A set of rules under which this content was created
    #[prost(message, optional, tag = "3")]
    pub implicit_rules: ::core::option::Option<super::core::Uri>,
    /// Language of the resource content
    #[prost(message, optional, tag = "4")]
    pub language: ::core::option::Option<super::core::Code>,
    /// Text summary of the resource, for human interpretation
    #[prost(message, optional, tag = "5")]
    pub text: ::core::option::Option<super::core::Narrative>,
    /// Contained, inline Resources
    #[prost(message, repeated, tag = "6")]
    pub contained: prost::alloc::vec::Vec<::prost_wkt_types::Any>,
    /// Extension
    #[prost(message, repeated, tag = "8")]
    pub extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// Extensions that cannot be ignored
    #[prost(message, repeated, tag = "9")]
    pub modifier_extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// External ids for this item
    #[prost(message, repeated, tag = "10")]
    pub identifier: prost::alloc::vec::Vec<super::core::Identifier>,
    /// active | inactive | resolved
    #[prost(message, optional, tag = "11")]
    pub clinical_status: ::core::option::Option<super::core::CodeableConcept>,
    /// unconfirmed | confirmed | refuted | entered-in-error
    #[prost(message, optional, tag = "12")]
    pub verification_status: ::core::option::Option<super::core::CodeableConcept>,
    #[prost(message, optional, tag = "13")]
    pub r#type: ::core::option::Option<qi_core_allergy_intolerance::TypeCode>,
    #[prost(message, repeated, tag = "14")]
    pub category: prost::alloc::vec::Vec<qi_core_allergy_intolerance::CategoryCode>,
    #[prost(message, optional, tag = "15")]
    pub criticality: ::core::option::Option<qi_core_allergy_intolerance::CriticalityCode>,
    /// Code that identifies the allergy or intolerance
    #[prost(message, optional, tag = "16")]
    pub code: ::core::option::Option<super::core::CodeableConcept>,
    /// Who the sensitivity is for
    #[prost(message, optional, tag = "17")]
    pub patient: ::core::option::Option<super::core::Reference>,
    /// Encounter when the allergy or intolerance was asserted
    #[prost(message, optional, tag = "18")]
    pub encounter: ::core::option::Option<super::core::Reference>,
    #[prost(message, optional, tag = "19")]
    pub onset: ::core::option::Option<qi_core_allergy_intolerance::OnsetX>,
    /// Date first version of the resource instance was recorded
    #[prost(message, optional, tag = "20")]
    pub recorded_date: ::core::option::Option<super::core::DateTime>,
    /// Who recorded the sensitivity
    #[prost(message, optional, tag = "21")]
    pub recorder: ::core::option::Option<super::core::Reference>,
    /// Source of the information about the allergy
    #[prost(message, optional, tag = "22")]
    pub asserter: ::core::option::Option<super::core::Reference>,
    /// Date(/time) of last known occurrence of a reaction
    #[prost(message, optional, tag = "23")]
    pub last_occurrence: ::core::option::Option<super::core::DateTime>,
    /// Additional text not captured in other fields
    #[prost(message, repeated, tag = "24")]
    pub note: prost::alloc::vec::Vec<super::core::Annotation>,
    #[prost(message, repeated, tag = "25")]
    pub reaction: prost::alloc::vec::Vec<qi_core_allergy_intolerance::Reaction>,
    /// Age that the allergy or intolerance resolved
    #[prost(message, optional, tag = "26")]
    pub resolution_age: ::core::option::Option<super::core::Age>,
    /// Explanation associated with refuted status
    #[prost(message, optional, tag = "27")]
    pub reason_refuted: ::core::option::Option<super::core::CodeableConcept>,
}
/// Nested message and enum types in `QICoreAllergyIntolerance`.
pub mod qi_core_allergy_intolerance {
    /// allergy | intolerance - Underlying mechanism (if known)
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct TypeCode {
        #[prost(
            enumeration = "super::super::core::allergy_intolerance_type_code::Value",
            tag = "1"
        )]
        pub value: i32,
        #[prost(message, optional, tag = "2")]
        pub id: ::core::option::Option<super::super::core::String>,
        #[prost(message, repeated, tag = "3")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
    }
    /// food | medication | environment | biologic
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct CategoryCode {
        #[prost(
            enumeration = "super::super::core::allergy_intolerance_category_code::Value",
            tag = "1"
        )]
        pub value: i32,
        #[prost(message, optional, tag = "2")]
        pub id: ::core::option::Option<super::super::core::String>,
        #[prost(message, repeated, tag = "3")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
    }
    /// low | high | unable-to-assess
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct CriticalityCode {
        #[prost(
            enumeration = "super::super::core::allergy_intolerance_criticality_code::Value",
            tag = "1"
        )]
        pub value: i32,
        #[prost(message, optional, tag = "2")]
        pub id: ::core::option::Option<super::super::core::String>,
        #[prost(message, repeated, tag = "3")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
    }
    /// When allergy or intolerance was identified
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct OnsetX {
        #[prost(oneof = "onset_x::Choice", tags = "1, 2, 3, 4")]
        pub choice: ::core::option::Option<onset_x::Choice>,
    }
    /// Nested message and enum types in `OnsetX`.
    pub mod onset_x {
        #[derive(Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Oneof)]
        pub enum Choice {
            #[prost(message, tag = "1")]
            DateTime(super::super::super::core::DateTime),
            #[prost(message, tag = "2")]
            Age(super::super::super::core::Age),
            #[prost(message, tag = "3")]
            Period(super::super::super::core::Period),
            #[prost(message, tag = "4")]
            Range(super::super::super::core::Range),
        }
    }
    /// Adverse Reaction Events linked to exposure to substance
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct Reaction {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag = "1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Extension
        #[prost(message, repeated, tag = "2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag = "3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Specific substance or pharmaceutical product considered to be responsible
        /// for event
        #[prost(message, optional, tag = "4")]
        pub substance: ::core::option::Option<super::super::core::CodeableConcept>,
        /// Clinical symptoms/signs associated with the Event
        #[prost(message, repeated, tag = "5")]
        pub manifestation: prost::alloc::vec::Vec<super::super::core::CodeableConcept>,
        /// Description of the event as a whole
        #[prost(message, optional, tag = "6")]
        pub description: ::core::option::Option<super::super::core::String>,
        /// Date(/time) when manifestations showed
        #[prost(message, optional, tag = "7")]
        pub onset: ::core::option::Option<super::super::core::DateTime>,
        #[prost(message, optional, tag = "8")]
        pub severity: ::core::option::Option<reaction::SeverityCode>,
        /// How the subject was exposed to the substance
        #[prost(message, optional, tag = "9")]
        pub exposure_route: ::core::option::Option<super::super::core::CodeableConcept>,
        /// Text about event not captured in other fields
        #[prost(message, repeated, tag = "10")]
        pub note: prost::alloc::vec::Vec<super::super::core::Annotation>,
        /// How long Manifestations persisted
        #[prost(message, optional, tag = "11")]
        pub reaction_duration: ::core::option::Option<super::super::core::Duration>,
    }
    /// Nested message and enum types in `Reaction`.
    pub mod reaction {
        /// mild | moderate | severe (of event as a whole)
        #[derive(Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Message)]
        pub struct SeverityCode {
            #[prost(
                enumeration = "super::super::super::core::allergy_intolerance_severity_code::Value",
                tag = "1"
            )]
            pub value: i32,
            #[prost(message, optional, tag = "2")]
            pub id: ::core::option::Option<super::super::super::core::String>,
            #[prost(message, repeated, tag = "3")]
            pub extension: prost::alloc::vec::Vec<super::super::super::core::Extension>,
        }
    }
}
/// Auto-generated from StructureDefinition for QICoreBodyStructure.
/// Specific and identified anatomical structure.
/// See <http://hl7.org/fhir/us/qicore/StructureDefinition/qicore-bodystructure>
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct QiCoreBodyStructure {
    /// Logical id of this artifact
    #[prost(message, optional, tag = "1")]
    pub id: ::core::option::Option<super::core::Id>,
    /// Metadata about the resource
    #[prost(message, optional, tag = "2")]
    pub meta: ::core::option::Option<super::core::Meta>,
    /// A set of rules under which this content was created
    #[prost(message, optional, tag = "3")]
    pub implicit_rules: ::core::option::Option<super::core::Uri>,
    /// Language of the resource content
    #[prost(message, optional, tag = "4")]
    pub language: ::core::option::Option<super::core::Code>,
    /// Text summary of the resource, for human interpretation
    #[prost(message, optional, tag = "5")]
    pub text: ::core::option::Option<super::core::Narrative>,
    /// Contained, inline Resources
    #[prost(message, repeated, tag = "6")]
    pub contained: prost::alloc::vec::Vec<::prost_wkt_types::Any>,
    /// Additional content defined by implementations
    #[prost(message, repeated, tag = "8")]
    pub extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// Extensions that cannot be ignored
    #[prost(message, repeated, tag = "9")]
    pub modifier_extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// Bodystructure identifier
    #[prost(message, repeated, tag = "10")]
    pub identifier: prost::alloc::vec::Vec<super::core::Identifier>,
    /// Whether this record is in active use
    #[prost(message, optional, tag = "11")]
    pub active: ::core::option::Option<super::core::Boolean>,
    /// Kind of Structure
    #[prost(message, optional, tag = "12")]
    pub morphology: ::core::option::Option<super::core::CodeableConcept>,
    /// Body site
    #[prost(message, optional, tag = "13")]
    pub location: ::core::option::Option<super::core::CodeableConcept>,
    /// Body site modifier
    #[prost(message, repeated, tag = "14")]
    pub location_qualifier: prost::alloc::vec::Vec<super::core::CodeableConcept>,
    /// Text description
    #[prost(message, optional, tag = "15")]
    pub description: ::core::option::Option<super::core::String>,
    /// Attached images
    #[prost(message, repeated, tag = "16")]
    pub image: prost::alloc::vec::Vec<super::core::Attachment>,
    /// Who this is about
    #[prost(message, optional, tag = "17")]
    pub patient: ::core::option::Option<super::core::Reference>,
}
/// Auto-generated from StructureDefinition for QICoreCarePlan.
/// Healthcare plan for patient or group.
/// See <http://hl7.org/fhir/us/qicore/StructureDefinition/qicore-careplan>
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct QiCoreCarePlan {
    /// Logical id of this artifact
    #[prost(message, optional, tag = "1")]
    pub id: ::core::option::Option<super::core::Id>,
    /// Metadata about the resource
    #[prost(message, optional, tag = "2")]
    pub meta: ::core::option::Option<super::core::Meta>,
    /// A set of rules under which this content was created
    #[prost(message, optional, tag = "3")]
    pub implicit_rules: ::core::option::Option<super::core::Uri>,
    /// Language of the resource content
    #[prost(message, optional, tag = "4")]
    pub language: ::core::option::Option<super::core::Code>,
    /// Text summary of the resource, for human interpretation
    #[prost(message, optional, tag = "5")]
    pub text: ::core::option::Option<super::core::Narrative>,
    /// Contained, inline Resources
    #[prost(message, repeated, tag = "6")]
    pub contained: prost::alloc::vec::Vec<::prost_wkt_types::Any>,
    /// Additional content defined by implementations
    #[prost(message, repeated, tag = "8")]
    pub extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// Extensions that cannot be ignored
    #[prost(message, repeated, tag = "9")]
    pub modifier_extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// External Ids for this plan
    #[prost(message, repeated, tag = "10")]
    pub identifier: prost::alloc::vec::Vec<super::core::Identifier>,
    /// Instantiates FHIR protocol or definition
    #[prost(message, repeated, tag = "11")]
    pub instantiates_canonical: prost::alloc::vec::Vec<super::core::Canonical>,
    /// Instantiates external protocol or definition
    #[prost(message, repeated, tag = "12")]
    pub instantiates_uri: prost::alloc::vec::Vec<super::core::Uri>,
    /// Fulfills CarePlan
    #[prost(message, repeated, tag = "13")]
    pub based_on: prost::alloc::vec::Vec<super::core::Reference>,
    /// CarePlan replaced by this CarePlan
    #[prost(message, repeated, tag = "14")]
    pub replaces: prost::alloc::vec::Vec<super::core::Reference>,
    /// Part of referenced CarePlan
    #[prost(message, repeated, tag = "15")]
    pub part_of: prost::alloc::vec::Vec<super::core::Reference>,
    #[prost(message, optional, tag = "16")]
    pub status: ::core::option::Option<qi_core_care_plan::StatusCode>,
    #[prost(message, optional, tag = "17")]
    pub intent: ::core::option::Option<qi_core_care_plan::IntentCode>,
    /// Type of plan
    #[prost(message, repeated, tag = "18")]
    pub category: prost::alloc::vec::Vec<super::core::CodeableConcept>,
    /// Human-friendly name for the care plan
    #[prost(message, optional, tag = "19")]
    pub title: ::core::option::Option<super::core::String>,
    /// Summary of nature of plan
    #[prost(message, optional, tag = "20")]
    pub description: ::core::option::Option<super::core::String>,
    /// Who the care plan is for
    #[prost(message, optional, tag = "21")]
    pub subject: ::core::option::Option<super::core::Reference>,
    /// Encounter created as part of
    #[prost(message, optional, tag = "22")]
    pub encounter: ::core::option::Option<super::core::Reference>,
    /// Time period plan covers
    #[prost(message, optional, tag = "23")]
    pub period: ::core::option::Option<super::core::Period>,
    /// Date record was first recorded
    #[prost(message, optional, tag = "24")]
    pub created: ::core::option::Option<super::core::DateTime>,
    /// Who is the designated responsible party
    #[prost(message, optional, tag = "25")]
    pub author: ::core::option::Option<super::core::Reference>,
    /// Who provided the content of the care plan
    #[prost(message, repeated, tag = "26")]
    pub contributor: prost::alloc::vec::Vec<super::core::Reference>,
    /// Who's involved in plan?
    #[prost(message, repeated, tag = "27")]
    pub care_team: prost::alloc::vec::Vec<super::core::Reference>,
    /// Health issues this plan addresses
    #[prost(message, repeated, tag = "28")]
    pub addresses: prost::alloc::vec::Vec<super::core::Reference>,
    /// Information considered as part of plan
    #[prost(message, repeated, tag = "29")]
    pub supporting_info: prost::alloc::vec::Vec<super::core::Reference>,
    /// Desired outcome of plan
    #[prost(message, repeated, tag = "30")]
    pub goal: prost::alloc::vec::Vec<super::core::Reference>,
    #[prost(message, repeated, tag = "31")]
    pub activity: prost::alloc::vec::Vec<qi_core_care_plan::Activity>,
    /// Comments about the plan
    #[prost(message, repeated, tag = "32")]
    pub note: prost::alloc::vec::Vec<super::core::Annotation>,
}
/// Nested message and enum types in `QICoreCarePlan`.
pub mod qi_core_care_plan {
    /// draft | active | suspended | completed | entered-in-error | cancelled |
    /// unknown
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct StatusCode {
        #[prost(
            enumeration = "super::super::core::request_status_code::Value",
            tag = "1"
        )]
        pub value: i32,
        #[prost(message, optional, tag = "2")]
        pub id: ::core::option::Option<super::super::core::String>,
        #[prost(message, repeated, tag = "3")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
    }
    /// proposal | plan | order | option
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct IntentCode {
        #[prost(
            enumeration = "super::super::core::care_plan_intent_value_set::Value",
            tag = "1"
        )]
        pub value: i32,
        #[prost(message, optional, tag = "2")]
        pub id: ::core::option::Option<super::super::core::String>,
        #[prost(message, repeated, tag = "3")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
    }
    /// Action to occur as part of plan
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct Activity {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag = "1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag = "2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag = "3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Results of the activity
        #[prost(message, repeated, tag = "4")]
        pub outcome_codeable_concept: prost::alloc::vec::Vec<super::super::core::CodeableConcept>,
        /// Appointment, Encounter, Procedure, etc.
        #[prost(message, repeated, tag = "5")]
        pub outcome_reference: prost::alloc::vec::Vec<super::super::core::Reference>,
        /// Comments about the activity status/progress
        #[prost(message, repeated, tag = "6")]
        pub progress: prost::alloc::vec::Vec<super::super::core::Annotation>,
        /// Activity details defined in specific resource
        #[prost(message, optional, tag = "7")]
        pub reference: ::core::option::Option<super::super::core::Reference>,
        #[prost(message, optional, tag = "8")]
        pub detail: ::core::option::Option<activity::Detail>,
    }
    /// Nested message and enum types in `Activity`.
    pub mod activity {
        /// In-line definition of activity
        #[derive(Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Message)]
        pub struct Detail {
            /// Unique id for inter-element referencing
            #[prost(message, optional, tag = "1")]
            pub id: ::core::option::Option<super::super::super::core::String>,
            /// Additional content defined by implementations
            #[prost(message, repeated, tag = "2")]
            pub extension: prost::alloc::vec::Vec<super::super::super::core::Extension>,
            /// Extensions that cannot be ignored even if unrecognized
            #[prost(message, repeated, tag = "3")]
            pub modifier_extension: prost::alloc::vec::Vec<super::super::super::core::Extension>,
            #[prost(message, optional, tag = "4")]
            pub kind: ::core::option::Option<detail::KindCode>,
            /// Instantiates FHIR protocol or definition
            #[prost(message, repeated, tag = "5")]
            pub instantiates_canonical:
                prost::alloc::vec::Vec<super::super::super::core::Canonical>,
            /// Instantiates external protocol or definition
            #[prost(message, repeated, tag = "6")]
            pub instantiates_uri: prost::alloc::vec::Vec<super::super::super::core::Uri>,
            /// Detail type of activity
            #[prost(message, optional, tag = "7")]
            pub code: ::core::option::Option<super::super::super::core::CodeableConcept>,
            /// Why activity should be done or why activity was prohibited
            #[prost(message, repeated, tag = "8")]
            pub reason_code: prost::alloc::vec::Vec<super::super::super::core::CodeableConcept>,
            /// Why activity is needed
            #[prost(message, repeated, tag = "9")]
            pub reason_reference: prost::alloc::vec::Vec<super::super::super::core::Reference>,
            /// Goals this activity relates to
            #[prost(message, repeated, tag = "10")]
            pub goal: prost::alloc::vec::Vec<super::super::super::core::Reference>,
            #[prost(message, optional, tag = "11")]
            pub status: ::core::option::Option<detail::StatusCode>,
            /// Reason for current status
            #[prost(message, optional, tag = "12")]
            pub status_reason: ::core::option::Option<super::super::super::core::CodeableConcept>,
            /// If true, activity is prohibiting action
            #[prost(message, optional, tag = "13")]
            pub do_not_perform: ::core::option::Option<super::super::super::core::Boolean>,
            #[prost(message, optional, tag = "14")]
            pub scheduled: ::core::option::Option<detail::ScheduledX>,
            /// Where it should happen
            #[prost(message, optional, tag = "15")]
            pub location: ::core::option::Option<super::super::super::core::Reference>,
            /// Who will be responsible?
            #[prost(message, repeated, tag = "16")]
            pub performer: prost::alloc::vec::Vec<super::super::super::core::Reference>,
            #[prost(message, optional, tag = "17")]
            pub product: ::core::option::Option<detail::ProductX>,
            /// How to consume/day?
            #[prost(message, optional, tag = "18")]
            pub daily_amount: ::core::option::Option<super::super::super::core::SimpleQuantity>,
            /// How much to administer/supply/consume
            #[prost(message, optional, tag = "19")]
            pub quantity: ::core::option::Option<super::super::super::core::SimpleQuantity>,
            /// Extra info describing activity to perform
            #[prost(message, optional, tag = "20")]
            pub description: ::core::option::Option<super::super::super::core::String>,
        }
        /// Nested message and enum types in `Detail`.
        pub mod detail {
            /// Kind of resource
            #[derive(Serialize, Deserialize)]
            #[serde(rename_all = "camelCase")]
            #[derive(Clone, PartialEq, prost::Message)]
            pub struct KindCode {
                #[prost(
                    enumeration = "super::super::super::super::core::care_plan_activity_kind_value_set::Value",
                    tag = "1"
                )]
                pub value: i32,
                #[prost(message, optional, tag = "2")]
                pub id: ::core::option::Option<super::super::super::super::core::String>,
                #[prost(message, repeated, tag = "3")]
                pub extension: prost::alloc::vec::Vec<super::super::super::super::core::Extension>,
            }
            /// not-started | scheduled | in-progress | on-hold | completed | cancelled
            /// | stopped | unknown | entered-in-error
            #[derive(Serialize, Deserialize)]
            #[serde(rename_all = "camelCase")]
            #[derive(Clone, PartialEq, prost::Message)]
            pub struct StatusCode {
                #[prost(
                    enumeration = "super::super::super::super::core::care_plan_activity_status_code::Value",
                    tag = "1"
                )]
                pub value: i32,
                #[prost(message, optional, tag = "2")]
                pub id: ::core::option::Option<super::super::super::super::core::String>,
                #[prost(message, repeated, tag = "3")]
                pub extension: prost::alloc::vec::Vec<super::super::super::super::core::Extension>,
            }
            /// When activity is to occur
            #[derive(Serialize, Deserialize)]
            #[serde(rename_all = "camelCase")]
            #[derive(Clone, PartialEq, prost::Message)]
            pub struct ScheduledX {
                #[prost(oneof = "scheduled_x::Choice", tags = "1, 2, 3")]
                pub choice: ::core::option::Option<scheduled_x::Choice>,
            }
            /// Nested message and enum types in `ScheduledX`.
            pub mod scheduled_x {
                #[derive(Serialize, Deserialize)]
                #[serde(rename_all = "camelCase")]
                #[derive(Clone, PartialEq, prost::Oneof)]
                pub enum Choice {
                    #[prost(message, tag = "1")]
                    Timing(super::super::super::super::super::core::Timing),
                    #[prost(message, tag = "2")]
                    Period(super::super::super::super::super::core::Period),
                    #[prost(message, tag = "3")]
                    StringValue(super::super::super::super::super::core::String),
                }
            }
            /// What is to be administered/supplied
            #[derive(Serialize, Deserialize)]
            #[serde(rename_all = "camelCase")]
            #[derive(Clone, PartialEq, prost::Message)]
            pub struct ProductX {
                #[prost(oneof = "product_x::Choice", tags = "1, 2")]
                pub choice: ::core::option::Option<product_x::Choice>,
            }
            /// Nested message and enum types in `ProductX`.
            pub mod product_x {
                #[derive(Serialize, Deserialize)]
                #[serde(rename_all = "camelCase")]
                #[derive(Clone, PartialEq, prost::Oneof)]
                pub enum Choice {
                    #[prost(message, tag = "1")]
                    CodeableConcept(super::super::super::super::super::core::CodeableConcept),
                    #[prost(message, tag = "2")]
                    Reference(super::super::super::super::super::core::Reference),
                }
            }
        }
    }
}
/// Auto-generated from StructureDefinition for QICoreCareTeam.
/// Planned participants in the coordination and delivery of care for a patient
/// or group. See
/// <http://hl7.org/fhir/us/qicore/StructureDefinition/qicore-careteam>
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct QiCoreCareTeam {
    /// Logical id of this artifact
    #[prost(message, optional, tag = "1")]
    pub id: ::core::option::Option<super::core::Id>,
    /// Metadata about the resource
    #[prost(message, optional, tag = "2")]
    pub meta: ::core::option::Option<super::core::Meta>,
    /// A set of rules under which this content was created
    #[prost(message, optional, tag = "3")]
    pub implicit_rules: ::core::option::Option<super::core::Uri>,
    /// Language of the resource content
    #[prost(message, optional, tag = "4")]
    pub language: ::core::option::Option<super::core::Code>,
    /// Text summary of the resource, for human interpretation
    #[prost(message, optional, tag = "5")]
    pub text: ::core::option::Option<super::core::Narrative>,
    /// Contained, inline Resources
    #[prost(message, repeated, tag = "6")]
    pub contained: prost::alloc::vec::Vec<::prost_wkt_types::Any>,
    /// Additional content defined by implementations
    #[prost(message, repeated, tag = "8")]
    pub extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// Extensions that cannot be ignored
    #[prost(message, repeated, tag = "9")]
    pub modifier_extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// External Ids for this team
    #[prost(message, repeated, tag = "10")]
    pub identifier: prost::alloc::vec::Vec<super::core::Identifier>,
    #[prost(message, optional, tag = "11")]
    pub status: ::core::option::Option<qi_core_care_team::StatusCode>,
    /// Type of team
    #[prost(message, repeated, tag = "12")]
    pub category: prost::alloc::vec::Vec<super::core::CodeableConcept>,
    /// Name of the team, such as crisis assessment team
    #[prost(message, optional, tag = "13")]
    pub name: ::core::option::Option<super::core::String>,
    /// Who care team is for
    #[prost(message, optional, tag = "14")]
    pub subject: ::core::option::Option<super::core::Reference>,
    /// Encounter created as part of
    #[prost(message, optional, tag = "15")]
    pub encounter: ::core::option::Option<super::core::Reference>,
    /// Time period team covers
    #[prost(message, optional, tag = "16")]
    pub period: ::core::option::Option<super::core::Period>,
    #[prost(message, repeated, tag = "17")]
    pub participant: prost::alloc::vec::Vec<qi_core_care_team::Participant>,
    /// Why the care team exists
    #[prost(message, repeated, tag = "18")]
    pub reason_code: prost::alloc::vec::Vec<super::core::CodeableConcept>,
    /// Why the care team exists
    #[prost(message, repeated, tag = "19")]
    pub reason_reference: prost::alloc::vec::Vec<super::core::Reference>,
    /// Organization responsible for the care team
    #[prost(message, repeated, tag = "20")]
    pub managing_organization: prost::alloc::vec::Vec<super::core::Reference>,
    /// A contact detail for the care team (that applies to all members)
    #[prost(message, repeated, tag = "21")]
    pub telecom: prost::alloc::vec::Vec<super::core::ContactPoint>,
    /// Comments made about the CareTeam
    #[prost(message, repeated, tag = "22")]
    pub note: prost::alloc::vec::Vec<super::core::Annotation>,
}
/// Nested message and enum types in `QICoreCareTeam`.
pub mod qi_core_care_team {
    /// proposed | active | suspended | inactive | entered-in-error
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct StatusCode {
        #[prost(
            enumeration = "super::super::core::care_team_status_code::Value",
            tag = "1"
        )]
        pub value: i32,
        #[prost(message, optional, tag = "2")]
        pub id: ::core::option::Option<super::super::core::String>,
        #[prost(message, repeated, tag = "3")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
    }
    /// Members of the team
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct Participant {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag = "1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag = "2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag = "3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Type of involvement
        #[prost(message, optional, tag = "4")]
        pub role: ::core::option::Option<super::super::core::CodeableConcept>,
        /// Who is involved
        #[prost(message, optional, tag = "5")]
        pub member: ::core::option::Option<super::super::core::Reference>,
        /// Organization of the practitioner
        #[prost(message, optional, tag = "6")]
        pub on_behalf_of: ::core::option::Option<super::super::core::Reference>,
        /// Time period of participant
        #[prost(message, optional, tag = "7")]
        pub period: ::core::option::Option<super::super::core::Period>,
    }
}
/// Auto-generated from StructureDefinition for QICoreClaim.
/// Claim, Pre-determination or Pre-authorization.
/// See <http://hl7.org/fhir/us/qicore/StructureDefinition/qicore-claim>
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct QiCoreClaim {
    /// Logical id of this artifact
    #[prost(message, optional, tag = "1")]
    pub id: ::core::option::Option<super::core::Id>,
    /// Metadata about the resource
    #[prost(message, optional, tag = "2")]
    pub meta: ::core::option::Option<super::core::Meta>,
    /// A set of rules under which this content was created
    #[prost(message, optional, tag = "3")]
    pub implicit_rules: ::core::option::Option<super::core::Uri>,
    /// Language of the resource content
    #[prost(message, optional, tag = "4")]
    pub language: ::core::option::Option<super::core::Code>,
    /// Text summary of the resource, for human interpretation
    #[prost(message, optional, tag = "5")]
    pub text: ::core::option::Option<super::core::Narrative>,
    /// Contained, inline Resources
    #[prost(message, repeated, tag = "6")]
    pub contained: prost::alloc::vec::Vec<::prost_wkt_types::Any>,
    /// Additional content defined by implementations
    #[prost(message, repeated, tag = "8")]
    pub extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// Extensions that cannot be ignored
    #[prost(message, repeated, tag = "9")]
    pub modifier_extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// Business Identifier for claim
    #[prost(message, repeated, tag = "10")]
    pub identifier: prost::alloc::vec::Vec<super::core::Identifier>,
    #[prost(message, optional, tag = "11")]
    pub status: ::core::option::Option<qi_core_claim::StatusCode>,
    /// Category or discipline
    #[prost(message, optional, tag = "12")]
    pub r#type: ::core::option::Option<super::core::CodeableConcept>,
    /// More granular claim type
    #[prost(message, optional, tag = "13")]
    pub sub_type: ::core::option::Option<super::core::CodeableConcept>,
    #[prost(message, optional, tag = "14")]
    pub r#use: ::core::option::Option<qi_core_claim::UseCode>,
    /// The recipient of the products and services
    #[prost(message, optional, tag = "15")]
    pub patient: ::core::option::Option<super::core::Reference>,
    /// Relevant time frame for the claim
    #[prost(message, optional, tag = "16")]
    pub billable_period: ::core::option::Option<super::core::Period>,
    /// Resource creation date
    #[prost(message, optional, tag = "17")]
    pub created: ::core::option::Option<super::core::DateTime>,
    /// Author of the claim
    #[prost(message, optional, tag = "18")]
    pub enterer: ::core::option::Option<super::core::Reference>,
    /// Target
    #[prost(message, optional, tag = "19")]
    pub insurer: ::core::option::Option<super::core::Reference>,
    /// Party responsible for the claim
    #[prost(message, optional, tag = "20")]
    pub provider: ::core::option::Option<super::core::Reference>,
    /// Desired processing ugency
    #[prost(message, optional, tag = "21")]
    pub priority: ::core::option::Option<super::core::CodeableConcept>,
    /// For whom to reserve funds
    #[prost(message, optional, tag = "22")]
    pub funds_reserve: ::core::option::Option<super::core::CodeableConcept>,
    #[prost(message, repeated, tag = "23")]
    pub related: prost::alloc::vec::Vec<qi_core_claim::RelatedClaim>,
    /// Prescription authorizing services and products
    #[prost(message, optional, tag = "24")]
    pub prescription: ::core::option::Option<super::core::Reference>,
    /// Original prescription if superseded by fulfiller
    #[prost(message, optional, tag = "25")]
    pub original_prescription: ::core::option::Option<super::core::Reference>,
    #[prost(message, optional, tag = "26")]
    pub payee: ::core::option::Option<qi_core_claim::Payee>,
    /// Treatment referral
    #[prost(message, optional, tag = "27")]
    pub referral: ::core::option::Option<super::core::Reference>,
    /// Servicing facility
    #[prost(message, optional, tag = "28")]
    pub facility: ::core::option::Option<super::core::Reference>,
    #[prost(message, repeated, tag = "29")]
    pub care_team: prost::alloc::vec::Vec<qi_core_claim::CareTeam>,
    #[prost(message, repeated, tag = "30")]
    pub supporting_info: prost::alloc::vec::Vec<qi_core_claim::SupportingInformation>,
    #[prost(message, repeated, tag = "31")]
    pub diagnosis: prost::alloc::vec::Vec<qi_core_claim::Diagnosis>,
    #[prost(message, repeated, tag = "32")]
    pub procedure: prost::alloc::vec::Vec<qi_core_claim::Procedure>,
    #[prost(message, repeated, tag = "33")]
    pub insurance: prost::alloc::vec::Vec<qi_core_claim::Insurance>,
    #[prost(message, optional, tag = "34")]
    pub accident: ::core::option::Option<qi_core_claim::Accident>,
    #[prost(message, repeated, tag = "35")]
    pub item: prost::alloc::vec::Vec<qi_core_claim::Item>,
    /// Total claim cost
    #[prost(message, optional, tag = "36")]
    pub total: ::core::option::Option<super::core::Money>,
}
/// Nested message and enum types in `QICoreClaim`.
pub mod qi_core_claim {
    /// active | cancelled | draft | entered-in-error
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct StatusCode {
        #[prost(
            enumeration = "super::super::core::financial_resource_status_code::Value",
            tag = "1"
        )]
        pub value: i32,
        #[prost(message, optional, tag = "2")]
        pub id: ::core::option::Option<super::super::core::String>,
        #[prost(message, repeated, tag = "3")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
    }
    /// claim | preauthorization | predetermination
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct UseCode {
        #[prost(enumeration = "super::super::core::use_code::Value", tag = "1")]
        pub value: i32,
        #[prost(message, optional, tag = "2")]
        pub id: ::core::option::Option<super::super::core::String>,
        #[prost(message, repeated, tag = "3")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
    }
    /// Prior or corollary claims
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct RelatedClaim {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag = "1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag = "2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag = "3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Reference to the related claim
        #[prost(message, optional, tag = "4")]
        pub claim: ::core::option::Option<super::super::core::Reference>,
        /// How the reference claim is related
        #[prost(message, optional, tag = "5")]
        pub relationship: ::core::option::Option<super::super::core::CodeableConcept>,
        /// File or case reference
        #[prost(message, optional, tag = "6")]
        pub reference: ::core::option::Option<super::super::core::Identifier>,
    }
    /// Recipient of benefits payable
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct Payee {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag = "1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag = "2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag = "3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Category of recipient
        #[prost(message, optional, tag = "4")]
        pub r#type: ::core::option::Option<super::super::core::CodeableConcept>,
        /// Recipient reference
        #[prost(message, optional, tag = "5")]
        pub party: ::core::option::Option<super::super::core::Reference>,
    }
    /// Members of the care team
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct CareTeam {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag = "1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag = "2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag = "3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Order of care team
        #[prost(message, optional, tag = "4")]
        pub sequence: ::core::option::Option<super::super::core::PositiveInt>,
        /// Practitioner or organization
        #[prost(message, optional, tag = "5")]
        pub provider: ::core::option::Option<super::super::core::Reference>,
        /// Indicator of the lead practitioner
        #[prost(message, optional, tag = "6")]
        pub responsible: ::core::option::Option<super::super::core::Boolean>,
        /// Function within the team
        #[prost(message, optional, tag = "7")]
        pub role: ::core::option::Option<super::super::core::CodeableConcept>,
        /// Practitioner credential or specialization
        #[prost(message, optional, tag = "8")]
        pub qualification: ::core::option::Option<super::super::core::CodeableConcept>,
    }
    /// Supporting information
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct SupportingInformation {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag = "1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag = "2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag = "3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Information instance identifier
        #[prost(message, optional, tag = "4")]
        pub sequence: ::core::option::Option<super::super::core::PositiveInt>,
        /// Classification of the supplied information
        #[prost(message, optional, tag = "5")]
        pub category: ::core::option::Option<super::super::core::CodeableConcept>,
        /// Type of information
        #[prost(message, optional, tag = "6")]
        pub code: ::core::option::Option<super::super::core::CodeableConcept>,
        #[prost(message, optional, tag = "7")]
        pub timing: ::core::option::Option<supporting_information::TimingX>,
        #[prost(message, optional, tag = "8")]
        pub value: ::core::option::Option<supporting_information::ValueX>,
        /// Explanation for the information
        #[prost(message, optional, tag = "9")]
        pub reason: ::core::option::Option<super::super::core::CodeableConcept>,
    }
    /// Nested message and enum types in `SupportingInformation`.
    pub mod supporting_information {
        /// When it occurred
        #[derive(Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Message)]
        pub struct TimingX {
            #[prost(oneof = "timing_x::Choice", tags = "1, 2")]
            pub choice: ::core::option::Option<timing_x::Choice>,
        }
        /// Nested message and enum types in `TimingX`.
        pub mod timing_x {
            #[derive(Serialize, Deserialize)]
            #[serde(rename_all = "camelCase")]
            #[derive(Clone, PartialEq, prost::Oneof)]
            pub enum Choice {
                #[prost(message, tag = "1")]
                Date(super::super::super::super::core::Date),
                #[prost(message, tag = "2")]
                Period(super::super::super::super::core::Period),
            }
        }
        /// Data to be provided
        #[derive(Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Message)]
        pub struct ValueX {
            #[prost(oneof = "value_x::Choice", tags = "1, 2, 3, 4, 5")]
            pub choice: ::core::option::Option<value_x::Choice>,
        }
        /// Nested message and enum types in `ValueX`.
        pub mod value_x {
            #[derive(Serialize, Deserialize)]
            #[serde(rename_all = "camelCase")]
            #[derive(Clone, PartialEq, prost::Oneof)]
            pub enum Choice {
                #[prost(message, tag = "1")]
                Boolean(super::super::super::super::core::Boolean),
                #[prost(message, tag = "2")]
                StringValue(super::super::super::super::core::String),
                #[prost(message, tag = "3")]
                Quantity(super::super::super::super::core::Quantity),
                #[prost(message, tag = "4")]
                Attachment(super::super::super::super::core::Attachment),
                #[prost(message, tag = "5")]
                Reference(super::super::super::super::core::Reference),
            }
        }
    }
    /// Pertinent diagnosis information
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct Diagnosis {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag = "1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag = "2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag = "3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Diagnosis instance identifier
        #[prost(message, optional, tag = "4")]
        pub sequence: ::core::option::Option<super::super::core::PositiveInt>,
        #[prost(message, optional, tag = "5")]
        pub diagnosis: ::core::option::Option<diagnosis::DiagnosisX>,
        /// Timing or nature of the diagnosis
        #[prost(message, repeated, tag = "6")]
        pub r#type: prost::alloc::vec::Vec<super::super::core::CodeableConcept>,
        /// Present on admission
        #[prost(message, optional, tag = "7")]
        pub on_admission: ::core::option::Option<super::super::core::CodeableConcept>,
        /// Package billing code
        #[prost(message, optional, tag = "8")]
        pub package_code: ::core::option::Option<super::super::core::CodeableConcept>,
    }
    /// Nested message and enum types in `Diagnosis`.
    pub mod diagnosis {
        /// Nature of illness or problem
        #[derive(Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Message)]
        pub struct DiagnosisX {
            #[prost(oneof = "diagnosis_x::Choice", tags = "1, 2")]
            pub choice: ::core::option::Option<diagnosis_x::Choice>,
        }
        /// Nested message and enum types in `DiagnosisX`.
        pub mod diagnosis_x {
            #[derive(Serialize, Deserialize)]
            #[serde(rename_all = "camelCase")]
            #[derive(Clone, PartialEq, prost::Oneof)]
            pub enum Choice {
                #[prost(message, tag = "1")]
                CodeableConcept(super::super::super::super::core::CodeableConcept),
                #[prost(message, tag = "2")]
                Reference(super::super::super::super::core::Reference),
            }
        }
    }
    /// Clinical procedures performed
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct Procedure {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag = "1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag = "2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag = "3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Procedure instance identifier
        #[prost(message, optional, tag = "4")]
        pub sequence: ::core::option::Option<super::super::core::PositiveInt>,
        /// Category of Procedure
        #[prost(message, repeated, tag = "5")]
        pub r#type: prost::alloc::vec::Vec<super::super::core::CodeableConcept>,
        /// When the procedure was performed
        #[prost(message, optional, tag = "6")]
        pub date: ::core::option::Option<super::super::core::DateTime>,
        #[prost(message, optional, tag = "7")]
        pub procedure: ::core::option::Option<procedure::ProcedureX>,
        /// Unique device identifier
        #[prost(message, repeated, tag = "8")]
        pub udi: prost::alloc::vec::Vec<super::super::core::Reference>,
    }
    /// Nested message and enum types in `Procedure`.
    pub mod procedure {
        /// Specific clinical procedure
        #[derive(Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Message)]
        pub struct ProcedureX {
            #[prost(oneof = "procedure_x::Choice", tags = "1, 2")]
            pub choice: ::core::option::Option<procedure_x::Choice>,
        }
        /// Nested message and enum types in `ProcedureX`.
        pub mod procedure_x {
            #[derive(Serialize, Deserialize)]
            #[serde(rename_all = "camelCase")]
            #[derive(Clone, PartialEq, prost::Oneof)]
            pub enum Choice {
                #[prost(message, tag = "1")]
                CodeableConcept(super::super::super::super::core::CodeableConcept),
                #[prost(message, tag = "2")]
                Reference(super::super::super::super::core::Reference),
            }
        }
    }
    /// Patient insurance information
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct Insurance {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag = "1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag = "2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag = "3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Insurance instance identifier
        #[prost(message, optional, tag = "4")]
        pub sequence: ::core::option::Option<super::super::core::PositiveInt>,
        /// Coverage to be used for adjudication
        #[prost(message, optional, tag = "5")]
        pub focal: ::core::option::Option<super::super::core::Boolean>,
        /// Pre-assigned Claim number
        #[prost(message, optional, tag = "6")]
        pub identifier: ::core::option::Option<super::super::core::Identifier>,
        /// Insurance information
        #[prost(message, optional, tag = "7")]
        pub coverage: ::core::option::Option<super::super::core::Reference>,
        /// Additional provider contract number
        #[prost(message, optional, tag = "8")]
        pub business_arrangement: ::core::option::Option<super::super::core::String>,
        /// Prior authorization reference number
        #[prost(message, repeated, tag = "9")]
        pub pre_auth_ref: prost::alloc::vec::Vec<super::super::core::String>,
        /// Adjudication results
        #[prost(message, optional, tag = "10")]
        pub claim_response: ::core::option::Option<super::super::core::Reference>,
    }
    /// Details of the event
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct Accident {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag = "1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag = "2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag = "3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// When the incident occurred
        #[prost(message, optional, tag = "4")]
        pub date: ::core::option::Option<super::super::core::Date>,
        /// The nature of the accident
        #[prost(message, optional, tag = "5")]
        pub r#type: ::core::option::Option<super::super::core::CodeableConcept>,
        #[prost(message, optional, tag = "6")]
        pub location: ::core::option::Option<accident::LocationX>,
    }
    /// Nested message and enum types in `Accident`.
    pub mod accident {
        /// Where the event occurred
        #[derive(Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Message)]
        pub struct LocationX {
            #[prost(oneof = "location_x::Choice", tags = "1, 2")]
            pub choice: ::core::option::Option<location_x::Choice>,
        }
        /// Nested message and enum types in `LocationX`.
        pub mod location_x {
            #[derive(Serialize, Deserialize)]
            #[serde(rename_all = "camelCase")]
            #[derive(Clone, PartialEq, prost::Oneof)]
            pub enum Choice {
                #[prost(message, tag = "1")]
                Address(super::super::super::super::core::Address),
                #[prost(message, tag = "2")]
                Reference(super::super::super::super::core::Reference),
            }
        }
    }
    /// Product or service provided
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct Item {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag = "1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag = "2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag = "3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Item instance identifier
        #[prost(message, optional, tag = "4")]
        pub sequence: ::core::option::Option<super::super::core::PositiveInt>,
        /// Applicable careTeam members
        #[prost(message, repeated, tag = "5")]
        pub care_team_sequence: prost::alloc::vec::Vec<super::super::core::PositiveInt>,
        /// Applicable diagnoses
        #[prost(message, repeated, tag = "6")]
        pub diagnosis_sequence: prost::alloc::vec::Vec<super::super::core::PositiveInt>,
        /// Applicable procedures
        #[prost(message, repeated, tag = "7")]
        pub procedure_sequence: prost::alloc::vec::Vec<super::super::core::PositiveInt>,
        /// Applicable exception and supporting information
        #[prost(message, repeated, tag = "8")]
        pub information_sequence: prost::alloc::vec::Vec<super::super::core::PositiveInt>,
        /// Revenue or cost center code
        #[prost(message, optional, tag = "9")]
        pub revenue: ::core::option::Option<super::super::core::CodeableConcept>,
        /// Benefit classification
        #[prost(message, optional, tag = "10")]
        pub category: ::core::option::Option<super::super::core::CodeableConcept>,
        /// Billing, service, product, or drug code
        #[prost(message, optional, tag = "11")]
        pub product_or_service: ::core::option::Option<super::super::core::CodeableConcept>,
        /// Product or service billing modifiers
        #[prost(message, repeated, tag = "12")]
        pub modifier: prost::alloc::vec::Vec<super::super::core::CodeableConcept>,
        /// Program the product or service is provided under
        #[prost(message, repeated, tag = "13")]
        pub program_code: prost::alloc::vec::Vec<super::super::core::CodeableConcept>,
        #[prost(message, optional, tag = "14")]
        pub serviced: ::core::option::Option<item::ServicedX>,
        #[prost(message, optional, tag = "15")]
        pub location: ::core::option::Option<item::LocationX>,
        /// Count of products or services
        #[prost(message, optional, tag = "16")]
        pub quantity: ::core::option::Option<super::super::core::SimpleQuantity>,
        /// Fee, charge or cost per item
        #[prost(message, optional, tag = "17")]
        pub unit_price: ::core::option::Option<super::super::core::Money>,
        /// Price scaling factor
        #[prost(message, optional, tag = "18")]
        pub factor: ::core::option::Option<super::super::core::Decimal>,
        /// Total item cost
        #[prost(message, optional, tag = "19")]
        pub net: ::core::option::Option<super::super::core::Money>,
        /// Unique device identifier
        #[prost(message, repeated, tag = "20")]
        pub udi: prost::alloc::vec::Vec<super::super::core::Reference>,
        /// Anatomical location
        #[prost(message, optional, tag = "21")]
        pub body_site: ::core::option::Option<super::super::core::CodeableConcept>,
        /// Anatomical sub-location
        #[prost(message, repeated, tag = "22")]
        pub sub_site: prost::alloc::vec::Vec<super::super::core::CodeableConcept>,
        /// Encounters related to this billed item
        #[prost(message, repeated, tag = "23")]
        pub encounter: prost::alloc::vec::Vec<super::super::core::Reference>,
        #[prost(message, repeated, tag = "24")]
        pub detail: prost::alloc::vec::Vec<item::Detail>,
    }
    /// Nested message and enum types in `Item`.
    pub mod item {
        /// Date or dates of service or product delivery
        #[derive(Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Message)]
        pub struct ServicedX {
            #[prost(oneof = "serviced_x::Choice", tags = "1, 2")]
            pub choice: ::core::option::Option<serviced_x::Choice>,
        }
        /// Nested message and enum types in `ServicedX`.
        pub mod serviced_x {
            #[derive(Serialize, Deserialize)]
            #[serde(rename_all = "camelCase")]
            #[derive(Clone, PartialEq, prost::Oneof)]
            pub enum Choice {
                #[prost(message, tag = "1")]
                Date(super::super::super::super::core::Date),
                #[prost(message, tag = "2")]
                Period(super::super::super::super::core::Period),
            }
        }
        /// Place of service or where product was supplied
        #[derive(Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Message)]
        pub struct LocationX {
            #[prost(oneof = "location_x::Choice", tags = "1, 2, 3")]
            pub choice: ::core::option::Option<location_x::Choice>,
        }
        /// Nested message and enum types in `LocationX`.
        pub mod location_x {
            #[derive(Serialize, Deserialize)]
            #[serde(rename_all = "camelCase")]
            #[derive(Clone, PartialEq, prost::Oneof)]
            pub enum Choice {
                #[prost(message, tag = "1")]
                CodeableConcept(super::super::super::super::core::CodeableConcept),
                #[prost(message, tag = "2")]
                Address(super::super::super::super::core::Address),
                #[prost(message, tag = "3")]
                Reference(super::super::super::super::core::Reference),
            }
        }
        /// Product or service provided
        #[derive(Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Message)]
        pub struct Detail {
            /// Unique id for inter-element referencing
            #[prost(message, optional, tag = "1")]
            pub id: ::core::option::Option<super::super::super::core::String>,
            /// Additional content defined by implementations
            #[prost(message, repeated, tag = "2")]
            pub extension: prost::alloc::vec::Vec<super::super::super::core::Extension>,
            /// Extensions that cannot be ignored even if unrecognized
            #[prost(message, repeated, tag = "3")]
            pub modifier_extension: prost::alloc::vec::Vec<super::super::super::core::Extension>,
            /// Item instance identifier
            #[prost(message, optional, tag = "4")]
            pub sequence: ::core::option::Option<super::super::super::core::PositiveInt>,
            /// Revenue or cost center code
            #[prost(message, optional, tag = "5")]
            pub revenue: ::core::option::Option<super::super::super::core::CodeableConcept>,
            /// Benefit classification
            #[prost(message, optional, tag = "6")]
            pub category: ::core::option::Option<super::super::super::core::CodeableConcept>,
            /// Billing, service, product, or drug code
            #[prost(message, optional, tag = "7")]
            pub product_or_service:
                ::core::option::Option<super::super::super::core::CodeableConcept>,
            /// Service/Product billing modifiers
            #[prost(message, repeated, tag = "8")]
            pub modifier: prost::alloc::vec::Vec<super::super::super::core::CodeableConcept>,
            /// Program the product or service is provided under
            #[prost(message, repeated, tag = "9")]
            pub program_code: prost::alloc::vec::Vec<super::super::super::core::CodeableConcept>,
            /// Count of products or services
            #[prost(message, optional, tag = "10")]
            pub quantity: ::core::option::Option<super::super::super::core::SimpleQuantity>,
            /// Fee, charge or cost per item
            #[prost(message, optional, tag = "11")]
            pub unit_price: ::core::option::Option<super::super::super::core::Money>,
            /// Price scaling factor
            #[prost(message, optional, tag = "12")]
            pub factor: ::core::option::Option<super::super::super::core::Decimal>,
            /// Total item cost
            #[prost(message, optional, tag = "13")]
            pub net: ::core::option::Option<super::super::super::core::Money>,
            /// Unique device identifier
            #[prost(message, repeated, tag = "14")]
            pub udi: prost::alloc::vec::Vec<super::super::super::core::Reference>,
            #[prost(message, repeated, tag = "15")]
            pub sub_detail: prost::alloc::vec::Vec<detail::SubDetail>,
        }
        /// Nested message and enum types in `Detail`.
        pub mod detail {
            /// Product or service provided
            #[derive(Serialize, Deserialize)]
            #[serde(rename_all = "camelCase")]
            #[derive(Clone, PartialEq, prost::Message)]
            pub struct SubDetail {
                /// Unique id for inter-element referencing
                #[prost(message, optional, tag = "1")]
                pub id: ::core::option::Option<super::super::super::super::core::String>,
                /// Additional content defined by implementations
                #[prost(message, repeated, tag = "2")]
                pub extension: prost::alloc::vec::Vec<super::super::super::super::core::Extension>,
                /// Extensions that cannot be ignored even if unrecognized
                #[prost(message, repeated, tag = "3")]
                pub modifier_extension:
                    prost::alloc::vec::Vec<super::super::super::super::core::Extension>,
                /// Item instance identifier
                #[prost(message, optional, tag = "4")]
                pub sequence: ::core::option::Option<super::super::super::super::core::PositiveInt>,
                /// Revenue or cost center code
                #[prost(message, optional, tag = "5")]
                pub revenue:
                    ::core::option::Option<super::super::super::super::core::CodeableConcept>,
                /// Benefit classification
                #[prost(message, optional, tag = "6")]
                pub category:
                    ::core::option::Option<super::super::super::super::core::CodeableConcept>,
                /// Billing, service, product, or drug code
                #[prost(message, optional, tag = "7")]
                pub product_or_service:
                    ::core::option::Option<super::super::super::super::core::CodeableConcept>,
                /// Service/Product billing modifiers
                #[prost(message, repeated, tag = "8")]
                pub modifier:
                    prost::alloc::vec::Vec<super::super::super::super::core::CodeableConcept>,
                /// Program the product or service is provided under
                #[prost(message, repeated, tag = "9")]
                pub program_code:
                    prost::alloc::vec::Vec<super::super::super::super::core::CodeableConcept>,
                /// Count of products or services
                #[prost(message, optional, tag = "10")]
                pub quantity:
                    ::core::option::Option<super::super::super::super::core::SimpleQuantity>,
                /// Fee, charge or cost per item
                #[prost(message, optional, tag = "11")]
                pub unit_price: ::core::option::Option<super::super::super::super::core::Money>,
                /// Price scaling factor
                #[prost(message, optional, tag = "12")]
                pub factor: ::core::option::Option<super::super::super::super::core::Decimal>,
                /// Total item cost
                #[prost(message, optional, tag = "13")]
                pub net: ::core::option::Option<super::super::super::super::core::Money>,
                /// Unique device identifier
                #[prost(message, repeated, tag = "14")]
                pub udi: prost::alloc::vec::Vec<super::super::super::super::core::Reference>,
            }
        }
    }
}
/// Auto-generated from StructureDefinition for QICoreCommunication.
/// A record of information transmitted from a sender to a receiver.
/// See <http://hl7.org/fhir/us/qicore/StructureDefinition/qicore-communication>
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct QiCoreCommunication {
    /// Logical id of this artifact
    #[prost(message, optional, tag = "1")]
    pub id: ::core::option::Option<super::core::Id>,
    /// Metadata about the resource
    #[prost(message, optional, tag = "2")]
    pub meta: ::core::option::Option<super::core::Meta>,
    /// A set of rules under which this content was created
    #[prost(message, optional, tag = "3")]
    pub implicit_rules: ::core::option::Option<super::core::Uri>,
    /// Language of the resource content
    #[prost(message, optional, tag = "4")]
    pub language: ::core::option::Option<super::core::Code>,
    /// Text summary of the resource, for human interpretation
    #[prost(message, optional, tag = "5")]
    pub text: ::core::option::Option<super::core::Narrative>,
    /// Contained, inline Resources
    #[prost(message, repeated, tag = "6")]
    pub contained: prost::alloc::vec::Vec<::prost_wkt_types::Any>,
    /// Additional content defined by implementations
    #[prost(message, repeated, tag = "8")]
    pub extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// Extensions that cannot be ignored
    #[prost(message, repeated, tag = "9")]
    pub modifier_extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// Unique identifier
    #[prost(message, repeated, tag = "10")]
    pub identifier: prost::alloc::vec::Vec<super::core::Identifier>,
    /// Instantiates FHIR protocol or definition
    #[prost(message, repeated, tag = "11")]
    pub instantiates_canonical: prost::alloc::vec::Vec<super::core::Canonical>,
    /// Instantiates external protocol or definition
    #[prost(message, repeated, tag = "12")]
    pub instantiates_uri: prost::alloc::vec::Vec<super::core::Uri>,
    /// Request fulfilled by this communication
    #[prost(message, repeated, tag = "13")]
    pub based_on: prost::alloc::vec::Vec<super::core::Reference>,
    /// Part of this action
    #[prost(message, repeated, tag = "14")]
    pub part_of: prost::alloc::vec::Vec<super::core::Reference>,
    /// Reply to
    #[prost(message, repeated, tag = "15")]
    pub in_response_to: prost::alloc::vec::Vec<super::core::Reference>,
    #[prost(message, optional, tag = "16")]
    pub status: ::core::option::Option<qi_core_communication::StatusCode>,
    /// Reason for current status
    #[prost(message, optional, tag = "17")]
    pub status_reason: ::core::option::Option<super::core::CodeableConcept>,
    /// Message category
    #[prost(message, repeated, tag = "18")]
    pub category: prost::alloc::vec::Vec<super::core::CodeableConcept>,
    #[prost(message, optional, tag = "19")]
    pub priority: ::core::option::Option<qi_core_communication::PriorityCode>,
    /// A channel of communication
    #[prost(message, repeated, tag = "20")]
    pub medium: prost::alloc::vec::Vec<super::core::CodeableConcept>,
    /// Focus of message
    #[prost(message, optional, tag = "21")]
    pub subject: ::core::option::Option<super::core::Reference>,
    /// Description of the purpose/content
    #[prost(message, optional, tag = "22")]
    pub topic: ::core::option::Option<super::core::CodeableConcept>,
    /// Resources that pertain to this communication
    #[prost(message, repeated, tag = "23")]
    pub about: prost::alloc::vec::Vec<super::core::Reference>,
    /// Encounter created as part of
    #[prost(message, optional, tag = "24")]
    pub encounter: ::core::option::Option<super::core::Reference>,
    /// When sent
    #[prost(message, optional, tag = "25")]
    pub sent: ::core::option::Option<super::core::DateTime>,
    /// When received
    #[prost(message, optional, tag = "26")]
    pub received: ::core::option::Option<super::core::DateTime>,
    /// Message recipient
    #[prost(message, repeated, tag = "27")]
    pub recipient: prost::alloc::vec::Vec<super::core::Reference>,
    /// Message sender
    #[prost(message, optional, tag = "28")]
    pub sender: ::core::option::Option<super::core::Reference>,
    /// Indication for message
    #[prost(message, repeated, tag = "29")]
    pub reason_code: prost::alloc::vec::Vec<super::core::CodeableConcept>,
    /// Why was communication done?
    #[prost(message, repeated, tag = "30")]
    pub reason_reference: prost::alloc::vec::Vec<super::core::Reference>,
    #[prost(message, repeated, tag = "31")]
    pub payload: prost::alloc::vec::Vec<qi_core_communication::Payload>,
    /// Comments made about the communication
    #[prost(message, repeated, tag = "32")]
    pub note: prost::alloc::vec::Vec<super::core::Annotation>,
}
/// Nested message and enum types in `QICoreCommunication`.
pub mod qi_core_communication {
    /// preparation | in-progress | not-done | suspended | aborted | completed |
    /// entered-in-error
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct StatusCode {
        #[prost(
            enumeration = "super::super::core::event_status_code::Value",
            tag = "1"
        )]
        pub value: i32,
        #[prost(message, optional, tag = "2")]
        pub id: ::core::option::Option<super::super::core::String>,
        #[prost(message, repeated, tag = "3")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
    }
    /// Message urgency
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct PriorityCode {
        #[prost(
            enumeration = "super::super::core::request_priority_code::Value",
            tag = "1"
        )]
        pub value: i32,
        #[prost(message, optional, tag = "2")]
        pub id: ::core::option::Option<super::super::core::String>,
        #[prost(message, repeated, tag = "3")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
    }
    /// Message payload
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct Payload {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag = "1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag = "2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag = "3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        #[prost(message, optional, tag = "4")]
        pub content: ::core::option::Option<payload::ContentX>,
    }
    /// Nested message and enum types in `Payload`.
    pub mod payload {
        /// Message part content
        #[derive(Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Message)]
        pub struct ContentX {
            #[prost(oneof = "content_x::Choice", tags = "1, 2, 3")]
            pub choice: ::core::option::Option<content_x::Choice>,
        }
        /// Nested message and enum types in `ContentX`.
        pub mod content_x {
            #[derive(Serialize, Deserialize)]
            #[serde(rename_all = "camelCase")]
            #[derive(Clone, PartialEq, prost::Oneof)]
            pub enum Choice {
                #[prost(message, tag = "1")]
                StringValue(super::super::super::super::core::String),
                #[prost(message, tag = "2")]
                Attachment(super::super::super::super::core::Attachment),
                #[prost(message, tag = "3")]
                Reference(super::super::super::super::core::Reference),
            }
        }
    }
}
/// Auto-generated from StructureDefinition for QICoreCommunicationRequest.
/// A request for information to be sent to a receiver.
/// See
/// <http://hl7.org/fhir/us/qicore/StructureDefinition/qicore-communicationrequest>
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct QiCoreCommunicationRequest {
    /// Logical id of this artifact
    #[prost(message, optional, tag = "1")]
    pub id: ::core::option::Option<super::core::Id>,
    /// Metadata about the resource
    #[prost(message, optional, tag = "2")]
    pub meta: ::core::option::Option<super::core::Meta>,
    /// A set of rules under which this content was created
    #[prost(message, optional, tag = "3")]
    pub implicit_rules: ::core::option::Option<super::core::Uri>,
    /// Language of the resource content
    #[prost(message, optional, tag = "4")]
    pub language: ::core::option::Option<super::core::Code>,
    /// Text summary of the resource, for human interpretation
    #[prost(message, optional, tag = "5")]
    pub text: ::core::option::Option<super::core::Narrative>,
    /// Contained, inline Resources
    #[prost(message, repeated, tag = "6")]
    pub contained: prost::alloc::vec::Vec<::prost_wkt_types::Any>,
    /// Additional content defined by implementations
    #[prost(message, repeated, tag = "8")]
    pub extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// Extensions that cannot be ignored
    #[prost(message, repeated, tag = "9")]
    pub modifier_extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// Unique identifier
    #[prost(message, repeated, tag = "10")]
    pub identifier: prost::alloc::vec::Vec<super::core::Identifier>,
    /// Fulfills plan or proposal
    #[prost(message, repeated, tag = "11")]
    pub based_on: prost::alloc::vec::Vec<super::core::Reference>,
    /// Request(s) replaced by this request
    #[prost(message, repeated, tag = "12")]
    pub replaces: prost::alloc::vec::Vec<super::core::Reference>,
    /// Composite request this is part of
    #[prost(message, optional, tag = "13")]
    pub group_identifier: ::core::option::Option<super::core::Identifier>,
    #[prost(message, optional, tag = "14")]
    pub status: ::core::option::Option<qi_core_communication_request::StatusCode>,
    /// Reason for current status
    #[prost(message, optional, tag = "15")]
    pub status_reason: ::core::option::Option<super::core::CodeableConcept>,
    /// Message category
    #[prost(message, repeated, tag = "16")]
    pub category: prost::alloc::vec::Vec<super::core::CodeableConcept>,
    #[prost(message, optional, tag = "17")]
    pub priority: ::core::option::Option<qi_core_communication_request::PriorityCode>,
    /// True if request is prohibiting action
    #[prost(message, optional, tag = "18")]
    pub do_not_perform: ::core::option::Option<super::core::Boolean>,
    /// A channel of communication
    #[prost(message, repeated, tag = "19")]
    pub medium: prost::alloc::vec::Vec<super::core::CodeableConcept>,
    /// Focus of message
    #[prost(message, optional, tag = "20")]
    pub subject: ::core::option::Option<super::core::Reference>,
    /// Resources that pertain to this communication request
    #[prost(message, repeated, tag = "21")]
    pub about: prost::alloc::vec::Vec<super::core::Reference>,
    /// Encounter created as part of
    #[prost(message, optional, tag = "22")]
    pub encounter: ::core::option::Option<super::core::Reference>,
    #[prost(message, repeated, tag = "23")]
    pub payload: prost::alloc::vec::Vec<qi_core_communication_request::Payload>,
    #[prost(message, optional, tag = "24")]
    pub occurrence: ::core::option::Option<qi_core_communication_request::OccurrenceX>,
    /// When request transitioned to being actionable
    #[prost(message, optional, tag = "25")]
    pub authored_on: ::core::option::Option<super::core::DateTime>,
    /// Who/what is requesting service
    #[prost(message, optional, tag = "26")]
    pub requester: ::core::option::Option<super::core::Reference>,
    /// Message recipient
    #[prost(message, repeated, tag = "27")]
    pub recipient: prost::alloc::vec::Vec<super::core::Reference>,
    /// Message sender
    #[prost(message, optional, tag = "28")]
    pub sender: ::core::option::Option<super::core::Reference>,
    /// Why is communication needed?
    #[prost(message, repeated, tag = "29")]
    pub reason_code: prost::alloc::vec::Vec<super::core::CodeableConcept>,
    /// Why is communication needed?
    #[prost(message, repeated, tag = "30")]
    pub reason_reference: prost::alloc::vec::Vec<super::core::Reference>,
    /// Comments made about communication request
    #[prost(message, repeated, tag = "31")]
    pub note: prost::alloc::vec::Vec<super::core::Annotation>,
}
/// Nested message and enum types in `QICoreCommunicationRequest`.
pub mod qi_core_communication_request {
    /// draft | active | suspended | cancelled | completed | entered-in-error |
    /// unknown
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct StatusCode {
        #[prost(
            enumeration = "super::super::core::request_status_code::Value",
            tag = "1"
        )]
        pub value: i32,
        #[prost(message, optional, tag = "2")]
        pub id: ::core::option::Option<super::super::core::String>,
        #[prost(message, repeated, tag = "3")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
    }
    /// Message urgency
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct PriorityCode {
        #[prost(
            enumeration = "super::super::core::request_priority_code::Value",
            tag = "1"
        )]
        pub value: i32,
        #[prost(message, optional, tag = "2")]
        pub id: ::core::option::Option<super::super::core::String>,
        #[prost(message, repeated, tag = "3")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
    }
    /// Message payload
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct Payload {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag = "1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag = "2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag = "3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        #[prost(message, optional, tag = "4")]
        pub content: ::core::option::Option<payload::ContentX>,
    }
    /// Nested message and enum types in `Payload`.
    pub mod payload {
        /// Message part content
        #[derive(Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Message)]
        pub struct ContentX {
            #[prost(oneof = "content_x::Choice", tags = "1, 2, 3")]
            pub choice: ::core::option::Option<content_x::Choice>,
        }
        /// Nested message and enum types in `ContentX`.
        pub mod content_x {
            #[derive(Serialize, Deserialize)]
            #[serde(rename_all = "camelCase")]
            #[derive(Clone, PartialEq, prost::Oneof)]
            pub enum Choice {
                #[prost(message, tag = "1")]
                StringValue(super::super::super::super::core::String),
                #[prost(message, tag = "2")]
                Attachment(super::super::super::super::core::Attachment),
                #[prost(message, tag = "3")]
                Reference(super::super::super::super::core::Reference),
            }
        }
    }
    /// When scheduled
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct OccurrenceX {
        #[prost(oneof = "occurrence_x::Choice", tags = "1, 2")]
        pub choice: ::core::option::Option<occurrence_x::Choice>,
    }
    /// Nested message and enum types in `OccurrenceX`.
    pub mod occurrence_x {
        #[derive(Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Oneof)]
        pub enum Choice {
            #[prost(message, tag = "1")]
            DateTime(super::super::super::core::DateTime),
            #[prost(message, tag = "2")]
            Period(super::super::super::core::Period),
        }
    }
}
/// Auto-generated from StructureDefinition for QICoreCondition.
/// Detailed information about conditions, problems or diagnoses.
/// See <http://hl7.org/fhir/us/qicore/StructureDefinition/qicore-condition>
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct QiCoreCondition {
    /// Logical id of this artifact
    #[prost(message, optional, tag = "1")]
    pub id: ::core::option::Option<super::core::Id>,
    /// Metadata about the resource
    #[prost(message, optional, tag = "2")]
    pub meta: ::core::option::Option<super::core::Meta>,
    /// A set of rules under which this content was created
    #[prost(message, optional, tag = "3")]
    pub implicit_rules: ::core::option::Option<super::core::Uri>,
    /// Language of the resource content
    #[prost(message, optional, tag = "4")]
    pub language: ::core::option::Option<super::core::Code>,
    /// Text summary of the resource, for human interpretation
    #[prost(message, optional, tag = "5")]
    pub text: ::core::option::Option<super::core::Narrative>,
    /// Contained, inline Resources
    #[prost(message, repeated, tag = "6")]
    pub contained: prost::alloc::vec::Vec<::prost_wkt_types::Any>,
    /// Extension
    #[prost(message, repeated, tag = "8")]
    pub extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// Extensions that cannot be ignored
    #[prost(message, repeated, tag = "9")]
    pub modifier_extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// External Ids for this condition
    #[prost(message, repeated, tag = "10")]
    pub identifier: prost::alloc::vec::Vec<super::core::Identifier>,
    /// active | recurrence | relapse | inactive | remission | resolved
    #[prost(message, optional, tag = "11")]
    pub clinical_status: ::core::option::Option<super::core::CodeableConcept>,
    /// unconfirmed | provisional | differential | confirmed | refuted |
    /// entered-in-error
    #[prost(message, optional, tag = "12")]
    pub verification_status: ::core::option::Option<super::core::CodeableConcept>,
    /// problem-list-item | encounter-diagnosis
    #[prost(message, repeated, tag = "13")]
    pub category: prost::alloc::vec::Vec<super::core::CodeableConcept>,
    /// Subjective severity of condition
    #[prost(message, optional, tag = "14")]
    pub severity: ::core::option::Option<super::core::CodeableConcept>,
    /// Identification of the condition, problem or diagnosis
    #[prost(message, optional, tag = "15")]
    pub code: ::core::option::Option<super::core::CodeableConcept>,
    /// Anatomical location, if relevant
    #[prost(message, repeated, tag = "16")]
    pub body_site: prost::alloc::vec::Vec<super::core::CodeableConcept>,
    /// Who has the condition?
    #[prost(message, optional, tag = "17")]
    pub subject: ::core::option::Option<super::core::Reference>,
    /// Encounter created as part of
    #[prost(message, optional, tag = "18")]
    pub encounter: ::core::option::Option<super::core::Reference>,
    #[prost(message, optional, tag = "19")]
    pub onset: ::core::option::Option<qi_core_condition::OnsetX>,
    #[prost(message, optional, tag = "20")]
    pub abatement: ::core::option::Option<qi_core_condition::AbatementX>,
    /// Date record was first recorded
    #[prost(message, optional, tag = "21")]
    pub recorded_date: ::core::option::Option<super::core::DateTime>,
    /// Who recorded the condition
    #[prost(message, optional, tag = "22")]
    pub recorder: ::core::option::Option<super::core::Reference>,
    /// Person who asserts this condition
    #[prost(message, optional, tag = "23")]
    pub asserter: ::core::option::Option<super::core::Reference>,
    #[prost(message, repeated, tag = "24")]
    pub stage: prost::alloc::vec::Vec<qi_core_condition::Stage>,
    #[prost(message, repeated, tag = "25")]
    pub evidence: prost::alloc::vec::Vec<qi_core_condition::Evidence>,
    /// Additional information about the Condition
    #[prost(message, repeated, tag = "26")]
    pub note: prost::alloc::vec::Vec<super::core::Annotation>,
    /// What caused the Condition?
    #[prost(message, repeated, tag = "27")]
    pub due_to: prost::alloc::vec::Vec<super::core::condition_due_to::ValueX>,
    /// Precedent for this Condition
    #[prost(message, repeated, tag = "28")]
    pub occurred_following:
        prost::alloc::vec::Vec<super::core::condition_occurred_following::ValueX>,
}
/// Nested message and enum types in `QICoreCondition`.
pub mod qi_core_condition {
    /// Estimated or actual date,  date-time, or age
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct OnsetX {
        #[prost(oneof = "onset_x::Choice", tags = "1, 2, 3, 4")]
        pub choice: ::core::option::Option<onset_x::Choice>,
    }
    /// Nested message and enum types in `OnsetX`.
    pub mod onset_x {
        #[derive(Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Oneof)]
        pub enum Choice {
            #[prost(message, tag = "1")]
            DateTime(super::super::super::core::DateTime),
            #[prost(message, tag = "2")]
            Age(super::super::super::core::Age),
            #[prost(message, tag = "3")]
            Period(super::super::super::core::Period),
            #[prost(message, tag = "4")]
            Range(super::super::super::core::Range),
        }
    }
    /// When in resolution/remission
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct AbatementX {
        #[prost(oneof = "abatement_x::Choice", tags = "1, 2, 3, 4")]
        pub choice: ::core::option::Option<abatement_x::Choice>,
    }
    /// Nested message and enum types in `AbatementX`.
    pub mod abatement_x {
        #[derive(Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Oneof)]
        pub enum Choice {
            #[prost(message, tag = "1")]
            DateTime(super::super::super::core::DateTime),
            #[prost(message, tag = "2")]
            Age(super::super::super::core::Age),
            #[prost(message, tag = "3")]
            Period(super::super::super::core::Period),
            #[prost(message, tag = "4")]
            Range(super::super::super::core::Range),
        }
    }
    /// Stage/grade, usually assessed formally
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct Stage {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag = "1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag = "2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag = "3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Simple summary (disease specific)
        #[prost(message, optional, tag = "4")]
        pub summary: ::core::option::Option<super::super::core::CodeableConcept>,
        /// Formal record of assessment
        #[prost(message, repeated, tag = "5")]
        pub assessment: prost::alloc::vec::Vec<super::super::core::Reference>,
        /// Kind of staging
        #[prost(message, optional, tag = "6")]
        pub r#type: ::core::option::Option<super::super::core::CodeableConcept>,
    }
    /// Supporting evidence
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct Evidence {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag = "1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag = "2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag = "3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Manifestation/symptom
        #[prost(message, repeated, tag = "4")]
        pub code: prost::alloc::vec::Vec<super::super::core::CodeableConcept>,
        /// Supporting information found elsewhere
        #[prost(message, repeated, tag = "5")]
        pub detail: prost::alloc::vec::Vec<super::super::core::Reference>,
    }
}
/// Auto-generated from StructureDefinition for QICoreCoverage.
/// Insurance or medical plan or a payment agreement.
/// See <http://hl7.org/fhir/us/qicore/StructureDefinition/qicore-coverage>
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct QiCoreCoverage {
    /// Logical id of this artifact
    #[prost(message, optional, tag = "1")]
    pub id: ::core::option::Option<super::core::Id>,
    /// Metadata about the resource
    #[prost(message, optional, tag = "2")]
    pub meta: ::core::option::Option<super::core::Meta>,
    /// A set of rules under which this content was created
    #[prost(message, optional, tag = "3")]
    pub implicit_rules: ::core::option::Option<super::core::Uri>,
    /// Language of the resource content
    #[prost(message, optional, tag = "4")]
    pub language: ::core::option::Option<super::core::Code>,
    /// Text summary of the resource, for human interpretation
    #[prost(message, optional, tag = "5")]
    pub text: ::core::option::Option<super::core::Narrative>,
    /// Contained, inline Resources
    #[prost(message, repeated, tag = "6")]
    pub contained: prost::alloc::vec::Vec<::prost_wkt_types::Any>,
    /// Additional content defined by implementations
    #[prost(message, repeated, tag = "8")]
    pub extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// Extensions that cannot be ignored
    #[prost(message, repeated, tag = "9")]
    pub modifier_extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// Business Identifier for the coverage
    #[prost(message, repeated, tag = "10")]
    pub identifier: prost::alloc::vec::Vec<super::core::Identifier>,
    #[prost(message, optional, tag = "11")]
    pub status: ::core::option::Option<qi_core_coverage::StatusCode>,
    /// Coverage category such as medical or accident
    #[prost(message, optional, tag = "12")]
    pub r#type: ::core::option::Option<super::core::CodeableConcept>,
    /// Owner of the policy
    #[prost(message, optional, tag = "13")]
    pub policy_holder: ::core::option::Option<super::core::Reference>,
    /// Subscriber to the policy
    #[prost(message, optional, tag = "14")]
    pub subscriber: ::core::option::Option<super::core::Reference>,
    /// ID assigned to the subscriber
    #[prost(message, optional, tag = "15")]
    pub subscriber_id: ::core::option::Option<super::core::String>,
    /// Plan beneficiary
    #[prost(message, optional, tag = "16")]
    pub beneficiary: ::core::option::Option<super::core::Reference>,
    /// Dependent number
    #[prost(message, optional, tag = "17")]
    pub dependent: ::core::option::Option<super::core::String>,
    /// Beneficiary relationship to the subscriber
    #[prost(message, optional, tag = "18")]
    pub relationship: ::core::option::Option<super::core::CodeableConcept>,
    /// Coverage start and end dates
    #[prost(message, optional, tag = "19")]
    pub period: ::core::option::Option<super::core::Period>,
    /// Issuer of the policy
    #[prost(message, repeated, tag = "20")]
    pub payor: prost::alloc::vec::Vec<super::core::Reference>,
    #[prost(message, repeated, tag = "21")]
    pub class_value: prost::alloc::vec::Vec<qi_core_coverage::Class>,
    /// Relative order of the coverage
    #[prost(message, optional, tag = "22")]
    pub order: ::core::option::Option<super::core::PositiveInt>,
    /// Insurer network
    #[prost(message, optional, tag = "23")]
    pub network: ::core::option::Option<super::core::String>,
    #[prost(message, repeated, tag = "24")]
    pub cost_to_beneficiary: prost::alloc::vec::Vec<qi_core_coverage::CostToBeneficiary>,
    /// Reimbursement to insurer
    #[prost(message, optional, tag = "25")]
    pub subrogation: ::core::option::Option<super::core::Boolean>,
    /// Contract details
    #[prost(message, repeated, tag = "26")]
    pub contract: prost::alloc::vec::Vec<super::core::Reference>,
}
/// Nested message and enum types in `QICoreCoverage`.
pub mod qi_core_coverage {
    /// active | cancelled | draft | entered-in-error
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct StatusCode {
        #[prost(
            enumeration = "super::super::core::financial_resource_status_code::Value",
            tag = "1"
        )]
        pub value: i32,
        #[prost(message, optional, tag = "2")]
        pub id: ::core::option::Option<super::super::core::String>,
        #[prost(message, repeated, tag = "3")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
    }
    /// Additional coverage classifications
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct Class {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag = "1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag = "2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag = "3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Type of class such as 'group' or 'plan'
        #[prost(message, optional, tag = "4")]
        pub r#type: ::core::option::Option<super::super::core::CodeableConcept>,
        /// Value associated with the type
        #[prost(message, optional, tag = "5")]
        pub value: ::core::option::Option<super::super::core::String>,
        /// Human readable description of the type and value
        #[prost(message, optional, tag = "6")]
        pub name: ::core::option::Option<super::super::core::String>,
    }
    /// Patient payments for services/products
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct CostToBeneficiary {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag = "1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag = "2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag = "3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Cost category
        #[prost(message, optional, tag = "4")]
        pub r#type: ::core::option::Option<super::super::core::CodeableConcept>,
        #[prost(message, optional, tag = "5")]
        pub value: ::core::option::Option<cost_to_beneficiary::ValueX>,
        #[prost(message, repeated, tag = "6")]
        pub exception: prost::alloc::vec::Vec<cost_to_beneficiary::Exemption>,
    }
    /// Nested message and enum types in `CostToBeneficiary`.
    pub mod cost_to_beneficiary {
        /// The amount or percentage due from the beneficiary
        #[derive(Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Message)]
        pub struct ValueX {
            #[prost(oneof = "value_x::Choice", tags = "1, 2")]
            pub choice: ::core::option::Option<value_x::Choice>,
        }
        /// Nested message and enum types in `ValueX`.
        pub mod value_x {
            #[derive(Serialize, Deserialize)]
            #[serde(rename_all = "camelCase")]
            #[derive(Clone, PartialEq, prost::Oneof)]
            pub enum Choice {
                #[prost(message, tag = "1")]
                Quantity(super::super::super::super::core::SimpleQuantity),
                #[prost(message, tag = "2")]
                Money(super::super::super::super::core::Money),
            }
        }
        /// Exceptions for patient payments
        #[derive(Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Message)]
        pub struct Exemption {
            /// Unique id for inter-element referencing
            #[prost(message, optional, tag = "1")]
            pub id: ::core::option::Option<super::super::super::core::String>,
            /// Additional content defined by implementations
            #[prost(message, repeated, tag = "2")]
            pub extension: prost::alloc::vec::Vec<super::super::super::core::Extension>,
            /// Extensions that cannot be ignored even if unrecognized
            #[prost(message, repeated, tag = "3")]
            pub modifier_extension: prost::alloc::vec::Vec<super::super::super::core::Extension>,
            /// Exception category
            #[prost(message, optional, tag = "4")]
            pub r#type: ::core::option::Option<super::super::super::core::CodeableConcept>,
            /// The effective period of the exception
            #[prost(message, optional, tag = "5")]
            pub period: ::core::option::Option<super::super::super::core::Period>,
        }
    }
}
/// Auto-generated from StructureDefinition for QICoreDevice.
/// Item used in healthcare.
/// See <http://hl7.org/fhir/us/qicore/StructureDefinition/qicore-device>
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct QiCoreDevice {
    /// Logical id of this artifact
    #[prost(message, optional, tag = "1")]
    pub id: ::core::option::Option<super::core::Id>,
    /// Metadata about the resource
    #[prost(message, optional, tag = "2")]
    pub meta: ::core::option::Option<super::core::Meta>,
    /// A set of rules under which this content was created
    #[prost(message, optional, tag = "3")]
    pub implicit_rules: ::core::option::Option<super::core::Uri>,
    /// Language of the resource content
    #[prost(message, optional, tag = "4")]
    pub language: ::core::option::Option<super::core::Code>,
    /// Text summary of the resource, for human interpretation
    #[prost(message, optional, tag = "5")]
    pub text: ::core::option::Option<super::core::Narrative>,
    /// Contained, inline Resources
    #[prost(message, repeated, tag = "6")]
    pub contained: prost::alloc::vec::Vec<::prost_wkt_types::Any>,
    /// Additional content defined by implementations
    #[prost(message, repeated, tag = "8")]
    pub extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// Extensions that cannot be ignored
    #[prost(message, repeated, tag = "9")]
    pub modifier_extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// Instance identifier
    #[prost(message, repeated, tag = "10")]
    pub identifier: prost::alloc::vec::Vec<super::core::Identifier>,
    /// The reference to the definition for the device
    #[prost(message, optional, tag = "11")]
    pub definition: ::core::option::Option<super::core::Reference>,
    #[prost(message, optional, tag = "12")]
    pub udi_carrier: ::core::option::Option<qi_core_device::UdiCarrier>,
    #[prost(message, optional, tag = "13")]
    pub status: ::core::option::Option<qi_core_device::StatusCode>,
    /// online | paused | standby | offline | not-ready | transduc-discon |
    /// hw-discon | off
    #[prost(message, repeated, tag = "14")]
    pub status_reason: prost::alloc::vec::Vec<super::core::CodeableConcept>,
    /// The distinct identification string
    #[prost(message, optional, tag = "15")]
    pub distinct_identifier: ::core::option::Option<super::core::String>,
    /// Name of device manufacturer
    #[prost(message, optional, tag = "16")]
    pub manufacturer: ::core::option::Option<super::core::String>,
    /// Date when the device was made
    #[prost(message, optional, tag = "17")]
    pub manufacture_date: ::core::option::Option<super::core::DateTime>,
    /// Date and time of expiry of this device (if applicable)
    #[prost(message, optional, tag = "18")]
    pub expiration_date: ::core::option::Option<super::core::DateTime>,
    /// Lot number of manufacture
    #[prost(message, optional, tag = "19")]
    pub lot_number: ::core::option::Option<super::core::String>,
    /// Serial number assigned by the manufacturer
    #[prost(message, optional, tag = "20")]
    pub serial_number: ::core::option::Option<super::core::String>,
    #[prost(message, repeated, tag = "21")]
    pub device_name: prost::alloc::vec::Vec<qi_core_device::DeviceName>,
    /// The model number for the device
    #[prost(message, optional, tag = "22")]
    pub model_number: ::core::option::Option<super::core::String>,
    /// The part number of the device
    #[prost(message, optional, tag = "23")]
    pub part_number: ::core::option::Option<super::core::String>,
    /// The kind or type of device
    #[prost(message, optional, tag = "24")]
    pub r#type: ::core::option::Option<super::core::CodeableConcept>,
    #[prost(message, repeated, tag = "25")]
    pub specialization: prost::alloc::vec::Vec<qi_core_device::Specialization>,
    #[prost(message, repeated, tag = "26")]
    pub version: prost::alloc::vec::Vec<qi_core_device::Version>,
    #[prost(message, repeated, tag = "27")]
    pub property: prost::alloc::vec::Vec<qi_core_device::Property>,
    /// Patient to whom Device is affixed
    #[prost(message, optional, tag = "28")]
    pub patient: ::core::option::Option<super::core::Reference>,
    /// Organization responsible for device
    #[prost(message, optional, tag = "29")]
    pub owner: ::core::option::Option<super::core::Reference>,
    /// Details for human/organization for support
    #[prost(message, repeated, tag = "30")]
    pub contact: prost::alloc::vec::Vec<super::core::ContactPoint>,
    /// Where the device is found
    #[prost(message, optional, tag = "31")]
    pub location: ::core::option::Option<super::core::Reference>,
    /// Network address to contact device
    #[prost(message, optional, tag = "32")]
    pub url: ::core::option::Option<super::core::Uri>,
    /// Device notes and comments
    #[prost(message, repeated, tag = "33")]
    pub note: prost::alloc::vec::Vec<super::core::Annotation>,
    /// Safety Characteristics of Device
    #[prost(message, repeated, tag = "34")]
    pub safety: prost::alloc::vec::Vec<super::core::CodeableConcept>,
    /// The parent device
    #[prost(message, optional, tag = "35")]
    pub parent: ::core::option::Option<super::core::Reference>,
}
/// Nested message and enum types in `QICoreDevice`.
pub mod qi_core_device {
    /// Unique Device Identifier (UDI) Barcode string
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct UdiCarrier {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag = "1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag = "2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag = "3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Mandatory fixed portion of UDI
        #[prost(message, optional, tag = "4")]
        pub device_identifier: ::core::option::Option<super::super::core::String>,
        /// UDI Issuing Organization
        #[prost(message, optional, tag = "5")]
        pub issuer: ::core::option::Option<super::super::core::Uri>,
        /// Regional UDI authority
        #[prost(message, optional, tag = "6")]
        pub jurisdiction: ::core::option::Option<super::super::core::Uri>,
        /// UDI Machine Readable Barcode String
        #[prost(message, optional, tag = "7")]
        pub carrier_aidc: ::core::option::Option<super::super::core::Base64Binary>,
        /// UDI Human Readable Barcode String
        #[prost(message, optional, tag = "8")]
        pub carrier_hrf: ::core::option::Option<super::super::core::String>,
        #[prost(message, optional, tag = "9")]
        pub entry_type: ::core::option::Option<udi_carrier::EntryTypeCode>,
    }
    /// Nested message and enum types in `UdiCarrier`.
    pub mod udi_carrier {
        /// barcode | rfid | manual +
        #[derive(Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Message)]
        pub struct EntryTypeCode {
            #[prost(
                enumeration = "super::super::super::core::udi_entry_type_code::Value",
                tag = "1"
            )]
            pub value: i32,
            #[prost(message, optional, tag = "2")]
            pub id: ::core::option::Option<super::super::super::core::String>,
            #[prost(message, repeated, tag = "3")]
            pub extension: prost::alloc::vec::Vec<super::super::super::core::Extension>,
        }
    }
    /// active | inactive | entered-in-error | unknown
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct StatusCode {
        #[prost(
            enumeration = "super::super::core::fhir_device_status_code::Value",
            tag = "1"
        )]
        pub value: i32,
        #[prost(message, optional, tag = "2")]
        pub id: ::core::option::Option<super::super::core::String>,
        #[prost(message, repeated, tag = "3")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
    }
    /// The name of the device as given by the manufacturer
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct DeviceName {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag = "1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag = "2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag = "3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// The name of the device
        #[prost(message, optional, tag = "4")]
        pub name: ::core::option::Option<super::super::core::String>,
        #[prost(message, optional, tag = "5")]
        pub r#type: ::core::option::Option<device_name::TypeCode>,
    }
    /// Nested message and enum types in `DeviceName`.
    pub mod device_name {
        /// udi-label-name | user-friendly-name | patient-reported-name |
        /// manufacturer-name | model-name | other
        #[derive(Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Message)]
        pub struct TypeCode {
            #[prost(
                enumeration = "super::super::super::core::device_name_type_code::Value",
                tag = "1"
            )]
            pub value: i32,
            #[prost(message, optional, tag = "2")]
            pub id: ::core::option::Option<super::super::super::core::String>,
            #[prost(message, repeated, tag = "3")]
            pub extension: prost::alloc::vec::Vec<super::super::super::core::Extension>,
        }
    }
    /// The capabilities supported on a  device, the standards to which the device
    /// conforms for a particular purpose, and used for the communication
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct Specialization {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag = "1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag = "2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag = "3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// The standard that is used to operate and communicate
        #[prost(message, optional, tag = "4")]
        pub system_type: ::core::option::Option<super::super::core::CodeableConcept>,
        /// The version of the standard that is used to operate and communicate
        #[prost(message, optional, tag = "5")]
        pub version: ::core::option::Option<super::super::core::String>,
    }
    /// The actual design of the device or software version running on the device
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct Version {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag = "1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag = "2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag = "3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// The type of the device version
        #[prost(message, optional, tag = "4")]
        pub r#type: ::core::option::Option<super::super::core::CodeableConcept>,
        /// A single component of the device version
        #[prost(message, optional, tag = "5")]
        pub component: ::core::option::Option<super::super::core::Identifier>,
        /// The version text
        #[prost(message, optional, tag = "6")]
        pub value: ::core::option::Option<super::super::core::String>,
    }
    /// The actual configuration settings of a device as it actually operates,
    /// e.g., regulation status, time properties
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct Property {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag = "1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag = "2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag = "3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Code that specifies the property DeviceDefinitionPropetyCode (Extensible)
        #[prost(message, optional, tag = "4")]
        pub r#type: ::core::option::Option<super::super::core::CodeableConcept>,
        /// Property value as a quantity
        #[prost(message, repeated, tag = "5")]
        pub value_quantity: prost::alloc::vec::Vec<super::super::core::Quantity>,
        /// Property value as a code, e.g., NTP4 (synced to NTP)
        #[prost(message, repeated, tag = "6")]
        pub value_code: prost::alloc::vec::Vec<super::super::core::CodeableConcept>,
    }
}
/// Auto-generated from StructureDefinition for QICoreDeviceRequest.
/// Medical device request.
/// See <http://hl7.org/fhir/us/qicore/StructureDefinition/qicore-devicerequest>
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct QiCoreDeviceRequest {
    /// Logical id of this artifact
    #[prost(message, optional, tag = "1")]
    pub id: ::core::option::Option<super::core::Id>,
    /// Metadata about the resource
    #[prost(message, optional, tag = "2")]
    pub meta: ::core::option::Option<super::core::Meta>,
    /// A set of rules under which this content was created
    #[prost(message, optional, tag = "3")]
    pub implicit_rules: ::core::option::Option<super::core::Uri>,
    /// Language of the resource content
    #[prost(message, optional, tag = "4")]
    pub language: ::core::option::Option<super::core::Code>,
    /// Text summary of the resource, for human interpretation
    #[prost(message, optional, tag = "5")]
    pub text: ::core::option::Option<super::core::Narrative>,
    /// Contained, inline Resources
    #[prost(message, repeated, tag = "6")]
    pub contained: prost::alloc::vec::Vec<::prost_wkt_types::Any>,
    /// Extension
    #[prost(message, repeated, tag = "8")]
    pub extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// Extensions that cannot be ignored
    #[prost(message, repeated, tag = "9")]
    pub modifier_extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// External Request identifier
    #[prost(message, repeated, tag = "10")]
    pub identifier: prost::alloc::vec::Vec<super::core::Identifier>,
    /// Instantiates FHIR protocol or definition
    #[prost(message, repeated, tag = "11")]
    pub instantiates_canonical: prost::alloc::vec::Vec<super::core::Canonical>,
    /// Instantiates external protocol or definition
    #[prost(message, repeated, tag = "12")]
    pub instantiates_uri: prost::alloc::vec::Vec<super::core::Uri>,
    /// What request fulfills
    #[prost(message, repeated, tag = "13")]
    pub based_on: prost::alloc::vec::Vec<super::core::Reference>,
    /// What request replaces
    #[prost(message, repeated, tag = "14")]
    pub prior_request: prost::alloc::vec::Vec<super::core::Reference>,
    /// Identifier of composite request
    #[prost(message, optional, tag = "15")]
    pub group_identifier: ::core::option::Option<super::core::Identifier>,
    #[prost(message, optional, tag = "16")]
    pub status: ::core::option::Option<qi_core_device_request::StatusCode>,
    #[prost(message, optional, tag = "17")]
    pub intent: ::core::option::Option<qi_core_device_request::IntentCode>,
    #[prost(message, optional, tag = "18")]
    pub priority: ::core::option::Option<qi_core_device_request::PriorityCode>,
    #[prost(message, optional, tag = "19")]
    pub code: ::core::option::Option<qi_core_device_request::CodeX>,
    #[prost(message, repeated, tag = "20")]
    pub parameter: prost::alloc::vec::Vec<qi_core_device_request::Parameter>,
    /// Focus of request
    #[prost(message, optional, tag = "21")]
    pub subject: ::core::option::Option<super::core::Reference>,
    /// Encounter motivating request
    #[prost(message, optional, tag = "22")]
    pub encounter: ::core::option::Option<super::core::Reference>,
    #[prost(message, optional, tag = "23")]
    pub occurrence: ::core::option::Option<qi_core_device_request::OccurrenceX>,
    /// When recorded
    #[prost(message, optional, tag = "24")]
    pub authored_on: ::core::option::Option<super::core::DateTime>,
    /// Who/what is requesting diagnostics
    #[prost(message, optional, tag = "25")]
    pub requester: ::core::option::Option<super::core::Reference>,
    /// Filler role
    #[prost(message, optional, tag = "26")]
    pub performer_type: ::core::option::Option<super::core::CodeableConcept>,
    /// Requested Filler
    #[prost(message, optional, tag = "27")]
    pub performer: ::core::option::Option<super::core::Reference>,
    /// Coded Reason for request
    #[prost(message, repeated, tag = "28")]
    pub reason_code: prost::alloc::vec::Vec<super::core::CodeableConcept>,
    /// Linked Reason for request
    #[prost(message, repeated, tag = "29")]
    pub reason_reference: prost::alloc::vec::Vec<super::core::Reference>,
    /// Associated insurance coverage
    #[prost(message, repeated, tag = "30")]
    pub insurance: prost::alloc::vec::Vec<super::core::Reference>,
    /// Additional clinical information
    #[prost(message, repeated, tag = "31")]
    pub supporting_info: prost::alloc::vec::Vec<super::core::Reference>,
    /// Notes or comments
    #[prost(message, repeated, tag = "32")]
    pub note: prost::alloc::vec::Vec<super::core::Annotation>,
    /// Request provenance
    #[prost(message, repeated, tag = "33")]
    pub relevant_history: prost::alloc::vec::Vec<super::core::Reference>,
    /// true if request is prohibiting action
    #[prost(message, optional, tag = "34")]
    pub do_not_perform: ::core::option::Option<super::core::Boolean>,
}
/// Nested message and enum types in `QICoreDeviceRequest`.
pub mod qi_core_device_request {
    /// draft | active | suspended | completed | entered-in-error | cancelled
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct StatusCode {
        #[prost(
            enumeration = "super::super::core::request_status_code::Value",
            tag = "1"
        )]
        pub value: i32,
        #[prost(message, optional, tag = "2")]
        pub id: ::core::option::Option<super::super::core::String>,
        #[prost(message, repeated, tag = "3")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
    }
    /// proposal | plan | original-order | encoded | reflex-order
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct IntentCode {
        #[prost(
            enumeration = "super::super::core::request_intent_code::Value",
            tag = "1"
        )]
        pub value: i32,
        #[prost(message, optional, tag = "2")]
        pub id: ::core::option::Option<super::super::core::String>,
        #[prost(message, repeated, tag = "3")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
    }
    /// Indicates how quickly the {{title}} should be addressed with respect to
    /// other requests
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct PriorityCode {
        #[prost(
            enumeration = "super::super::core::request_priority_code::Value",
            tag = "1"
        )]
        pub value: i32,
        #[prost(message, optional, tag = "2")]
        pub id: ::core::option::Option<super::super::core::String>,
        #[prost(message, repeated, tag = "3")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
    }
    /// Device requested
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct CodeX {
        #[prost(oneof = "code_x::Choice", tags = "1, 2")]
        pub choice: ::core::option::Option<code_x::Choice>,
    }
    /// Nested message and enum types in `CodeX`.
    pub mod code_x {
        #[derive(Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Oneof)]
        pub enum Choice {
            #[prost(message, tag = "1")]
            Reference(super::super::super::core::Reference),
            #[prost(message, tag = "2")]
            CodeableConcept(super::super::super::core::CodeableConcept),
        }
    }
    /// Device details
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct Parameter {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag = "1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag = "2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag = "3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Device detail
        #[prost(message, optional, tag = "4")]
        pub code: ::core::option::Option<super::super::core::CodeableConcept>,
        #[prost(message, optional, tag = "5")]
        pub value: ::core::option::Option<parameter::ValueX>,
    }
    /// Nested message and enum types in `Parameter`.
    pub mod parameter {
        /// Value of detail
        #[derive(Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Message)]
        pub struct ValueX {
            #[prost(oneof = "value_x::Choice", tags = "1, 2, 3, 4")]
            pub choice: ::core::option::Option<value_x::Choice>,
        }
        /// Nested message and enum types in `ValueX`.
        pub mod value_x {
            #[derive(Serialize, Deserialize)]
            #[serde(rename_all = "camelCase")]
            #[derive(Clone, PartialEq, prost::Oneof)]
            pub enum Choice {
                #[prost(message, tag = "1")]
                CodeableConcept(super::super::super::super::core::CodeableConcept),
                #[prost(message, tag = "2")]
                Quantity(super::super::super::super::core::Quantity),
                #[prost(message, tag = "3")]
                Range(super::super::super::super::core::Range),
                #[prost(message, tag = "4")]
                Boolean(super::super::super::super::core::Boolean),
            }
        }
    }
    /// Desired time or schedule for use
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct OccurrenceX {
        #[prost(oneof = "occurrence_x::Choice", tags = "1, 2, 3")]
        pub choice: ::core::option::Option<occurrence_x::Choice>,
    }
    /// Nested message and enum types in `OccurrenceX`.
    pub mod occurrence_x {
        #[derive(Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Oneof)]
        pub enum Choice {
            #[prost(message, tag = "1")]
            DateTime(super::super::super::core::DateTime),
            #[prost(message, tag = "2")]
            Period(super::super::super::core::Period),
            #[prost(message, tag = "3")]
            Timing(super::super::super::core::Timing),
        }
    }
}
/// Auto-generated from StructureDefinition for QICore_Not_Done_Extension.
/// Device Not Used Extension.
/// See
/// <http://hl7.org/fhir/us/qicore/StructureDefinition/qicore-deviceusestatement-notDone>
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct DeviceUseStatementQiCoreNotDoneExtension {
    /// Unique id for inter-element referencing
    #[prost(message, optional, tag = "1")]
    pub id: ::core::option::Option<super::core::String>,
    /// Additional content defined by implementations
    #[prost(message, repeated, tag = "2")]
    pub extension: prost::alloc::vec::Vec<super::core::Extension>,
    #[prost(message, optional, tag = "3")]
    pub value: ::core::option::Option<device_use_statement_qi_core_not_done_extension::ValueX>,
}
/// Nested message and enum types in `DeviceUseStatementQICoreNotDoneExtension`.
pub mod device_use_statement_qi_core_not_done_extension {
    /// Value of extension
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct ValueX {
        #[prost(oneof = "value_x::Choice", tags = "1")]
        pub choice: ::core::option::Option<value_x::Choice>,
    }
    /// Nested message and enum types in `ValueX`.
    pub mod value_x {
        #[derive(Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Oneof)]
        pub enum Choice {
            #[prost(message, tag = "1")]
            Boolean(super::super::super::core::Boolean),
        }
    }
}
/// Auto-generated from StructureDefinition for QICoreDeviceUseStatement.
/// Record of use of a device.
/// See
/// <http://hl7.org/fhir/us/qicore/StructureDefinition/qicore-deviceusestatement>
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct QiCoreDeviceUseStatement {
    /// Logical id of this artifact
    #[prost(message, optional, tag = "1")]
    pub id: ::core::option::Option<super::core::Id>,
    /// Metadata about the resource
    #[prost(message, optional, tag = "2")]
    pub meta: ::core::option::Option<super::core::Meta>,
    /// A set of rules under which this content was created
    #[prost(message, optional, tag = "3")]
    pub implicit_rules: ::core::option::Option<super::core::Uri>,
    /// Language of the resource content
    #[prost(message, optional, tag = "4")]
    pub language: ::core::option::Option<super::core::Code>,
    /// Text summary of the resource, for human interpretation
    #[prost(message, optional, tag = "5")]
    pub text: ::core::option::Option<super::core::Narrative>,
    /// Contained, inline Resources
    #[prost(message, repeated, tag = "6")]
    pub contained: prost::alloc::vec::Vec<::prost_wkt_types::Any>,
    /// Extension
    #[prost(message, repeated, tag = "8")]
    pub extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// Extensions that cannot be ignored
    #[prost(message, repeated, tag = "9")]
    pub modifier_extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// External identifier for this record
    #[prost(message, repeated, tag = "10")]
    pub identifier: prost::alloc::vec::Vec<super::core::Identifier>,
    /// Fulfills plan, proposal or order
    #[prost(message, repeated, tag = "11")]
    pub based_on: prost::alloc::vec::Vec<super::core::Reference>,
    #[prost(message, optional, tag = "12")]
    pub status: ::core::option::Option<qi_core_device_use_statement::StatusCode>,
    /// Patient using device
    #[prost(message, optional, tag = "13")]
    pub subject: ::core::option::Option<super::core::Reference>,
    /// Supporting information
    #[prost(message, repeated, tag = "14")]
    pub derived_from: prost::alloc::vec::Vec<super::core::Reference>,
    #[prost(message, optional, tag = "15")]
    pub timing: ::core::option::Option<qi_core_device_use_statement::TimingX>,
    /// When statement was recorded
    #[prost(message, optional, tag = "16")]
    pub recorded_on: ::core::option::Option<super::core::DateTime>,
    /// Who made the statement
    #[prost(message, optional, tag = "17")]
    pub source: ::core::option::Option<super::core::Reference>,
    /// Reference to device used
    #[prost(message, optional, tag = "18")]
    pub device: ::core::option::Option<super::core::Reference>,
    /// Why device was used
    #[prost(message, repeated, tag = "19")]
    pub reason_code: prost::alloc::vec::Vec<super::core::CodeableConcept>,
    /// Why was DeviceUseStatement performed?
    #[prost(message, repeated, tag = "20")]
    pub reason_reference: prost::alloc::vec::Vec<super::core::Reference>,
    /// Target body site
    #[prost(message, optional, tag = "21")]
    pub body_site: ::core::option::Option<super::core::CodeableConcept>,
    /// Addition details (comments, instructions)
    #[prost(message, repeated, tag = "22")]
    pub note: prost::alloc::vec::Vec<super::core::Annotation>,
    /// Device Not Used Extension
    #[prost(message, optional, tag = "23")]
    pub not_done: ::core::option::Option<super::core::Boolean>,
}
/// Nested message and enum types in `QICoreDeviceUseStatement`.
pub mod qi_core_device_use_statement {
    /// active | completed | entered-in-error +
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct StatusCode {
        #[prost(
            enumeration = "super::super::core::device_use_statement_status_code::Value",
            tag = "1"
        )]
        pub value: i32,
        #[prost(message, optional, tag = "2")]
        pub id: ::core::option::Option<super::super::core::String>,
        #[prost(message, repeated, tag = "3")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
    }
    /// How often  the device was used
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct TimingX {
        #[prost(oneof = "timing_x::Choice", tags = "1, 2, 3")]
        pub choice: ::core::option::Option<timing_x::Choice>,
    }
    /// Nested message and enum types in `TimingX`.
    pub mod timing_x {
        #[derive(Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Oneof)]
        pub enum Choice {
            #[prost(message, tag = "1")]
            Timing(super::super::super::core::Timing),
            #[prost(message, tag = "2")]
            Period(super::super::super::core::Period),
            #[prost(message, tag = "3")]
            DateTime(super::super::super::core::DateTime),
        }
    }
}
/// Auto-generated from StructureDefinition for QICoreDiagnosticReportLab.
/// A Diagnostic report - a combination of request information, atomic results,
/// images, interpretation, as well as formatted reports. See
/// <http://hl7.org/fhir/us/qicore/StructureDefinition/qicore-diagnosticreport-lab>
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct QiCoreDiagnosticReportLab {
    /// Logical id of this artifact
    #[prost(message, optional, tag = "1")]
    pub id: ::core::option::Option<super::core::Id>,
    /// Metadata about the resource
    #[prost(message, optional, tag = "2")]
    pub meta: ::core::option::Option<super::core::Meta>,
    /// A set of rules under which this content was created
    #[prost(message, optional, tag = "3")]
    pub implicit_rules: ::core::option::Option<super::core::Uri>,
    /// Language of the resource content
    #[prost(message, optional, tag = "4")]
    pub language: ::core::option::Option<super::core::Code>,
    /// Text summary of the resource, for human interpretation
    #[prost(message, optional, tag = "5")]
    pub text: ::core::option::Option<super::core::Narrative>,
    /// Contained, inline Resources
    #[prost(message, repeated, tag = "6")]
    pub contained: prost::alloc::vec::Vec<::prost_wkt_types::Any>,
    /// Extension
    #[prost(message, repeated, tag = "8")]
    pub extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// Extensions that cannot be ignored
    #[prost(message, repeated, tag = "9")]
    pub modifier_extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// Business identifier for report
    #[prost(message, repeated, tag = "10")]
    pub identifier: prost::alloc::vec::Vec<super::core::Identifier>,
    /// What was requested
    #[prost(message, repeated, tag = "11")]
    pub based_on: prost::alloc::vec::Vec<super::core::Reference>,
    #[prost(message, optional, tag = "12")]
    pub status: ::core::option::Option<qi_core_diagnostic_report_lab::StatusCode>,
    /// Service category
    #[prost(message, repeated, tag = "13")]
    pub category: prost::alloc::vec::Vec<super::core::CodeableConcept>,
    /// US Core Laboratory Report Order Code
    #[prost(message, optional, tag = "14")]
    pub code: ::core::option::Option<super::core::CodeableConcept>,
    /// The subject of the report - usually, but not always, the patient
    #[prost(message, optional, tag = "15")]
    pub subject: ::core::option::Option<super::core::Reference>,
    /// Health care event when test ordered
    #[prost(message, optional, tag = "16")]
    pub encounter: ::core::option::Option<super::core::Reference>,
    #[prost(message, optional, tag = "17")]
    pub effective: ::core::option::Option<qi_core_diagnostic_report_lab::EffectiveX>,
    /// DateTime this version was made
    #[prost(message, optional, tag = "18")]
    pub issued: ::core::option::Option<super::core::Instant>,
    /// Responsible Diagnostic Service
    #[prost(message, repeated, tag = "19")]
    pub performer: prost::alloc::vec::Vec<super::core::Reference>,
    /// Primary result interpreter
    #[prost(message, repeated, tag = "20")]
    pub results_interpreter: prost::alloc::vec::Vec<super::core::Reference>,
    /// Specimens this report is based on
    #[prost(message, repeated, tag = "21")]
    pub specimen: prost::alloc::vec::Vec<super::core::Reference>,
    /// Observations
    #[prost(message, repeated, tag = "22")]
    pub result: prost::alloc::vec::Vec<super::core::Reference>,
    /// Reference to full details of imaging associated with the diagnostic report
    #[prost(message, repeated, tag = "23")]
    pub imaging_study: prost::alloc::vec::Vec<super::core::Reference>,
    #[prost(message, repeated, tag = "24")]
    pub media: prost::alloc::vec::Vec<qi_core_diagnostic_report_lab::Media>,
    /// Clinical conclusion (interpretation) of test results
    #[prost(message, optional, tag = "25")]
    pub conclusion: ::core::option::Option<super::core::String>,
    /// Codes for the clinical conclusion of test results
    #[prost(message, repeated, tag = "26")]
    pub conclusion_code: prost::alloc::vec::Vec<super::core::CodeableConcept>,
    /// Entire report as issued
    #[prost(message, repeated, tag = "27")]
    pub presented_form: prost::alloc::vec::Vec<super::core::Attachment>,
    /// Location Performed
    #[prost(message, optional, tag = "28")]
    pub location_performed: ::core::option::Option<super::core::Reference>,
}
/// Nested message and enum types in `QICoreDiagnosticReportLab`.
pub mod qi_core_diagnostic_report_lab {
    /// registered | partial | preliminary | final +
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct StatusCode {
        #[prost(
            enumeration = "super::super::core::diagnostic_report_status_code::Value",
            tag = "1"
        )]
        pub value: i32,
        #[prost(message, optional, tag = "2")]
        pub id: ::core::option::Option<super::super::core::String>,
        #[prost(message, repeated, tag = "3")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
    }
    /// Specimen Collection Datetime or Period
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct EffectiveX {
        #[prost(oneof = "effective_x::Choice", tags = "1, 2")]
        pub choice: ::core::option::Option<effective_x::Choice>,
    }
    /// Nested message and enum types in `EffectiveX`.
    pub mod effective_x {
        #[derive(Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Oneof)]
        pub enum Choice {
            #[prost(message, tag = "1")]
            DateTime(super::super::super::core::DateTime),
            #[prost(message, tag = "2")]
            Period(super::super::super::core::Period),
        }
    }
    /// Key images associated with this report
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct Media {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag = "1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag = "2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag = "3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Comment about the image (e.g. explanation)
        #[prost(message, optional, tag = "4")]
        pub comment: ::core::option::Option<super::super::core::String>,
        /// Reference to the image source
        #[prost(message, optional, tag = "5")]
        pub link: ::core::option::Option<super::super::core::Reference>,
    }
}
/// Auto-generated from StructureDefinition for QICoreDiagnosticReportNote.
/// US Core Diagnostic Report Profile for Report and Note exchange.
/// See
/// <http://hl7.org/fhir/us/qicore/StructureDefinition/qicore-diagnosticreport-note>
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct QiCoreDiagnosticReportNote {
    /// Logical id of this artifact
    #[prost(message, optional, tag = "1")]
    pub id: ::core::option::Option<super::core::Id>,
    /// Metadata about the resource
    #[prost(message, optional, tag = "2")]
    pub meta: ::core::option::Option<super::core::Meta>,
    /// A set of rules under which this content was created
    #[prost(message, optional, tag = "3")]
    pub implicit_rules: ::core::option::Option<super::core::Uri>,
    /// Language of the resource content
    #[prost(message, optional, tag = "4")]
    pub language: ::core::option::Option<super::core::Code>,
    /// Text summary of the resource, for human interpretation
    #[prost(message, optional, tag = "5")]
    pub text: ::core::option::Option<super::core::Narrative>,
    /// Contained, inline Resources
    #[prost(message, repeated, tag = "6")]
    pub contained: prost::alloc::vec::Vec<::prost_wkt_types::Any>,
    /// Extension
    #[prost(message, repeated, tag = "8")]
    pub extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// Extensions that cannot be ignored
    #[prost(message, repeated, tag = "9")]
    pub modifier_extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// Business identifier for report
    #[prost(message, repeated, tag = "10")]
    pub identifier: prost::alloc::vec::Vec<super::core::Identifier>,
    /// What was requested
    #[prost(message, repeated, tag = "11")]
    pub based_on: prost::alloc::vec::Vec<super::core::Reference>,
    #[prost(message, optional, tag = "12")]
    pub status: ::core::option::Option<qi_core_diagnostic_report_note::StatusCode>,
    /// Service category
    #[prost(message, repeated, tag = "13")]
    pub category: prost::alloc::vec::Vec<super::core::CodeableConcept>,
    /// US Core Report Code
    #[prost(message, optional, tag = "14")]
    pub code: ::core::option::Option<super::core::CodeableConcept>,
    /// The subject of the report - usually, but not always, the patient
    #[prost(message, optional, tag = "15")]
    pub subject: ::core::option::Option<super::core::Reference>,
    /// Health care event when test ordered
    #[prost(message, optional, tag = "16")]
    pub encounter: ::core::option::Option<super::core::Reference>,
    #[prost(message, optional, tag = "17")]
    pub effective: ::core::option::Option<qi_core_diagnostic_report_note::EffectiveX>,
    /// DateTime this version was made
    #[prost(message, optional, tag = "18")]
    pub issued: ::core::option::Option<super::core::Instant>,
    /// Responsible Diagnostic Service
    #[prost(message, repeated, tag = "19")]
    pub performer: prost::alloc::vec::Vec<super::core::Reference>,
    /// Primary result interpreter
    #[prost(message, repeated, tag = "20")]
    pub results_interpreter: prost::alloc::vec::Vec<super::core::Reference>,
    /// Specimens this report is based on
    #[prost(message, repeated, tag = "21")]
    pub specimen: prost::alloc::vec::Vec<super::core::Reference>,
    /// Observations
    #[prost(message, repeated, tag = "22")]
    pub result: prost::alloc::vec::Vec<super::core::Reference>,
    /// Reference to full details of imaging associated with the diagnostic report
    #[prost(message, repeated, tag = "23")]
    pub imaging_study: prost::alloc::vec::Vec<super::core::Reference>,
    #[prost(message, repeated, tag = "24")]
    pub media: prost::alloc::vec::Vec<qi_core_diagnostic_report_note::Media>,
    /// Clinical conclusion (interpretation) of test results
    #[prost(message, optional, tag = "25")]
    pub conclusion: ::core::option::Option<super::core::String>,
    /// Codes for the clinical conclusion of test results
    #[prost(message, repeated, tag = "26")]
    pub conclusion_code: prost::alloc::vec::Vec<super::core::CodeableConcept>,
    /// Entire report as issued
    #[prost(message, repeated, tag = "27")]
    pub presented_form: prost::alloc::vec::Vec<super::core::Attachment>,
    /// Location Performed
    #[prost(message, optional, tag = "28")]
    pub location_performed: ::core::option::Option<super::core::Reference>,
}
/// Nested message and enum types in `QICoreDiagnosticReportNote`.
pub mod qi_core_diagnostic_report_note {
    /// registered | partial | preliminary | final +
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct StatusCode {
        #[prost(
            enumeration = "super::super::core::diagnostic_report_status_code::Value",
            tag = "1"
        )]
        pub value: i32,
        #[prost(message, optional, tag = "2")]
        pub id: ::core::option::Option<super::super::core::String>,
        #[prost(message, repeated, tag = "3")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
    }
    /// Time of the report or note
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct EffectiveX {
        #[prost(oneof = "effective_x::Choice", tags = "1, 2")]
        pub choice: ::core::option::Option<effective_x::Choice>,
    }
    /// Nested message and enum types in `EffectiveX`.
    pub mod effective_x {
        #[derive(Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Oneof)]
        pub enum Choice {
            #[prost(message, tag = "1")]
            DateTime(super::super::super::core::DateTime),
            #[prost(message, tag = "2")]
            Period(super::super::super::core::Period),
        }
    }
    /// Key images associated with this report
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct Media {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag = "1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag = "2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag = "3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Comment about the image (e.g. explanation)
        #[prost(message, optional, tag = "4")]
        pub comment: ::core::option::Option<super::super::core::String>,
        /// Reference to the image source
        #[prost(message, optional, tag = "5")]
        pub link: ::core::option::Option<super::super::core::Reference>,
    }
}
/// Auto-generated from StructureDefinition for EncounterProcedureExtension.
/// Encounter Procedure Extension.
/// See
/// <http://hl7.org/fhir/us/qicore/StructureDefinition/qicore-encounter-procedure>
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct EncounterEncounterProcedureExtension {
    /// Unique id for inter-element referencing
    #[prost(message, optional, tag = "1")]
    pub id: ::core::option::Option<super::core::String>,
    /// Additional content defined by implementations
    #[prost(message, repeated, tag = "2")]
    pub extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// type
    #[prost(message, optional, tag = "4")]
    pub r#type: ::core::option::Option<super::core::CodeableConcept>,
    /// rank
    #[prost(message, optional, tag = "5")]
    pub rank: ::core::option::Option<super::core::PositiveInt>,
    /// procedure
    #[prost(message, optional, tag = "6")]
    pub procedure: ::core::option::Option<super::core::Reference>,
}
/// Auto-generated from StructureDefinition for QICoreEncounter.
/// An interaction during which services are provided to the patient.
/// See <http://hl7.org/fhir/us/qicore/StructureDefinition/qicore-encounter>
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct QiCoreEncounter {
    /// Logical id of this artifact
    #[prost(message, optional, tag = "1")]
    pub id: ::core::option::Option<super::core::Id>,
    /// Metadata about the resource
    #[prost(message, optional, tag = "2")]
    pub meta: ::core::option::Option<super::core::Meta>,
    /// A set of rules under which this content was created
    #[prost(message, optional, tag = "3")]
    pub implicit_rules: ::core::option::Option<super::core::Uri>,
    /// Language of the resource content
    #[prost(message, optional, tag = "4")]
    pub language: ::core::option::Option<super::core::Code>,
    /// Text summary of the resource, for human interpretation
    #[prost(message, optional, tag = "5")]
    pub text: ::core::option::Option<super::core::Narrative>,
    /// Contained, inline Resources
    #[prost(message, repeated, tag = "6")]
    pub contained: prost::alloc::vec::Vec<::prost_wkt_types::Any>,
    /// Extension
    #[prost(message, repeated, tag = "8")]
    pub extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// Extensions that cannot be ignored
    #[prost(message, repeated, tag = "9")]
    pub modifier_extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// Identifier(s) by which this encounter is known
    #[prost(message, repeated, tag = "10")]
    pub identifier: prost::alloc::vec::Vec<super::core::Identifier>,
    #[prost(message, optional, tag = "11")]
    pub status: ::core::option::Option<qi_core_encounter::StatusCode>,
    #[prost(message, repeated, tag = "12")]
    pub status_history: prost::alloc::vec::Vec<qi_core_encounter::StatusHistory>,
    /// Classification of patient encounter
    #[prost(message, optional, tag = "13")]
    pub class_value: ::core::option::Option<super::core::Coding>,
    #[prost(message, repeated, tag = "14")]
    pub class_history: prost::alloc::vec::Vec<qi_core_encounter::ClassHistory>,
    /// Specific type of encounter
    #[prost(message, repeated, tag = "15")]
    pub r#type: prost::alloc::vec::Vec<super::core::CodeableConcept>,
    /// Specific type of service
    #[prost(message, optional, tag = "16")]
    pub service_type: ::core::option::Option<super::core::CodeableConcept>,
    /// Indicates the urgency of the encounter
    #[prost(message, optional, tag = "17")]
    pub priority: ::core::option::Option<super::core::CodeableConcept>,
    /// The patient or group present at the encounter
    #[prost(message, optional, tag = "18")]
    pub subject: ::core::option::Option<super::core::Reference>,
    /// Episode(s) of care that this encounter should be recorded against
    #[prost(message, repeated, tag = "19")]
    pub episode_of_care: prost::alloc::vec::Vec<super::core::Reference>,
    /// The ServiceRequest that initiated this encounter
    #[prost(message, repeated, tag = "20")]
    pub based_on: prost::alloc::vec::Vec<super::core::Reference>,
    #[prost(message, repeated, tag = "21")]
    pub participant: prost::alloc::vec::Vec<qi_core_encounter::Participant>,
    /// The appointment that scheduled this encounter
    #[prost(message, repeated, tag = "22")]
    pub appointment: prost::alloc::vec::Vec<super::core::Reference>,
    /// The start and end time of the encounter
    #[prost(message, optional, tag = "23")]
    pub period: ::core::option::Option<super::core::Period>,
    /// Quantity of time the encounter lasted (less time absent)
    #[prost(message, optional, tag = "24")]
    pub length: ::core::option::Option<super::core::Duration>,
    /// Coded reason the encounter takes place
    #[prost(message, repeated, tag = "25")]
    pub reason_code: prost::alloc::vec::Vec<super::core::CodeableConcept>,
    /// Reason the encounter takes place (reference)
    #[prost(message, repeated, tag = "26")]
    pub reason_reference: prost::alloc::vec::Vec<super::core::Reference>,
    #[prost(message, repeated, tag = "27")]
    pub diagnosis: prost::alloc::vec::Vec<qi_core_encounter::Diagnosis>,
    /// The set of accounts that may be used for billing for this Encounter
    #[prost(message, repeated, tag = "28")]
    pub account: prost::alloc::vec::Vec<super::core::Reference>,
    #[prost(message, optional, tag = "29")]
    pub hospitalization: ::core::option::Option<qi_core_encounter::Hospitalization>,
    #[prost(message, repeated, tag = "30")]
    pub location: prost::alloc::vec::Vec<qi_core_encounter::Location>,
    /// The organization (facility) responsible for this encounter
    #[prost(message, optional, tag = "31")]
    pub service_provider: ::core::option::Option<super::core::Reference>,
    /// Another Encounter this encounter is part of
    #[prost(message, optional, tag = "32")]
    pub part_of: ::core::option::Option<super::core::Reference>,
    /// Explanation for cancellation
    #[prost(message, optional, tag = "33")]
    pub reason_cancelled: ::core::option::Option<super::core::CodeableConcept>,
    /// Encounter Procedure Extension
    #[prost(message, repeated, tag = "34")]
    pub procedure: prost::alloc::vec::Vec<EncounterEncounterProcedureExtension>,
}
/// Nested message and enum types in `QICoreEncounter`.
pub mod qi_core_encounter {
    /// planned | arrived | triaged | in-progress | onleave | finished | cancelled
    /// +
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct StatusCode {
        #[prost(
            enumeration = "super::super::core::encounter_status_code::Value",
            tag = "1"
        )]
        pub value: i32,
        #[prost(message, optional, tag = "2")]
        pub id: ::core::option::Option<super::super::core::String>,
        #[prost(message, repeated, tag = "3")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
    }
    /// List of past encounter statuses
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct StatusHistory {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag = "1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag = "2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag = "3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        #[prost(message, optional, tag = "4")]
        pub status: ::core::option::Option<status_history::StatusCode>,
        /// The time that the episode was in the specified status
        #[prost(message, optional, tag = "5")]
        pub period: ::core::option::Option<super::super::core::Period>,
    }
    /// Nested message and enum types in `StatusHistory`.
    pub mod status_history {
        /// planned | arrived | triaged | in-progress | onleave | finished |
        /// cancelled +
        #[derive(Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Message)]
        pub struct StatusCode {
            #[prost(
                enumeration = "super::super::super::core::encounter_status_code::Value",
                tag = "1"
            )]
            pub value: i32,
            #[prost(message, optional, tag = "2")]
            pub id: ::core::option::Option<super::super::super::core::String>,
            #[prost(message, repeated, tag = "3")]
            pub extension: prost::alloc::vec::Vec<super::super::super::core::Extension>,
        }
    }
    /// List of past encounter classes
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct ClassHistory {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag = "1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag = "2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag = "3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// inpatient | outpatient | ambulatory | emergency +
        #[prost(message, optional, tag = "4")]
        pub class_value: ::core::option::Option<super::super::core::Coding>,
        /// The time that the episode was in the specified class
        #[prost(message, optional, tag = "5")]
        pub period: ::core::option::Option<super::super::core::Period>,
    }
    /// List of participants involved in the encounter
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct Participant {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag = "1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag = "2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag = "3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Role of participant in encounter
        #[prost(message, repeated, tag = "4")]
        pub r#type: prost::alloc::vec::Vec<super::super::core::CodeableConcept>,
        /// Period of time during the encounter that the participant participated
        #[prost(message, optional, tag = "5")]
        pub period: ::core::option::Option<super::super::core::Period>,
        /// Persons involved in the encounter other than the patient
        #[prost(message, optional, tag = "6")]
        pub individual: ::core::option::Option<super::super::core::Reference>,
    }
    /// The list of diagnosis relevant to this encounter
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct Diagnosis {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag = "1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag = "2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag = "3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// The diagnosis or procedure relevant to the encounter
        #[prost(message, optional, tag = "4")]
        pub condition: ::core::option::Option<super::super::core::Reference>,
        /// Role that this diagnosis has within the encounter (e.g. admission,
        /// billing, discharge …)
        #[prost(message, optional, tag = "5")]
        pub r#use: ::core::option::Option<super::super::core::CodeableConcept>,
        /// Ranking of the diagnosis (for each role type)
        #[prost(message, optional, tag = "6")]
        pub rank: ::core::option::Option<super::super::core::PositiveInt>,
    }
    /// Details about the admission to a healthcare service
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct Hospitalization {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag = "1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag = "2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag = "3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Pre-admission identifier
        #[prost(message, optional, tag = "4")]
        pub pre_admission_identifier: ::core::option::Option<super::super::core::Identifier>,
        /// The location/organization from which the patient came before admission
        #[prost(message, optional, tag = "5")]
        pub origin: ::core::option::Option<super::super::core::Reference>,
        /// From where patient was admitted (physician referral, transfer)
        #[prost(message, optional, tag = "6")]
        pub admit_source: ::core::option::Option<super::super::core::CodeableConcept>,
        /// The type of hospital re-admission that has occurred (if any). If the
        /// value is absent, then this is not identified as a readmission
        #[prost(message, optional, tag = "7")]
        pub re_admission: ::core::option::Option<super::super::core::CodeableConcept>,
        /// Diet preferences reported by the patient
        #[prost(message, repeated, tag = "8")]
        pub diet_preference: prost::alloc::vec::Vec<super::super::core::CodeableConcept>,
        /// Special courtesies (VIP, board member)
        #[prost(message, repeated, tag = "9")]
        pub special_courtesy: prost::alloc::vec::Vec<super::super::core::CodeableConcept>,
        /// Wheelchair, translator, stretcher, etc.
        #[prost(message, repeated, tag = "10")]
        pub special_arrangement: prost::alloc::vec::Vec<super::super::core::CodeableConcept>,
        /// Location/organization to which the patient is discharged
        #[prost(message, optional, tag = "11")]
        pub destination: ::core::option::Option<super::super::core::Reference>,
        /// Category or kind of location after discharge
        #[prost(message, optional, tag = "12")]
        pub discharge_disposition: ::core::option::Option<super::super::core::CodeableConcept>,
    }
    /// List of locations where the patient has been
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct Location {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag = "1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag = "2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag = "3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Location the encounter takes place
        #[prost(message, optional, tag = "4")]
        pub location: ::core::option::Option<super::super::core::Reference>,
        #[prost(message, optional, tag = "5")]
        pub status: ::core::option::Option<location::StatusCode>,
        /// The physical type of the location (usually the level in the location
        /// hierachy - bed room ward etc.)
        #[prost(message, optional, tag = "6")]
        pub physical_type: ::core::option::Option<super::super::core::CodeableConcept>,
        /// Time period during which the patient was present at the location
        #[prost(message, optional, tag = "7")]
        pub period: ::core::option::Option<super::super::core::Period>,
    }
    /// Nested message and enum types in `Location`.
    pub mod location {
        /// planned | active | reserved | completed
        #[derive(Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Message)]
        pub struct StatusCode {
            #[prost(
                enumeration = "super::super::super::core::encounter_location_status_code::Value",
                tag = "1"
            )]
            pub value: i32,
            #[prost(message, optional, tag = "2")]
            pub id: ::core::option::Option<super::super::super::core::String>,
            #[prost(message, repeated, tag = "3")]
            pub extension: prost::alloc::vec::Vec<super::super::super::core::Extension>,
        }
    }
}
/// Auto-generated from StructureDefinition for QICoreFamilyMemberHistory.
/// Information about patient's relatives, relevant for patient.
/// See
/// <http://hl7.org/fhir/us/qicore/StructureDefinition/qicore-familymemberhistory>
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct QiCoreFamilyMemberHistory {
    /// Logical id of this artifact
    #[prost(message, optional, tag = "1")]
    pub id: ::core::option::Option<super::core::Id>,
    /// Metadata about the resource
    #[prost(message, optional, tag = "2")]
    pub meta: ::core::option::Option<super::core::Meta>,
    /// A set of rules under which this content was created
    #[prost(message, optional, tag = "3")]
    pub implicit_rules: ::core::option::Option<super::core::Uri>,
    /// Language of the resource content
    #[prost(message, optional, tag = "4")]
    pub language: ::core::option::Option<super::core::Code>,
    /// Text summary of the resource, for human interpretation
    #[prost(message, optional, tag = "5")]
    pub text: ::core::option::Option<super::core::Narrative>,
    /// Contained, inline Resources
    #[prost(message, repeated, tag = "6")]
    pub contained: prost::alloc::vec::Vec<::prost_wkt_types::Any>,
    /// Additional content defined by implementations
    #[prost(message, repeated, tag = "8")]
    pub extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// Extensions that cannot be ignored
    #[prost(message, repeated, tag = "9")]
    pub modifier_extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// External Id(s) for this record
    #[prost(message, repeated, tag = "10")]
    pub identifier: prost::alloc::vec::Vec<super::core::Identifier>,
    /// Instantiates FHIR protocol or definition
    #[prost(message, repeated, tag = "11")]
    pub instantiates_canonical: prost::alloc::vec::Vec<super::core::Canonical>,
    /// Instantiates external protocol or definition
    #[prost(message, repeated, tag = "12")]
    pub instantiates_uri: prost::alloc::vec::Vec<super::core::Uri>,
    #[prost(message, optional, tag = "13")]
    pub status: ::core::option::Option<qi_core_family_member_history::StatusCode>,
    /// subject-unknown | withheld | unable-to-obtain | deferred
    #[prost(message, optional, tag = "14")]
    pub data_absent_reason: ::core::option::Option<super::core::CodeableConcept>,
    /// Patient history is about
    #[prost(message, optional, tag = "15")]
    pub patient: ::core::option::Option<super::core::Reference>,
    /// When history was recorded or last updated
    #[prost(message, optional, tag = "16")]
    pub date: ::core::option::Option<super::core::DateTime>,
    /// The family member described
    #[prost(message, optional, tag = "17")]
    pub name: ::core::option::Option<super::core::String>,
    /// Relationship to the subject
    #[prost(message, optional, tag = "18")]
    pub relationship: ::core::option::Option<super::core::CodeableConcept>,
    /// male | female | other | unknown
    #[prost(message, optional, tag = "19")]
    pub sex: ::core::option::Option<super::core::CodeableConcept>,
    #[prost(message, optional, tag = "20")]
    pub born: ::core::option::Option<qi_core_family_member_history::BornX>,
    #[prost(message, optional, tag = "21")]
    pub age: ::core::option::Option<qi_core_family_member_history::AgeX>,
    /// Age is estimated?
    #[prost(message, optional, tag = "22")]
    pub estimated_age: ::core::option::Option<super::core::Boolean>,
    #[prost(message, optional, tag = "23")]
    pub deceased: ::core::option::Option<qi_core_family_member_history::DeceasedX>,
    /// Why was family member history performed?
    #[prost(message, repeated, tag = "24")]
    pub reason_code: prost::alloc::vec::Vec<super::core::CodeableConcept>,
    /// Why was family member history performed?
    #[prost(message, repeated, tag = "25")]
    pub reason_reference: prost::alloc::vec::Vec<super::core::Reference>,
    /// General note about related person
    #[prost(message, repeated, tag = "26")]
    pub note: prost::alloc::vec::Vec<super::core::Annotation>,
    #[prost(message, repeated, tag = "27")]
    pub condition: prost::alloc::vec::Vec<qi_core_family_member_history::Condition>,
}
/// Nested message and enum types in `QICoreFamilyMemberHistory`.
pub mod qi_core_family_member_history {
    /// partial | completed | entered-in-error | health-unknown
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct StatusCode {
        #[prost(
            enumeration = "super::super::core::family_history_status_code::Value",
            tag = "1"
        )]
        pub value: i32,
        #[prost(message, optional, tag = "2")]
        pub id: ::core::option::Option<super::super::core::String>,
        #[prost(message, repeated, tag = "3")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
    }
    /// (approximate) date of birth
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct BornX {
        #[prost(oneof = "born_x::Choice", tags = "1, 2, 3")]
        pub choice: ::core::option::Option<born_x::Choice>,
    }
    /// Nested message and enum types in `BornX`.
    pub mod born_x {
        #[derive(Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Oneof)]
        pub enum Choice {
            #[prost(message, tag = "1")]
            Period(super::super::super::core::Period),
            #[prost(message, tag = "2")]
            Date(super::super::super::core::Date),
            #[prost(message, tag = "3")]
            StringValue(super::super::super::core::String),
        }
    }
    /// (approximate) age
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct AgeX {
        #[prost(oneof = "age_x::Choice", tags = "1, 2, 3")]
        pub choice: ::core::option::Option<age_x::Choice>,
    }
    /// Nested message and enum types in `AgeX`.
    pub mod age_x {
        #[derive(Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Oneof)]
        pub enum Choice {
            #[prost(message, tag = "1")]
            Age(super::super::super::core::Age),
            #[prost(message, tag = "2")]
            Range(super::super::super::core::Range),
            #[prost(message, tag = "3")]
            StringValue(super::super::super::core::String),
        }
    }
    /// Dead? How old/when?
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct DeceasedX {
        #[prost(oneof = "deceased_x::Choice", tags = "1, 2, 3, 4, 5")]
        pub choice: ::core::option::Option<deceased_x::Choice>,
    }
    /// Nested message and enum types in `DeceasedX`.
    pub mod deceased_x {
        #[derive(Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Oneof)]
        pub enum Choice {
            #[prost(message, tag = "1")]
            Boolean(super::super::super::core::Boolean),
            #[prost(message, tag = "2")]
            Age(super::super::super::core::Age),
            #[prost(message, tag = "3")]
            Range(super::super::super::core::Range),
            #[prost(message, tag = "4")]
            Date(super::super::super::core::Date),
            #[prost(message, tag = "5")]
            StringValue(super::super::super::core::String),
        }
    }
    /// Condition that the related person had
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct Condition {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag = "1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Extension
        #[prost(message, repeated, tag = "2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag = "3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Condition suffered by relation
        #[prost(message, optional, tag = "4")]
        pub code: ::core::option::Option<super::super::core::CodeableConcept>,
        /// deceased | permanent disability | etc.
        #[prost(message, optional, tag = "5")]
        pub outcome: ::core::option::Option<super::super::core::CodeableConcept>,
        /// Whether the condition contributed to the cause of death
        #[prost(message, optional, tag = "6")]
        pub contributed_to_death: ::core::option::Option<super::super::core::Boolean>,
        #[prost(message, optional, tag = "7")]
        pub onset: ::core::option::Option<condition::OnsetX>,
        /// Extra information about condition
        #[prost(message, repeated, tag = "8")]
        pub note: prost::alloc::vec::Vec<super::super::core::Annotation>,
        /// When (or if) the family member's condition resolved
        #[prost(message, optional, tag = "9")]
        pub condition_abatement:
            ::core::option::Option<super::super::core::family_member_history_abatement::ValueX>,
        /// The seriousness of the family member condition
        #[prost(message, optional, tag = "10")]
        pub condition_severity: ::core::option::Option<super::super::core::CodeableConcept>,
    }
    /// Nested message and enum types in `Condition`.
    pub mod condition {
        /// When condition first manifested
        #[derive(Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Message)]
        pub struct OnsetX {
            #[prost(oneof = "onset_x::Choice", tags = "1, 2, 3, 4")]
            pub choice: ::core::option::Option<onset_x::Choice>,
        }
        /// Nested message and enum types in `OnsetX`.
        pub mod onset_x {
            #[derive(Serialize, Deserialize)]
            #[serde(rename_all = "camelCase")]
            #[derive(Clone, PartialEq, prost::Oneof)]
            pub enum Choice {
                #[prost(message, tag = "1")]
                Age(super::super::super::super::core::Age),
                #[prost(message, tag = "2")]
                Range(super::super::super::super::core::Range),
                #[prost(message, tag = "3")]
                Period(super::super::super::super::core::Period),
                #[prost(message, tag = "4")]
                StringValue(super::super::super::super::core::String),
            }
        }
    }
}
/// Auto-generated from StructureDefinition for QICoreFlag.
/// Key information to flag to healthcare providers.
/// See <http://hl7.org/fhir/us/qicore/StructureDefinition/qicore-flag>
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct QiCoreFlag {
    /// Logical id of this artifact
    #[prost(message, optional, tag = "1")]
    pub id: ::core::option::Option<super::core::Id>,
    /// Metadata about the resource
    #[prost(message, optional, tag = "2")]
    pub meta: ::core::option::Option<super::core::Meta>,
    /// A set of rules under which this content was created
    #[prost(message, optional, tag = "3")]
    pub implicit_rules: ::core::option::Option<super::core::Uri>,
    /// Language of the resource content
    #[prost(message, optional, tag = "4")]
    pub language: ::core::option::Option<super::core::Code>,
    /// Text summary of the resource, for human interpretation
    #[prost(message, optional, tag = "5")]
    pub text: ::core::option::Option<super::core::Narrative>,
    /// Contained, inline Resources
    #[prost(message, repeated, tag = "6")]
    pub contained: prost::alloc::vec::Vec<::prost_wkt_types::Any>,
    /// Additional content defined by implementations
    #[prost(message, repeated, tag = "8")]
    pub extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// Extensions that cannot be ignored
    #[prost(message, repeated, tag = "9")]
    pub modifier_extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// Business identifier
    #[prost(message, repeated, tag = "10")]
    pub identifier: prost::alloc::vec::Vec<super::core::Identifier>,
    #[prost(message, optional, tag = "11")]
    pub status: ::core::option::Option<qi_core_flag::StatusCode>,
    /// Clinical, administrative, etc.
    #[prost(message, repeated, tag = "12")]
    pub category: prost::alloc::vec::Vec<super::core::CodeableConcept>,
    /// Coded or textual message to display to user
    #[prost(message, optional, tag = "13")]
    pub code: ::core::option::Option<super::core::CodeableConcept>,
    /// Who/What is flag about?
    #[prost(message, optional, tag = "14")]
    pub subject: ::core::option::Option<super::core::Reference>,
    /// Time period when flag is active
    #[prost(message, optional, tag = "15")]
    pub period: ::core::option::Option<super::core::Period>,
    /// Alert relevant during encounter
    #[prost(message, optional, tag = "16")]
    pub encounter: ::core::option::Option<super::core::Reference>,
    /// Flag creator
    #[prost(message, optional, tag = "17")]
    pub author: ::core::option::Option<super::core::Reference>,
}
/// Nested message and enum types in `QICoreFlag`.
pub mod qi_core_flag {
    /// active | inactive | entered-in-error
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct StatusCode {
        #[prost(enumeration = "super::super::core::flag_status_code::Value", tag = "1")]
        pub value: i32,
        #[prost(message, optional, tag = "2")]
        pub id: ::core::option::Option<super::super::core::String>,
        #[prost(message, repeated, tag = "3")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
    }
}
/// Auto-generated from StructureDefinition for QICoreGoal.
/// Describes the intended objective(s) for a patient, group or organization.
/// See <http://hl7.org/fhir/us/qicore/StructureDefinition/qicore-goal>
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct QiCoreGoal {
    /// Logical id of this artifact
    #[prost(message, optional, tag = "1")]
    pub id: ::core::option::Option<super::core::Id>,
    /// Metadata about the resource
    #[prost(message, optional, tag = "2")]
    pub meta: ::core::option::Option<super::core::Meta>,
    /// A set of rules under which this content was created
    #[prost(message, optional, tag = "3")]
    pub implicit_rules: ::core::option::Option<super::core::Uri>,
    /// Language of the resource content
    #[prost(message, optional, tag = "4")]
    pub language: ::core::option::Option<super::core::Code>,
    /// Text summary of the resource, for human interpretation
    #[prost(message, optional, tag = "5")]
    pub text: ::core::option::Option<super::core::Narrative>,
    /// Contained, inline Resources
    #[prost(message, repeated, tag = "6")]
    pub contained: prost::alloc::vec::Vec<::prost_wkt_types::Any>,
    /// Extension
    #[prost(message, repeated, tag = "8")]
    pub extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// Extensions that cannot be ignored
    #[prost(message, repeated, tag = "9")]
    pub modifier_extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// External Ids for this goal
    #[prost(message, repeated, tag = "10")]
    pub identifier: prost::alloc::vec::Vec<super::core::Identifier>,
    #[prost(message, optional, tag = "11")]
    pub lifecycle_status: ::core::option::Option<qi_core_goal::LifecycleStatusCode>,
    /// in-progress | improving | worsening | no-change | achieved | sustaining |
    /// not-achieved | no-progress | not-attainable
    #[prost(message, optional, tag = "12")]
    pub achievement_status: ::core::option::Option<super::core::CodeableConcept>,
    /// E.g. Treatment, dietary, behavioral, etc.
    #[prost(message, repeated, tag = "13")]
    pub category: prost::alloc::vec::Vec<super::core::CodeableConcept>,
    /// high-priority | medium-priority | low-priority
    #[prost(message, optional, tag = "14")]
    pub priority: ::core::option::Option<super::core::CodeableConcept>,
    /// Code or text describing goal
    #[prost(message, optional, tag = "15")]
    pub description: ::core::option::Option<super::core::CodeableConcept>,
    /// Who this goal is intended for
    #[prost(message, optional, tag = "16")]
    pub subject: ::core::option::Option<super::core::Reference>,
    #[prost(message, optional, tag = "17")]
    pub start: ::core::option::Option<qi_core_goal::StartX>,
    #[prost(message, repeated, tag = "18")]
    pub target: prost::alloc::vec::Vec<qi_core_goal::Target>,
    /// When goal status took effect
    #[prost(message, optional, tag = "19")]
    pub status_date: ::core::option::Option<super::core::Date>,
    /// Reason for current status
    #[prost(message, optional, tag = "20")]
    pub status_reason: ::core::option::Option<super::core::String>,
    /// Who's responsible for creating Goal?
    #[prost(message, optional, tag = "21")]
    pub expressed_by: ::core::option::Option<super::core::Reference>,
    /// Issues addressed by this goal
    #[prost(message, repeated, tag = "22")]
    pub addresses: prost::alloc::vec::Vec<super::core::Reference>,
    /// Comments about the goal
    #[prost(message, repeated, tag = "23")]
    pub note: prost::alloc::vec::Vec<super::core::Annotation>,
    /// What result was achieved regarding the goal?
    #[prost(message, repeated, tag = "24")]
    pub outcome_code: prost::alloc::vec::Vec<super::core::CodeableConcept>,
    /// Observation that resulted from goal
    #[prost(message, repeated, tag = "25")]
    pub outcome_reference: prost::alloc::vec::Vec<super::core::Reference>,
    /// The reason the goal was not accepted
    #[prost(message, optional, tag = "26")]
    pub reason_rejected: ::core::option::Option<super::core::CodeableConcept>,
}
/// Nested message and enum types in `QICoreGoal`.
pub mod qi_core_goal {
    /// proposed | planned | accepted | active | on-hold | completed | cancelled |
    /// entered-in-error | rejected
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct LifecycleStatusCode {
        #[prost(
            enumeration = "super::super::core::goal_lifecycle_status_code::Value",
            tag = "1"
        )]
        pub value: i32,
        #[prost(message, optional, tag = "2")]
        pub id: ::core::option::Option<super::super::core::String>,
        #[prost(message, repeated, tag = "3")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
    }
    /// When goal pursuit begins
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct StartX {
        #[prost(oneof = "start_x::Choice", tags = "1")]
        pub choice: ::core::option::Option<start_x::Choice>,
    }
    /// Nested message and enum types in `StartX`.
    pub mod start_x {
        #[derive(Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Oneof)]
        pub enum Choice {
            #[prost(message, tag = "1")]
            Date(super::super::super::core::Date),
        }
    }
    /// Target outcome for the goal
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct Target {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag = "1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag = "2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag = "3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// The parameter whose value is being tracked
        #[prost(message, optional, tag = "4")]
        pub measure: ::core::option::Option<super::super::core::CodeableConcept>,
        #[prost(message, optional, tag = "5")]
        pub detail: ::core::option::Option<target::DetailX>,
        #[prost(message, optional, tag = "6")]
        pub due: ::core::option::Option<target::DueX>,
    }
    /// Nested message and enum types in `Target`.
    pub mod target {
        /// The target value to be achieved
        #[derive(Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Message)]
        pub struct DetailX {
            #[prost(oneof = "detail_x::Choice", tags = "1, 2, 3, 4, 5, 6, 7")]
            pub choice: ::core::option::Option<detail_x::Choice>,
        }
        /// Nested message and enum types in `DetailX`.
        pub mod detail_x {
            #[derive(Serialize, Deserialize)]
            #[serde(rename_all = "camelCase")]
            #[derive(Clone, PartialEq, prost::Oneof)]
            pub enum Choice {
                #[prost(message, tag = "1")]
                Quantity(super::super::super::super::core::Quantity),
                #[prost(message, tag = "2")]
                Range(super::super::super::super::core::Range),
                #[prost(message, tag = "3")]
                CodeableConcept(super::super::super::super::core::CodeableConcept),
                #[prost(message, tag = "4")]
                StringValue(super::super::super::super::core::String),
                #[prost(message, tag = "5")]
                Boolean(super::super::super::super::core::Boolean),
                #[prost(message, tag = "6")]
                Integer(super::super::super::super::core::Integer),
                #[prost(message, tag = "7")]
                Ratio(super::super::super::super::core::Ratio),
            }
        }
        /// Reach goal on or before
        #[derive(Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Message)]
        pub struct DueX {
            #[prost(oneof = "due_x::Choice", tags = "1, 2")]
            pub choice: ::core::option::Option<due_x::Choice>,
        }
        /// Nested message and enum types in `DueX`.
        pub mod due_x {
            #[derive(Serialize, Deserialize)]
            #[serde(rename_all = "camelCase")]
            #[derive(Clone, PartialEq, prost::Oneof)]
            pub enum Choice {
                #[prost(message, tag = "1")]
                Date(super::super::super::super::core::Date),
                #[prost(message, tag = "2")]
                Duration(super::super::super::super::core::Duration),
            }
        }
    }
}
/// Auto-generated from StructureDefinition for QICoreImagingStudy.
/// A set of images produced in single study (one or more series of references
/// images). See
/// <http://hl7.org/fhir/us/qicore/StructureDefinition/qicore-imagingstudy>
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct QiCoreImagingStudy {
    /// Logical id of this artifact
    #[prost(message, optional, tag = "1")]
    pub id: ::core::option::Option<super::core::Id>,
    /// Metadata about the resource
    #[prost(message, optional, tag = "2")]
    pub meta: ::core::option::Option<super::core::Meta>,
    /// A set of rules under which this content was created
    #[prost(message, optional, tag = "3")]
    pub implicit_rules: ::core::option::Option<super::core::Uri>,
    /// Language of the resource content
    #[prost(message, optional, tag = "4")]
    pub language: ::core::option::Option<super::core::Code>,
    /// Text summary of the resource, for human interpretation
    #[prost(message, optional, tag = "5")]
    pub text: ::core::option::Option<super::core::Narrative>,
    /// Contained, inline Resources
    #[prost(message, repeated, tag = "6")]
    pub contained: prost::alloc::vec::Vec<::prost_wkt_types::Any>,
    /// Additional content defined by implementations
    #[prost(message, repeated, tag = "8")]
    pub extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// Extensions that cannot be ignored
    #[prost(message, repeated, tag = "9")]
    pub modifier_extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// Identifiers for the whole study
    #[prost(message, repeated, tag = "10")]
    pub identifier: prost::alloc::vec::Vec<super::core::Identifier>,
    #[prost(message, optional, tag = "11")]
    pub status: ::core::option::Option<qi_core_imaging_study::StatusCode>,
    /// All series modality if actual acquisition modalities
    #[prost(message, repeated, tag = "12")]
    pub modality: prost::alloc::vec::Vec<super::core::Coding>,
    /// Who or what is the subject of the study
    #[prost(message, optional, tag = "13")]
    pub subject: ::core::option::Option<super::core::Reference>,
    /// Encounter with which this imaging study is associated
    #[prost(message, optional, tag = "14")]
    pub encounter: ::core::option::Option<super::core::Reference>,
    /// When the study was started
    #[prost(message, optional, tag = "15")]
    pub started: ::core::option::Option<super::core::DateTime>,
    /// Request fulfilled
    #[prost(message, repeated, tag = "16")]
    pub based_on: prost::alloc::vec::Vec<super::core::Reference>,
    /// Referring physician
    #[prost(message, optional, tag = "17")]
    pub referrer: ::core::option::Option<super::core::Reference>,
    /// Who interpreted images
    #[prost(message, repeated, tag = "18")]
    pub interpreter: prost::alloc::vec::Vec<super::core::Reference>,
    /// Study access endpoint
    #[prost(message, repeated, tag = "19")]
    pub endpoint: prost::alloc::vec::Vec<super::core::Reference>,
    /// Number of Study Related Series
    #[prost(message, optional, tag = "20")]
    pub number_of_series: ::core::option::Option<super::core::UnsignedInt>,
    /// Number of Study Related Instances
    #[prost(message, optional, tag = "21")]
    pub number_of_instances: ::core::option::Option<super::core::UnsignedInt>,
    /// The performed Procedure reference
    #[prost(message, optional, tag = "22")]
    pub procedure_reference: ::core::option::Option<super::core::Reference>,
    /// The performed procedure code
    #[prost(message, repeated, tag = "23")]
    pub procedure_code: prost::alloc::vec::Vec<super::core::CodeableConcept>,
    /// Where ImagingStudy occurred
    #[prost(message, optional, tag = "24")]
    pub location: ::core::option::Option<super::core::Reference>,
    /// Why the study was requested
    #[prost(message, repeated, tag = "25")]
    pub reason_code: prost::alloc::vec::Vec<super::core::CodeableConcept>,
    /// Why was study performed
    #[prost(message, repeated, tag = "26")]
    pub reason_reference: prost::alloc::vec::Vec<super::core::Reference>,
    /// User-defined comments
    #[prost(message, repeated, tag = "27")]
    pub note: prost::alloc::vec::Vec<super::core::Annotation>,
    /// Institution-generated description
    #[prost(message, optional, tag = "28")]
    pub description: ::core::option::Option<super::core::String>,
    #[prost(message, repeated, tag = "29")]
    pub series: prost::alloc::vec::Vec<qi_core_imaging_study::Series>,
}
/// Nested message and enum types in `QICoreImagingStudy`.
pub mod qi_core_imaging_study {
    /// registered | available | cancelled | entered-in-error | unknown
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct StatusCode {
        #[prost(
            enumeration = "super::super::core::imaging_study_status_code::Value",
            tag = "1"
        )]
        pub value: i32,
        #[prost(message, optional, tag = "2")]
        pub id: ::core::option::Option<super::super::core::String>,
        #[prost(message, repeated, tag = "3")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
    }
    /// Each study has one or more series of instances
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct Series {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag = "1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag = "2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag = "3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// DICOM Series Instance UID for the series
        #[prost(message, optional, tag = "4")]
        pub uid: ::core::option::Option<super::super::core::Id>,
        /// Numeric identifier of this series
        #[prost(message, optional, tag = "5")]
        pub number: ::core::option::Option<super::super::core::UnsignedInt>,
        /// The modality of the instances in the series
        #[prost(message, optional, tag = "6")]
        pub modality: ::core::option::Option<super::super::core::Coding>,
        /// A short human readable summary of the series
        #[prost(message, optional, tag = "7")]
        pub description: ::core::option::Option<super::super::core::String>,
        /// Number of Series Related Instances
        #[prost(message, optional, tag = "8")]
        pub number_of_instances: ::core::option::Option<super::super::core::UnsignedInt>,
        /// Series access endpoint
        #[prost(message, repeated, tag = "9")]
        pub endpoint: prost::alloc::vec::Vec<super::super::core::Reference>,
        /// Body part examined
        #[prost(message, optional, tag = "10")]
        pub body_site: ::core::option::Option<super::super::core::Coding>,
        /// Body part laterality
        #[prost(message, optional, tag = "11")]
        pub laterality: ::core::option::Option<super::super::core::Coding>,
        /// Specimen imaged
        #[prost(message, repeated, tag = "12")]
        pub specimen: prost::alloc::vec::Vec<super::super::core::Reference>,
        /// When the series started
        #[prost(message, optional, tag = "13")]
        pub started: ::core::option::Option<super::super::core::DateTime>,
        #[prost(message, repeated, tag = "14")]
        pub performer: prost::alloc::vec::Vec<series::Performer>,
        #[prost(message, repeated, tag = "15")]
        pub instance: prost::alloc::vec::Vec<series::Instance>,
    }
    /// Nested message and enum types in `Series`.
    pub mod series {
        /// Who performed the series
        #[derive(Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Message)]
        pub struct Performer {
            /// Unique id for inter-element referencing
            #[prost(message, optional, tag = "1")]
            pub id: ::core::option::Option<super::super::super::core::String>,
            /// Additional content defined by implementations
            #[prost(message, repeated, tag = "2")]
            pub extension: prost::alloc::vec::Vec<super::super::super::core::Extension>,
            /// Extensions that cannot be ignored even if unrecognized
            #[prost(message, repeated, tag = "3")]
            pub modifier_extension: prost::alloc::vec::Vec<super::super::super::core::Extension>,
            /// Type of performance
            #[prost(message, optional, tag = "4")]
            pub function: ::core::option::Option<super::super::super::core::CodeableConcept>,
            /// Who performed the series
            #[prost(message, optional, tag = "5")]
            pub actor: ::core::option::Option<super::super::super::core::Reference>,
        }
        /// A single SOP instance from the series
        #[derive(Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Message)]
        pub struct Instance {
            /// Unique id for inter-element referencing
            #[prost(message, optional, tag = "1")]
            pub id: ::core::option::Option<super::super::super::core::String>,
            /// Additional content defined by implementations
            #[prost(message, repeated, tag = "2")]
            pub extension: prost::alloc::vec::Vec<super::super::super::core::Extension>,
            /// Extensions that cannot be ignored even if unrecognized
            #[prost(message, repeated, tag = "3")]
            pub modifier_extension: prost::alloc::vec::Vec<super::super::super::core::Extension>,
            /// DICOM SOP Instance UID
            #[prost(message, optional, tag = "4")]
            pub uid: ::core::option::Option<super::super::super::core::Id>,
            /// DICOM class type
            #[prost(message, optional, tag = "5")]
            pub sop_class: ::core::option::Option<super::super::super::core::Coding>,
            /// The number of this instance in the series
            #[prost(message, optional, tag = "6")]
            pub number: ::core::option::Option<super::super::super::core::UnsignedInt>,
            /// Description of instance
            #[prost(message, optional, tag = "7")]
            pub title: ::core::option::Option<super::super::super::core::String>,
        }
    }
}
/// Auto-generated from StructureDefinition for QICoreImmunization.
/// Immunization event information.
/// See <http://hl7.org/fhir/us/qicore/StructureDefinition/qicore-immunization>
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct QiCoreImmunization {
    /// Logical id of this artifact
    #[prost(message, optional, tag = "1")]
    pub id: ::core::option::Option<super::core::Id>,
    /// Metadata about the resource
    #[prost(message, optional, tag = "2")]
    pub meta: ::core::option::Option<super::core::Meta>,
    /// A set of rules under which this content was created
    #[prost(message, optional, tag = "3")]
    pub implicit_rules: ::core::option::Option<super::core::Uri>,
    /// Language of the resource content
    #[prost(message, optional, tag = "4")]
    pub language: ::core::option::Option<super::core::Code>,
    /// Text summary of the resource, for human interpretation
    #[prost(message, optional, tag = "5")]
    pub text: ::core::option::Option<super::core::Narrative>,
    /// Contained, inline Resources
    #[prost(message, repeated, tag = "6")]
    pub contained: prost::alloc::vec::Vec<::prost_wkt_types::Any>,
    /// Additional content defined by implementations
    #[prost(message, repeated, tag = "8")]
    pub extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// Extensions that cannot be ignored
    #[prost(message, repeated, tag = "9")]
    pub modifier_extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// Business identifier
    #[prost(message, repeated, tag = "10")]
    pub identifier: prost::alloc::vec::Vec<super::core::Identifier>,
    #[prost(message, optional, tag = "11")]
    pub status: ::core::option::Option<qi_core_immunization::StatusCode>,
    /// Reason not done
    #[prost(message, optional, tag = "12")]
    pub status_reason: ::core::option::Option<super::core::CodeableConcept>,
    /// Vaccine Product Type (bind to CVX)
    #[prost(message, optional, tag = "13")]
    pub vaccine_code: ::core::option::Option<super::core::CodeableConcept>,
    /// Who was immunized
    #[prost(message, optional, tag = "14")]
    pub patient: ::core::option::Option<super::core::Reference>,
    /// Encounter immunization was part of
    #[prost(message, optional, tag = "15")]
    pub encounter: ::core::option::Option<super::core::Reference>,
    #[prost(message, optional, tag = "16")]
    pub occurrence: ::core::option::Option<qi_core_immunization::OccurrenceX>,
    /// When the immunization was first captured in the subject's record
    #[prost(message, optional, tag = "17")]
    pub recorded: ::core::option::Option<super::core::DateTime>,
    /// Indicates context the data was recorded in
    #[prost(message, optional, tag = "18")]
    pub primary_source: ::core::option::Option<super::core::Boolean>,
    /// Indicates the source of a secondarily reported record
    #[prost(message, optional, tag = "19")]
    pub report_origin: ::core::option::Option<super::core::CodeableConcept>,
    /// Where immunization occurred
    #[prost(message, optional, tag = "20")]
    pub location: ::core::option::Option<super::core::Reference>,
    /// Vaccine manufacturer
    #[prost(message, optional, tag = "21")]
    pub manufacturer: ::core::option::Option<super::core::Reference>,
    /// Vaccine lot number
    #[prost(message, optional, tag = "22")]
    pub lot_number: ::core::option::Option<super::core::String>,
    /// Vaccine expiration date
    #[prost(message, optional, tag = "23")]
    pub expiration_date: ::core::option::Option<super::core::Date>,
    /// Body site vaccine  was administered
    #[prost(message, optional, tag = "24")]
    pub site: ::core::option::Option<super::core::CodeableConcept>,
    /// How vaccine entered body
    #[prost(message, optional, tag = "25")]
    pub route: ::core::option::Option<super::core::CodeableConcept>,
    /// Amount of vaccine administered
    #[prost(message, optional, tag = "26")]
    pub dose_quantity: ::core::option::Option<super::core::SimpleQuantity>,
    #[prost(message, repeated, tag = "27")]
    pub performer: prost::alloc::vec::Vec<qi_core_immunization::Performer>,
    /// Additional immunization notes
    #[prost(message, repeated, tag = "28")]
    pub note: prost::alloc::vec::Vec<super::core::Annotation>,
    /// Why immunization occurred
    #[prost(message, repeated, tag = "29")]
    pub reason_code: prost::alloc::vec::Vec<super::core::CodeableConcept>,
    /// Why immunization occurred
    #[prost(message, repeated, tag = "30")]
    pub reason_reference: prost::alloc::vec::Vec<super::core::Reference>,
    /// Dose potency
    #[prost(message, optional, tag = "31")]
    pub is_subpotent: ::core::option::Option<super::core::Boolean>,
    /// Reason for being subpotent
    #[prost(message, repeated, tag = "32")]
    pub subpotent_reason: prost::alloc::vec::Vec<super::core::CodeableConcept>,
    #[prost(message, repeated, tag = "33")]
    pub education: prost::alloc::vec::Vec<qi_core_immunization::Education>,
    /// Patient eligibility for a vaccination program
    #[prost(message, repeated, tag = "34")]
    pub program_eligibility: prost::alloc::vec::Vec<super::core::CodeableConcept>,
    /// Funding source for the vaccine
    #[prost(message, optional, tag = "35")]
    pub funding_source: ::core::option::Option<super::core::CodeableConcept>,
    #[prost(message, repeated, tag = "36")]
    pub reaction: prost::alloc::vec::Vec<qi_core_immunization::Reaction>,
    #[prost(message, repeated, tag = "37")]
    pub protocol_applied: prost::alloc::vec::Vec<qi_core_immunization::ProtocolApplied>,
}
/// Nested message and enum types in `QICoreImmunization`.
pub mod qi_core_immunization {
    /// completed | entered-in-error | not-done
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct StatusCode {
        #[prost(
            enumeration = "super::super::core::immunization_status_codes_value_set::Value",
            tag = "1"
        )]
        pub value: i32,
        #[prost(message, optional, tag = "2")]
        pub id: ::core::option::Option<super::super::core::String>,
        #[prost(message, repeated, tag = "3")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
    }
    /// Vaccine administration date
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct OccurrenceX {
        #[prost(oneof = "occurrence_x::Choice", tags = "1, 2")]
        pub choice: ::core::option::Option<occurrence_x::Choice>,
    }
    /// Nested message and enum types in `OccurrenceX`.
    pub mod occurrence_x {
        #[derive(Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Oneof)]
        pub enum Choice {
            #[prost(message, tag = "1")]
            DateTime(super::super::super::core::DateTime),
            #[prost(message, tag = "2")]
            StringValue(super::super::super::core::String),
        }
    }
    /// Who performed event
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct Performer {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag = "1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag = "2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag = "3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// What type of performance was done
        #[prost(message, optional, tag = "4")]
        pub function: ::core::option::Option<super::super::core::CodeableConcept>,
        /// Individual or organization who was performing
        #[prost(message, optional, tag = "5")]
        pub actor: ::core::option::Option<super::super::core::Reference>,
    }
    /// Educational material presented to patient
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct Education {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag = "1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag = "2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag = "3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Educational material document identifier
        #[prost(message, optional, tag = "4")]
        pub document_type: ::core::option::Option<super::super::core::String>,
        /// Educational material reference pointer
        #[prost(message, optional, tag = "5")]
        pub reference: ::core::option::Option<super::super::core::Uri>,
        /// Educational material publication date
        #[prost(message, optional, tag = "6")]
        pub publication_date: ::core::option::Option<super::super::core::DateTime>,
        /// Educational material presentation date
        #[prost(message, optional, tag = "7")]
        pub presentation_date: ::core::option::Option<super::super::core::DateTime>,
    }
    /// Details of a reaction that follows immunization
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct Reaction {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag = "1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag = "2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag = "3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// When reaction started
        #[prost(message, optional, tag = "4")]
        pub date: ::core::option::Option<super::super::core::DateTime>,
        /// Additional information on reaction
        #[prost(message, optional, tag = "5")]
        pub detail: ::core::option::Option<super::super::core::Reference>,
        /// Indicates self-reported reaction
        #[prost(message, optional, tag = "6")]
        pub reported: ::core::option::Option<super::super::core::Boolean>,
    }
    /// Protocol followed by the provider
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct ProtocolApplied {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag = "1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag = "2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag = "3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Name of vaccine series
        #[prost(message, optional, tag = "4")]
        pub series: ::core::option::Option<super::super::core::String>,
        /// Who is responsible for publishing the recommendations
        #[prost(message, optional, tag = "5")]
        pub authority: ::core::option::Option<super::super::core::Reference>,
        /// Vaccine preventatable disease being targetted
        #[prost(message, repeated, tag = "6")]
        pub target_disease: prost::alloc::vec::Vec<super::super::core::CodeableConcept>,
        #[prost(message, optional, tag = "7")]
        pub dose_number: ::core::option::Option<protocol_applied::DoseNumberX>,
        #[prost(message, optional, tag = "8")]
        pub series_doses: ::core::option::Option<protocol_applied::SeriesDosesX>,
    }
    /// Nested message and enum types in `ProtocolApplied`.
    pub mod protocol_applied {
        /// Dose number within series
        #[derive(Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Message)]
        pub struct DoseNumberX {
            #[prost(oneof = "dose_number_x::Choice", tags = "1, 2")]
            pub choice: ::core::option::Option<dose_number_x::Choice>,
        }
        /// Nested message and enum types in `DoseNumberX`.
        pub mod dose_number_x {
            #[derive(Serialize, Deserialize)]
            #[serde(rename_all = "camelCase")]
            #[derive(Clone, PartialEq, prost::Oneof)]
            pub enum Choice {
                #[prost(message, tag = "1")]
                PositiveInt(super::super::super::super::core::PositiveInt),
                #[prost(message, tag = "2")]
                StringValue(super::super::super::super::core::String),
            }
        }
        /// Recommended number of doses for immunity
        #[derive(Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Message)]
        pub struct SeriesDosesX {
            #[prost(oneof = "series_doses_x::Choice", tags = "1, 2")]
            pub choice: ::core::option::Option<series_doses_x::Choice>,
        }
        /// Nested message and enum types in `SeriesDosesX`.
        pub mod series_doses_x {
            #[derive(Serialize, Deserialize)]
            #[serde(rename_all = "camelCase")]
            #[derive(Clone, PartialEq, prost::Oneof)]
            pub enum Choice {
                #[prost(message, tag = "1")]
                PositiveInt(super::super::super::super::core::PositiveInt),
                #[prost(message, tag = "2")]
                StringValue(super::super::super::super::core::String),
            }
        }
    }
}
/// Auto-generated from StructureDefinition for QICoreImmunizationEvaluation.
/// Immunization evaluation information.
/// See
/// <http://hl7.org/fhir/us/qicore/StructureDefinition/qicore-immunizationevaluation>
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct QiCoreImmunizationEvaluation {
    /// Logical id of this artifact
    #[prost(message, optional, tag = "1")]
    pub id: ::core::option::Option<super::core::Id>,
    /// Metadata about the resource
    #[prost(message, optional, tag = "2")]
    pub meta: ::core::option::Option<super::core::Meta>,
    /// A set of rules under which this content was created
    #[prost(message, optional, tag = "3")]
    pub implicit_rules: ::core::option::Option<super::core::Uri>,
    /// Language of the resource content
    #[prost(message, optional, tag = "4")]
    pub language: ::core::option::Option<super::core::Code>,
    /// Text summary of the resource, for human interpretation
    #[prost(message, optional, tag = "5")]
    pub text: ::core::option::Option<super::core::Narrative>,
    /// Contained, inline Resources
    #[prost(message, repeated, tag = "6")]
    pub contained: prost::alloc::vec::Vec<::prost_wkt_types::Any>,
    /// Additional content defined by implementations
    #[prost(message, repeated, tag = "8")]
    pub extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// Extensions that cannot be ignored
    #[prost(message, repeated, tag = "9")]
    pub modifier_extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// Business identifier
    #[prost(message, repeated, tag = "10")]
    pub identifier: prost::alloc::vec::Vec<super::core::Identifier>,
    #[prost(message, optional, tag = "11")]
    pub status: ::core::option::Option<qi_core_immunization_evaluation::StatusCode>,
    /// Who this evaluation is for
    #[prost(message, optional, tag = "12")]
    pub patient: ::core::option::Option<super::core::Reference>,
    /// Date evaluation was performed
    #[prost(message, optional, tag = "13")]
    pub date: ::core::option::Option<super::core::DateTime>,
    /// Who is responsible for publishing the recommendations
    #[prost(message, optional, tag = "14")]
    pub authority: ::core::option::Option<super::core::Reference>,
    /// Evaluation target disease
    #[prost(message, optional, tag = "15")]
    pub target_disease: ::core::option::Option<super::core::CodeableConcept>,
    /// Immunization being evaluated
    #[prost(message, optional, tag = "16")]
    pub immunization_event: ::core::option::Option<super::core::Reference>,
    /// Status of the dose relative to published recommendations
    #[prost(message, optional, tag = "17")]
    pub dose_status: ::core::option::Option<super::core::CodeableConcept>,
    /// Reason for the dose status
    #[prost(message, repeated, tag = "18")]
    pub dose_status_reason: prost::alloc::vec::Vec<super::core::CodeableConcept>,
    /// Evaluation notes
    #[prost(message, optional, tag = "19")]
    pub description: ::core::option::Option<super::core::String>,
    /// Name of vaccine series
    #[prost(message, optional, tag = "20")]
    pub series: ::core::option::Option<super::core::String>,
    #[prost(message, optional, tag = "21")]
    pub dose_number: ::core::option::Option<qi_core_immunization_evaluation::DoseNumberX>,
    #[prost(message, optional, tag = "22")]
    pub series_doses: ::core::option::Option<qi_core_immunization_evaluation::SeriesDosesX>,
}
/// Nested message and enum types in `QICoreImmunizationEvaluation`.
pub mod qi_core_immunization_evaluation {
    /// completed | entered-in-error
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct StatusCode {
        #[prost(
            enumeration = "super::super::core::immunization_evaluation_status_codes_value_set::Value",
            tag = "1"
        )]
        pub value: i32,
        #[prost(message, optional, tag = "2")]
        pub id: ::core::option::Option<super::super::core::String>,
        #[prost(message, repeated, tag = "3")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
    }
    /// Dose number within series
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct DoseNumberX {
        #[prost(oneof = "dose_number_x::Choice", tags = "1, 2")]
        pub choice: ::core::option::Option<dose_number_x::Choice>,
    }
    /// Nested message and enum types in `DoseNumberX`.
    pub mod dose_number_x {
        #[derive(Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Oneof)]
        pub enum Choice {
            #[prost(message, tag = "1")]
            PositiveInt(super::super::super::core::PositiveInt),
            #[prost(message, tag = "2")]
            StringValue(super::super::super::core::String),
        }
    }
    /// Recommended number of doses for immunity
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct SeriesDosesX {
        #[prost(oneof = "series_doses_x::Choice", tags = "1, 2")]
        pub choice: ::core::option::Option<series_doses_x::Choice>,
    }
    /// Nested message and enum types in `SeriesDosesX`.
    pub mod series_doses_x {
        #[derive(Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Oneof)]
        pub enum Choice {
            #[prost(message, tag = "1")]
            PositiveInt(super::super::super::core::PositiveInt),
            #[prost(message, tag = "2")]
            StringValue(super::super::super::core::String),
        }
    }
}
/// Auto-generated from StructureDefinition for QICoreImmunizationRecommendation.
/// Guidance or advice relating to an immunization.
/// See <http://hl7.org/fhir/us/qicore/StructureDefinition/qicore-immunizationrec>
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct QiCoreImmunizationRecommendation {
    /// Logical id of this artifact
    #[prost(message, optional, tag = "1")]
    pub id: ::core::option::Option<super::core::Id>,
    /// Metadata about the resource
    #[prost(message, optional, tag = "2")]
    pub meta: ::core::option::Option<super::core::Meta>,
    /// A set of rules under which this content was created
    #[prost(message, optional, tag = "3")]
    pub implicit_rules: ::core::option::Option<super::core::Uri>,
    /// Language of the resource content
    #[prost(message, optional, tag = "4")]
    pub language: ::core::option::Option<super::core::Code>,
    /// Text summary of the resource, for human interpretation
    #[prost(message, optional, tag = "5")]
    pub text: ::core::option::Option<super::core::Narrative>,
    /// Contained, inline Resources
    #[prost(message, repeated, tag = "6")]
    pub contained: prost::alloc::vec::Vec<::prost_wkt_types::Any>,
    /// Additional content defined by implementations
    #[prost(message, repeated, tag = "8")]
    pub extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// Extensions that cannot be ignored
    #[prost(message, repeated, tag = "9")]
    pub modifier_extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// Business identifier
    #[prost(message, repeated, tag = "10")]
    pub identifier: prost::alloc::vec::Vec<super::core::Identifier>,
    /// Who this profile is for
    #[prost(message, optional, tag = "11")]
    pub patient: ::core::option::Option<super::core::Reference>,
    /// Date recommendation(s) created
    #[prost(message, optional, tag = "12")]
    pub date: ::core::option::Option<super::core::DateTime>,
    /// Who is responsible for protocol
    #[prost(message, optional, tag = "13")]
    pub authority: ::core::option::Option<super::core::Reference>,
    #[prost(message, repeated, tag = "14")]
    pub recommendation: prost::alloc::vec::Vec<qi_core_immunization_recommendation::Recommendation>,
}
/// Nested message and enum types in `QICoreImmunizationRecommendation`.
pub mod qi_core_immunization_recommendation {
    /// Vaccine administration recommendations
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct Recommendation {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag = "1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag = "2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag = "3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Vaccine  or vaccine group recommendation applies to
        #[prost(message, repeated, tag = "4")]
        pub vaccine_code: prost::alloc::vec::Vec<super::super::core::CodeableConcept>,
        /// Disease to be immunized against
        #[prost(message, optional, tag = "5")]
        pub target_disease: ::core::option::Option<super::super::core::CodeableConcept>,
        /// Vaccine which is contraindicated to fulfill the recommendation
        #[prost(message, repeated, tag = "6")]
        pub contraindicated_vaccine_code:
            prost::alloc::vec::Vec<super::super::core::CodeableConcept>,
        /// Vaccine recommendation status
        #[prost(message, optional, tag = "7")]
        pub forecast_status: ::core::option::Option<super::super::core::CodeableConcept>,
        /// Vaccine administration status reason
        #[prost(message, repeated, tag = "8")]
        pub forecast_reason: prost::alloc::vec::Vec<super::super::core::CodeableConcept>,
        #[prost(message, repeated, tag = "9")]
        pub date_criterion: prost::alloc::vec::Vec<recommendation::DateCriterion>,
        /// Protocol details
        #[prost(message, optional, tag = "10")]
        pub description: ::core::option::Option<super::super::core::String>,
        /// Name of vaccination series
        #[prost(message, optional, tag = "11")]
        pub series: ::core::option::Option<super::super::core::String>,
        #[prost(message, optional, tag = "12")]
        pub dose_number: ::core::option::Option<recommendation::DoseNumberX>,
        #[prost(message, optional, tag = "13")]
        pub series_doses: ::core::option::Option<recommendation::SeriesDosesX>,
        /// Past immunizations supporting recommendation
        #[prost(message, repeated, tag = "14")]
        pub supporting_immunization: prost::alloc::vec::Vec<super::super::core::Reference>,
        /// Patient observations supporting recommendation
        #[prost(message, repeated, tag = "15")]
        pub supporting_patient_information: prost::alloc::vec::Vec<super::super::core::Reference>,
    }
    /// Nested message and enum types in `Recommendation`.
    pub mod recommendation {
        /// Dates governing proposed immunization
        #[derive(Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Message)]
        pub struct DateCriterion {
            /// Unique id for inter-element referencing
            #[prost(message, optional, tag = "1")]
            pub id: ::core::option::Option<super::super::super::core::String>,
            /// Additional content defined by implementations
            #[prost(message, repeated, tag = "2")]
            pub extension: prost::alloc::vec::Vec<super::super::super::core::Extension>,
            /// Extensions that cannot be ignored even if unrecognized
            #[prost(message, repeated, tag = "3")]
            pub modifier_extension: prost::alloc::vec::Vec<super::super::super::core::Extension>,
            /// Type of date
            #[prost(message, optional, tag = "4")]
            pub code: ::core::option::Option<super::super::super::core::CodeableConcept>,
            /// Recommended date
            #[prost(message, optional, tag = "5")]
            pub value: ::core::option::Option<super::super::super::core::DateTime>,
        }
        /// Recommended dose number within series
        #[derive(Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Message)]
        pub struct DoseNumberX {
            #[prost(oneof = "dose_number_x::Choice", tags = "1, 2")]
            pub choice: ::core::option::Option<dose_number_x::Choice>,
        }
        /// Nested message and enum types in `DoseNumberX`.
        pub mod dose_number_x {
            #[derive(Serialize, Deserialize)]
            #[serde(rename_all = "camelCase")]
            #[derive(Clone, PartialEq, prost::Oneof)]
            pub enum Choice {
                #[prost(message, tag = "1")]
                PositiveInt(super::super::super::super::core::PositiveInt),
                #[prost(message, tag = "2")]
                StringValue(super::super::super::super::core::String),
            }
        }
        /// Recommended number of doses for immunity
        #[derive(Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Message)]
        pub struct SeriesDosesX {
            #[prost(oneof = "series_doses_x::Choice", tags = "1, 2")]
            pub choice: ::core::option::Option<series_doses_x::Choice>,
        }
        /// Nested message and enum types in `SeriesDosesX`.
        pub mod series_doses_x {
            #[derive(Serialize, Deserialize)]
            #[serde(rename_all = "camelCase")]
            #[derive(Clone, PartialEq, prost::Oneof)]
            pub enum Choice {
                #[prost(message, tag = "1")]
                PositiveInt(super::super::super::super::core::PositiveInt),
                #[prost(message, tag = "2")]
                StringValue(super::super::super::super::core::String),
            }
        }
    }
}
/// Auto-generated from StructureDefinition for QICoreLocation.
/// Details and position information for a physical place.
/// See <http://hl7.org/fhir/us/qicore/StructureDefinition/qicore-location>
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct QiCoreLocation {
    /// Logical id of this artifact
    #[prost(message, optional, tag = "1")]
    pub id: ::core::option::Option<super::core::Id>,
    /// Metadata about the resource
    #[prost(message, optional, tag = "2")]
    pub meta: ::core::option::Option<super::core::Meta>,
    /// A set of rules under which this content was created
    #[prost(message, optional, tag = "3")]
    pub implicit_rules: ::core::option::Option<super::core::Uri>,
    /// Language of the resource content
    #[prost(message, optional, tag = "4")]
    pub language: ::core::option::Option<super::core::Code>,
    /// Text summary of the resource, for human interpretation
    #[prost(message, optional, tag = "5")]
    pub text: ::core::option::Option<super::core::Narrative>,
    /// Contained, inline Resources
    #[prost(message, repeated, tag = "6")]
    pub contained: prost::alloc::vec::Vec<::prost_wkt_types::Any>,
    /// Additional content defined by implementations
    #[prost(message, repeated, tag = "8")]
    pub extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// Extensions that cannot be ignored
    #[prost(message, repeated, tag = "9")]
    pub modifier_extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// Unique code or number identifying the location to its users
    #[prost(message, repeated, tag = "10")]
    pub identifier: prost::alloc::vec::Vec<super::core::Identifier>,
    #[prost(message, optional, tag = "11")]
    pub status: ::core::option::Option<qi_core_location::StatusCode>,
    /// The operational status of the location (typically only for a bed/room)
    #[prost(message, optional, tag = "12")]
    pub operational_status: ::core::option::Option<super::core::Coding>,
    /// Name of the location as used by humans
    #[prost(message, optional, tag = "13")]
    pub name: ::core::option::Option<super::core::String>,
    /// A list of alternate names that the location is known as, or was known as,
    /// in the past
    #[prost(message, repeated, tag = "14")]
    pub alias: prost::alloc::vec::Vec<super::core::String>,
    /// Additional details about the location that could be displayed as further
    /// information to identify the location beyond its name
    #[prost(message, optional, tag = "15")]
    pub description: ::core::option::Option<super::core::String>,
    #[prost(message, optional, tag = "16")]
    pub mode: ::core::option::Option<qi_core_location::ModeCode>,
    /// Type of function performed
    #[prost(message, repeated, tag = "17")]
    pub r#type: prost::alloc::vec::Vec<super::core::CodeableConcept>,
    /// Contact details of the location
    #[prost(message, repeated, tag = "18")]
    pub telecom: prost::alloc::vec::Vec<super::core::ContactPoint>,
    /// Physical location
    #[prost(message, optional, tag = "19")]
    pub address: ::core::option::Option<super::core::Address>,
    /// Physical form of the location
    #[prost(message, optional, tag = "20")]
    pub physical_type: ::core::option::Option<super::core::CodeableConcept>,
    #[prost(message, optional, tag = "21")]
    pub position: ::core::option::Option<qi_core_location::Position>,
    /// Organization responsible for provisioning and upkeep
    #[prost(message, optional, tag = "22")]
    pub managing_organization: ::core::option::Option<super::core::Reference>,
    /// Another Location this one is physically a part of
    #[prost(message, optional, tag = "23")]
    pub part_of: ::core::option::Option<super::core::Reference>,
    #[prost(message, repeated, tag = "24")]
    pub hours_of_operation: prost::alloc::vec::Vec<qi_core_location::HoursOfOperation>,
    /// Description of availability exceptions
    #[prost(message, optional, tag = "25")]
    pub availability_exceptions: ::core::option::Option<super::core::String>,
    /// Technical endpoints providing access to services operated for the location
    #[prost(message, repeated, tag = "26")]
    pub endpoint: prost::alloc::vec::Vec<super::core::Reference>,
}
/// Nested message and enum types in `QICoreLocation`.
pub mod qi_core_location {
    /// active | suspended | inactive
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct StatusCode {
        #[prost(
            enumeration = "super::super::core::location_status_code::Value",
            tag = "1"
        )]
        pub value: i32,
        #[prost(message, optional, tag = "2")]
        pub id: ::core::option::Option<super::super::core::String>,
        #[prost(message, repeated, tag = "3")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
    }
    /// instance | kind
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct ModeCode {
        #[prost(
            enumeration = "super::super::core::location_mode_code::Value",
            tag = "1"
        )]
        pub value: i32,
        #[prost(message, optional, tag = "2")]
        pub id: ::core::option::Option<super::super::core::String>,
        #[prost(message, repeated, tag = "3")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
    }
    /// The absolute geographic location
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct Position {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag = "1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag = "2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag = "3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Longitude with WGS84 datum
        #[prost(message, optional, tag = "4")]
        pub longitude: ::core::option::Option<super::super::core::Decimal>,
        /// Latitude with WGS84 datum
        #[prost(message, optional, tag = "5")]
        pub latitude: ::core::option::Option<super::super::core::Decimal>,
        /// Altitude with WGS84 datum
        #[prost(message, optional, tag = "6")]
        pub altitude: ::core::option::Option<super::super::core::Decimal>,
    }
    /// What days/times during a week is this location usually open
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct HoursOfOperation {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag = "1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag = "2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag = "3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        #[prost(message, repeated, tag = "4")]
        pub days_of_week: prost::alloc::vec::Vec<hours_of_operation::DaysOfWeekCode>,
        /// The Location is open all day
        #[prost(message, optional, tag = "5")]
        pub all_day: ::core::option::Option<super::super::core::Boolean>,
        /// Time that the Location opens
        #[prost(message, optional, tag = "6")]
        pub opening_time: ::core::option::Option<super::super::core::Time>,
        /// Time that the Location closes
        #[prost(message, optional, tag = "7")]
        pub closing_time: ::core::option::Option<super::super::core::Time>,
    }
    /// Nested message and enum types in `HoursOfOperation`.
    pub mod hours_of_operation {
        /// mon | tue | wed | thu | fri | sat | sun
        #[derive(Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Message)]
        pub struct DaysOfWeekCode {
            #[prost(
                enumeration = "super::super::super::core::days_of_week_code::Value",
                tag = "1"
            )]
            pub value: i32,
            #[prost(message, optional, tag = "2")]
            pub id: ::core::option::Option<super::super::super::core::String>,
            #[prost(message, repeated, tag = "3")]
            pub extension: prost::alloc::vec::Vec<super::super::super::core::Extension>,
        }
    }
}
/// Auto-generated from StructureDefinition for QICoreMedication.
/// Definition of a Medication.
/// See <http://hl7.org/fhir/us/qicore/StructureDefinition/qicore-medication>
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct QiCoreMedication {
    /// Logical id of this artifact
    #[prost(message, optional, tag = "1")]
    pub id: ::core::option::Option<super::core::Id>,
    /// Metadata about the resource
    #[prost(message, optional, tag = "2")]
    pub meta: ::core::option::Option<super::core::Meta>,
    /// A set of rules under which this content was created
    #[prost(message, optional, tag = "3")]
    pub implicit_rules: ::core::option::Option<super::core::Uri>,
    /// Language of the resource content
    #[prost(message, optional, tag = "4")]
    pub language: ::core::option::Option<super::core::Code>,
    /// Text summary of the resource, for human interpretation
    #[prost(message, optional, tag = "5")]
    pub text: ::core::option::Option<super::core::Narrative>,
    /// Contained, inline Resources
    #[prost(message, repeated, tag = "6")]
    pub contained: prost::alloc::vec::Vec<::prost_wkt_types::Any>,
    /// Additional content defined by implementations
    #[prost(message, repeated, tag = "8")]
    pub extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// Extensions that cannot be ignored
    #[prost(message, repeated, tag = "9")]
    pub modifier_extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// Business identifier for this medication
    #[prost(message, repeated, tag = "10")]
    pub identifier: prost::alloc::vec::Vec<super::core::Identifier>,
    /// Codes that identify this medication
    #[prost(message, optional, tag = "11")]
    pub code: ::core::option::Option<super::core::CodeableConcept>,
    #[prost(message, optional, tag = "12")]
    pub status: ::core::option::Option<qi_core_medication::StatusCode>,
    /// Manufacturer of the item
    #[prost(message, optional, tag = "13")]
    pub manufacturer: ::core::option::Option<super::core::Reference>,
    /// powder | tablets | capsule +
    #[prost(message, optional, tag = "14")]
    pub form: ::core::option::Option<super::core::CodeableConcept>,
    /// Amount of drug in package
    #[prost(message, optional, tag = "15")]
    pub amount: ::core::option::Option<super::core::Ratio>,
    #[prost(message, repeated, tag = "16")]
    pub ingredient: prost::alloc::vec::Vec<qi_core_medication::Ingredient>,
    #[prost(message, optional, tag = "17")]
    pub batch: ::core::option::Option<qi_core_medication::Batch>,
}
/// Nested message and enum types in `QICoreMedication`.
pub mod qi_core_medication {
    /// active | inactive | entered-in-error
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct StatusCode {
        #[prost(
            enumeration = "super::super::core::medication_status_code::Value",
            tag = "1"
        )]
        pub value: i32,
        #[prost(message, optional, tag = "2")]
        pub id: ::core::option::Option<super::super::core::String>,
        #[prost(message, repeated, tag = "3")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
    }
    /// Active or inactive ingredient
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct Ingredient {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag = "1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag = "2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag = "3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        #[prost(message, optional, tag = "4")]
        pub item: ::core::option::Option<ingredient::ItemX>,
        /// Active ingredient indicator
        #[prost(message, optional, tag = "5")]
        pub is_active: ::core::option::Option<super::super::core::Boolean>,
        /// Quantity of ingredient present
        #[prost(message, optional, tag = "6")]
        pub strength: ::core::option::Option<super::super::core::Ratio>,
    }
    /// Nested message and enum types in `Ingredient`.
    pub mod ingredient {
        /// The actual ingredient or content
        #[derive(Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Message)]
        pub struct ItemX {
            #[prost(oneof = "item_x::Choice", tags = "1, 2")]
            pub choice: ::core::option::Option<item_x::Choice>,
        }
        /// Nested message and enum types in `ItemX`.
        pub mod item_x {
            #[derive(Serialize, Deserialize)]
            #[serde(rename_all = "camelCase")]
            #[derive(Clone, PartialEq, prost::Oneof)]
            pub enum Choice {
                #[prost(message, tag = "1")]
                CodeableConcept(super::super::super::super::core::CodeableConcept),
                #[prost(message, tag = "2")]
                Reference(super::super::super::super::core::Reference),
            }
        }
    }
    /// Details about packaged medications
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct Batch {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag = "1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag = "2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag = "3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Identifier assigned to batch
        #[prost(message, optional, tag = "4")]
        pub lot_number: ::core::option::Option<super::super::core::String>,
        /// When batch will expire
        #[prost(message, optional, tag = "5")]
        pub expiration_date: ::core::option::Option<super::super::core::DateTime>,
    }
}
/// Auto-generated from StructureDefinition for QICoreMedicationAdministration.
/// Administration of medication to a patient.
/// See
/// <http://hl7.org/fhir/us/qicore/StructureDefinition/qicore-medicationadministration>
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct QiCoreMedicationAdministration {
    /// Logical id of this artifact
    #[prost(message, optional, tag = "1")]
    pub id: ::core::option::Option<super::core::Id>,
    /// Metadata about the resource
    #[prost(message, optional, tag = "2")]
    pub meta: ::core::option::Option<super::core::Meta>,
    /// A set of rules under which this content was created
    #[prost(message, optional, tag = "3")]
    pub implicit_rules: ::core::option::Option<super::core::Uri>,
    /// Language of the resource content
    #[prost(message, optional, tag = "4")]
    pub language: ::core::option::Option<super::core::Code>,
    /// Text summary of the resource, for human interpretation
    #[prost(message, optional, tag = "5")]
    pub text: ::core::option::Option<super::core::Narrative>,
    /// Contained, inline Resources
    #[prost(message, repeated, tag = "6")]
    pub contained: prost::alloc::vec::Vec<::prost_wkt_types::Any>,
    /// Additional content defined by implementations
    #[prost(message, repeated, tag = "8")]
    pub extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// Extensions that cannot be ignored
    #[prost(message, repeated, tag = "9")]
    pub modifier_extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// External identifier
    #[prost(message, repeated, tag = "10")]
    pub identifier: prost::alloc::vec::Vec<super::core::Identifier>,
    /// Instantiates protocol or definition
    #[prost(message, repeated, tag = "11")]
    pub instantiates: prost::alloc::vec::Vec<super::core::Uri>,
    /// Part of referenced event
    #[prost(message, repeated, tag = "12")]
    pub part_of: prost::alloc::vec::Vec<super::core::Reference>,
    #[prost(message, optional, tag = "13")]
    pub status: ::core::option::Option<qi_core_medication_administration::StatusCode>,
    /// Reason administration not performed
    #[prost(message, repeated, tag = "14")]
    pub status_reason: prost::alloc::vec::Vec<super::core::CodeableConcept>,
    /// Type of medication usage
    #[prost(message, optional, tag = "15")]
    pub category: ::core::option::Option<super::core::CodeableConcept>,
    #[prost(message, optional, tag = "16")]
    pub medication: ::core::option::Option<qi_core_medication_administration::MedicationX>,
    /// Who received medication
    #[prost(message, optional, tag = "17")]
    pub subject: ::core::option::Option<super::core::Reference>,
    /// Encounter or Episode of Care administered as part of
    #[prost(message, optional, tag = "18")]
    pub context: ::core::option::Option<super::core::Reference>,
    /// Additional information to support administration
    #[prost(message, repeated, tag = "19")]
    pub supporting_information: prost::alloc::vec::Vec<super::core::Reference>,
    #[prost(message, optional, tag = "20")]
    pub effective: ::core::option::Option<qi_core_medication_administration::EffectiveX>,
    #[prost(message, repeated, tag = "21")]
    pub performer: prost::alloc::vec::Vec<qi_core_medication_administration::Performer>,
    /// Reason administration performed
    #[prost(message, repeated, tag = "22")]
    pub reason_code: prost::alloc::vec::Vec<super::core::CodeableConcept>,
    /// Condition or observation that supports why the medication was administered
    #[prost(message, repeated, tag = "23")]
    pub reason_reference: prost::alloc::vec::Vec<super::core::Reference>,
    /// Request administration performed against
    #[prost(message, optional, tag = "24")]
    pub request: ::core::option::Option<super::core::Reference>,
    /// Device used to administer
    #[prost(message, repeated, tag = "25")]
    pub device: prost::alloc::vec::Vec<super::core::Reference>,
    /// Information about the administration
    #[prost(message, repeated, tag = "26")]
    pub note: prost::alloc::vec::Vec<super::core::Annotation>,
    #[prost(message, optional, tag = "27")]
    pub dosage: ::core::option::Option<qi_core_medication_administration::Dosage>,
    /// A list of events of interest in the lifecycle
    #[prost(message, repeated, tag = "28")]
    pub event_history: prost::alloc::vec::Vec<super::core::Reference>,
}
/// Nested message and enum types in `QICoreMedicationAdministration`.
pub mod qi_core_medication_administration {
    /// in-progress | not-done | on-hold | completed | entered-in-error | stopped |
    /// unknown
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct StatusCode {
        #[prost(
            enumeration = "super::super::core::medication_administration_status_code::Value",
            tag = "1"
        )]
        pub value: i32,
        #[prost(message, optional, tag = "2")]
        pub id: ::core::option::Option<super::super::core::String>,
        #[prost(message, repeated, tag = "3")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
    }
    /// What was administered
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct MedicationX {
        #[prost(oneof = "medication_x::Choice", tags = "1, 2")]
        pub choice: ::core::option::Option<medication_x::Choice>,
    }
    /// Nested message and enum types in `MedicationX`.
    pub mod medication_x {
        #[derive(Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Oneof)]
        pub enum Choice {
            #[prost(message, tag = "1")]
            CodeableConcept(super::super::super::core::CodeableConcept),
            #[prost(message, tag = "2")]
            Reference(super::super::super::core::Reference),
        }
    }
    /// Start and end time of administration
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct EffectiveX {
        #[prost(oneof = "effective_x::Choice", tags = "1, 2")]
        pub choice: ::core::option::Option<effective_x::Choice>,
    }
    /// Nested message and enum types in `EffectiveX`.
    pub mod effective_x {
        #[derive(Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Oneof)]
        pub enum Choice {
            #[prost(message, tag = "1")]
            DateTime(super::super::super::core::DateTime),
            #[prost(message, tag = "2")]
            Period(super::super::super::core::Period),
        }
    }
    /// Who performed the medication administration and what they did
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct Performer {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag = "1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag = "2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag = "3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Type of performance
        #[prost(message, optional, tag = "4")]
        pub function: ::core::option::Option<super::super::core::CodeableConcept>,
        /// Who performed the medication administration
        #[prost(message, optional, tag = "5")]
        pub actor: ::core::option::Option<super::super::core::Reference>,
    }
    /// Details of how medication was taken
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct Dosage {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag = "1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag = "2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag = "3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Free text dosage instructions e.g. SIG
        #[prost(message, optional, tag = "4")]
        pub text: ::core::option::Option<super::super::core::String>,
        /// Body site administered to
        #[prost(message, optional, tag = "5")]
        pub site: ::core::option::Option<super::super::core::CodeableConcept>,
        /// Path of substance into body
        #[prost(message, optional, tag = "6")]
        pub route: ::core::option::Option<super::super::core::CodeableConcept>,
        /// How drug was administered
        #[prost(message, optional, tag = "7")]
        pub method: ::core::option::Option<super::super::core::CodeableConcept>,
        /// Amount of medication per dose
        #[prost(message, optional, tag = "8")]
        pub dose: ::core::option::Option<super::super::core::SimpleQuantity>,
        #[prost(message, optional, tag = "9")]
        pub rate: ::core::option::Option<dosage::RateX>,
    }
    /// Nested message and enum types in `Dosage`.
    pub mod dosage {
        /// Dose quantity per unit of time
        #[derive(Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Message)]
        pub struct RateX {
            #[prost(oneof = "rate_x::Choice", tags = "1, 2")]
            pub choice: ::core::option::Option<rate_x::Choice>,
        }
        /// Nested message and enum types in `RateX`.
        pub mod rate_x {
            #[derive(Serialize, Deserialize)]
            #[serde(rename_all = "camelCase")]
            #[derive(Clone, PartialEq, prost::Oneof)]
            pub enum Choice {
                #[prost(message, tag = "1")]
                Ratio(super::super::super::super::core::Ratio),
                #[prost(message, tag = "2")]
                Quantity(super::super::super::super::core::SimpleQuantity),
            }
        }
    }
}
/// Auto-generated from StructureDefinition for QICoreMedicationDispense.
/// Dispensing a medication to a named patient.
/// See
/// <http://hl7.org/fhir/us/qicore/StructureDefinition/qicore-medicationdispense>
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct QiCoreMedicationDispense {
    /// Logical id of this artifact
    #[prost(message, optional, tag = "1")]
    pub id: ::core::option::Option<super::core::Id>,
    /// Metadata about the resource
    #[prost(message, optional, tag = "2")]
    pub meta: ::core::option::Option<super::core::Meta>,
    /// A set of rules under which this content was created
    #[prost(message, optional, tag = "3")]
    pub implicit_rules: ::core::option::Option<super::core::Uri>,
    /// Language of the resource content
    #[prost(message, optional, tag = "4")]
    pub language: ::core::option::Option<super::core::Code>,
    /// Text summary of the resource, for human interpretation
    #[prost(message, optional, tag = "5")]
    pub text: ::core::option::Option<super::core::Narrative>,
    /// Contained, inline Resources
    #[prost(message, repeated, tag = "6")]
    pub contained: prost::alloc::vec::Vec<::prost_wkt_types::Any>,
    /// Additional content defined by implementations
    #[prost(message, repeated, tag = "8")]
    pub extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// Extensions that cannot be ignored
    #[prost(message, repeated, tag = "9")]
    pub modifier_extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// External identifier
    #[prost(message, repeated, tag = "10")]
    pub identifier: prost::alloc::vec::Vec<super::core::Identifier>,
    /// Event that dispense is part of
    #[prost(message, repeated, tag = "11")]
    pub part_of: prost::alloc::vec::Vec<super::core::Reference>,
    #[prost(message, optional, tag = "12")]
    pub status: ::core::option::Option<qi_core_medication_dispense::StatusCode>,
    #[prost(message, optional, tag = "13")]
    pub status_reason: ::core::option::Option<qi_core_medication_dispense::StatusReasonX>,
    /// Type of medication dispense
    #[prost(message, optional, tag = "14")]
    pub category: ::core::option::Option<super::core::CodeableConcept>,
    #[prost(message, optional, tag = "15")]
    pub medication: ::core::option::Option<qi_core_medication_dispense::MedicationX>,
    /// Who the dispense is for
    #[prost(message, optional, tag = "16")]
    pub subject: ::core::option::Option<super::core::Reference>,
    /// Encounter / Episode associated with event
    #[prost(message, optional, tag = "17")]
    pub context: ::core::option::Option<super::core::Reference>,
    /// Information that supports the dispensing of the medication
    #[prost(message, repeated, tag = "18")]
    pub supporting_information: prost::alloc::vec::Vec<super::core::Reference>,
    #[prost(message, repeated, tag = "19")]
    pub performer: prost::alloc::vec::Vec<qi_core_medication_dispense::Performer>,
    /// Where the dispense occurred
    #[prost(message, optional, tag = "20")]
    pub location: ::core::option::Option<super::core::Reference>,
    /// Medication order that authorizes the dispense
    #[prost(message, repeated, tag = "21")]
    pub authorizing_prescription: prost::alloc::vec::Vec<super::core::Reference>,
    /// Trial fill, partial fill, emergency fill, etc.
    #[prost(message, optional, tag = "22")]
    pub r#type: ::core::option::Option<super::core::CodeableConcept>,
    /// Amount dispensed
    #[prost(message, optional, tag = "23")]
    pub quantity: ::core::option::Option<super::core::SimpleQuantity>,
    /// Amount of medication expressed as a timing amount
    #[prost(message, optional, tag = "24")]
    pub days_supply: ::core::option::Option<super::core::SimpleQuantity>,
    /// When product was packaged and reviewed
    #[prost(message, optional, tag = "25")]
    pub when_prepared: ::core::option::Option<super::core::DateTime>,
    /// When product was given out
    #[prost(message, optional, tag = "26")]
    pub when_handed_over: ::core::option::Option<super::core::DateTime>,
    /// Where the medication was sent
    #[prost(message, optional, tag = "27")]
    pub destination: ::core::option::Option<super::core::Reference>,
    /// Who collected the medication
    #[prost(message, repeated, tag = "28")]
    pub receiver: prost::alloc::vec::Vec<super::core::Reference>,
    /// Information about the dispense
    #[prost(message, repeated, tag = "29")]
    pub note: prost::alloc::vec::Vec<super::core::Annotation>,
    /// How the medication is to be used by the patient or administered by the
    /// caregiver
    #[prost(message, repeated, tag = "30")]
    pub dosage_instruction: prost::alloc::vec::Vec<super::core::Dosage>,
    #[prost(message, optional, tag = "31")]
    pub substitution: ::core::option::Option<qi_core_medication_dispense::Substitution>,
    /// Clinical issue with action
    #[prost(message, repeated, tag = "32")]
    pub detected_issue: prost::alloc::vec::Vec<super::core::Reference>,
    /// A list of relevant lifecycle events
    #[prost(message, repeated, tag = "33")]
    pub event_history: prost::alloc::vec::Vec<super::core::Reference>,
}
/// Nested message and enum types in `QICoreMedicationDispense`.
pub mod qi_core_medication_dispense {
    /// preparation | in-progress | cancelled | on-hold | completed |
    /// entered-in-error | stopped | unknown
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct StatusCode {
        #[prost(
            enumeration = "super::super::core::medication_dispense_status_code::Value",
            tag = "1"
        )]
        pub value: i32,
        #[prost(message, optional, tag = "2")]
        pub id: ::core::option::Option<super::super::core::String>,
        #[prost(message, repeated, tag = "3")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
    }
    /// Why a dispense was not performed
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct StatusReasonX {
        #[prost(oneof = "status_reason_x::Choice", tags = "1, 2")]
        pub choice: ::core::option::Option<status_reason_x::Choice>,
    }
    /// Nested message and enum types in `StatusReasonX`.
    pub mod status_reason_x {
        #[derive(Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Oneof)]
        pub enum Choice {
            #[prost(message, tag = "1")]
            CodeableConcept(super::super::super::core::CodeableConcept),
            #[prost(message, tag = "2")]
            Reference(super::super::super::core::Reference),
        }
    }
    /// What medication was supplied
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct MedicationX {
        #[prost(oneof = "medication_x::Choice", tags = "1, 2")]
        pub choice: ::core::option::Option<medication_x::Choice>,
    }
    /// Nested message and enum types in `MedicationX`.
    pub mod medication_x {
        #[derive(Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Oneof)]
        pub enum Choice {
            #[prost(message, tag = "1")]
            CodeableConcept(super::super::super::core::CodeableConcept),
            #[prost(message, tag = "2")]
            Reference(super::super::super::core::Reference),
        }
    }
    /// Who performed event
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct Performer {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag = "1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag = "2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag = "3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Who performed the dispense and what they did
        #[prost(message, optional, tag = "4")]
        pub function: ::core::option::Option<super::super::core::CodeableConcept>,
        /// Individual who was performing
        #[prost(message, optional, tag = "5")]
        pub actor: ::core::option::Option<super::super::core::Reference>,
    }
    /// Whether a substitution was performed on the dispense
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct Substitution {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag = "1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag = "2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag = "3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Whether a substitution was or was not performed on the dispense
        #[prost(message, optional, tag = "4")]
        pub was_substituted: ::core::option::Option<super::super::core::Boolean>,
        /// Code signifying whether a different drug was dispensed from what was
        /// prescribed
        #[prost(message, optional, tag = "5")]
        pub r#type: ::core::option::Option<super::super::core::CodeableConcept>,
        /// Why was substitution made
        #[prost(message, repeated, tag = "6")]
        pub reason: prost::alloc::vec::Vec<super::super::core::CodeableConcept>,
        /// Who is responsible for the substitution
        #[prost(message, repeated, tag = "7")]
        pub responsible_party: prost::alloc::vec::Vec<super::super::core::Reference>,
    }
}
/// Auto-generated from StructureDefinition for QICoreMedicationRequest.
/// Ordering of medication for patient or group.
/// See
/// <http://hl7.org/fhir/us/qicore/StructureDefinition/qicore-medicationrequest>
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct QiCoreMedicationRequest {
    /// Logical id of this artifact
    #[prost(message, optional, tag = "1")]
    pub id: ::core::option::Option<super::core::Id>,
    /// Metadata about the resource
    #[prost(message, optional, tag = "2")]
    pub meta: ::core::option::Option<super::core::Meta>,
    /// A set of rules under which this content was created
    #[prost(message, optional, tag = "3")]
    pub implicit_rules: ::core::option::Option<super::core::Uri>,
    /// Language of the resource content
    #[prost(message, optional, tag = "4")]
    pub language: ::core::option::Option<super::core::Code>,
    /// Text summary of the resource, for human interpretation
    #[prost(message, optional, tag = "5")]
    pub text: ::core::option::Option<super::core::Narrative>,
    /// Contained, inline Resources
    #[prost(message, repeated, tag = "6")]
    pub contained: prost::alloc::vec::Vec<::prost_wkt_types::Any>,
    /// Additional content defined by implementations
    #[prost(message, repeated, tag = "8")]
    pub extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// Extensions that cannot be ignored
    #[prost(message, repeated, tag = "9")]
    pub modifier_extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// External ids for this request
    #[prost(message, repeated, tag = "10")]
    pub identifier: prost::alloc::vec::Vec<super::core::Identifier>,
    #[prost(message, optional, tag = "11")]
    pub status: ::core::option::Option<qi_core_medication_request::StatusCode>,
    /// Reason for current status
    #[prost(message, optional, tag = "12")]
    pub status_reason: ::core::option::Option<super::core::CodeableConcept>,
    #[prost(message, optional, tag = "13")]
    pub intent: ::core::option::Option<qi_core_medication_request::IntentCode>,
    /// Type of medication usage
    #[prost(message, repeated, tag = "14")]
    pub category: prost::alloc::vec::Vec<super::core::CodeableConcept>,
    #[prost(message, optional, tag = "15")]
    pub priority: ::core::option::Option<qi_core_medication_request::PriorityCode>,
    /// True if request is prohibiting action
    #[prost(message, optional, tag = "16")]
    pub do_not_perform: ::core::option::Option<super::core::Boolean>,
    #[prost(message, optional, tag = "17")]
    pub reported: ::core::option::Option<qi_core_medication_request::ReportedX>,
    #[prost(message, optional, tag = "18")]
    pub medication: ::core::option::Option<qi_core_medication_request::MedicationX>,
    /// Who or group medication request is for
    #[prost(message, optional, tag = "19")]
    pub subject: ::core::option::Option<super::core::Reference>,
    /// Encounter created as part of encounter/admission/stay
    #[prost(message, optional, tag = "20")]
    pub encounter: ::core::option::Option<super::core::Reference>,
    /// Information to support ordering of the medication
    #[prost(message, repeated, tag = "21")]
    pub supporting_information: prost::alloc::vec::Vec<super::core::Reference>,
    /// When request was initially authored
    #[prost(message, optional, tag = "22")]
    pub authored_on: ::core::option::Option<super::core::DateTime>,
    /// Who/What requested the Request
    #[prost(message, optional, tag = "23")]
    pub requester: ::core::option::Option<super::core::Reference>,
    /// Intended performer of administration
    #[prost(message, optional, tag = "24")]
    pub performer: ::core::option::Option<super::core::Reference>,
    /// Desired kind of performer of the medication administration
    #[prost(message, optional, tag = "25")]
    pub performer_type: ::core::option::Option<super::core::CodeableConcept>,
    /// Person who entered the request
    #[prost(message, optional, tag = "26")]
    pub recorder: ::core::option::Option<super::core::Reference>,
    /// Reason or indication for ordering or not ordering the medication
    #[prost(message, repeated, tag = "27")]
    pub reason_code: prost::alloc::vec::Vec<super::core::CodeableConcept>,
    /// Condition or observation that supports why the prescription is being
    /// written
    #[prost(message, repeated, tag = "28")]
    pub reason_reference: prost::alloc::vec::Vec<super::core::Reference>,
    /// Instantiates FHIR protocol or definition
    #[prost(message, repeated, tag = "29")]
    pub instantiates_canonical: prost::alloc::vec::Vec<super::core::Canonical>,
    /// Instantiates external protocol or definition
    #[prost(message, repeated, tag = "30")]
    pub instantiates_uri: prost::alloc::vec::Vec<super::core::Uri>,
    /// What request fulfills
    #[prost(message, repeated, tag = "31")]
    pub based_on: prost::alloc::vec::Vec<super::core::Reference>,
    /// Composite request this is part of
    #[prost(message, optional, tag = "32")]
    pub group_identifier: ::core::option::Option<super::core::Identifier>,
    /// Overall pattern of medication administration
    #[prost(message, optional, tag = "33")]
    pub course_of_therapy_type: ::core::option::Option<super::core::CodeableConcept>,
    /// Associated insurance coverage
    #[prost(message, repeated, tag = "34")]
    pub insurance: prost::alloc::vec::Vec<super::core::Reference>,
    /// Information about the prescription
    #[prost(message, repeated, tag = "35")]
    pub note: prost::alloc::vec::Vec<super::core::Annotation>,
    /// How the medication should be taken
    #[prost(message, repeated, tag = "36")]
    pub dosage_instruction: prost::alloc::vec::Vec<super::core::Dosage>,
    #[prost(message, optional, tag = "37")]
    pub dispense_request: ::core::option::Option<qi_core_medication_request::DispenseRequest>,
    #[prost(message, optional, tag = "38")]
    pub substitution: ::core::option::Option<qi_core_medication_request::Substitution>,
    /// An order/prescription that is being replaced
    #[prost(message, optional, tag = "39")]
    pub prior_prescription: ::core::option::Option<super::core::Reference>,
    /// Clinical Issue with action
    #[prost(message, repeated, tag = "40")]
    pub detected_issue: prost::alloc::vec::Vec<super::core::Reference>,
    /// A list of events of interest in the lifecycle
    #[prost(message, repeated, tag = "41")]
    pub event_history: prost::alloc::vec::Vec<super::core::Reference>,
}
/// Nested message and enum types in `QICoreMedicationRequest`.
pub mod qi_core_medication_request {
    /// active | on-hold | cancelled | completed | entered-in-error | stopped |
    /// draft | unknown
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct StatusCode {
        #[prost(
            enumeration = "super::super::core::medicationrequest_status_code::Value",
            tag = "1"
        )]
        pub value: i32,
        #[prost(message, optional, tag = "2")]
        pub id: ::core::option::Option<super::super::core::String>,
        #[prost(message, repeated, tag = "3")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
    }
    /// proposal | plan | order | original-order | instance-order | option
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct IntentCode {
        #[prost(
            enumeration = "super::super::core::medication_request_intent_code::Value",
            tag = "1"
        )]
        pub value: i32,
        #[prost(message, optional, tag = "2")]
        pub id: ::core::option::Option<super::super::core::String>,
        #[prost(message, repeated, tag = "3")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
    }
    /// routine | urgent | asap | stat
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct PriorityCode {
        #[prost(
            enumeration = "super::super::core::request_priority_code::Value",
            tag = "1"
        )]
        pub value: i32,
        #[prost(message, optional, tag = "2")]
        pub id: ::core::option::Option<super::super::core::String>,
        #[prost(message, repeated, tag = "3")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
    }
    /// Reported rather than primary record
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct ReportedX {
        #[prost(oneof = "reported_x::Choice", tags = "1, 2")]
        pub choice: ::core::option::Option<reported_x::Choice>,
    }
    /// Nested message and enum types in `ReportedX`.
    pub mod reported_x {
        #[derive(Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Oneof)]
        pub enum Choice {
            #[prost(message, tag = "1")]
            Boolean(super::super::super::core::Boolean),
            #[prost(message, tag = "2")]
            Reference(super::super::super::core::Reference),
        }
    }
    /// Medication to be taken
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct MedicationX {
        #[prost(oneof = "medication_x::Choice", tags = "1, 2")]
        pub choice: ::core::option::Option<medication_x::Choice>,
    }
    /// Nested message and enum types in `MedicationX`.
    pub mod medication_x {
        #[derive(Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Oneof)]
        pub enum Choice {
            #[prost(message, tag = "1")]
            CodeableConcept(super::super::super::core::CodeableConcept),
            #[prost(message, tag = "2")]
            Reference(super::super::super::core::Reference),
        }
    }
    /// Medication supply authorization
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct DispenseRequest {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag = "1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag = "2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag = "3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        #[prost(message, optional, tag = "4")]
        pub initial_fill: ::core::option::Option<dispense_request::InitialFill>,
        /// Minimum period of time between dispenses
        #[prost(message, optional, tag = "5")]
        pub dispense_interval: ::core::option::Option<super::super::core::Duration>,
        /// Time period supply is authorized for
        #[prost(message, optional, tag = "6")]
        pub validity_period: ::core::option::Option<super::super::core::Period>,
        /// Number of refills authorized
        #[prost(message, optional, tag = "7")]
        pub number_of_repeats_allowed: ::core::option::Option<super::super::core::UnsignedInt>,
        /// Amount of medication to supply per dispense
        #[prost(message, optional, tag = "8")]
        pub quantity: ::core::option::Option<super::super::core::SimpleQuantity>,
        /// Number of days supply per dispense
        #[prost(message, optional, tag = "9")]
        pub expected_supply_duration: ::core::option::Option<super::super::core::Duration>,
        /// Intended dispenser
        #[prost(message, optional, tag = "10")]
        pub performer: ::core::option::Option<super::super::core::Reference>,
    }
    /// Nested message and enum types in `DispenseRequest`.
    pub mod dispense_request {
        /// First fill details
        #[derive(Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Message)]
        pub struct InitialFill {
            /// Unique id for inter-element referencing
            #[prost(message, optional, tag = "1")]
            pub id: ::core::option::Option<super::super::super::core::String>,
            /// Additional content defined by implementations
            #[prost(message, repeated, tag = "2")]
            pub extension: prost::alloc::vec::Vec<super::super::super::core::Extension>,
            /// Extensions that cannot be ignored even if unrecognized
            #[prost(message, repeated, tag = "3")]
            pub modifier_extension: prost::alloc::vec::Vec<super::super::super::core::Extension>,
            /// First fill quantity
            #[prost(message, optional, tag = "4")]
            pub quantity: ::core::option::Option<super::super::super::core::SimpleQuantity>,
            /// First fill duration
            #[prost(message, optional, tag = "5")]
            pub duration: ::core::option::Option<super::super::super::core::Duration>,
        }
    }
    /// Any restrictions on medication substitution
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct Substitution {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag = "1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag = "2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag = "3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        #[prost(message, optional, tag = "4")]
        pub allowed: ::core::option::Option<substitution::AllowedX>,
        /// Why should (not) substitution be made
        #[prost(message, optional, tag = "5")]
        pub reason: ::core::option::Option<super::super::core::CodeableConcept>,
    }
    /// Nested message and enum types in `Substitution`.
    pub mod substitution {
        /// Whether substitution is allowed or not
        #[derive(Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Message)]
        pub struct AllowedX {
            #[prost(oneof = "allowed_x::Choice", tags = "1, 2")]
            pub choice: ::core::option::Option<allowed_x::Choice>,
        }
        /// Nested message and enum types in `AllowedX`.
        pub mod allowed_x {
            #[derive(Serialize, Deserialize)]
            #[serde(rename_all = "camelCase")]
            #[derive(Clone, PartialEq, prost::Oneof)]
            pub enum Choice {
                #[prost(message, tag = "1")]
                Boolean(super::super::super::super::core::Boolean),
                #[prost(message, tag = "2")]
                CodeableConcept(super::super::super::super::core::CodeableConcept),
            }
        }
    }
}
/// Auto-generated from StructureDefinition for QICoreMedicationStatement.
/// Record of medication being taken by a patient.
/// See
/// <http://hl7.org/fhir/us/qicore/StructureDefinition/qicore-medicationstatement>
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct QiCoreMedicationStatement {
    /// Logical id of this artifact
    #[prost(message, optional, tag = "1")]
    pub id: ::core::option::Option<super::core::Id>,
    /// Metadata about the resource
    #[prost(message, optional, tag = "2")]
    pub meta: ::core::option::Option<super::core::Meta>,
    /// A set of rules under which this content was created
    #[prost(message, optional, tag = "3")]
    pub implicit_rules: ::core::option::Option<super::core::Uri>,
    /// Language of the resource content
    #[prost(message, optional, tag = "4")]
    pub language: ::core::option::Option<super::core::Code>,
    /// Text summary of the resource, for human interpretation
    #[prost(message, optional, tag = "5")]
    pub text: ::core::option::Option<super::core::Narrative>,
    /// Contained, inline Resources
    #[prost(message, repeated, tag = "6")]
    pub contained: prost::alloc::vec::Vec<::prost_wkt_types::Any>,
    /// Additional content defined by implementations
    #[prost(message, repeated, tag = "8")]
    pub extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// Extensions that cannot be ignored
    #[prost(message, repeated, tag = "9")]
    pub modifier_extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// External identifier
    #[prost(message, repeated, tag = "10")]
    pub identifier: prost::alloc::vec::Vec<super::core::Identifier>,
    /// Fulfils plan, proposal or order
    #[prost(message, repeated, tag = "11")]
    pub based_on: prost::alloc::vec::Vec<super::core::Reference>,
    /// Part of referenced event
    #[prost(message, repeated, tag = "12")]
    pub part_of: prost::alloc::vec::Vec<super::core::Reference>,
    #[prost(message, optional, tag = "13")]
    pub status: ::core::option::Option<qi_core_medication_statement::StatusCode>,
    /// Reason for current status
    #[prost(message, repeated, tag = "14")]
    pub status_reason: prost::alloc::vec::Vec<super::core::CodeableConcept>,
    /// Type of medication usage
    #[prost(message, optional, tag = "15")]
    pub category: ::core::option::Option<super::core::CodeableConcept>,
    #[prost(message, optional, tag = "16")]
    pub medication: ::core::option::Option<qi_core_medication_statement::MedicationX>,
    /// Who is/was taking  the medication
    #[prost(message, optional, tag = "17")]
    pub subject: ::core::option::Option<super::core::Reference>,
    /// Encounter / Episode associated with MedicationStatement
    #[prost(message, optional, tag = "18")]
    pub context: ::core::option::Option<super::core::Reference>,
    #[prost(message, optional, tag = "19")]
    pub effective: ::core::option::Option<qi_core_medication_statement::EffectiveX>,
    /// When the statement was asserted?
    #[prost(message, optional, tag = "20")]
    pub date_asserted: ::core::option::Option<super::core::DateTime>,
    /// Person or organization that provided the information about the taking of
    /// this medication
    #[prost(message, optional, tag = "21")]
    pub information_source: ::core::option::Option<super::core::Reference>,
    /// Additional supporting information
    #[prost(message, repeated, tag = "22")]
    pub derived_from: prost::alloc::vec::Vec<super::core::Reference>,
    /// Reason for why the medication is being/was taken
    #[prost(message, repeated, tag = "23")]
    pub reason_code: prost::alloc::vec::Vec<super::core::CodeableConcept>,
    /// Condition or observation that supports why the medication is being/was
    /// taken
    #[prost(message, repeated, tag = "24")]
    pub reason_reference: prost::alloc::vec::Vec<super::core::Reference>,
    /// Further information about the statement
    #[prost(message, repeated, tag = "25")]
    pub note: prost::alloc::vec::Vec<super::core::Annotation>,
    /// Details of how medication is/was taken or should be taken
    #[prost(message, repeated, tag = "26")]
    pub dosage: prost::alloc::vec::Vec<super::core::Dosage>,
}
/// Nested message and enum types in `QICoreMedicationStatement`.
pub mod qi_core_medication_statement {
    /// active | completed | entered-in-error | intended | stopped | on-hold |
    /// unknown | not-taken
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct StatusCode {
        #[prost(
            enumeration = "super::super::core::medication_statement_status_codes::Value",
            tag = "1"
        )]
        pub value: i32,
        #[prost(message, optional, tag = "2")]
        pub id: ::core::option::Option<super::super::core::String>,
        #[prost(message, repeated, tag = "3")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
    }
    /// What medication was taken
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct MedicationX {
        #[prost(oneof = "medication_x::Choice", tags = "1, 2")]
        pub choice: ::core::option::Option<medication_x::Choice>,
    }
    /// Nested message and enum types in `MedicationX`.
    pub mod medication_x {
        #[derive(Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Oneof)]
        pub enum Choice {
            #[prost(message, tag = "1")]
            CodeableConcept(super::super::super::core::CodeableConcept),
            #[prost(message, tag = "2")]
            Reference(super::super::super::core::Reference),
        }
    }
    /// The date/time or interval when the medication is/was/will be taken
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct EffectiveX {
        #[prost(oneof = "effective_x::Choice", tags = "1, 2")]
        pub choice: ::core::option::Option<effective_x::Choice>,
    }
    /// Nested message and enum types in `EffectiveX`.
    pub mod effective_x {
        #[derive(Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Oneof)]
        pub enum Choice {
            #[prost(message, tag = "1")]
            DateTime(super::super::super::core::DateTime),
            #[prost(message, tag = "2")]
            Period(super::super::super::core::Period),
        }
    }
}
/// Auto-generated from StructureDefinition for MilitaryService.
/// Explanation associated with refuted status.
/// See <http://hl7.org/fhir/us/qicore/StructureDefinition/qicore-military-service>
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct PatientMilitaryService {
    /// Unique id for inter-element referencing
    #[prost(message, optional, tag = "1")]
    pub id: ::core::option::Option<super::core::String>,
    /// Value of extension
    #[prost(message, optional, tag = "3")]
    pub value_codeable_concept: ::core::option::Option<super::core::CodeableConcept>,
}
/// Auto-generated from StructureDefinition for QICoreNutritionOrder.
/// Diet, formula or nutritional supplement request.
/// See <http://hl7.org/fhir/us/qicore/StructureDefinition/qicore-nutritionorder>
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct QiCoreNutritionOrder {
    /// Logical id of this artifact
    #[prost(message, optional, tag = "1")]
    pub id: ::core::option::Option<super::core::Id>,
    /// Metadata about the resource
    #[prost(message, optional, tag = "2")]
    pub meta: ::core::option::Option<super::core::Meta>,
    /// A set of rules under which this content was created
    #[prost(message, optional, tag = "3")]
    pub implicit_rules: ::core::option::Option<super::core::Uri>,
    /// Language of the resource content
    #[prost(message, optional, tag = "4")]
    pub language: ::core::option::Option<super::core::Code>,
    /// Text summary of the resource, for human interpretation
    #[prost(message, optional, tag = "5")]
    pub text: ::core::option::Option<super::core::Narrative>,
    /// Contained, inline Resources
    #[prost(message, repeated, tag = "6")]
    pub contained: prost::alloc::vec::Vec<::prost_wkt_types::Any>,
    /// Additional content defined by implementations
    #[prost(message, repeated, tag = "8")]
    pub extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// Extensions that cannot be ignored
    #[prost(message, repeated, tag = "9")]
    pub modifier_extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// Identifiers assigned to this order
    #[prost(message, repeated, tag = "10")]
    pub identifier: prost::alloc::vec::Vec<super::core::Identifier>,
    /// Instantiates FHIR protocol or definition
    #[prost(message, repeated, tag = "11")]
    pub instantiates_canonical: prost::alloc::vec::Vec<super::core::Canonical>,
    /// Instantiates external protocol or definition
    #[prost(message, repeated, tag = "12")]
    pub instantiates_uri: prost::alloc::vec::Vec<super::core::Uri>,
    /// Instantiates protocol or definition
    #[prost(message, repeated, tag = "13")]
    pub instantiates: prost::alloc::vec::Vec<super::core::Uri>,
    #[prost(message, optional, tag = "14")]
    pub status: ::core::option::Option<qi_core_nutrition_order::StatusCode>,
    #[prost(message, optional, tag = "15")]
    pub intent: ::core::option::Option<qi_core_nutrition_order::IntentCode>,
    /// The person who requires the diet, formula or nutritional supplement
    #[prost(message, optional, tag = "16")]
    pub patient: ::core::option::Option<super::core::Reference>,
    /// The encounter associated with this nutrition order
    #[prost(message, optional, tag = "17")]
    pub encounter: ::core::option::Option<super::core::Reference>,
    /// Date and time the nutrition order was requested
    #[prost(message, optional, tag = "18")]
    pub date_time: ::core::option::Option<super::core::DateTime>,
    /// Who ordered the diet, formula or nutritional supplement
    #[prost(message, optional, tag = "19")]
    pub orderer: ::core::option::Option<super::core::Reference>,
    /// List of the patient's food and nutrition-related allergies and intolerances
    #[prost(message, repeated, tag = "20")]
    pub allergy_intolerance: prost::alloc::vec::Vec<super::core::Reference>,
    /// Order-specific modifier about the type of food that should be given
    #[prost(message, repeated, tag = "21")]
    pub food_preference_modifier: prost::alloc::vec::Vec<super::core::CodeableConcept>,
    /// Order-specific modifier about the type of food that should not be given
    #[prost(message, repeated, tag = "22")]
    pub exclude_food_modifier: prost::alloc::vec::Vec<super::core::CodeableConcept>,
    #[prost(message, optional, tag = "23")]
    pub oral_diet: ::core::option::Option<qi_core_nutrition_order::OralDiet>,
    #[prost(message, repeated, tag = "24")]
    pub supplement: prost::alloc::vec::Vec<qi_core_nutrition_order::Supplement>,
    #[prost(message, optional, tag = "25")]
    pub enteral_formula: ::core::option::Option<qi_core_nutrition_order::EnteralFormula>,
    /// Comments
    #[prost(message, repeated, tag = "26")]
    pub note: prost::alloc::vec::Vec<super::core::Annotation>,
}
/// Nested message and enum types in `QICoreNutritionOrder`.
pub mod qi_core_nutrition_order {
    /// proposed | draft | planned | requested | active | on-hold | completed |
    /// cancelled | entered-in-error
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct StatusCode {
        #[prost(
            enumeration = "super::super::core::request_status_code::Value",
            tag = "1"
        )]
        pub value: i32,
        #[prost(message, optional, tag = "2")]
        pub id: ::core::option::Option<super::super::core::String>,
        #[prost(message, repeated, tag = "3")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
    }
    /// proposal | plan | order
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct IntentCode {
        #[prost(
            enumeration = "super::super::core::request_intent_code::Value",
            tag = "1"
        )]
        pub value: i32,
        #[prost(message, optional, tag = "2")]
        pub id: ::core::option::Option<super::super::core::String>,
        #[prost(message, repeated, tag = "3")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
    }
    /// Oral diet components
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct OralDiet {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag = "1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag = "2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag = "3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Type of oral diet or diet restrictions that describe what can be consumed
        /// orally
        #[prost(message, repeated, tag = "4")]
        pub r#type: prost::alloc::vec::Vec<super::super::core::CodeableConcept>,
        /// Scheduled frequency of diet
        #[prost(message, repeated, tag = "5")]
        pub schedule: prost::alloc::vec::Vec<super::super::core::Timing>,
        #[prost(message, repeated, tag = "6")]
        pub nutrient: prost::alloc::vec::Vec<oral_diet::Nutrient>,
        #[prost(message, repeated, tag = "7")]
        pub texture: prost::alloc::vec::Vec<oral_diet::Texture>,
        /// The required consistency of fluids and liquids provided to the patient
        #[prost(message, repeated, tag = "8")]
        pub fluid_consistency_type: prost::alloc::vec::Vec<super::super::core::CodeableConcept>,
        /// Instructions or additional information about the oral diet
        #[prost(message, optional, tag = "9")]
        pub instruction: ::core::option::Option<super::super::core::String>,
    }
    /// Nested message and enum types in `OralDiet`.
    pub mod oral_diet {
        /// Required  nutrient modifications
        #[derive(Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Message)]
        pub struct Nutrient {
            /// Unique id for inter-element referencing
            #[prost(message, optional, tag = "1")]
            pub id: ::core::option::Option<super::super::super::core::String>,
            /// Additional content defined by implementations
            #[prost(message, repeated, tag = "2")]
            pub extension: prost::alloc::vec::Vec<super::super::super::core::Extension>,
            /// Extensions that cannot be ignored even if unrecognized
            #[prost(message, repeated, tag = "3")]
            pub modifier_extension: prost::alloc::vec::Vec<super::super::super::core::Extension>,
            /// Type of nutrient that is being modified
            #[prost(message, optional, tag = "4")]
            pub modifier: ::core::option::Option<super::super::super::core::CodeableConcept>,
            /// Quantity of the specified nutrient
            #[prost(message, optional, tag = "5")]
            pub amount: ::core::option::Option<super::super::super::core::SimpleQuantity>,
        }
        /// Required  texture modifications
        #[derive(Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Message)]
        pub struct Texture {
            /// Unique id for inter-element referencing
            #[prost(message, optional, tag = "1")]
            pub id: ::core::option::Option<super::super::super::core::String>,
            /// Additional content defined by implementations
            #[prost(message, repeated, tag = "2")]
            pub extension: prost::alloc::vec::Vec<super::super::super::core::Extension>,
            /// Extensions that cannot be ignored even if unrecognized
            #[prost(message, repeated, tag = "3")]
            pub modifier_extension: prost::alloc::vec::Vec<super::super::super::core::Extension>,
            /// Code to indicate how to alter the texture of the foods, e.g. pureed
            #[prost(message, optional, tag = "4")]
            pub modifier: ::core::option::Option<super::super::super::core::CodeableConcept>,
            /// Concepts that are used to identify an entity that is ingested for
            /// nutritional purposes
            #[prost(message, optional, tag = "5")]
            pub food_type: ::core::option::Option<super::super::super::core::CodeableConcept>,
        }
    }
    /// Supplement components
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct Supplement {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag = "1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag = "2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag = "3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Type of supplement product requested
        #[prost(message, optional, tag = "4")]
        pub r#type: ::core::option::Option<super::super::core::CodeableConcept>,
        /// Product or brand name of the nutritional supplement
        #[prost(message, optional, tag = "5")]
        pub product_name: ::core::option::Option<super::super::core::String>,
        /// Scheduled frequency of supplement
        #[prost(message, repeated, tag = "6")]
        pub schedule: prost::alloc::vec::Vec<super::super::core::Timing>,
        /// Amount of the nutritional supplement
        #[prost(message, optional, tag = "7")]
        pub quantity: ::core::option::Option<super::super::core::SimpleQuantity>,
        /// Instructions or additional information about the oral supplement
        #[prost(message, optional, tag = "8")]
        pub instruction: ::core::option::Option<super::super::core::String>,
    }
    /// Enteral formula components
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct EnteralFormula {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag = "1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag = "2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag = "3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Type of enteral or infant formula
        #[prost(message, optional, tag = "4")]
        pub base_formula_type: ::core::option::Option<super::super::core::CodeableConcept>,
        /// Product or brand name of the enteral or infant formula
        #[prost(message, optional, tag = "5")]
        pub base_formula_product_name: ::core::option::Option<super::super::core::String>,
        /// Type of modular component to add to the feeding
        #[prost(message, optional, tag = "6")]
        pub additive_type: ::core::option::Option<super::super::core::CodeableConcept>,
        /// Product or brand name of the modular additive
        #[prost(message, optional, tag = "7")]
        pub additive_product_name: ::core::option::Option<super::super::core::String>,
        /// Amount of energy per specified volume that is required
        #[prost(message, optional, tag = "8")]
        pub caloric_density: ::core::option::Option<super::super::core::SimpleQuantity>,
        /// How the formula should enter the patient's gastrointestinal tract
        #[prost(message, optional, tag = "9")]
        pub routeof_administration: ::core::option::Option<super::super::core::CodeableConcept>,
        #[prost(message, repeated, tag = "10")]
        pub administration: prost::alloc::vec::Vec<enteral_formula::Administration>,
        /// Upper limit on formula volume per unit of time
        #[prost(message, optional, tag = "11")]
        pub max_volume_to_deliver: ::core::option::Option<super::super::core::SimpleQuantity>,
        /// Formula feeding instructions expressed as text
        #[prost(message, optional, tag = "12")]
        pub administration_instruction: ::core::option::Option<super::super::core::String>,
    }
    /// Nested message and enum types in `EnteralFormula`.
    pub mod enteral_formula {
        /// Formula feeding instruction as structured data
        #[derive(Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Message)]
        pub struct Administration {
            /// Unique id for inter-element referencing
            #[prost(message, optional, tag = "1")]
            pub id: ::core::option::Option<super::super::super::core::String>,
            /// Additional content defined by implementations
            #[prost(message, repeated, tag = "2")]
            pub extension: prost::alloc::vec::Vec<super::super::super::core::Extension>,
            /// Extensions that cannot be ignored even if unrecognized
            #[prost(message, repeated, tag = "3")]
            pub modifier_extension: prost::alloc::vec::Vec<super::super::super::core::Extension>,
            /// Scheduled frequency of enteral feeding
            #[prost(message, optional, tag = "4")]
            pub schedule: ::core::option::Option<super::super::super::core::Timing>,
            /// The volume of formula to provide
            #[prost(message, optional, tag = "5")]
            pub quantity: ::core::option::Option<super::super::super::core::SimpleQuantity>,
            #[prost(message, optional, tag = "6")]
            pub rate: ::core::option::Option<administration::RateX>,
        }
        /// Nested message and enum types in `Administration`.
        pub mod administration {
            /// Speed with which the formula is provided per period of time
            #[derive(Serialize, Deserialize)]
            #[serde(rename_all = "camelCase")]
            #[derive(Clone, PartialEq, prost::Message)]
            pub struct RateX {
                #[prost(oneof = "rate_x::Choice", tags = "1, 2")]
                pub choice: ::core::option::Option<rate_x::Choice>,
            }
            /// Nested message and enum types in `RateX`.
            pub mod rate_x {
                #[derive(Serialize, Deserialize)]
                #[serde(rename_all = "camelCase")]
                #[derive(Clone, PartialEq, prost::Oneof)]
                pub enum Choice {
                    #[prost(message, tag = "1")]
                    Quantity(super::super::super::super::super::core::SimpleQuantity),
                    #[prost(message, tag = "2")]
                    Ratio(super::super::super::super::super::core::Ratio),
                }
            }
        }
    }
}
/// Auto-generated from StructureDefinition for QICoreObservation.
/// Measurements and simple assertions.
/// See <http://hl7.org/fhir/us/qicore/StructureDefinition/qicore-observation>
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct QiCoreObservation {
    /// Logical id of this artifact
    #[prost(message, optional, tag = "1")]
    pub id: ::core::option::Option<super::core::Id>,
    /// Metadata about the resource
    #[prost(message, optional, tag = "2")]
    pub meta: ::core::option::Option<super::core::Meta>,
    /// A set of rules under which this content was created
    #[prost(message, optional, tag = "3")]
    pub implicit_rules: ::core::option::Option<super::core::Uri>,
    /// Language of the resource content
    #[prost(message, optional, tag = "4")]
    pub language: ::core::option::Option<super::core::Code>,
    /// Text summary of the resource, for human interpretation
    #[prost(message, optional, tag = "5")]
    pub text: ::core::option::Option<super::core::Narrative>,
    /// Contained, inline Resources
    #[prost(message, repeated, tag = "6")]
    pub contained: prost::alloc::vec::Vec<::prost_wkt_types::Any>,
    /// Extension
    #[prost(message, repeated, tag = "8")]
    pub extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// Extensions that cannot be ignored
    #[prost(message, repeated, tag = "9")]
    pub modifier_extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// Business Identifier for observation
    #[prost(message, repeated, tag = "10")]
    pub identifier: prost::alloc::vec::Vec<super::core::Identifier>,
    /// Fulfills plan, proposal or order
    #[prost(message, repeated, tag = "11")]
    pub based_on: prost::alloc::vec::Vec<super::core::Reference>,
    /// Part of referenced event
    #[prost(message, repeated, tag = "12")]
    pub part_of: prost::alloc::vec::Vec<super::core::Reference>,
    #[prost(message, optional, tag = "13")]
    pub status: ::core::option::Option<qi_core_observation::StatusCode>,
    /// Classification of  type of observation
    #[prost(message, repeated, tag = "14")]
    pub category: prost::alloc::vec::Vec<super::core::CodeableConcept>,
    /// Type of observation (code / type)
    #[prost(message, optional, tag = "15")]
    pub code: ::core::option::Option<super::core::CodeableConcept>,
    /// Who and/or what the observation is about
    #[prost(message, optional, tag = "16")]
    pub subject: ::core::option::Option<super::core::Reference>,
    /// What the observation is about, when it is not about the subject of record
    #[prost(message, repeated, tag = "17")]
    pub focus: prost::alloc::vec::Vec<super::core::Reference>,
    /// Healthcare event during which this observation is made
    #[prost(message, optional, tag = "18")]
    pub encounter: ::core::option::Option<super::core::Reference>,
    #[prost(message, optional, tag = "19")]
    pub effective: ::core::option::Option<qi_core_observation::EffectiveX>,
    /// Date/Time this version was made available
    #[prost(message, optional, tag = "20")]
    pub issued: ::core::option::Option<super::core::Instant>,
    /// Who is responsible for the observation
    #[prost(message, repeated, tag = "21")]
    pub performer: prost::alloc::vec::Vec<super::core::Reference>,
    #[prost(message, optional, tag = "22")]
    pub value: ::core::option::Option<qi_core_observation::ValueX>,
    /// Why the result is missing
    #[prost(message, optional, tag = "23")]
    pub data_absent_reason: ::core::option::Option<super::core::CodeableConcept>,
    /// High, low, normal, etc.
    #[prost(message, repeated, tag = "24")]
    pub interpretation: prost::alloc::vec::Vec<super::core::CodeableConcept>,
    /// Comments about the observation
    #[prost(message, repeated, tag = "25")]
    pub note: prost::alloc::vec::Vec<super::core::Annotation>,
    /// Observed body part
    #[prost(message, optional, tag = "26")]
    pub body_site: ::core::option::Option<super::core::CodeableConcept>,
    /// How it was done
    #[prost(message, optional, tag = "27")]
    pub method: ::core::option::Option<super::core::CodeableConcept>,
    /// Specimen used for this observation
    #[prost(message, optional, tag = "28")]
    pub specimen: ::core::option::Option<super::core::Reference>,
    /// (Measurement) Device
    #[prost(message, optional, tag = "29")]
    pub device: ::core::option::Option<super::core::Reference>,
    #[prost(message, repeated, tag = "30")]
    pub reference_range: prost::alloc::vec::Vec<qi_core_observation::ReferenceRange>,
    /// Related resource that belongs to the Observation group
    #[prost(message, repeated, tag = "31")]
    pub has_member: prost::alloc::vec::Vec<super::core::Reference>,
    /// Related measurements the observation is made from
    #[prost(message, repeated, tag = "32")]
    pub derived_from: prost::alloc::vec::Vec<super::core::Reference>,
    #[prost(message, repeated, tag = "33")]
    pub component: prost::alloc::vec::Vec<qi_core_observation::Component>,
    /// The body position during the observation
    #[prost(message, optional, tag = "34")]
    pub body_position: ::core::option::Option<super::core::CodeableConcept>,
    /// Qualitative change or trend in the measurement
    #[prost(message, optional, tag = "35")]
    pub delta: ::core::option::Option<super::core::CodeableConcept>,
}
/// Nested message and enum types in `QICoreObservation`.
pub mod qi_core_observation {
    /// registered | preliminary | final | amended +
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct StatusCode {
        #[prost(
            enumeration = "super::super::core::observation_status_code::Value",
            tag = "1"
        )]
        pub value: i32,
        #[prost(message, optional, tag = "2")]
        pub id: ::core::option::Option<super::super::core::String>,
        #[prost(message, repeated, tag = "3")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
    }
    /// Clinically relevant time/time-period for observation
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct EffectiveX {
        #[prost(oneof = "effective_x::Choice", tags = "1, 2, 3, 4")]
        pub choice: ::core::option::Option<effective_x::Choice>,
    }
    /// Nested message and enum types in `EffectiveX`.
    pub mod effective_x {
        #[derive(Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Oneof)]
        pub enum Choice {
            #[prost(message, tag = "1")]
            DateTime(super::super::super::core::DateTime),
            #[prost(message, tag = "2")]
            Period(super::super::super::core::Period),
            #[prost(message, tag = "3")]
            Timing(super::super::super::core::Timing),
            #[prost(message, tag = "4")]
            Instant(super::super::super::core::Instant),
        }
    }
    /// Actual result
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct ValueX {
        #[prost(oneof = "value_x::Choice", tags = "1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11")]
        pub choice: ::core::option::Option<value_x::Choice>,
    }
    /// Nested message and enum types in `ValueX`.
    pub mod value_x {
        #[derive(Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Oneof)]
        pub enum Choice {
            #[prost(message, tag = "1")]
            Quantity(super::super::super::core::Quantity),
            #[prost(message, tag = "2")]
            CodeableConcept(super::super::super::core::CodeableConcept),
            #[prost(message, tag = "3")]
            StringValue(super::super::super::core::String),
            #[prost(message, tag = "4")]
            Boolean(super::super::super::core::Boolean),
            #[prost(message, tag = "5")]
            Integer(super::super::super::core::Integer),
            #[prost(message, tag = "6")]
            Range(super::super::super::core::Range),
            #[prost(message, tag = "7")]
            Ratio(super::super::super::core::Ratio),
            #[prost(message, tag = "8")]
            SampledData(super::super::super::core::SampledData),
            #[prost(message, tag = "9")]
            Time(super::super::super::core::Time),
            #[prost(message, tag = "10")]
            DateTime(super::super::super::core::DateTime),
            #[prost(message, tag = "11")]
            Period(super::super::super::core::Period),
        }
    }
    /// Provides guide for interpretation
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct ReferenceRange {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag = "1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag = "2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag = "3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Low Range, if relevant
        #[prost(message, optional, tag = "4")]
        pub low: ::core::option::Option<super::super::core::SimpleQuantity>,
        /// High Range, if relevant
        #[prost(message, optional, tag = "5")]
        pub high: ::core::option::Option<super::super::core::SimpleQuantity>,
        /// Reference range qualifier
        #[prost(message, optional, tag = "6")]
        pub r#type: ::core::option::Option<super::super::core::CodeableConcept>,
        /// Reference range population
        #[prost(message, repeated, tag = "7")]
        pub applies_to: prost::alloc::vec::Vec<super::super::core::CodeableConcept>,
        /// Applicable age range, if relevant
        #[prost(message, optional, tag = "8")]
        pub age: ::core::option::Option<super::super::core::Range>,
        /// Text based reference range in an observation
        #[prost(message, optional, tag = "9")]
        pub text: ::core::option::Option<super::super::core::String>,
    }
    /// Component results
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct Component {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag = "1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag = "2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag = "3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Type of component observation (code / type)
        #[prost(message, optional, tag = "4")]
        pub code: ::core::option::Option<super::super::core::CodeableConcept>,
        #[prost(message, optional, tag = "5")]
        pub value: ::core::option::Option<component::ValueX>,
        /// Why the component result is missing
        #[prost(message, optional, tag = "6")]
        pub data_absent_reason: ::core::option::Option<super::super::core::CodeableConcept>,
        /// High, low, normal, etc.
        #[prost(message, repeated, tag = "7")]
        pub interpretation: prost::alloc::vec::Vec<super::super::core::CodeableConcept>,
        /// Provides guide for interpretation of component result
        #[prost(message, repeated, tag = "8")]
        pub reference_range: prost::alloc::vec::Vec<ReferenceRange>,
    }
    /// Nested message and enum types in `Component`.
    pub mod component {
        /// Actual component result
        #[derive(Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Message)]
        pub struct ValueX {
            #[prost(oneof = "value_x::Choice", tags = "1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11")]
            pub choice: ::core::option::Option<value_x::Choice>,
        }
        /// Nested message and enum types in `ValueX`.
        pub mod value_x {
            #[derive(Serialize, Deserialize)]
            #[serde(rename_all = "camelCase")]
            #[derive(Clone, PartialEq, prost::Oneof)]
            pub enum Choice {
                #[prost(message, tag = "1")]
                Quantity(super::super::super::super::core::Quantity),
                #[prost(message, tag = "2")]
                CodeableConcept(super::super::super::super::core::CodeableConcept),
                #[prost(message, tag = "3")]
                StringValue(super::super::super::super::core::String),
                #[prost(message, tag = "4")]
                Boolean(super::super::super::super::core::Boolean),
                #[prost(message, tag = "5")]
                Integer(super::super::super::super::core::Integer),
                #[prost(message, tag = "6")]
                Range(super::super::super::super::core::Range),
                #[prost(message, tag = "7")]
                Ratio(super::super::super::super::core::Ratio),
                #[prost(message, tag = "8")]
                SampledData(super::super::super::super::core::SampledData),
                #[prost(message, tag = "9")]
                Time(super::super::super::super::core::Time),
                #[prost(message, tag = "10")]
                DateTime(super::super::super::super::core::DateTime),
                #[prost(message, tag = "11")]
                Period(super::super::super::super::core::Period),
            }
        }
    }
}
/// Auto-generated from StructureDefinition for QICoreOrganization.
/// A grouping of people or organizations with a common purpose.
/// See <http://hl7.org/fhir/us/qicore/StructureDefinition/qicore-organization>
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct QiCoreOrganization {
    /// Logical id of this artifact
    #[prost(message, optional, tag = "1")]
    pub id: ::core::option::Option<super::core::Id>,
    /// Metadata about the resource
    #[prost(message, optional, tag = "2")]
    pub meta: ::core::option::Option<super::core::Meta>,
    /// A set of rules under which this content was created
    #[prost(message, optional, tag = "3")]
    pub implicit_rules: ::core::option::Option<super::core::Uri>,
    /// Language of the resource content
    #[prost(message, optional, tag = "4")]
    pub language: ::core::option::Option<super::core::Code>,
    /// Text summary of the resource, for human interpretation
    #[prost(message, optional, tag = "5")]
    pub text: ::core::option::Option<super::core::Narrative>,
    /// Contained, inline Resources
    #[prost(message, repeated, tag = "6")]
    pub contained: prost::alloc::vec::Vec<::prost_wkt_types::Any>,
    /// Additional content defined by implementations
    #[prost(message, repeated, tag = "8")]
    pub extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// Extensions that cannot be ignored
    #[prost(message, repeated, tag = "9")]
    pub modifier_extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// Identifies this organization  across multiple systems
    #[prost(message, repeated, tag = "10")]
    pub identifier: prost::alloc::vec::Vec<super::core::Identifier>,
    /// Whether the organization's record is still in active use
    #[prost(message, optional, tag = "11")]
    pub active: ::core::option::Option<super::core::Boolean>,
    /// Kind of organization
    #[prost(message, repeated, tag = "12")]
    pub r#type: prost::alloc::vec::Vec<super::core::CodeableConcept>,
    /// Name used for the organization
    #[prost(message, optional, tag = "13")]
    pub name: ::core::option::Option<super::core::String>,
    /// A list of alternate names that the organization is known as, or was known
    /// as in the past
    #[prost(message, repeated, tag = "14")]
    pub alias: prost::alloc::vec::Vec<super::core::String>,
    /// A contact detail for the organization
    #[prost(message, repeated, tag = "15")]
    pub telecom: prost::alloc::vec::Vec<super::core::ContactPoint>,
    /// An address for the organization
    #[prost(message, repeated, tag = "16")]
    pub address: prost::alloc::vec::Vec<super::core::Address>,
    /// The organization of which this organization forms a part
    #[prost(message, optional, tag = "17")]
    pub part_of: ::core::option::Option<super::core::Reference>,
    #[prost(message, repeated, tag = "18")]
    pub contact: prost::alloc::vec::Vec<qi_core_organization::Contact>,
    /// Technical endpoints providing access to services operated for the
    /// organization
    #[prost(message, repeated, tag = "19")]
    pub endpoint: prost::alloc::vec::Vec<super::core::Reference>,
}
/// Nested message and enum types in `QICoreOrganization`.
pub mod qi_core_organization {
    /// Contact for the organization for a certain purpose
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct Contact {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag = "1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag = "2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag = "3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// The type of contact
        #[prost(message, optional, tag = "4")]
        pub purpose: ::core::option::Option<super::super::core::CodeableConcept>,
        /// A name associated with the contact
        #[prost(message, optional, tag = "5")]
        pub name: ::core::option::Option<super::super::core::HumanName>,
        /// Contact details (telephone, email, etc.)  for a contact
        #[prost(message, repeated, tag = "6")]
        pub telecom: prost::alloc::vec::Vec<super::super::core::ContactPoint>,
        /// Visiting or postal addresses for the contact
        #[prost(message, optional, tag = "7")]
        pub address: ::core::option::Option<super::super::core::Address>,
    }
}
/// Auto-generated from StructureDefinition for QICorePatient.
/// Information about an individual or animal receiving health care services.
/// See <http://hl7.org/fhir/us/qicore/StructureDefinition/qicore-patient>
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct QiCorePatient {
    /// Logical id of this artifact
    #[prost(message, optional, tag = "1")]
    pub id: ::core::option::Option<super::core::Id>,
    /// Metadata about the resource
    #[prost(message, optional, tag = "2")]
    pub meta: ::core::option::Option<super::core::Meta>,
    /// A set of rules under which this content was created
    #[prost(message, optional, tag = "3")]
    pub implicit_rules: ::core::option::Option<super::core::Uri>,
    /// Language of the resource content
    #[prost(message, optional, tag = "4")]
    pub language: ::core::option::Option<super::core::Code>,
    /// Text summary of the resource, for human interpretation
    #[prost(message, optional, tag = "5")]
    pub text: ::core::option::Option<super::core::Narrative>,
    /// Contained, inline Resources
    #[prost(message, repeated, tag = "6")]
    pub contained: prost::alloc::vec::Vec<::prost_wkt_types::Any>,
    /// Extension
    #[prost(message, repeated, tag = "8")]
    pub extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// Extensions that cannot be ignored
    #[prost(message, repeated, tag = "9")]
    pub modifier_extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// An identifier for this patient
    #[prost(message, repeated, tag = "10")]
    pub identifier: prost::alloc::vec::Vec<super::core::Identifier>,
    /// Whether this patient's record is in active use
    #[prost(message, optional, tag = "11")]
    pub active: ::core::option::Option<super::core::Boolean>,
    /// A name associated with the patient
    #[prost(message, repeated, tag = "12")]
    pub name: prost::alloc::vec::Vec<super::core::HumanName>,
    /// A contact detail for the individual
    #[prost(message, repeated, tag = "13")]
    pub telecom: prost::alloc::vec::Vec<super::core::ContactPoint>,
    #[prost(message, optional, tag = "14")]
    pub gender: ::core::option::Option<qi_core_patient::GenderCode>,
    /// The date of birth for the individual
    #[prost(message, optional, tag = "15")]
    pub birth_date: ::core::option::Option<super::core::Date>,
    #[prost(message, optional, tag = "16")]
    pub deceased: ::core::option::Option<qi_core_patient::DeceasedX>,
    /// An address for the individual
    #[prost(message, repeated, tag = "17")]
    pub address: prost::alloc::vec::Vec<super::core::Address>,
    /// Marital (civil) status of a patient
    #[prost(message, optional, tag = "18")]
    pub marital_status: ::core::option::Option<super::core::CodeableConcept>,
    #[prost(message, optional, tag = "19")]
    pub multiple_birth: ::core::option::Option<qi_core_patient::MultipleBirthX>,
    /// Image of the patient
    #[prost(message, repeated, tag = "20")]
    pub photo: prost::alloc::vec::Vec<super::core::Attachment>,
    #[prost(message, repeated, tag = "21")]
    pub contact: prost::alloc::vec::Vec<qi_core_patient::Contact>,
    #[prost(message, repeated, tag = "22")]
    pub communication: prost::alloc::vec::Vec<qi_core_patient::Communication>,
    /// Patient's nominated primary care provider
    #[prost(message, repeated, tag = "23")]
    pub general_practitioner: prost::alloc::vec::Vec<super::core::Reference>,
    /// Organization that is the custodian of the patient record
    #[prost(message, optional, tag = "24")]
    pub managing_organization: ::core::option::Option<super::core::Reference>,
    #[prost(message, repeated, tag = "25")]
    pub link: prost::alloc::vec::Vec<qi_core_patient::Link>,
    /// Extension
    #[prost(message, optional, tag = "26")]
    pub race: ::core::option::Option<super::uscore::PatientUsCoreRaceExtension>,
    /// Extension
    #[prost(message, optional, tag = "27")]
    pub ethnicity: ::core::option::Option<super::uscore::PatientUsCoreEthnicityExtension>,
    /// Extension
    #[prost(message, optional, tag = "28")]
    pub birthsex:
        ::core::option::Option<super::uscore::patient_us_core_birth_sex_extension::ValueCode>,
    /// The patient's professed religious affiliations
    #[prost(message, optional, tag = "29")]
    pub religion: ::core::option::Option<super::core::CodeableConcept>,
    /// Place of Birth for patient
    #[prost(message, optional, tag = "30")]
    pub birth_place: ::core::option::Option<super::core::Address>,
    /// Condition(s) limiting movement, senses, or activities
    #[prost(message, repeated, tag = "31")]
    pub disability: prost::alloc::vec::Vec<super::core::CodeableConcept>,
    /// Nationality
    #[prost(message, repeated, tag = "32")]
    pub nationality: prost::alloc::vec::Vec<super::core::PatientNationality>,
    /// Post-mortem donor status
    #[prost(message, optional, tag = "33")]
    pub cadaveric_donor: ::core::option::Option<super::core::Boolean>,
    /// Explanation associated with refuted status
    #[prost(message, optional, tag = "34")]
    pub military_service: ::core::option::Option<super::core::CodeableConcept>,
    /// Time of day of birth
    #[prost(message, optional, tag = "35")]
    pub birth_time: ::core::option::Option<super::core::DateTime>,
}
/// Nested message and enum types in `QICorePatient`.
pub mod qi_core_patient {
    /// male | female | other | unknown
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct GenderCode {
        #[prost(
            enumeration = "super::super::core::administrative_gender_code::Value",
            tag = "1"
        )]
        pub value: i32,
        #[prost(message, optional, tag = "2")]
        pub id: ::core::option::Option<super::super::core::String>,
        #[prost(message, repeated, tag = "3")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
    }
    /// Indicates if the individual is deceased or not
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct DeceasedX {
        #[prost(oneof = "deceased_x::Choice", tags = "1, 2")]
        pub choice: ::core::option::Option<deceased_x::Choice>,
    }
    /// Nested message and enum types in `DeceasedX`.
    pub mod deceased_x {
        #[derive(Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Oneof)]
        pub enum Choice {
            #[prost(message, tag = "1")]
            Boolean(super::super::super::core::Boolean),
            #[prost(message, tag = "2")]
            DateTime(super::super::super::core::DateTime),
        }
    }
    /// Whether patient is part of a multiple birth
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct MultipleBirthX {
        #[prost(oneof = "multiple_birth_x::Choice", tags = "1, 2")]
        pub choice: ::core::option::Option<multiple_birth_x::Choice>,
    }
    /// Nested message and enum types in `MultipleBirthX`.
    pub mod multiple_birth_x {
        #[derive(Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Oneof)]
        pub enum Choice {
            #[prost(message, tag = "1")]
            Boolean(super::super::super::core::Boolean),
            #[prost(message, tag = "2")]
            Integer(super::super::super::core::Integer),
        }
    }
    /// A contact party (e.g. guardian, partner, friend) for the patient
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct Contact {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag = "1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag = "2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag = "3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// The kind of relationship
        #[prost(message, repeated, tag = "4")]
        pub relationship: prost::alloc::vec::Vec<super::super::core::CodeableConcept>,
        /// A name associated with the contact person
        #[prost(message, optional, tag = "5")]
        pub name: ::core::option::Option<super::super::core::HumanName>,
        /// A contact detail for the person
        #[prost(message, repeated, tag = "6")]
        pub telecom: prost::alloc::vec::Vec<super::super::core::ContactPoint>,
        /// Address for the contact person
        #[prost(message, optional, tag = "7")]
        pub address: ::core::option::Option<super::super::core::Address>,
        #[prost(message, optional, tag = "8")]
        pub gender: ::core::option::Option<contact::GenderCode>,
        /// Organization that is associated with the contact
        #[prost(message, optional, tag = "9")]
        pub organization: ::core::option::Option<super::super::core::Reference>,
        /// The period during which this contact person or organization is valid to
        /// be contacted relating to this patient
        #[prost(message, optional, tag = "10")]
        pub period: ::core::option::Option<super::super::core::Period>,
    }
    /// Nested message and enum types in `Contact`.
    pub mod contact {
        /// male | female | other | unknown
        #[derive(Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Message)]
        pub struct GenderCode {
            #[prost(
                enumeration = "super::super::super::core::administrative_gender_code::Value",
                tag = "1"
            )]
            pub value: i32,
            #[prost(message, optional, tag = "2")]
            pub id: ::core::option::Option<super::super::super::core::String>,
            #[prost(message, repeated, tag = "3")]
            pub extension: prost::alloc::vec::Vec<super::super::super::core::Extension>,
        }
    }
    /// A language which may be used to communicate with the patient about his or
    /// her health
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct Communication {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag = "1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag = "2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag = "3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// The language which can be used to communicate with the patient about his
        /// or her health
        #[prost(message, optional, tag = "4")]
        pub language: ::core::option::Option<super::super::core::CodeableConcept>,
        /// Language preference indicator
        #[prost(message, optional, tag = "5")]
        pub preferred: ::core::option::Option<super::super::core::Boolean>,
    }
    /// Link to another patient resource that concerns the same actual person
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct Link {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag = "1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag = "2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag = "3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// The other patient or related person resource that the link refers to
        #[prost(message, optional, tag = "4")]
        pub other: ::core::option::Option<super::super::core::Reference>,
        #[prost(message, optional, tag = "5")]
        pub r#type: ::core::option::Option<link::TypeCode>,
    }
    /// Nested message and enum types in `Link`.
    pub mod link {
        /// replaced-by | replaces | refer | seealso
        #[derive(Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Message)]
        pub struct TypeCode {
            #[prost(
                enumeration = "super::super::super::core::link_type_code::Value",
                tag = "1"
            )]
            pub value: i32,
            #[prost(message, optional, tag = "2")]
            pub id: ::core::option::Option<super::super::super::core::String>,
            #[prost(message, repeated, tag = "3")]
            pub extension: prost::alloc::vec::Vec<super::super::super::core::Extension>,
        }
    }
}
/// Auto-generated from StructureDefinition for QICorePractitioner.
/// A person with a  formal responsibility in the provisioning of healthcare or
/// related services. See
/// <http://hl7.org/fhir/us/qicore/StructureDefinition/qicore-practitioner>
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct QiCorePractitioner {
    /// Logical id of this artifact
    #[prost(message, optional, tag = "1")]
    pub id: ::core::option::Option<super::core::Id>,
    /// Metadata about the resource
    #[prost(message, optional, tag = "2")]
    pub meta: ::core::option::Option<super::core::Meta>,
    /// A set of rules under which this content was created
    #[prost(message, optional, tag = "3")]
    pub implicit_rules: ::core::option::Option<super::core::Uri>,
    /// Language of the resource content
    #[prost(message, optional, tag = "4")]
    pub language: ::core::option::Option<super::core::Code>,
    /// Text summary of the resource, for human interpretation
    #[prost(message, optional, tag = "5")]
    pub text: ::core::option::Option<super::core::Narrative>,
    /// Contained, inline Resources
    #[prost(message, repeated, tag = "6")]
    pub contained: prost::alloc::vec::Vec<::prost_wkt_types::Any>,
    /// Additional content defined by implementations
    #[prost(message, repeated, tag = "8")]
    pub extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// Extensions that cannot be ignored
    #[prost(message, repeated, tag = "9")]
    pub modifier_extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// An identifier for the person as this agent
    #[prost(message, repeated, tag = "10")]
    pub identifier: prost::alloc::vec::Vec<super::core::Identifier>,
    /// Whether this practitioner's record is in active use
    #[prost(message, optional, tag = "11")]
    pub active: ::core::option::Option<super::core::Boolean>,
    /// The name(s) associated with the practitioner
    #[prost(message, optional, tag = "12")]
    pub name: ::core::option::Option<super::core::HumanName>,
    /// A contact detail for the practitioner (that apply to all roles)
    #[prost(message, repeated, tag = "13")]
    pub telecom: prost::alloc::vec::Vec<super::core::ContactPoint>,
    /// Address(es) of the practitioner that are not role specific (typically home
    /// address)
    #[prost(message, repeated, tag = "14")]
    pub address: prost::alloc::vec::Vec<super::core::Address>,
    #[prost(message, optional, tag = "15")]
    pub gender: ::core::option::Option<qi_core_practitioner::GenderCode>,
    /// The date  on which the practitioner was born
    #[prost(message, optional, tag = "16")]
    pub birth_date: ::core::option::Option<super::core::Date>,
    /// Image of the person
    #[prost(message, repeated, tag = "17")]
    pub photo: prost::alloc::vec::Vec<super::core::Attachment>,
    #[prost(message, repeated, tag = "18")]
    pub qualification: prost::alloc::vec::Vec<qi_core_practitioner::Qualification>,
    /// A language the practitioner can use in patient communication
    #[prost(message, repeated, tag = "19")]
    pub communication: prost::alloc::vec::Vec<super::core::CodeableConcept>,
}
/// Nested message and enum types in `QICorePractitioner`.
pub mod qi_core_practitioner {
    /// male | female | other | unknown
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct GenderCode {
        #[prost(
            enumeration = "super::super::core::administrative_gender_code::Value",
            tag = "1"
        )]
        pub value: i32,
        #[prost(message, optional, tag = "2")]
        pub id: ::core::option::Option<super::super::core::String>,
        #[prost(message, repeated, tag = "3")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
    }
    /// Certification, licenses, or training pertaining to the provision of care
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct Qualification {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag = "1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag = "2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag = "3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// An identifier for this qualification for the practitioner
        #[prost(message, repeated, tag = "4")]
        pub identifier: prost::alloc::vec::Vec<super::super::core::Identifier>,
        /// Coded representation of the qualification
        #[prost(message, optional, tag = "5")]
        pub code: ::core::option::Option<super::super::core::CodeableConcept>,
        /// Period during which the qualification is valid
        #[prost(message, optional, tag = "6")]
        pub period: ::core::option::Option<super::super::core::Period>,
        /// Organization that regulates and issues the qualification
        #[prost(message, optional, tag = "7")]
        pub issuer: ::core::option::Option<super::super::core::Reference>,
    }
}
/// Auto-generated from StructureDefinition for QICorePractitionerRole.
/// Roles/organizations the practitioner is associated with.
/// See <http://hl7.org/fhir/us/qicore/StructureDefinition/qicore-practitionerrole>
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct QiCorePractitionerRole {
    /// Logical id of this artifact
    #[prost(message, optional, tag = "1")]
    pub id: ::core::option::Option<super::core::Id>,
    /// Metadata about the resource
    #[prost(message, optional, tag = "2")]
    pub meta: ::core::option::Option<super::core::Meta>,
    /// A set of rules under which this content was created
    #[prost(message, optional, tag = "3")]
    pub implicit_rules: ::core::option::Option<super::core::Uri>,
    /// Language of the resource content
    #[prost(message, optional, tag = "4")]
    pub language: ::core::option::Option<super::core::Code>,
    /// Text summary of the resource, for human interpretation
    #[prost(message, optional, tag = "5")]
    pub text: ::core::option::Option<super::core::Narrative>,
    /// Contained, inline Resources
    #[prost(message, repeated, tag = "6")]
    pub contained: prost::alloc::vec::Vec<::prost_wkt_types::Any>,
    /// Additional content defined by implementations
    #[prost(message, repeated, tag = "8")]
    pub extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// Extensions that cannot be ignored
    #[prost(message, repeated, tag = "9")]
    pub modifier_extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// The list of healthcare services that this worker provides for this role's
    /// Organization/Location(s)
    #[prost(message, repeated, tag = "10")]
    pub healthcare_service: prost::alloc::vec::Vec<super::core::Reference>,
    /// Contact details that are specific to the role/location/service
    #[prost(message, repeated, tag = "11")]
    pub telecom: prost::alloc::vec::Vec<super::core::ContactPoint>,
    #[prost(message, repeated, tag = "12")]
    pub available_time: prost::alloc::vec::Vec<qi_core_practitioner_role::AvailableTime>,
    #[prost(message, repeated, tag = "13")]
    pub not_available: prost::alloc::vec::Vec<qi_core_practitioner_role::NotAvailable>,
    /// Description of availability exceptions
    #[prost(message, optional, tag = "14")]
    pub availability_exceptions: ::core::option::Option<super::core::String>,
    /// Technical endpoints providing access to services operated for the
    /// practitioner with this role
    #[prost(message, repeated, tag = "15")]
    pub endpoint: prost::alloc::vec::Vec<super::core::Reference>,
}
/// Nested message and enum types in `QICorePractitionerRole`.
pub mod qi_core_practitioner_role {
    /// Times the Service Site is available
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct AvailableTime {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag = "1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag = "2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag = "3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        #[prost(message, repeated, tag = "4")]
        pub days_of_week: prost::alloc::vec::Vec<available_time::DaysOfWeekCode>,
        /// Always available? e.g. 24 hour service
        #[prost(message, optional, tag = "5")]
        pub all_day: ::core::option::Option<super::super::core::Boolean>,
        /// Opening time of day (ignored if allDay = true)
        #[prost(message, optional, tag = "6")]
        pub available_start_time: ::core::option::Option<super::super::core::Time>,
        /// Closing time of day (ignored if allDay = true)
        #[prost(message, optional, tag = "7")]
        pub available_end_time: ::core::option::Option<super::super::core::Time>,
    }
    /// Nested message and enum types in `AvailableTime`.
    pub mod available_time {
        /// mon | tue | wed | thu | fri | sat | sun
        #[derive(Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Message)]
        pub struct DaysOfWeekCode {
            #[prost(
                enumeration = "super::super::super::core::days_of_week_code::Value",
                tag = "1"
            )]
            pub value: i32,
            #[prost(message, optional, tag = "2")]
            pub id: ::core::option::Option<super::super::super::core::String>,
            #[prost(message, repeated, tag = "3")]
            pub extension: prost::alloc::vec::Vec<super::super::super::core::Extension>,
        }
    }
    /// Not available during this time due to provided reason
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct NotAvailable {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag = "1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag = "2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag = "3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Reason presented to the user explaining why time not available
        #[prost(message, optional, tag = "4")]
        pub description: ::core::option::Option<super::super::core::String>,
        /// Service not available from this date
        #[prost(message, optional, tag = "5")]
        pub during: ::core::option::Option<super::super::core::Period>,
    }
}
/// Auto-generated from StructureDefinition for QICoreProcedure.
/// An action that is being or was performed on a patient.
/// See <http://hl7.org/fhir/us/qicore/StructureDefinition/qicore-procedure>
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct QiCoreProcedure {
    /// Logical id of this artifact
    #[prost(message, optional, tag = "1")]
    pub id: ::core::option::Option<super::core::Id>,
    /// Metadata about the resource
    #[prost(message, optional, tag = "2")]
    pub meta: ::core::option::Option<super::core::Meta>,
    /// A set of rules under which this content was created
    #[prost(message, optional, tag = "3")]
    pub implicit_rules: ::core::option::Option<super::core::Uri>,
    /// Language of the resource content
    #[prost(message, optional, tag = "4")]
    pub language: ::core::option::Option<super::core::Code>,
    /// Text summary of the resource, for human interpretation
    #[prost(message, optional, tag = "5")]
    pub text: ::core::option::Option<super::core::Narrative>,
    /// Contained, inline Resources
    #[prost(message, repeated, tag = "6")]
    pub contained: prost::alloc::vec::Vec<::prost_wkt_types::Any>,
    /// Extension
    #[prost(message, repeated, tag = "8")]
    pub extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// Extensions that cannot be ignored
    #[prost(message, repeated, tag = "9")]
    pub modifier_extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// External Identifiers for this procedure
    #[prost(message, repeated, tag = "10")]
    pub identifier: prost::alloc::vec::Vec<super::core::Identifier>,
    /// Instantiates FHIR protocol or definition
    #[prost(message, repeated, tag = "11")]
    pub instantiates_canonical: prost::alloc::vec::Vec<super::core::Canonical>,
    /// Instantiates external protocol or definition
    #[prost(message, repeated, tag = "12")]
    pub instantiates_uri: prost::alloc::vec::Vec<super::core::Uri>,
    /// A request for this procedure
    #[prost(message, repeated, tag = "13")]
    pub based_on: prost::alloc::vec::Vec<super::core::Reference>,
    /// Part of referenced event
    #[prost(message, repeated, tag = "14")]
    pub part_of: prost::alloc::vec::Vec<super::core::Reference>,
    #[prost(message, optional, tag = "15")]
    pub status: ::core::option::Option<qi_core_procedure::StatusCode>,
    /// Reason for current status
    #[prost(message, optional, tag = "16")]
    pub status_reason: ::core::option::Option<super::core::CodeableConcept>,
    /// Classification of the procedure
    #[prost(message, optional, tag = "17")]
    pub category: ::core::option::Option<super::core::CodeableConcept>,
    /// SNOMED-CT | ICD-10 | CPT-4
    #[prost(message, optional, tag = "18")]
    pub code: ::core::option::Option<super::core::CodeableConcept>,
    /// Who the procedure was performed on
    #[prost(message, optional, tag = "19")]
    pub subject: ::core::option::Option<super::core::Reference>,
    /// Encounter created as part of
    #[prost(message, optional, tag = "20")]
    pub encounter: ::core::option::Option<super::core::Reference>,
    #[prost(message, optional, tag = "21")]
    pub performed: ::core::option::Option<qi_core_procedure::PerformedX>,
    /// Who recorded the procedure
    #[prost(message, optional, tag = "22")]
    pub recorder: ::core::option::Option<super::core::Reference>,
    /// Person who asserts this procedure
    #[prost(message, optional, tag = "23")]
    pub asserter: ::core::option::Option<super::core::Reference>,
    #[prost(message, repeated, tag = "24")]
    pub performer: prost::alloc::vec::Vec<qi_core_procedure::Performer>,
    /// Where the procedure happened
    #[prost(message, optional, tag = "25")]
    pub location: ::core::option::Option<super::core::Reference>,
    /// Coded reason procedure performed
    #[prost(message, repeated, tag = "26")]
    pub reason_code: prost::alloc::vec::Vec<super::core::CodeableConcept>,
    /// The justification that the procedure was performed
    #[prost(message, repeated, tag = "27")]
    pub reason_reference: prost::alloc::vec::Vec<super::core::Reference>,
    /// Target body sites
    #[prost(message, repeated, tag = "28")]
    pub body_site: prost::alloc::vec::Vec<super::core::CodeableConcept>,
    /// The result of procedure
    #[prost(message, optional, tag = "29")]
    pub outcome: ::core::option::Option<super::core::CodeableConcept>,
    /// Any report resulting from the procedure
    #[prost(message, repeated, tag = "30")]
    pub report: prost::alloc::vec::Vec<super::core::Reference>,
    /// Complication following the procedure
    #[prost(message, repeated, tag = "31")]
    pub complication: prost::alloc::vec::Vec<super::core::CodeableConcept>,
    /// A condition that is a result of the procedure
    #[prost(message, repeated, tag = "32")]
    pub complication_detail: prost::alloc::vec::Vec<super::core::Reference>,
    /// Instructions for follow up
    #[prost(message, repeated, tag = "33")]
    pub follow_up: prost::alloc::vec::Vec<super::core::CodeableConcept>,
    /// Additional information about the procedure
    #[prost(message, repeated, tag = "34")]
    pub note: prost::alloc::vec::Vec<super::core::Annotation>,
    #[prost(message, repeated, tag = "35")]
    pub focal_device: prost::alloc::vec::Vec<qi_core_procedure::FocalDevice>,
    /// Items used during procedure
    #[prost(message, repeated, tag = "36")]
    pub used_reference: prost::alloc::vec::Vec<super::core::Reference>,
    /// Coded items used during the procedure
    #[prost(message, repeated, tag = "37")]
    pub used_code: prost::alloc::vec::Vec<super::core::CodeableConcept>,
    /// The access point or points used for this procedure
    #[prost(message, repeated, tag = "38")]
    pub approach_body_structure: prost::alloc::vec::Vec<super::core::Reference>,
    /// The first incision time
    #[prost(message, optional, tag = "39")]
    pub incision_date_time: ::core::option::Option<super::core::DateTime>,
}
/// Nested message and enum types in `QICoreProcedure`.
pub mod qi_core_procedure {
    /// preparation | in-progress | not-done | suspended | aborted | completed |
    /// entered-in-error | unknown
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct StatusCode {
        #[prost(
            enumeration = "super::super::core::event_status_code::Value",
            tag = "1"
        )]
        pub value: i32,
        #[prost(message, optional, tag = "2")]
        pub id: ::core::option::Option<super::super::core::String>,
        #[prost(message, repeated, tag = "3")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
    }
    /// When the procedure was performed
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct PerformedX {
        #[prost(oneof = "performed_x::Choice", tags = "1, 2")]
        pub choice: ::core::option::Option<performed_x::Choice>,
    }
    /// Nested message and enum types in `PerformedX`.
    pub mod performed_x {
        #[derive(Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Oneof)]
        pub enum Choice {
            #[prost(message, tag = "1")]
            DateTime(super::super::super::core::DateTime),
            #[prost(message, tag = "2")]
            Period(super::super::super::core::Period),
        }
    }
    /// The people who performed the procedure
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct Performer {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag = "1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag = "2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag = "3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Type of performance
        #[prost(message, optional, tag = "4")]
        pub function: ::core::option::Option<super::super::core::CodeableConcept>,
        /// The reference to the practitioner
        #[prost(message, optional, tag = "5")]
        pub actor: ::core::option::Option<super::super::core::Reference>,
        /// Organization the device or practitioner was acting for
        #[prost(message, optional, tag = "6")]
        pub on_behalf_of: ::core::option::Option<super::super::core::Reference>,
    }
    /// Manipulated, implanted, or removed device
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct FocalDevice {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag = "1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag = "2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag = "3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Kind of change to device
        #[prost(message, optional, tag = "4")]
        pub action: ::core::option::Option<super::super::core::CodeableConcept>,
        /// Device that was changed
        #[prost(message, optional, tag = "5")]
        pub manipulated: ::core::option::Option<super::super::core::Reference>,
    }
}
/// Auto-generated from StructureDefinition for QICoreRelatedPerson.
/// A person that is related to a patient, but who is not a direct target of
/// care. See
/// <http://hl7.org/fhir/us/qicore/StructureDefinition/qicore-relatedperson>
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct QiCoreRelatedPerson {
    /// Logical id of this artifact
    #[prost(message, optional, tag = "1")]
    pub id: ::core::option::Option<super::core::Id>,
    /// Metadata about the resource
    #[prost(message, optional, tag = "2")]
    pub meta: ::core::option::Option<super::core::Meta>,
    /// A set of rules under which this content was created
    #[prost(message, optional, tag = "3")]
    pub implicit_rules: ::core::option::Option<super::core::Uri>,
    /// Language of the resource content
    #[prost(message, optional, tag = "4")]
    pub language: ::core::option::Option<super::core::Code>,
    /// Text summary of the resource, for human interpretation
    #[prost(message, optional, tag = "5")]
    pub text: ::core::option::Option<super::core::Narrative>,
    /// Contained, inline Resources
    #[prost(message, repeated, tag = "6")]
    pub contained: prost::alloc::vec::Vec<::prost_wkt_types::Any>,
    /// Additional content defined by implementations
    #[prost(message, repeated, tag = "8")]
    pub extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// Extensions that cannot be ignored
    #[prost(message, repeated, tag = "9")]
    pub modifier_extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// A human identifier for this person
    #[prost(message, repeated, tag = "10")]
    pub identifier: prost::alloc::vec::Vec<super::core::Identifier>,
    /// Whether this related person's record is in active use
    #[prost(message, optional, tag = "11")]
    pub active: ::core::option::Option<super::core::Boolean>,
    /// The patient this person is related to
    #[prost(message, optional, tag = "12")]
    pub patient: ::core::option::Option<super::core::Reference>,
    /// The nature of the relationship
    #[prost(message, repeated, tag = "13")]
    pub relationship: prost::alloc::vec::Vec<super::core::CodeableConcept>,
    /// A name associated with the person
    #[prost(message, repeated, tag = "14")]
    pub name: prost::alloc::vec::Vec<super::core::HumanName>,
    /// A contact detail for the person
    #[prost(message, repeated, tag = "15")]
    pub telecom: prost::alloc::vec::Vec<super::core::ContactPoint>,
    #[prost(message, optional, tag = "16")]
    pub gender: ::core::option::Option<qi_core_related_person::GenderCode>,
    /// The date on which the related person was born
    #[prost(message, optional, tag = "17")]
    pub birth_date: ::core::option::Option<super::core::Date>,
    /// Address where the related person can be contacted or visited
    #[prost(message, repeated, tag = "18")]
    pub address: prost::alloc::vec::Vec<super::core::Address>,
    /// Image of the person
    #[prost(message, repeated, tag = "19")]
    pub photo: prost::alloc::vec::Vec<super::core::Attachment>,
    /// Period of time that this relationship is considered valid
    #[prost(message, optional, tag = "20")]
    pub period: ::core::option::Option<super::core::Period>,
    #[prost(message, repeated, tag = "21")]
    pub communication: prost::alloc::vec::Vec<qi_core_related_person::Communication>,
}
/// Nested message and enum types in `QICoreRelatedPerson`.
pub mod qi_core_related_person {
    /// male | female | other | unknown
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct GenderCode {
        #[prost(
            enumeration = "super::super::core::administrative_gender_code::Value",
            tag = "1"
        )]
        pub value: i32,
        #[prost(message, optional, tag = "2")]
        pub id: ::core::option::Option<super::super::core::String>,
        #[prost(message, repeated, tag = "3")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
    }
    /// A language which may be used to communicate with about the patient's health
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct Communication {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag = "1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag = "2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag = "3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// The language which can be used to communicate with the patient about his
        /// or her health
        #[prost(message, optional, tag = "4")]
        pub language: ::core::option::Option<super::super::core::CodeableConcept>,
        /// Language preference indicator
        #[prost(message, optional, tag = "5")]
        pub preferred: ::core::option::Option<super::super::core::Boolean>,
    }
}
/// Auto-generated from StructureDefinition for
/// QICoreServiceRequestAppropriatenessScore. Appropriateness Score. See
/// <http://hl7.org/fhir/us/qicore/StructureDefinition/qicore-servicerequest-appropriatenessScore>
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct ServiceRequestQiCoreServiceRequestAppropriatenessScore {
    /// xml:id (or equivalent in JSON)
    #[prost(message, optional, tag = "1")]
    pub id: ::core::option::Option<super::core::Id>,
    #[prost(message, optional, tag = "3")]
    pub value: ::core::option::Option<
        service_request_qi_core_service_request_appropriateness_score::ValueX,
    >,
}
/// Nested message and enum types in `ServiceRequestQICoreServiceRequestAppropriatenessScore`.
pub mod service_request_qi_core_service_request_appropriateness_score {
    /// Value of extension
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct ValueX {
        #[prost(oneof = "value_x::Choice", tags = "1, 2")]
        pub choice: ::core::option::Option<value_x::Choice>,
    }
    /// Nested message and enum types in `ValueX`.
    pub mod value_x {
        #[derive(Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Oneof)]
        pub enum Choice {
            #[prost(message, tag = "1")]
            Code(super::super::super::core::Code),
            #[prost(message, tag = "2")]
            Decimal(super::super::super::core::Decimal),
        }
    }
}
/// Auto-generated from StructureDefinition for QICoreServiceRequest.
/// A request for a service to be performed.
/// See <http://hl7.org/fhir/us/qicore/StructureDefinition/qicore-servicerequest>
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct QiCoreServiceRequest {
    /// Logical id of this artifact
    #[prost(message, optional, tag = "1")]
    pub id: ::core::option::Option<super::core::Id>,
    /// Metadata about the resource
    #[prost(message, optional, tag = "2")]
    pub meta: ::core::option::Option<super::core::Meta>,
    /// A set of rules under which this content was created
    #[prost(message, optional, tag = "3")]
    pub implicit_rules: ::core::option::Option<super::core::Uri>,
    /// Language of the resource content
    #[prost(message, optional, tag = "4")]
    pub language: ::core::option::Option<super::core::Code>,
    /// Text summary of the resource, for human interpretation
    #[prost(message, optional, tag = "5")]
    pub text: ::core::option::Option<super::core::Narrative>,
    /// Contained, inline Resources
    #[prost(message, repeated, tag = "6")]
    pub contained: prost::alloc::vec::Vec<::prost_wkt_types::Any>,
    /// Extension
    #[prost(message, repeated, tag = "8")]
    pub extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// Extensions that cannot be ignored
    #[prost(message, repeated, tag = "9")]
    pub modifier_extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// Identifiers assigned to this order
    #[prost(message, repeated, tag = "10")]
    pub identifier: prost::alloc::vec::Vec<super::core::Identifier>,
    /// Instantiates FHIR protocol or definition
    #[prost(message, repeated, tag = "11")]
    pub instantiates_canonical: prost::alloc::vec::Vec<super::core::Canonical>,
    /// Instantiates external protocol or definition
    #[prost(message, repeated, tag = "12")]
    pub instantiates_uri: prost::alloc::vec::Vec<super::core::Uri>,
    /// What request fulfills
    #[prost(message, repeated, tag = "13")]
    pub based_on: prost::alloc::vec::Vec<super::core::Reference>,
    /// What request replaces
    #[prost(message, repeated, tag = "14")]
    pub replaces: prost::alloc::vec::Vec<super::core::Reference>,
    /// Composite Request ID
    #[prost(message, optional, tag = "15")]
    pub requisition: ::core::option::Option<super::core::Identifier>,
    #[prost(message, optional, tag = "16")]
    pub status: ::core::option::Option<qi_core_service_request::StatusCode>,
    #[prost(message, optional, tag = "17")]
    pub intent: ::core::option::Option<qi_core_service_request::IntentCode>,
    /// Classification of service
    #[prost(message, repeated, tag = "18")]
    pub category: prost::alloc::vec::Vec<super::core::CodeableConcept>,
    #[prost(message, optional, tag = "19")]
    pub priority: ::core::option::Option<qi_core_service_request::PriorityCode>,
    /// True if service/procedure should not be performed
    #[prost(message, optional, tag = "20")]
    pub do_not_perform: ::core::option::Option<super::core::Boolean>,
    /// What is being requested/ordered
    #[prost(message, optional, tag = "21")]
    pub code: ::core::option::Option<super::core::CodeableConcept>,
    /// Additional order information
    #[prost(message, repeated, tag = "22")]
    pub order_detail: prost::alloc::vec::Vec<super::core::CodeableConcept>,
    #[prost(message, optional, tag = "23")]
    pub quantity: ::core::option::Option<qi_core_service_request::QuantityX>,
    /// Individual or Entity the service is ordered for
    #[prost(message, optional, tag = "24")]
    pub subject: ::core::option::Option<super::core::Reference>,
    /// Encounter in which the request was created
    #[prost(message, optional, tag = "25")]
    pub encounter: ::core::option::Option<super::core::Reference>,
    #[prost(message, optional, tag = "26")]
    pub occurrence: ::core::option::Option<qi_core_service_request::OccurrenceX>,
    #[prost(message, optional, tag = "27")]
    pub as_needed: ::core::option::Option<qi_core_service_request::AsNeededX>,
    /// Date request signed
    #[prost(message, optional, tag = "28")]
    pub authored_on: ::core::option::Option<super::core::DateTime>,
    /// Who/what is requesting service
    #[prost(message, optional, tag = "29")]
    pub requester: ::core::option::Option<super::core::Reference>,
    /// Performer role
    #[prost(message, optional, tag = "30")]
    pub performer_type: ::core::option::Option<super::core::CodeableConcept>,
    /// Requested performer
    #[prost(message, repeated, tag = "31")]
    pub performer: prost::alloc::vec::Vec<super::core::Reference>,
    /// Requested location
    #[prost(message, repeated, tag = "32")]
    pub location_code: prost::alloc::vec::Vec<super::core::CodeableConcept>,
    /// Requested location
    #[prost(message, repeated, tag = "33")]
    pub location_reference: prost::alloc::vec::Vec<super::core::Reference>,
    /// Explanation/Justification for procedure or service
    #[prost(message, repeated, tag = "34")]
    pub reason_code: prost::alloc::vec::Vec<super::core::CodeableConcept>,
    /// Explanation/Justification for service or service
    #[prost(message, repeated, tag = "35")]
    pub reason_reference: prost::alloc::vec::Vec<super::core::Reference>,
    /// Associated insurance coverage
    #[prost(message, repeated, tag = "36")]
    pub insurance: prost::alloc::vec::Vec<super::core::Reference>,
    /// Additional clinical information
    #[prost(message, repeated, tag = "37")]
    pub supporting_info: prost::alloc::vec::Vec<super::core::Reference>,
    /// Procedure Samples
    #[prost(message, repeated, tag = "38")]
    pub specimen: prost::alloc::vec::Vec<super::core::Reference>,
    /// Location on Body
    #[prost(message, repeated, tag = "39")]
    pub body_site: prost::alloc::vec::Vec<super::core::CodeableConcept>,
    /// Comments
    #[prost(message, repeated, tag = "40")]
    pub note: prost::alloc::vec::Vec<super::core::Annotation>,
    /// Patient or consumer-oriented instructions
    #[prost(message, optional, tag = "41")]
    pub patient_instruction: ::core::option::Option<super::core::String>,
    /// Request provenance
    #[prost(message, repeated, tag = "42")]
    pub relevant_history: prost::alloc::vec::Vec<super::core::Reference>,
    /// Reason for current status
    #[prost(message, optional, tag = "43")]
    pub status_reason: ::core::option::Option<super::core::CodeableConcept>,
    /// The access point or points used for this procedure
    #[prost(message, repeated, tag = "44")]
    pub approach_body_structure: prost::alloc::vec::Vec<super::core::Reference>,
    /// Appropriateness Score
    #[prost(message, optional, tag = "45")]
    pub appropriateness_score: ::core::option::Option<
        service_request_qi_core_service_request_appropriateness_score::ValueX,
    >,
}
/// Nested message and enum types in `QICoreServiceRequest`.
pub mod qi_core_service_request {
    /// draft | active | suspended | completed | entered-in-error | cancelled
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct StatusCode {
        #[prost(
            enumeration = "super::super::core::request_status_code::Value",
            tag = "1"
        )]
        pub value: i32,
        #[prost(message, optional, tag = "2")]
        pub id: ::core::option::Option<super::super::core::String>,
        #[prost(message, repeated, tag = "3")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
    }
    /// proposal | plan | order +
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct IntentCode {
        #[prost(
            enumeration = "super::super::core::request_intent_code::Value",
            tag = "1"
        )]
        pub value: i32,
        #[prost(message, optional, tag = "2")]
        pub id: ::core::option::Option<super::super::core::String>,
        #[prost(message, repeated, tag = "3")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
    }
    /// routine | urgent | asap | stat
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct PriorityCode {
        #[prost(
            enumeration = "super::super::core::request_priority_code::Value",
            tag = "1"
        )]
        pub value: i32,
        #[prost(message, optional, tag = "2")]
        pub id: ::core::option::Option<super::super::core::String>,
        #[prost(message, repeated, tag = "3")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
    }
    /// Service amount
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct QuantityX {
        #[prost(oneof = "quantity_x::Choice", tags = "1, 2, 3")]
        pub choice: ::core::option::Option<quantity_x::Choice>,
    }
    /// Nested message and enum types in `QuantityX`.
    pub mod quantity_x {
        #[derive(Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Oneof)]
        pub enum Choice {
            #[prost(message, tag = "1")]
            Quantity(super::super::super::core::Quantity),
            #[prost(message, tag = "2")]
            Ratio(super::super::super::core::Ratio),
            #[prost(message, tag = "3")]
            Range(super::super::super::core::Range),
        }
    }
    /// When service should occur
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct OccurrenceX {
        #[prost(oneof = "occurrence_x::Choice", tags = "1, 2, 3")]
        pub choice: ::core::option::Option<occurrence_x::Choice>,
    }
    /// Nested message and enum types in `OccurrenceX`.
    pub mod occurrence_x {
        #[derive(Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Oneof)]
        pub enum Choice {
            #[prost(message, tag = "1")]
            DateTime(super::super::super::core::DateTime),
            #[prost(message, tag = "2")]
            Period(super::super::super::core::Period),
            #[prost(message, tag = "3")]
            Timing(super::super::super::core::Timing),
        }
    }
    /// Preconditions for service
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct AsNeededX {
        #[prost(oneof = "as_needed_x::Choice", tags = "1, 2")]
        pub choice: ::core::option::Option<as_needed_x::Choice>,
    }
    /// Nested message and enum types in `AsNeededX`.
    pub mod as_needed_x {
        #[derive(Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Oneof)]
        pub enum Choice {
            #[prost(message, tag = "1")]
            Boolean(super::super::super::core::Boolean),
            #[prost(message, tag = "2")]
            CodeableConcept(super::super::super::core::CodeableConcept),
        }
    }
}
/// Auto-generated from StructureDefinition for QICoreSpecimen.
/// Sample for analysis.
/// See <http://hl7.org/fhir/us/qicore/StructureDefinition/qicore-specimen>
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct QiCoreSpecimen {
    /// Logical id of this artifact
    #[prost(message, optional, tag = "1")]
    pub id: ::core::option::Option<super::core::Id>,
    /// Metadata about the resource
    #[prost(message, optional, tag = "2")]
    pub meta: ::core::option::Option<super::core::Meta>,
    /// A set of rules under which this content was created
    #[prost(message, optional, tag = "3")]
    pub implicit_rules: ::core::option::Option<super::core::Uri>,
    /// Language of the resource content
    #[prost(message, optional, tag = "4")]
    pub language: ::core::option::Option<super::core::Code>,
    /// Text summary of the resource, for human interpretation
    #[prost(message, optional, tag = "5")]
    pub text: ::core::option::Option<super::core::Narrative>,
    /// Contained, inline Resources
    #[prost(message, repeated, tag = "6")]
    pub contained: prost::alloc::vec::Vec<::prost_wkt_types::Any>,
    /// Additional content defined by implementations
    #[prost(message, repeated, tag = "8")]
    pub extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// Extensions that cannot be ignored
    #[prost(message, repeated, tag = "9")]
    pub modifier_extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// External Identifier
    #[prost(message, repeated, tag = "10")]
    pub identifier: prost::alloc::vec::Vec<super::core::Identifier>,
    /// Identifier assigned by the lab
    #[prost(message, optional, tag = "11")]
    pub accession_identifier: ::core::option::Option<super::core::Identifier>,
    #[prost(message, optional, tag = "12")]
    pub status: ::core::option::Option<qi_core_specimen::StatusCode>,
    /// Kind of material that forms the specimen
    #[prost(message, optional, tag = "13")]
    pub r#type: ::core::option::Option<super::core::CodeableConcept>,
    /// Where the specimen came from. This may be from patient(s), from a location
    /// (e.g., the source of an environmental sample), or a sampling of a substance
    /// or a device
    #[prost(message, optional, tag = "14")]
    pub subject: ::core::option::Option<super::core::Reference>,
    /// The time when specimen was received for processing
    #[prost(message, optional, tag = "15")]
    pub received_time: ::core::option::Option<super::core::DateTime>,
    /// Specimen from which this specimen originated
    #[prost(message, repeated, tag = "16")]
    pub parent: prost::alloc::vec::Vec<super::core::Reference>,
    /// Why the specimen was collected
    #[prost(message, repeated, tag = "17")]
    pub request: prost::alloc::vec::Vec<super::core::Reference>,
    #[prost(message, optional, tag = "18")]
    pub collection: ::core::option::Option<qi_core_specimen::Collection>,
    #[prost(message, repeated, tag = "19")]
    pub processing: prost::alloc::vec::Vec<qi_core_specimen::Processing>,
    #[prost(message, repeated, tag = "20")]
    pub container: prost::alloc::vec::Vec<qi_core_specimen::Container>,
    /// State of the specimen
    #[prost(message, repeated, tag = "21")]
    pub condition: prost::alloc::vec::Vec<super::core::CodeableConcept>,
    /// Comments
    #[prost(message, repeated, tag = "22")]
    pub note: prost::alloc::vec::Vec<super::core::Annotation>,
}
/// Nested message and enum types in `QICoreSpecimen`.
pub mod qi_core_specimen {
    /// available | unavailable | unsatisfactory | entered-in-error
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct StatusCode {
        #[prost(
            enumeration = "super::super::core::specimen_status_code::Value",
            tag = "1"
        )]
        pub value: i32,
        #[prost(message, optional, tag = "2")]
        pub id: ::core::option::Option<super::super::core::String>,
        #[prost(message, repeated, tag = "3")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
    }
    /// Collection details
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct Collection {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag = "1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag = "2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag = "3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Who collected the specimen
        #[prost(message, optional, tag = "4")]
        pub collector: ::core::option::Option<super::super::core::Reference>,
        #[prost(message, optional, tag = "5")]
        pub collected: ::core::option::Option<collection::CollectedX>,
        /// How long it took to collect specimen
        #[prost(message, optional, tag = "6")]
        pub duration: ::core::option::Option<super::super::core::Duration>,
        /// The quantity of specimen collected
        #[prost(message, optional, tag = "7")]
        pub quantity: ::core::option::Option<super::super::core::SimpleQuantity>,
        /// Technique used to perform collection
        #[prost(message, optional, tag = "8")]
        pub method: ::core::option::Option<super::super::core::CodeableConcept>,
        /// Anatomical collection site
        #[prost(message, optional, tag = "9")]
        pub body_site: ::core::option::Option<super::super::core::CodeableConcept>,
        #[prost(message, optional, tag = "10")]
        pub fasting_status: ::core::option::Option<collection::FastingStatusX>,
    }
    /// Nested message and enum types in `Collection`.
    pub mod collection {
        /// Collection time
        #[derive(Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Message)]
        pub struct CollectedX {
            #[prost(oneof = "collected_x::Choice", tags = "1, 2")]
            pub choice: ::core::option::Option<collected_x::Choice>,
        }
        /// Nested message and enum types in `CollectedX`.
        pub mod collected_x {
            #[derive(Serialize, Deserialize)]
            #[serde(rename_all = "camelCase")]
            #[derive(Clone, PartialEq, prost::Oneof)]
            pub enum Choice {
                #[prost(message, tag = "1")]
                DateTime(super::super::super::super::core::DateTime),
                #[prost(message, tag = "2")]
                Period(super::super::super::super::core::Period),
            }
        }
        /// Whether or how long patient abstained from food and/or drink
        #[derive(Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Message)]
        pub struct FastingStatusX {
            #[prost(oneof = "fasting_status_x::Choice", tags = "1, 2")]
            pub choice: ::core::option::Option<fasting_status_x::Choice>,
        }
        /// Nested message and enum types in `FastingStatusX`.
        pub mod fasting_status_x {
            #[derive(Serialize, Deserialize)]
            #[serde(rename_all = "camelCase")]
            #[derive(Clone, PartialEq, prost::Oneof)]
            pub enum Choice {
                #[prost(message, tag = "1")]
                CodeableConcept(super::super::super::super::core::CodeableConcept),
                #[prost(message, tag = "2")]
                Duration(super::super::super::super::core::Duration),
            }
        }
    }
    /// Processing and processing step details
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct Processing {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag = "1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag = "2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag = "3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Textual description of procedure
        #[prost(message, optional, tag = "4")]
        pub description: ::core::option::Option<super::super::core::String>,
        /// Indicates the treatment step  applied to the specimen
        #[prost(message, optional, tag = "5")]
        pub procedure: ::core::option::Option<super::super::core::CodeableConcept>,
        /// Material used in the processing step
        #[prost(message, repeated, tag = "6")]
        pub additive: prost::alloc::vec::Vec<super::super::core::Reference>,
        #[prost(message, optional, tag = "7")]
        pub time: ::core::option::Option<processing::TimeX>,
    }
    /// Nested message and enum types in `Processing`.
    pub mod processing {
        /// Date and time of specimen processing
        #[derive(Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Message)]
        pub struct TimeX {
            #[prost(oneof = "time_x::Choice", tags = "1, 2")]
            pub choice: ::core::option::Option<time_x::Choice>,
        }
        /// Nested message and enum types in `TimeX`.
        pub mod time_x {
            #[derive(Serialize, Deserialize)]
            #[serde(rename_all = "camelCase")]
            #[derive(Clone, PartialEq, prost::Oneof)]
            pub enum Choice {
                #[prost(message, tag = "1")]
                DateTime(super::super::super::super::core::DateTime),
                #[prost(message, tag = "2")]
                Period(super::super::super::super::core::Period),
            }
        }
    }
    /// Direct container of specimen (tube/slide, etc.)
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct Container {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag = "1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Extension
        #[prost(message, repeated, tag = "2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag = "3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Id for the container
        #[prost(message, repeated, tag = "4")]
        pub identifier: prost::alloc::vec::Vec<super::super::core::Identifier>,
        /// Textual description of the container
        #[prost(message, optional, tag = "5")]
        pub description: ::core::option::Option<super::super::core::String>,
        /// Kind of container directly associated with specimen
        #[prost(message, optional, tag = "6")]
        pub r#type: ::core::option::Option<super::super::core::CodeableConcept>,
        /// Container volume or size
        #[prost(message, optional, tag = "7")]
        pub capacity: ::core::option::Option<super::super::core::SimpleQuantity>,
        /// Quantity of specimen within container
        #[prost(message, optional, tag = "8")]
        pub specimen_quantity: ::core::option::Option<super::super::core::SimpleQuantity>,
        #[prost(message, optional, tag = "9")]
        pub additive: ::core::option::Option<container::AdditiveX>,
        /// The sequence number of the sample
        #[prost(message, optional, tag = "10")]
        pub container_sequence_number: ::core::option::Option<super::super::core::Integer>,
    }
    /// Nested message and enum types in `Container`.
    pub mod container {
        /// Additive associated with container
        #[derive(Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Message)]
        pub struct AdditiveX {
            #[prost(oneof = "additive_x::Choice", tags = "1, 2")]
            pub choice: ::core::option::Option<additive_x::Choice>,
        }
        /// Nested message and enum types in `AdditiveX`.
        pub mod additive_x {
            #[derive(Serialize, Deserialize)]
            #[serde(rename_all = "camelCase")]
            #[derive(Clone, PartialEq, prost::Oneof)]
            pub enum Choice {
                #[prost(message, tag = "1")]
                CodeableConcept(super::super::super::super::core::CodeableConcept),
                #[prost(message, tag = "2")]
                Reference(super::super::super::super::core::Reference),
            }
        }
    }
}
/// Auto-generated from StructureDefinition for QICoreSubstance.
/// A homogeneous material with a definite composition.
/// See <http://hl7.org/fhir/us/qicore/StructureDefinition/qicore-substance>
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct QiCoreSubstance {
    /// Logical id of this artifact
    #[prost(message, optional, tag = "1")]
    pub id: ::core::option::Option<super::core::Id>,
    /// Metadata about the resource
    #[prost(message, optional, tag = "2")]
    pub meta: ::core::option::Option<super::core::Meta>,
    /// A set of rules under which this content was created
    #[prost(message, optional, tag = "3")]
    pub implicit_rules: ::core::option::Option<super::core::Uri>,
    /// Language of the resource content
    #[prost(message, optional, tag = "4")]
    pub language: ::core::option::Option<super::core::Code>,
    /// Text summary of the resource, for human interpretation
    #[prost(message, optional, tag = "5")]
    pub text: ::core::option::Option<super::core::Narrative>,
    /// Contained, inline Resources
    #[prost(message, repeated, tag = "6")]
    pub contained: prost::alloc::vec::Vec<::prost_wkt_types::Any>,
    /// Additional content defined by implementations
    #[prost(message, repeated, tag = "8")]
    pub extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// Extensions that cannot be ignored
    #[prost(message, repeated, tag = "9")]
    pub modifier_extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// Unique identifier
    #[prost(message, repeated, tag = "10")]
    pub identifier: prost::alloc::vec::Vec<super::core::Identifier>,
    #[prost(message, optional, tag = "11")]
    pub status: ::core::option::Option<qi_core_substance::StatusCode>,
    /// What class/type of substance this is
    #[prost(message, repeated, tag = "12")]
    pub category: prost::alloc::vec::Vec<super::core::CodeableConcept>,
    /// What substance this is
    #[prost(message, optional, tag = "13")]
    pub code: ::core::option::Option<super::core::CodeableConcept>,
    /// Textual description of the substance, comments
    #[prost(message, optional, tag = "14")]
    pub description: ::core::option::Option<super::core::String>,
    #[prost(message, repeated, tag = "15")]
    pub instance: prost::alloc::vec::Vec<qi_core_substance::Instance>,
    #[prost(message, repeated, tag = "16")]
    pub ingredient: prost::alloc::vec::Vec<qi_core_substance::Ingredient>,
}
/// Nested message and enum types in `QICoreSubstance`.
pub mod qi_core_substance {
    /// active | inactive | entered-in-error
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct StatusCode {
        #[prost(
            enumeration = "super::super::core::fhir_substance_status_code::Value",
            tag = "1"
        )]
        pub value: i32,
        #[prost(message, optional, tag = "2")]
        pub id: ::core::option::Option<super::super::core::String>,
        #[prost(message, repeated, tag = "3")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
    }
    /// If this describes a specific package/container of the substance
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct Instance {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag = "1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag = "2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag = "3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Identifier of the package/container
        #[prost(message, optional, tag = "4")]
        pub identifier: ::core::option::Option<super::super::core::Identifier>,
        /// When no longer valid to use
        #[prost(message, optional, tag = "5")]
        pub expiry: ::core::option::Option<super::super::core::DateTime>,
        /// Amount of substance in the package
        #[prost(message, optional, tag = "6")]
        pub quantity: ::core::option::Option<super::super::core::SimpleQuantity>,
    }
    /// Composition information about the substance
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct Ingredient {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag = "1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag = "2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag = "3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Optional amount (concentration)
        #[prost(message, optional, tag = "4")]
        pub quantity: ::core::option::Option<super::super::core::Ratio>,
        #[prost(message, optional, tag = "5")]
        pub substance: ::core::option::Option<ingredient::SubstanceX>,
    }
    /// Nested message and enum types in `Ingredient`.
    pub mod ingredient {
        /// A component of the substance
        #[derive(Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Message)]
        pub struct SubstanceX {
            #[prost(oneof = "substance_x::Choice", tags = "2")]
            pub choice: ::core::option::Option<substance_x::Choice>,
        }
        /// Nested message and enum types in `SubstanceX`.
        pub mod substance_x {
            #[derive(Serialize, Deserialize)]
            #[serde(rename_all = "camelCase")]
            #[derive(Clone, PartialEq, prost::Oneof)]
            pub enum Choice {
                #[prost(message, tag = "2")]
                Reference(super::super::super::super::core::Reference),
            }
        }
    }
}
/// Auto-generated from StructureDefinition for QICoreTask.
/// A task to be performed.
/// See <http://hl7.org/fhir/us/qicore/StructureDefinition/qicore-task>
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct QiCoreTask {
    /// Logical id of this artifact
    #[prost(message, optional, tag = "1")]
    pub id: ::core::option::Option<super::core::Id>,
    /// Metadata about the resource
    #[prost(message, optional, tag = "2")]
    pub meta: ::core::option::Option<super::core::Meta>,
    /// A set of rules under which this content was created
    #[prost(message, optional, tag = "3")]
    pub implicit_rules: ::core::option::Option<super::core::Uri>,
    /// Language of the resource content
    #[prost(message, optional, tag = "4")]
    pub language: ::core::option::Option<super::core::Code>,
    /// Text summary of the resource, for human interpretation
    #[prost(message, optional, tag = "5")]
    pub text: ::core::option::Option<super::core::Narrative>,
    /// Contained, inline Resources
    #[prost(message, repeated, tag = "6")]
    pub contained: prost::alloc::vec::Vec<::prost_wkt_types::Any>,
    /// Additional content defined by implementations
    #[prost(message, repeated, tag = "8")]
    pub extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// Extensions that cannot be ignored
    #[prost(message, repeated, tag = "9")]
    pub modifier_extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// Formal definition of task
    #[prost(message, optional, tag = "10")]
    pub instantiates_canonical: ::core::option::Option<super::core::Canonical>,
    /// Formal definition of task
    #[prost(message, optional, tag = "11")]
    pub instantiates_uri: ::core::option::Option<super::core::Uri>,
    /// Request fulfilled by this task
    #[prost(message, repeated, tag = "12")]
    pub based_on: prost::alloc::vec::Vec<super::core::Reference>,
    /// Requisition or grouper id
    #[prost(message, optional, tag = "13")]
    pub group_identifier: ::core::option::Option<super::core::Identifier>,
    /// Composite task
    #[prost(message, repeated, tag = "14")]
    pub part_of: prost::alloc::vec::Vec<super::core::Reference>,
    /// Reason for current status
    #[prost(message, optional, tag = "15")]
    pub status_reason: ::core::option::Option<super::core::CodeableConcept>,
    /// E.g. "Specimen collected", "IV prepped"
    #[prost(message, optional, tag = "16")]
    pub business_status: ::core::option::Option<super::core::CodeableConcept>,
    /// Human-readable explanation of task
    #[prost(message, optional, tag = "17")]
    pub description: ::core::option::Option<super::core::String>,
    /// What task is acting on
    #[prost(message, optional, tag = "18")]
    pub focus: ::core::option::Option<super::core::Reference>,
    /// Beneficiary of the Task
    #[prost(message, optional, tag = "19")]
    pub for_value: ::core::option::Option<super::core::Reference>,
    /// Healthcare event during which this task originated
    #[prost(message, optional, tag = "20")]
    pub encounter: ::core::option::Option<super::core::Reference>,
    /// Task Creation Date
    #[prost(message, optional, tag = "21")]
    pub authored_on: ::core::option::Option<super::core::DateTime>,
    /// Task Last Modified Date
    #[prost(message, optional, tag = "22")]
    pub last_modified: ::core::option::Option<super::core::DateTime>,
    /// Who is asking for task to be done
    #[prost(message, optional, tag = "23")]
    pub requester: ::core::option::Option<super::core::Reference>,
    /// Requested performer
    #[prost(message, repeated, tag = "24")]
    pub performer_type: prost::alloc::vec::Vec<super::core::CodeableConcept>,
    /// Responsible individual
    #[prost(message, optional, tag = "25")]
    pub owner: ::core::option::Option<super::core::Reference>,
    /// Where task occurs
    #[prost(message, optional, tag = "26")]
    pub location: ::core::option::Option<super::core::Reference>,
    /// Why task is needed
    #[prost(message, optional, tag = "27")]
    pub reason_code: ::core::option::Option<super::core::CodeableConcept>,
    /// Why task is needed
    #[prost(message, optional, tag = "28")]
    pub reason_reference: ::core::option::Option<super::core::Reference>,
    /// Associated insurance coverage
    #[prost(message, repeated, tag = "29")]
    pub insurance: prost::alloc::vec::Vec<super::core::Reference>,
    /// Comments made about the task
    #[prost(message, repeated, tag = "30")]
    pub note: prost::alloc::vec::Vec<super::core::Annotation>,
    /// Key events in history of the Task
    #[prost(message, repeated, tag = "31")]
    pub relevant_history: prost::alloc::vec::Vec<super::core::Reference>,
    #[prost(message, optional, tag = "32")]
    pub restriction: ::core::option::Option<qi_core_task::Restriction>,
    #[prost(message, repeated, tag = "33")]
    pub input: prost::alloc::vec::Vec<qi_core_task::Parameter>,
    #[prost(message, repeated, tag = "34")]
    pub output: prost::alloc::vec::Vec<qi_core_task::Output>,
}
/// Nested message and enum types in `QICoreTask`.
pub mod qi_core_task {
    /// Constraints on fulfillment tasks
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct Restriction {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag = "1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag = "2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag = "3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// How many times to repeat
        #[prost(message, optional, tag = "4")]
        pub repetitions: ::core::option::Option<super::super::core::PositiveInt>,
        /// When fulfillment sought
        #[prost(message, optional, tag = "5")]
        pub period: ::core::option::Option<super::super::core::Period>,
        /// For whom is fulfillment sought?
        #[prost(message, repeated, tag = "6")]
        pub recipient: prost::alloc::vec::Vec<super::super::core::Reference>,
    }
    /// Information used to perform task
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct Parameter {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag = "1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag = "2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag = "3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Label for the input
        #[prost(message, optional, tag = "4")]
        pub r#type: ::core::option::Option<super::super::core::CodeableConcept>,
        #[prost(message, optional, tag = "5")]
        pub value: ::core::option::Option<parameter::ValueX>,
    }
    /// Nested message and enum types in `Parameter`.
    pub mod parameter {
        /// Content to use in performing the task
        #[derive(Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Message)]
        pub struct ValueX {
            #[prost(
                oneof = "value_x::Choice",
                tags = "1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49"
            )]
            pub choice: ::core::option::Option<value_x::Choice>,
        }
        /// Nested message and enum types in `ValueX`.
        pub mod value_x {
            #[derive(Serialize, Deserialize)]
            #[serde(rename_all = "camelCase")]
            #[derive(Clone, PartialEq, prost::Oneof)]
            pub enum Choice {
                #[prost(message, tag = "1")]
                Base64Binary(super::super::super::super::core::Base64Binary),
                #[prost(message, tag = "2")]
                Boolean(super::super::super::super::core::Boolean),
                #[prost(message, tag = "3")]
                Canonical(super::super::super::super::core::Canonical),
                #[prost(message, tag = "4")]
                Code(super::super::super::super::core::Code),
                #[prost(message, tag = "5")]
                Date(super::super::super::super::core::Date),
                #[prost(message, tag = "6")]
                DateTime(super::super::super::super::core::DateTime),
                #[prost(message, tag = "7")]
                Decimal(super::super::super::super::core::Decimal),
                #[prost(message, tag = "8")]
                Id(super::super::super::super::core::Id),
                #[prost(message, tag = "9")]
                Instant(super::super::super::super::core::Instant),
                #[prost(message, tag = "10")]
                Integer(super::super::super::super::core::Integer),
                #[prost(message, tag = "11")]
                Markdown(super::super::super::super::core::Markdown),
                #[prost(message, tag = "12")]
                Oid(super::super::super::super::core::Oid),
                #[prost(message, tag = "13")]
                PositiveInt(super::super::super::super::core::PositiveInt),
                #[prost(message, tag = "14")]
                StringValue(super::super::super::super::core::String),
                #[prost(message, tag = "15")]
                Time(super::super::super::super::core::Time),
                #[prost(message, tag = "16")]
                UnsignedInt(super::super::super::super::core::UnsignedInt),
                #[prost(message, tag = "17")]
                Uri(super::super::super::super::core::Uri),
                #[prost(message, tag = "18")]
                Url(super::super::super::super::core::Url),
                #[prost(message, tag = "19")]
                Uuid(super::super::super::super::core::Uuid),
                #[prost(message, tag = "20")]
                Address(super::super::super::super::core::Address),
                #[prost(message, tag = "21")]
                Age(super::super::super::super::core::Age),
                #[prost(message, tag = "22")]
                Annotation(super::super::super::super::core::Annotation),
                #[prost(message, tag = "23")]
                Attachment(super::super::super::super::core::Attachment),
                #[prost(message, tag = "24")]
                CodeableConcept(super::super::super::super::core::CodeableConcept),
                #[prost(message, tag = "25")]
                Coding(super::super::super::super::core::Coding),
                #[prost(message, tag = "26")]
                ContactPoint(super::super::super::super::core::ContactPoint),
                #[prost(message, tag = "27")]
                Count(super::super::super::super::core::Count),
                #[prost(message, tag = "28")]
                Distance(super::super::super::super::core::Distance),
                #[prost(message, tag = "29")]
                Duration(super::super::super::super::core::Duration),
                #[prost(message, tag = "30")]
                HumanName(super::super::super::super::core::HumanName),
                #[prost(message, tag = "31")]
                Identifier(super::super::super::super::core::Identifier),
                #[prost(message, tag = "32")]
                Money(super::super::super::super::core::Money),
                #[prost(message, tag = "33")]
                Period(super::super::super::super::core::Period),
                #[prost(message, tag = "34")]
                Quantity(super::super::super::super::core::Quantity),
                #[prost(message, tag = "35")]
                Range(super::super::super::super::core::Range),
                #[prost(message, tag = "36")]
                Ratio(super::super::super::super::core::Ratio),
                #[prost(message, tag = "37")]
                Reference(super::super::super::super::core::Reference),
                #[prost(message, tag = "38")]
                SampledData(super::super::super::super::core::SampledData),
                #[prost(message, tag = "39")]
                Signature(super::super::super::super::core::Signature),
                #[prost(message, tag = "40")]
                Timing(super::super::super::super::core::Timing),
                #[prost(message, tag = "41")]
                ContactDetail(super::super::super::super::core::ContactDetail),
                #[prost(message, tag = "42")]
                Contributor(super::super::super::super::core::Contributor),
                #[prost(message, tag = "43")]
                DataRequirement(super::super::super::super::core::DataRequirement),
                #[prost(message, tag = "44")]
                Expression(super::super::super::super::core::Expression),
                #[prost(message, tag = "45")]
                ParameterDefinition(super::super::super::super::core::ParameterDefinition),
                #[prost(message, tag = "46")]
                RelatedArtifact(super::super::super::super::core::RelatedArtifact),
                #[prost(message, tag = "47")]
                TriggerDefinition(super::super::super::super::core::TriggerDefinition),
                #[prost(message, tag = "48")]
                UsageContext(super::super::super::super::core::UsageContext),
                #[prost(message, tag = "49")]
                Dosage(super::super::super::super::core::Dosage),
            }
        }
    }
    /// Information produced as part of task
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct Output {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag = "1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag = "2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag = "3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Label for output
        #[prost(message, optional, tag = "4")]
        pub r#type: ::core::option::Option<super::super::core::CodeableConcept>,
        #[prost(message, optional, tag = "5")]
        pub value: ::core::option::Option<output::ValueX>,
    }
    /// Nested message and enum types in `Output`.
    pub mod output {
        /// Result of output
        #[derive(Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Message)]
        pub struct ValueX {
            #[prost(
                oneof = "value_x::Choice",
                tags = "1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49"
            )]
            pub choice: ::core::option::Option<value_x::Choice>,
        }
        /// Nested message and enum types in `ValueX`.
        pub mod value_x {
            #[derive(Serialize, Deserialize)]
            #[serde(rename_all = "camelCase")]
            #[derive(Clone, PartialEq, prost::Oneof)]
            pub enum Choice {
                #[prost(message, tag = "1")]
                Base64Binary(super::super::super::super::core::Base64Binary),
                #[prost(message, tag = "2")]
                Boolean(super::super::super::super::core::Boolean),
                #[prost(message, tag = "3")]
                Canonical(super::super::super::super::core::Canonical),
                #[prost(message, tag = "4")]
                Code(super::super::super::super::core::Code),
                #[prost(message, tag = "5")]
                Date(super::super::super::super::core::Date),
                #[prost(message, tag = "6")]
                DateTime(super::super::super::super::core::DateTime),
                #[prost(message, tag = "7")]
                Decimal(super::super::super::super::core::Decimal),
                #[prost(message, tag = "8")]
                Id(super::super::super::super::core::Id),
                #[prost(message, tag = "9")]
                Instant(super::super::super::super::core::Instant),
                #[prost(message, tag = "10")]
                Integer(super::super::super::super::core::Integer),
                #[prost(message, tag = "11")]
                Markdown(super::super::super::super::core::Markdown),
                #[prost(message, tag = "12")]
                Oid(super::super::super::super::core::Oid),
                #[prost(message, tag = "13")]
                PositiveInt(super::super::super::super::core::PositiveInt),
                #[prost(message, tag = "14")]
                StringValue(super::super::super::super::core::String),
                #[prost(message, tag = "15")]
                Time(super::super::super::super::core::Time),
                #[prost(message, tag = "16")]
                UnsignedInt(super::super::super::super::core::UnsignedInt),
                #[prost(message, tag = "17")]
                Uri(super::super::super::super::core::Uri),
                #[prost(message, tag = "18")]
                Url(super::super::super::super::core::Url),
                #[prost(message, tag = "19")]
                Uuid(super::super::super::super::core::Uuid),
                #[prost(message, tag = "20")]
                Address(super::super::super::super::core::Address),
                #[prost(message, tag = "21")]
                Age(super::super::super::super::core::Age),
                #[prost(message, tag = "22")]
                Annotation(super::super::super::super::core::Annotation),
                #[prost(message, tag = "23")]
                Attachment(super::super::super::super::core::Attachment),
                #[prost(message, tag = "24")]
                CodeableConcept(super::super::super::super::core::CodeableConcept),
                #[prost(message, tag = "25")]
                Coding(super::super::super::super::core::Coding),
                #[prost(message, tag = "26")]
                ContactPoint(super::super::super::super::core::ContactPoint),
                #[prost(message, tag = "27")]
                Count(super::super::super::super::core::Count),
                #[prost(message, tag = "28")]
                Distance(super::super::super::super::core::Distance),
                #[prost(message, tag = "29")]
                Duration(super::super::super::super::core::Duration),
                #[prost(message, tag = "30")]
                HumanName(super::super::super::super::core::HumanName),
                #[prost(message, tag = "31")]
                Identifier(super::super::super::super::core::Identifier),
                #[prost(message, tag = "32")]
                Money(super::super::super::super::core::Money),
                #[prost(message, tag = "33")]
                Period(super::super::super::super::core::Period),
                #[prost(message, tag = "34")]
                Quantity(super::super::super::super::core::Quantity),
                #[prost(message, tag = "35")]
                Range(super::super::super::super::core::Range),
                #[prost(message, tag = "36")]
                Ratio(super::super::super::super::core::Ratio),
                #[prost(message, tag = "37")]
                Reference(super::super::super::super::core::Reference),
                #[prost(message, tag = "38")]
                SampledData(super::super::super::super::core::SampledData),
                #[prost(message, tag = "39")]
                Signature(super::super::super::super::core::Signature),
                #[prost(message, tag = "40")]
                Timing(super::super::super::super::core::Timing),
                #[prost(message, tag = "41")]
                ContactDetail(super::super::super::super::core::ContactDetail),
                #[prost(message, tag = "42")]
                Contributor(super::super::super::super::core::Contributor),
                #[prost(message, tag = "43")]
                DataRequirement(super::super::super::super::core::DataRequirement),
                #[prost(message, tag = "44")]
                Expression(super::super::super::super::core::Expression),
                #[prost(message, tag = "45")]
                ParameterDefinition(super::super::super::super::core::ParameterDefinition),
                #[prost(message, tag = "46")]
                RelatedArtifact(super::super::super::super::core::RelatedArtifact),
                #[prost(message, tag = "47")]
                TriggerDefinition(super::super::super::super::core::TriggerDefinition),
                #[prost(message, tag = "48")]
                UsageContext(super::super::super::super::core::UsageContext),
                #[prost(message, tag = "49")]
                Dosage(super::super::super::super::core::Dosage),
            }
        }
    }
}
