/// A type defined by US Core for Birthsex
/// <http://hl7.org/fhir/us/core/ValueSet/us-core-birthsex>
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UsCoreBirthSexCode {
    #[prost(enumeration="us_core_birth_sex_code::Value", tag="1")]
    pub value: i32,
    /// xml:id (or equivalent in JSON)
    #[prost(message, optional, tag="2")]
    pub id: ::core::option::Option<super::proto::String>,
    /// Additional Content defined by implementations
    #[prost(message, repeated, tag="3")]
    pub extension: ::prost::alloc::vec::Vec<super::proto::Extension>,
}
/// Nested message and enum types in `UsCoreBirthSexCode`.
pub mod us_core_birth_sex_code {
    /// Primitive value for code
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Value {
        InvalidUninitialized = 0,
        Female = 1,
        Male = 2,
        Unknown = 3,
    }
}
/// Auto-generated from StructureDefinition for US Core Allergies Profile.
/// US Core Allergies Profile.
/// See
/// <http://hl7.org/fhir/us/core/StructureDefinition/us-core-allergyintolerance>
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UsCoreAllergyintolerance {
    /// Logical id of this artifact
    #[prost(message, optional, tag="1")]
    pub id: ::core::option::Option<super::proto::Id>,
    /// Metadata about the resource
    #[prost(message, optional, tag="2")]
    pub meta: ::core::option::Option<super::proto::Meta>,
    /// A set of rules under which this content was created
    #[prost(message, optional, tag="3")]
    pub implicit_rules: ::core::option::Option<super::proto::Uri>,
    /// Language of the resource content
    #[prost(message, optional, tag="4")]
    pub language: ::core::option::Option<super::proto::LanguageCode>,
    /// Text summary of the resource, for human interpretation
    #[prost(message, optional, tag="5")]
    pub text: ::core::option::Option<super::proto::Narrative>,
    /// Contained, inline Resources
    #[prost(message, repeated, tag="6")]
    pub contained: ::prost::alloc::vec::Vec<super::proto::ContainedResource>,
    /// Additional Content defined by implementations
    #[prost(message, repeated, tag="7")]
    pub extension: ::prost::alloc::vec::Vec<super::proto::Extension>,
    /// Extensions that cannot be ignored
    #[prost(message, repeated, tag="8")]
    pub modifier_extension: ::prost::alloc::vec::Vec<super::proto::Extension>,
    /// External ids for this item
    #[prost(message, repeated, tag="9")]
    pub identifier: ::prost::alloc::vec::Vec<super::proto::Identifier>,
    /// active | inactive | resolved
    #[prost(message, optional, tag="10")]
    pub clinical_status: ::core::option::Option<super::proto::AllergyIntoleranceClinicalStatusCode>,
    /// unconfirmed | confirmed | refuted | entered-in-error
    #[prost(message, optional, tag="11")]
    pub verification_status: ::core::option::Option<super::proto::AllergyIntoleranceVerificationStatusCode>,
    /// allergy | intolerance - Underlying mechanism (if known)
    #[prost(message, optional, tag="12")]
    pub r#type: ::core::option::Option<super::proto::AllergyIntoleranceTypeCode>,
    /// food | medication | environment | biologic
    #[prost(message, repeated, tag="13")]
    pub category: ::prost::alloc::vec::Vec<super::proto::AllergyIntoleranceCategoryCode>,
    /// low | high | unable-to-assess
    #[prost(message, optional, tag="14")]
    pub criticality: ::core::option::Option<super::proto::AllergyIntoleranceCriticalityCode>,
    /// Code that identifies the allergy or intolerance
    #[prost(message, optional, tag="15")]
    pub code: ::core::option::Option<super::proto::CodeableConcept>,
    /// Who the sensitivity is for
    #[prost(message, optional, tag="16")]
    pub patient: ::core::option::Option<super::proto::Reference>,
    #[prost(message, optional, tag="17")]
    pub onset: ::core::option::Option<us_core_allergyintolerance::Onset>,
    /// Date record was believed accurate
    #[prost(message, optional, tag="18")]
    pub asserted_date: ::core::option::Option<super::proto::DateTime>,
    /// Who recorded the sensitivity
    #[prost(message, optional, tag="19")]
    pub recorder: ::core::option::Option<super::proto::Reference>,
    /// Source of the information about the allergy
    #[prost(message, optional, tag="20")]
    pub asserter: ::core::option::Option<super::proto::Reference>,
    /// Date(/time) of last known occurrence of a reaction
    #[prost(message, optional, tag="21")]
    pub last_occurrence: ::core::option::Option<super::proto::DateTime>,
    /// Additional text not captured in other fields
    #[prost(message, repeated, tag="22")]
    pub note: ::prost::alloc::vec::Vec<super::proto::Annotation>,
    #[prost(message, repeated, tag="23")]
    pub reaction: ::prost::alloc::vec::Vec<us_core_allergyintolerance::Reaction>,
}
/// Nested message and enum types in `UsCoreAllergyintolerance`.
pub mod us_core_allergyintolerance {
    /// When allergy or intolerance was identified
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Onset {
        #[prost(oneof="onset::Onset", tags="1, 2, 3, 4, 5")]
        pub onset: ::core::option::Option<onset::Onset>,
    }
    /// Nested message and enum types in `Onset`.
    pub mod onset {
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Onset {
            #[prost(message, tag="1")]
            DateTime(super::super::super::proto::DateTime),
            #[prost(message, tag="2")]
            Age(super::super::super::proto::Age),
            #[prost(message, tag="3")]
            Period(super::super::super::proto::Period),
            #[prost(message, tag="4")]
            Range(super::super::super::proto::Range),
            #[prost(message, tag="5")]
            StringValue(super::super::super::proto::String),
        }
    }
    /// Adverse Reaction Events linked to exposure to substance
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Reaction {
        /// xml:id (or equivalent in JSON)
        #[prost(message, optional, tag="1")]
        pub id: ::core::option::Option<super::super::proto::String>,
        /// Additional Content defined by implementations
        #[prost(message, repeated, tag="2")]
        pub extension: ::prost::alloc::vec::Vec<super::super::proto::Extension>,
        /// Extensions that cannot be ignored
        #[prost(message, repeated, tag="3")]
        pub modifier_extension: ::prost::alloc::vec::Vec<super::super::proto::Extension>,
        /// Specific substance or pharmaceutical product considered to be responsible
        /// for event
        #[prost(message, optional, tag="4")]
        pub substance: ::core::option::Option<super::super::proto::CodeableConcept>,
        /// Clinical symptoms/signs associated with the Event
        #[prost(message, repeated, tag="5")]
        pub manifestation: ::prost::alloc::vec::Vec<super::super::proto::CodeableConcept>,
        /// Description of the event as a whole
        #[prost(message, optional, tag="6")]
        pub description: ::core::option::Option<super::super::proto::String>,
        /// Date(/time) when manifestations showed
        #[prost(message, optional, tag="7")]
        pub onset: ::core::option::Option<super::super::proto::DateTime>,
        /// mild | moderate | severe (of event as a whole)
        #[prost(message, optional, tag="8")]
        pub severity: ::core::option::Option<super::super::proto::AllergyIntoleranceSeverityCode>,
        /// How the subject was exposed to the substance
        #[prost(message, optional, tag="9")]
        pub exposure_route: ::core::option::Option<super::super::proto::CodeableConcept>,
        /// Text about event not captured in other fields
        #[prost(message, repeated, tag="10")]
        pub note: ::prost::alloc::vec::Vec<super::super::proto::Annotation>,
    }
}
/// Auto-generated from StructureDefinition for US Core Birth Sex Extension.
/// Extension.
/// See <http://hl7.org/fhir/us/core/StructureDefinition/us-core-birthsex>
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PatientUsCoreBirthSexExtension {
    /// xml:id (or equivalent in JSON)
    #[prost(message, optional, tag="1")]
    pub id: ::core::option::Option<super::proto::String>,
    /// Additional Content defined by implementations
    #[prost(message, repeated, tag="2")]
    pub extension: ::prost::alloc::vec::Vec<super::proto::Extension>,
    /// Value of extension
    #[prost(message, optional, tag="3")]
    pub value_code: ::core::option::Option<UsCoreBirthSexCode>,
}
/// Auto-generated from StructureDefinition for US Core CarePlan Profile.
/// US Core CarePlan Profile.
/// See <http://hl7.org/fhir/us/core/StructureDefinition/us-core-careplan>
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UsCoreCareplan {
    /// Logical id of this artifact
    #[prost(message, optional, tag="1")]
    pub id: ::core::option::Option<super::proto::Id>,
    /// Metadata about the resource
    #[prost(message, optional, tag="2")]
    pub meta: ::core::option::Option<super::proto::Meta>,
    /// A set of rules under which this content was created
    #[prost(message, optional, tag="3")]
    pub implicit_rules: ::core::option::Option<super::proto::Uri>,
    /// Language of the resource content
    #[prost(message, optional, tag="4")]
    pub language: ::core::option::Option<super::proto::LanguageCode>,
    /// Text summary of the resource, for human interpretation
    #[prost(message, optional, tag="5")]
    pub text: ::core::option::Option<super::proto::Narrative>,
    /// Contained, inline Resources
    #[prost(message, repeated, tag="6")]
    pub contained: ::prost::alloc::vec::Vec<super::proto::ContainedResource>,
    /// Additional Content defined by implementations
    #[prost(message, repeated, tag="7")]
    pub extension: ::prost::alloc::vec::Vec<super::proto::Extension>,
    /// Extensions that cannot be ignored
    #[prost(message, repeated, tag="8")]
    pub modifier_extension: ::prost::alloc::vec::Vec<super::proto::Extension>,
    /// External Ids for this plan
    #[prost(message, repeated, tag="9")]
    pub identifier: ::prost::alloc::vec::Vec<super::proto::Identifier>,
    /// Protocol or definition
    #[prost(message, repeated, tag="10")]
    pub definition: ::prost::alloc::vec::Vec<super::proto::Reference>,
    /// Fulfills care plan
    #[prost(message, repeated, tag="11")]
    pub based_on: ::prost::alloc::vec::Vec<super::proto::Reference>,
    /// CarePlan replaced by this CarePlan
    #[prost(message, repeated, tag="12")]
    pub replaces: ::prost::alloc::vec::Vec<super::proto::Reference>,
    /// Part of referenced CarePlan
    #[prost(message, repeated, tag="13")]
    pub part_of: ::prost::alloc::vec::Vec<super::proto::Reference>,
    /// draft | active | suspended | completed | entered-in-error | cancelled |
    /// unknown
    #[prost(message, optional, tag="14")]
    pub status: ::core::option::Option<super::proto::CarePlanStatusCode>,
    /// proposal | plan | order | option
    #[prost(message, optional, tag="15")]
    pub intent: ::core::option::Option<super::proto::CarePlanIntentCode>,
    /// Type of plan
    #[prost(message, repeated, tag="16")]
    pub category: ::prost::alloc::vec::Vec<super::proto::CodeableConcept>,
    /// Human-friendly name for the CarePlan
    #[prost(message, optional, tag="17")]
    pub title: ::core::option::Option<super::proto::String>,
    /// Summary of nature of plan
    #[prost(message, optional, tag="18")]
    pub description: ::core::option::Option<super::proto::String>,
    /// Who care plan is for
    #[prost(message, optional, tag="19")]
    pub subject: ::core::option::Option<super::proto::Reference>,
    /// Created in context of
    #[prost(message, optional, tag="20")]
    pub context: ::core::option::Option<super::proto::Reference>,
    /// Time period plan covers
    #[prost(message, optional, tag="21")]
    pub period: ::core::option::Option<super::proto::Period>,
    /// Who is responsible for contents of the plan
    #[prost(message, repeated, tag="22")]
    pub author: ::prost::alloc::vec::Vec<super::proto::Reference>,
    /// Who's involved in plan?
    #[prost(message, repeated, tag="23")]
    pub care_team: ::prost::alloc::vec::Vec<super::proto::Reference>,
    /// Health issues this plan addresses
    #[prost(message, repeated, tag="24")]
    pub addresses: ::prost::alloc::vec::Vec<super::proto::Reference>,
    /// Information considered as part of plan
    #[prost(message, repeated, tag="25")]
    pub supporting_info: ::prost::alloc::vec::Vec<super::proto::Reference>,
    /// Desired outcome of plan
    #[prost(message, repeated, tag="26")]
    pub goal: ::prost::alloc::vec::Vec<super::proto::Reference>,
    #[prost(message, repeated, tag="27")]
    pub activity: ::prost::alloc::vec::Vec<us_core_careplan::Activity>,
    /// Comments about the plan
    #[prost(message, repeated, tag="28")]
    pub note: ::prost::alloc::vec::Vec<super::proto::Annotation>,
}
/// Nested message and enum types in `UsCoreCareplan`.
pub mod us_core_careplan {
    /// Action to occur as part of plan
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Activity {
        /// xml:id (or equivalent in JSON)
        #[prost(message, optional, tag="1")]
        pub id: ::core::option::Option<super::super::proto::String>,
        /// Additional Content defined by implementations
        #[prost(message, repeated, tag="2")]
        pub extension: ::prost::alloc::vec::Vec<super::super::proto::Extension>,
        /// Extensions that cannot be ignored
        #[prost(message, repeated, tag="3")]
        pub modifier_extension: ::prost::alloc::vec::Vec<super::super::proto::Extension>,
        /// Results of the activity
        #[prost(message, repeated, tag="4")]
        pub outcome_codeable_concept: ::prost::alloc::vec::Vec<super::super::proto::CodeableConcept>,
        /// Appointment, Encounter, Procedure, etc.
        #[prost(message, repeated, tag="5")]
        pub outcome_reference: ::prost::alloc::vec::Vec<super::super::proto::Reference>,
        /// Comments about the activity status/progress
        #[prost(message, repeated, tag="6")]
        pub progress: ::prost::alloc::vec::Vec<super::super::proto::Annotation>,
        /// Activity details defined in specific resource
        #[prost(message, optional, tag="7")]
        pub reference: ::core::option::Option<super::super::proto::Reference>,
        #[prost(message, optional, tag="8")]
        pub detail: ::core::option::Option<activity::Detail>,
    }
    /// Nested message and enum types in `Activity`.
    pub mod activity {
        /// In-line definition of activity
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Detail {
            /// xml:id (or equivalent in JSON)
            #[prost(message, optional, tag="1")]
            pub id: ::core::option::Option<super::super::super::proto::String>,
            /// Additional Content defined by implementations
            #[prost(message, repeated, tag="2")]
            pub extension: ::prost::alloc::vec::Vec<super::super::super::proto::Extension>,
            /// Extensions that cannot be ignored
            #[prost(message, repeated, tag="3")]
            pub modifier_extension: ::prost::alloc::vec::Vec<super::super::super::proto::Extension>,
            /// diet | drug | encounter | observation | procedure | supply | other
            #[prost(message, optional, tag="4")]
            pub category: ::core::option::Option<super::super::super::proto::CodeableConcept>,
            /// Protocol or definition
            #[prost(message, optional, tag="5")]
            pub definition: ::core::option::Option<super::super::super::proto::Reference>,
            /// Detail type of activity
            #[prost(message, optional, tag="6")]
            pub code: ::core::option::Option<super::super::super::proto::CodeableConcept>,
            /// Why activity should be done or why activity was prohibited
            #[prost(message, repeated, tag="7")]
            pub reason_code: ::prost::alloc::vec::Vec<super::super::super::proto::CodeableConcept>,
            /// Condition triggering need for activity
            #[prost(message, repeated, tag="8")]
            pub reason_reference: ::prost::alloc::vec::Vec<super::super::super::proto::Reference>,
            /// Goals this activity relates to
            #[prost(message, repeated, tag="9")]
            pub goal: ::prost::alloc::vec::Vec<super::super::super::proto::Reference>,
            /// not-started | scheduled | in-progress | on-hold | completed | cancelled
            /// | unknown
            #[prost(message, optional, tag="10")]
            pub status: ::core::option::Option<super::super::super::proto::CarePlanActivityStatusCode>,
            /// Reason for current status
            #[prost(message, optional, tag="11")]
            pub status_reason: ::core::option::Option<super::super::super::proto::String>,
            /// Do NOT do
            #[prost(message, optional, tag="12")]
            pub prohibited: ::core::option::Option<super::super::super::proto::Boolean>,
            #[prost(message, optional, tag="13")]
            pub scheduled: ::core::option::Option<detail::Scheduled>,
            /// Where it should happen
            #[prost(message, optional, tag="14")]
            pub location: ::core::option::Option<super::super::super::proto::Reference>,
            /// Who will be responsible?
            #[prost(message, repeated, tag="15")]
            pub performer: ::prost::alloc::vec::Vec<super::super::super::proto::Reference>,
            #[prost(message, optional, tag="16")]
            pub product: ::core::option::Option<detail::Product>,
            /// How to consume/day?
            #[prost(message, optional, tag="17")]
            pub daily_amount: ::core::option::Option<super::super::super::proto::SimpleQuantity>,
            /// How much to administer/supply/consume
            #[prost(message, optional, tag="18")]
            pub quantity: ::core::option::Option<super::super::super::proto::SimpleQuantity>,
            /// Extra info describing activity to perform
            #[prost(message, optional, tag="19")]
            pub description: ::core::option::Option<super::super::super::proto::String>,
        }
        /// Nested message and enum types in `Detail`.
        pub mod detail {
            /// When activity is to occur
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct Scheduled {
                #[prost(oneof="scheduled::Scheduled", tags="1, 2, 3")]
                pub scheduled: ::core::option::Option<scheduled::Scheduled>,
            }
            /// Nested message and enum types in `Scheduled`.
            pub mod scheduled {
                #[derive(Clone, PartialEq, ::prost::Oneof)]
                pub enum Scheduled {
                    #[prost(message, tag="1")]
                    Timing(super::super::super::super::super::proto::Timing),
                    #[prost(message, tag="2")]
                    Period(super::super::super::super::super::proto::Period),
                    #[prost(message, tag="3")]
                    StringValue(super::super::super::super::super::proto::String),
                }
            }
            /// What is to be administered/supplied
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct Product {
                #[prost(oneof="product::Product", tags="1, 2")]
                pub product: ::core::option::Option<product::Product>,
            }
            /// Nested message and enum types in `Product`.
            pub mod product {
                #[derive(Clone, PartialEq, ::prost::Oneof)]
                pub enum Product {
                    #[prost(message, tag="1")]
                    CodeableConcept(super::super::super::super::super::proto::CodeableConcept),
                    #[prost(message, tag="2")]
                    Reference(super::super::super::super::super::proto::Reference),
                }
            }
        }
    }
}
/// Auto-generated from StructureDefinition for US Core CareTeam Profile.
/// US Core CareTeam Profile.
/// See <http://hl7.org/fhir/us/core/StructureDefinition/us-core-careteam>
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UsCoreCareteam {
    /// Logical id of this artifact
    #[prost(message, optional, tag="1")]
    pub id: ::core::option::Option<super::proto::Id>,
    /// Metadata about the resource
    #[prost(message, optional, tag="2")]
    pub meta: ::core::option::Option<super::proto::Meta>,
    /// A set of rules under which this content was created
    #[prost(message, optional, tag="3")]
    pub implicit_rules: ::core::option::Option<super::proto::Uri>,
    /// Language of the resource content
    #[prost(message, optional, tag="4")]
    pub language: ::core::option::Option<super::proto::LanguageCode>,
    /// Text summary of the resource, for human interpretation
    #[prost(message, optional, tag="5")]
    pub text: ::core::option::Option<super::proto::Narrative>,
    /// Contained, inline Resources
    #[prost(message, repeated, tag="6")]
    pub contained: ::prost::alloc::vec::Vec<super::proto::ContainedResource>,
    /// Additional Content defined by implementations
    #[prost(message, repeated, tag="7")]
    pub extension: ::prost::alloc::vec::Vec<super::proto::Extension>,
    /// Extensions that cannot be ignored
    #[prost(message, repeated, tag="8")]
    pub modifier_extension: ::prost::alloc::vec::Vec<super::proto::Extension>,
    /// External Ids for this team
    #[prost(message, repeated, tag="9")]
    pub identifier: ::prost::alloc::vec::Vec<super::proto::Identifier>,
    /// proposed | active | suspended | inactive | entered-in-error
    #[prost(message, optional, tag="10")]
    pub status: ::core::option::Option<super::proto::CareTeamStatusCode>,
    /// Type of team
    #[prost(message, repeated, tag="11")]
    pub category: ::prost::alloc::vec::Vec<super::proto::CodeableConcept>,
    /// Name of the team, such as crisis assessment team
    #[prost(message, optional, tag="12")]
    pub name: ::core::option::Option<super::proto::String>,
    /// Who care team is for
    #[prost(message, optional, tag="13")]
    pub subject: ::core::option::Option<super::proto::Reference>,
    /// Encounter or episode associated with CareTeam
    #[prost(message, optional, tag="14")]
    pub context: ::core::option::Option<super::proto::Reference>,
    /// Time period team covers
    #[prost(message, optional, tag="15")]
    pub period: ::core::option::Option<super::proto::Period>,
    #[prost(message, repeated, tag="16")]
    pub participant: ::prost::alloc::vec::Vec<us_core_careteam::Participant>,
    /// Why the care team exists
    #[prost(message, repeated, tag="17")]
    pub reason_code: ::prost::alloc::vec::Vec<super::proto::CodeableConcept>,
    /// Why the care team exists
    #[prost(message, repeated, tag="18")]
    pub reason_reference: ::prost::alloc::vec::Vec<super::proto::Reference>,
    /// Organization responsible for the care team
    #[prost(message, repeated, tag="19")]
    pub managing_organization: ::prost::alloc::vec::Vec<super::proto::Reference>,
    /// Comments made about the CareTeam
    #[prost(message, repeated, tag="20")]
    pub note: ::prost::alloc::vec::Vec<super::proto::Annotation>,
}
/// Nested message and enum types in `UsCoreCareteam`.
pub mod us_core_careteam {
    /// Members of the team
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Participant {
        /// xml:id (or equivalent in JSON)
        #[prost(message, optional, tag="1")]
        pub id: ::core::option::Option<super::super::proto::String>,
        /// Additional Content defined by implementations
        #[prost(message, repeated, tag="2")]
        pub extension: ::prost::alloc::vec::Vec<super::super::proto::Extension>,
        /// Extensions that cannot be ignored
        #[prost(message, repeated, tag="3")]
        pub modifier_extension: ::prost::alloc::vec::Vec<super::super::proto::Extension>,
        /// Type of involvement
        #[prost(message, optional, tag="4")]
        pub role: ::core::option::Option<super::super::proto::CodeableConcept>,
        /// Who is involved
        #[prost(message, optional, tag="5")]
        pub member: ::core::option::Option<super::super::proto::Reference>,
        /// Organization of the practitioner
        #[prost(message, optional, tag="6")]
        pub on_behalf_of: ::core::option::Option<super::super::proto::Reference>,
        /// Time period of participant
        #[prost(message, optional, tag="7")]
        pub period: ::core::option::Option<super::super::proto::Period>,
    }
}
/// Auto-generated from StructureDefinition for US Core Condition Profile.
/// US Core Condition Profile.
/// See <http://hl7.org/fhir/us/core/StructureDefinition/us-core-condition>
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UsCoreCondition {
    /// Logical id of this artifact
    #[prost(message, optional, tag="1")]
    pub id: ::core::option::Option<super::proto::Id>,
    /// Metadata about the resource
    #[prost(message, optional, tag="2")]
    pub meta: ::core::option::Option<super::proto::Meta>,
    /// A set of rules under which this content was created
    #[prost(message, optional, tag="3")]
    pub implicit_rules: ::core::option::Option<super::proto::Uri>,
    /// Language of the resource content
    #[prost(message, optional, tag="4")]
    pub language: ::core::option::Option<super::proto::LanguageCode>,
    /// Text summary of the resource, for human interpretation
    #[prost(message, optional, tag="5")]
    pub text: ::core::option::Option<super::proto::Narrative>,
    /// Contained, inline Resources
    #[prost(message, repeated, tag="6")]
    pub contained: ::prost::alloc::vec::Vec<super::proto::ContainedResource>,
    /// Additional Content defined by implementations
    #[prost(message, repeated, tag="7")]
    pub extension: ::prost::alloc::vec::Vec<super::proto::Extension>,
    /// Extensions that cannot be ignored
    #[prost(message, repeated, tag="8")]
    pub modifier_extension: ::prost::alloc::vec::Vec<super::proto::Extension>,
    /// External Ids for this condition
    #[prost(message, repeated, tag="9")]
    pub identifier: ::prost::alloc::vec::Vec<super::proto::Identifier>,
    /// active | recurrence | inactive | remission | resolved
    #[prost(message, optional, tag="10")]
    pub clinical_status: ::core::option::Option<super::proto::ConditionClinicalStatusCodesCode>,
    /// provisional | differential | confirmed | refuted | entered-in-error |
    /// unknown
    #[prost(message, optional, tag="11")]
    pub verification_status: ::core::option::Option<super::proto::ConditionVerificationStatusCode>,
    /// problem-list-item | encounter-diagnosis
    #[prost(message, repeated, tag="12")]
    pub category: ::prost::alloc::vec::Vec<super::proto::CodeableConcept>,
    /// Subjective severity of condition
    #[prost(message, optional, tag="13")]
    pub severity: ::core::option::Option<super::proto::CodeableConcept>,
    /// Identification of the condition, problem or diagnosis
    #[prost(message, optional, tag="14")]
    pub code: ::core::option::Option<super::proto::CodeableConcept>,
    /// Anatomical location, if relevant
    #[prost(message, repeated, tag="15")]
    pub body_site: ::prost::alloc::vec::Vec<super::proto::CodeableConcept>,
    /// Who has the condition?
    #[prost(message, optional, tag="16")]
    pub subject: ::core::option::Option<super::proto::Reference>,
    /// Encounter or episode when condition first asserted
    #[prost(message, optional, tag="17")]
    pub context: ::core::option::Option<super::proto::Reference>,
    #[prost(message, optional, tag="18")]
    pub onset: ::core::option::Option<us_core_condition::Onset>,
    #[prost(message, optional, tag="19")]
    pub abatement: ::core::option::Option<us_core_condition::Abatement>,
    /// Date record was believed accurate
    #[prost(message, optional, tag="20")]
    pub asserted_date: ::core::option::Option<super::proto::DateTime>,
    /// Person who asserts this condition
    #[prost(message, optional, tag="21")]
    pub asserter: ::core::option::Option<super::proto::Reference>,
    #[prost(message, optional, tag="22")]
    pub stage: ::core::option::Option<us_core_condition::Stage>,
    #[prost(message, repeated, tag="23")]
    pub evidence: ::prost::alloc::vec::Vec<us_core_condition::Evidence>,
    /// Additional information about the Condition
    #[prost(message, repeated, tag="24")]
    pub note: ::prost::alloc::vec::Vec<super::proto::Annotation>,
}
/// Nested message and enum types in `UsCoreCondition`.
pub mod us_core_condition {
    /// Estimated or actual date,  date-time, or age
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Onset {
        #[prost(oneof="onset::Onset", tags="1, 2, 3, 4, 5")]
        pub onset: ::core::option::Option<onset::Onset>,
    }
    /// Nested message and enum types in `Onset`.
    pub mod onset {
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Onset {
            #[prost(message, tag="1")]
            DateTime(super::super::super::proto::DateTime),
            #[prost(message, tag="2")]
            Age(super::super::super::proto::Age),
            #[prost(message, tag="3")]
            Period(super::super::super::proto::Period),
            #[prost(message, tag="4")]
            Range(super::super::super::proto::Range),
            #[prost(message, tag="5")]
            StringValue(super::super::super::proto::String),
        }
    }
    /// If/when in resolution/remission
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Abatement {
        #[prost(oneof="abatement::Abatement", tags="1, 2, 3, 4, 5, 6")]
        pub abatement: ::core::option::Option<abatement::Abatement>,
    }
    /// Nested message and enum types in `Abatement`.
    pub mod abatement {
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Abatement {
            #[prost(message, tag="1")]
            DateTime(super::super::super::proto::DateTime),
            #[prost(message, tag="2")]
            Age(super::super::super::proto::Age),
            #[prost(message, tag="3")]
            Boolean(super::super::super::proto::Boolean),
            #[prost(message, tag="4")]
            Period(super::super::super::proto::Period),
            #[prost(message, tag="5")]
            Range(super::super::super::proto::Range),
            #[prost(message, tag="6")]
            StringValue(super::super::super::proto::String),
        }
    }
    /// Stage/grade, usually assessed formally
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Stage {
        /// xml:id (or equivalent in JSON)
        #[prost(message, optional, tag="1")]
        pub id: ::core::option::Option<super::super::proto::String>,
        /// Additional Content defined by implementations
        #[prost(message, repeated, tag="2")]
        pub extension: ::prost::alloc::vec::Vec<super::super::proto::Extension>,
        /// Extensions that cannot be ignored
        #[prost(message, repeated, tag="3")]
        pub modifier_extension: ::prost::alloc::vec::Vec<super::super::proto::Extension>,
        /// Simple summary (disease specific)
        #[prost(message, optional, tag="4")]
        pub summary: ::core::option::Option<super::super::proto::CodeableConcept>,
        /// Formal record of assessment
        #[prost(message, repeated, tag="5")]
        pub assessment: ::prost::alloc::vec::Vec<super::super::proto::Reference>,
    }
    /// Supporting evidence
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Evidence {
        /// xml:id (or equivalent in JSON)
        #[prost(message, optional, tag="1")]
        pub id: ::core::option::Option<super::super::proto::String>,
        /// Additional Content defined by implementations
        #[prost(message, repeated, tag="2")]
        pub extension: ::prost::alloc::vec::Vec<super::super::proto::Extension>,
        /// Extensions that cannot be ignored
        #[prost(message, repeated, tag="3")]
        pub modifier_extension: ::prost::alloc::vec::Vec<super::super::proto::Extension>,
        /// Manifestation/symptom
        #[prost(message, repeated, tag="4")]
        pub code: ::prost::alloc::vec::Vec<super::super::proto::CodeableConcept>,
        /// Supporting information found elsewhere
        #[prost(message, repeated, tag="5")]
        pub detail: ::prost::alloc::vec::Vec<super::super::proto::Reference>,
    }
}
/// Auto-generated from StructureDefinition for US Core Implanted Device Profile.
/// US Core Implanted Device Profile.
/// See <http://hl7.org/fhir/us/core/StructureDefinition/us-core-device>
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UsCoreDevice {
    /// Logical id of this artifact
    #[prost(message, optional, tag="1")]
    pub id: ::core::option::Option<super::proto::Id>,
    /// Metadata about the resource
    #[prost(message, optional, tag="2")]
    pub meta: ::core::option::Option<super::proto::Meta>,
    /// A set of rules under which this content was created
    #[prost(message, optional, tag="3")]
    pub implicit_rules: ::core::option::Option<super::proto::Uri>,
    /// Language of the resource content
    #[prost(message, optional, tag="4")]
    pub language: ::core::option::Option<super::proto::LanguageCode>,
    /// Text summary of the resource, for human interpretation
    #[prost(message, optional, tag="5")]
    pub text: ::core::option::Option<super::proto::Narrative>,
    /// Contained, inline Resources
    #[prost(message, repeated, tag="6")]
    pub contained: ::prost::alloc::vec::Vec<super::proto::ContainedResource>,
    /// Additional Content defined by implementations
    #[prost(message, repeated, tag="7")]
    pub extension: ::prost::alloc::vec::Vec<super::proto::Extension>,
    /// Extensions that cannot be ignored
    #[prost(message, repeated, tag="8")]
    pub modifier_extension: ::prost::alloc::vec::Vec<super::proto::Extension>,
    /// Instance identifier
    #[prost(message, repeated, tag="9")]
    pub identifier: ::prost::alloc::vec::Vec<super::proto::Identifier>,
    #[prost(message, optional, tag="10")]
    pub udi: ::core::option::Option<us_core_device::Udi>,
    /// active | inactive | entered-in-error | unknown
    #[prost(message, optional, tag="11")]
    pub status: ::core::option::Option<super::proto::FhirDeviceStatusCode>,
    /// What kind of device this is
    #[prost(message, optional, tag="12")]
    pub r#type: ::core::option::Option<super::proto::CodeableConcept>,
    /// Lot number of manufacture
    #[prost(message, optional, tag="13")]
    pub lot_number: ::core::option::Option<super::proto::String>,
    /// Name of device manufacturer
    #[prost(message, optional, tag="14")]
    pub manufacturer: ::core::option::Option<super::proto::String>,
    /// Date when the device was made
    #[prost(message, optional, tag="15")]
    pub manufacture_date: ::core::option::Option<super::proto::DateTime>,
    /// Date and time of expiry of this device (if applicable)
    #[prost(message, optional, tag="16")]
    pub expiration_date: ::core::option::Option<super::proto::DateTime>,
    /// Model id assigned by the manufacturer
    #[prost(message, optional, tag="17")]
    pub model: ::core::option::Option<super::proto::String>,
    /// Version number (i.e. software)
    #[prost(message, optional, tag="18")]
    pub version: ::core::option::Option<super::proto::String>,
    /// Patient to whom Device is affixed
    #[prost(message, optional, tag="19")]
    pub patient: ::core::option::Option<super::proto::Reference>,
    /// Organization responsible for device
    #[prost(message, optional, tag="20")]
    pub owner: ::core::option::Option<super::proto::Reference>,
    /// Details for human/organization for support
    #[prost(message, repeated, tag="21")]
    pub contact: ::prost::alloc::vec::Vec<super::proto::ContactPoint>,
    /// Where the resource is found
    #[prost(message, optional, tag="22")]
    pub location: ::core::option::Option<super::proto::Reference>,
    /// Network address to contact device
    #[prost(message, optional, tag="23")]
    pub url: ::core::option::Option<super::proto::Uri>,
    /// Device notes and comments
    #[prost(message, repeated, tag="24")]
    pub note: ::prost::alloc::vec::Vec<super::proto::Annotation>,
    /// Safety Characteristics of Device
    #[prost(message, repeated, tag="25")]
    pub safety: ::prost::alloc::vec::Vec<super::proto::CodeableConcept>,
}
/// Nested message and enum types in `UsCoreDevice`.
pub mod us_core_device {
    /// Unique Device Identifier (UDI) Barcode string
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Udi {
        /// xml:id (or equivalent in JSON)
        #[prost(message, optional, tag="1")]
        pub id: ::core::option::Option<super::super::proto::String>,
        /// Additional Content defined by implementations
        #[prost(message, repeated, tag="2")]
        pub extension: ::prost::alloc::vec::Vec<super::super::proto::Extension>,
        /// Extensions that cannot be ignored
        #[prost(message, repeated, tag="3")]
        pub modifier_extension: ::prost::alloc::vec::Vec<super::super::proto::Extension>,
        /// Mandatory fixed portion of UDI
        #[prost(message, optional, tag="4")]
        pub device_identifier: ::core::option::Option<super::super::proto::String>,
        /// Device Name as appears on UDI label
        #[prost(message, optional, tag="5")]
        pub name: ::core::option::Option<super::super::proto::String>,
        /// Regional UDI authority
        #[prost(message, optional, tag="6")]
        pub jurisdiction: ::core::option::Option<super::super::proto::Uri>,
        /// UDI Human Readable Barcode String
        #[prost(message, optional, tag="7")]
        pub carrier_hrf: ::core::option::Option<super::super::proto::String>,
        /// UDI Machine Readable Barcode String
        #[prost(message, optional, tag="8")]
        pub carrier_aidc: ::core::option::Option<super::super::proto::Base64Binary>,
        /// UDI Issuing Organization
        #[prost(message, optional, tag="9")]
        pub issuer: ::core::option::Option<super::super::proto::Uri>,
        /// barcode | rfid | manual +
        #[prost(message, optional, tag="10")]
        pub entry_type: ::core::option::Option<super::super::proto::UdiEntryTypeCode>,
    }
}
/// Auto-generated from StructureDefinition for US Core Diagnostic Report
/// Profile. US Core Diagnostic Report Profile. See
/// <http://hl7.org/fhir/us/core/StructureDefinition/us-core-diagnosticreport>
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UsCoreDiagnosticreport {
    /// Logical id of this artifact
    #[prost(message, optional, tag="1")]
    pub id: ::core::option::Option<super::proto::Id>,
    /// Metadata about the resource
    #[prost(message, optional, tag="2")]
    pub meta: ::core::option::Option<super::proto::Meta>,
    /// A set of rules under which this content was created
    #[prost(message, optional, tag="3")]
    pub implicit_rules: ::core::option::Option<super::proto::Uri>,
    /// Language of the resource content
    #[prost(message, optional, tag="4")]
    pub language: ::core::option::Option<super::proto::LanguageCode>,
    /// Text summary of the resource, for human interpretation
    #[prost(message, optional, tag="5")]
    pub text: ::core::option::Option<super::proto::Narrative>,
    /// Contained, inline Resources
    #[prost(message, repeated, tag="6")]
    pub contained: ::prost::alloc::vec::Vec<super::proto::ContainedResource>,
    /// Additional Content defined by implementations
    #[prost(message, repeated, tag="7")]
    pub extension: ::prost::alloc::vec::Vec<super::proto::Extension>,
    /// Extensions that cannot be ignored
    #[prost(message, repeated, tag="8")]
    pub modifier_extension: ::prost::alloc::vec::Vec<super::proto::Extension>,
    /// Business identifier for report
    #[prost(message, repeated, tag="9")]
    pub identifier: ::prost::alloc::vec::Vec<super::proto::Identifier>,
    /// What was requested
    #[prost(message, repeated, tag="10")]
    pub based_on: ::prost::alloc::vec::Vec<super::proto::Reference>,
    /// registered | partial | preliminary | final +
    #[prost(message, optional, tag="11")]
    pub status: ::core::option::Option<super::proto::DiagnosticReportStatusCode>,
    /// Service category
    #[prost(message, optional, tag="12")]
    pub category: ::core::option::Option<super::proto::CodeableConcept>,
    /// US Core Laboratory Report Order Code
    #[prost(message, optional, tag="13")]
    pub code: ::core::option::Option<super::proto::CodeableConcept>,
    /// The subject of the report - usually, but not always, the patient
    #[prost(message, optional, tag="14")]
    pub subject: ::core::option::Option<super::proto::Reference>,
    /// Health care event when test ordered
    #[prost(message, optional, tag="15")]
    pub context: ::core::option::Option<super::proto::Reference>,
    #[prost(message, optional, tag="16")]
    pub effective: ::core::option::Option<us_core_diagnosticreport::Effective>,
    /// DateTime this version was released
    #[prost(message, optional, tag="17")]
    pub issued: ::core::option::Option<super::proto::Instant>,
    #[prost(message, repeated, tag="18")]
    pub performer: ::prost::alloc::vec::Vec<us_core_diagnosticreport::Performer>,
    /// Specimens this report is based on
    #[prost(message, repeated, tag="19")]
    pub specimen: ::prost::alloc::vec::Vec<super::proto::Reference>,
    /// Observations - simple, or complex nested groups
    #[prost(message, repeated, tag="20")]
    pub result: ::prost::alloc::vec::Vec<super::proto::Reference>,
    /// Reference to full details of imaging associated with the diagnostic report
    #[prost(message, repeated, tag="21")]
    pub imaging_study: ::prost::alloc::vec::Vec<super::proto::Reference>,
    #[prost(message, repeated, tag="22")]
    pub image: ::prost::alloc::vec::Vec<us_core_diagnosticreport::Image>,
    /// Clinical Interpretation of test results
    #[prost(message, optional, tag="23")]
    pub conclusion: ::core::option::Option<super::proto::String>,
    /// Codes for the conclusion
    #[prost(message, repeated, tag="24")]
    pub coded_diagnosis: ::prost::alloc::vec::Vec<super::proto::CodeableConcept>,
    /// Entire report as issued
    #[prost(message, repeated, tag="25")]
    pub presented_form: ::prost::alloc::vec::Vec<super::proto::Attachment>,
}
/// Nested message and enum types in `UsCoreDiagnosticreport`.
pub mod us_core_diagnosticreport {
    /// Specimen Collection Datetime or Period
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Effective {
        #[prost(oneof="effective::Effective", tags="1, 2")]
        pub effective: ::core::option::Option<effective::Effective>,
    }
    /// Nested message and enum types in `Effective`.
    pub mod effective {
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Effective {
            #[prost(message, tag="1")]
            DateTime(super::super::super::proto::DateTime),
            #[prost(message, tag="2")]
            Period(super::super::super::proto::Period),
        }
    }
    /// Participants in producing the report
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Performer {
        /// xml:id (or equivalent in JSON)
        #[prost(message, optional, tag="1")]
        pub id: ::core::option::Option<super::super::proto::String>,
        /// Additional Content defined by implementations
        #[prost(message, repeated, tag="2")]
        pub extension: ::prost::alloc::vec::Vec<super::super::proto::Extension>,
        /// Extensions that cannot be ignored
        #[prost(message, repeated, tag="3")]
        pub modifier_extension: ::prost::alloc::vec::Vec<super::super::proto::Extension>,
        /// Type of performer
        #[prost(message, optional, tag="4")]
        pub role: ::core::option::Option<super::super::proto::CodeableConcept>,
        /// Practitioner or Organization  participant
        #[prost(message, optional, tag="5")]
        pub actor: ::core::option::Option<super::super::proto::Reference>,
    }
    /// Key images associated with this report
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Image {
        /// xml:id (or equivalent in JSON)
        #[prost(message, optional, tag="1")]
        pub id: ::core::option::Option<super::super::proto::String>,
        /// Additional Content defined by implementations
        #[prost(message, repeated, tag="2")]
        pub extension: ::prost::alloc::vec::Vec<super::super::proto::Extension>,
        /// Extensions that cannot be ignored
        #[prost(message, repeated, tag="3")]
        pub modifier_extension: ::prost::alloc::vec::Vec<super::super::proto::Extension>,
        /// Comment about the image (e.g. explanation)
        #[prost(message, optional, tag="4")]
        pub comment: ::core::option::Option<super::super::proto::String>,
        /// Reference to the image source
        #[prost(message, optional, tag="5")]
        pub link: ::core::option::Option<super::super::proto::Reference>,
    }
}
/// Auto-generated from StructureDefinition for Email is a "direct" email.
/// Email is a "direct" email.
/// See <http://hl7.org/fhir/us/core/StructureDefinition/us-core-direct>
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UsCoreDirectEmail {
    /// xml:id (or equivalent in JSON)
    #[prost(message, optional, tag="1")]
    pub id: ::core::option::Option<super::proto::String>,
    /// Additional Content defined by implementations
    #[prost(message, repeated, tag="2")]
    pub extension: ::prost::alloc::vec::Vec<super::proto::Extension>,
    /// Value of extension
    #[prost(message, optional, tag="3")]
    pub value_boolean: ::core::option::Option<super::proto::Boolean>,
}
/// Auto-generated from StructureDefinition for US Core DocumentReference
/// Profile. US Core DocumentReference Profile. See
/// <http://hl7.org/fhir/us/core/StructureDefinition/us-core-documentreference>
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UsCoreDocumentreference {
    /// Logical id of this artifact
    #[prost(message, optional, tag="1")]
    pub id: ::core::option::Option<super::proto::Id>,
    /// Metadata about the resource
    #[prost(message, optional, tag="2")]
    pub meta: ::core::option::Option<super::proto::Meta>,
    /// A set of rules under which this content was created
    #[prost(message, optional, tag="3")]
    pub implicit_rules: ::core::option::Option<super::proto::Uri>,
    /// Language of the resource content
    #[prost(message, optional, tag="4")]
    pub language: ::core::option::Option<super::proto::LanguageCode>,
    /// Text summary of the resource, for human interpretation
    #[prost(message, optional, tag="5")]
    pub text: ::core::option::Option<super::proto::Narrative>,
    /// Contained, inline Resources
    #[prost(message, repeated, tag="6")]
    pub contained: ::prost::alloc::vec::Vec<super::proto::ContainedResource>,
    /// Additional Content defined by implementations
    #[prost(message, repeated, tag="7")]
    pub extension: ::prost::alloc::vec::Vec<super::proto::Extension>,
    /// Extensions that cannot be ignored
    #[prost(message, repeated, tag="8")]
    pub modifier_extension: ::prost::alloc::vec::Vec<super::proto::Extension>,
    /// Master Version Specific Identifier
    #[prost(message, optional, tag="9")]
    pub master_identifier: ::core::option::Option<super::proto::Identifier>,
    /// Other identifiers for the document
    #[prost(message, optional, tag="10")]
    pub identifier: ::core::option::Option<super::proto::Identifier>,
    /// current | superseded | entered-in-error
    #[prost(message, optional, tag="11")]
    pub status: ::core::option::Option<super::proto::DocumentReferenceStatusCode>,
    /// preliminary | final | appended | amended | entered-in-error
    #[prost(message, optional, tag="12")]
    pub doc_status: ::core::option::Option<super::proto::CompositionStatusCode>,
    /// Kind of document (LOINC if possible)
    #[prost(message, optional, tag="13")]
    pub r#type: ::core::option::Option<super::proto::CodeableConcept>,
    /// Categorization of document
    #[prost(message, optional, tag="14")]
    pub class_value: ::core::option::Option<super::proto::CodeableConcept>,
    /// Who/what is the subject of the document
    #[prost(message, optional, tag="15")]
    pub subject: ::core::option::Option<super::proto::Reference>,
    /// Document creation time
    #[prost(message, optional, tag="16")]
    pub created: ::core::option::Option<super::proto::DateTime>,
    /// When this document reference was created
    #[prost(message, optional, tag="17")]
    pub indexed: ::core::option::Option<super::proto::Instant>,
    /// Who and/or what authored the document
    #[prost(message, repeated, tag="18")]
    pub author: ::prost::alloc::vec::Vec<super::proto::Reference>,
    /// Who/what authenticated the document
    #[prost(message, optional, tag="19")]
    pub authenticator: ::core::option::Option<super::proto::Reference>,
    /// Organization which maintains the document
    #[prost(message, optional, tag="20")]
    pub custodian: ::core::option::Option<super::proto::Reference>,
    #[prost(message, repeated, tag="21")]
    pub relates_to: ::prost::alloc::vec::Vec<us_core_documentreference::RelatesTo>,
    /// Human-readable description (title)
    #[prost(message, optional, tag="22")]
    pub description: ::core::option::Option<super::proto::String>,
    /// Document security-tags
    #[prost(message, repeated, tag="23")]
    pub security_label: ::prost::alloc::vec::Vec<super::proto::CodeableConcept>,
    #[prost(message, optional, tag="24")]
    pub content: ::core::option::Option<us_core_documentreference::Content>,
    #[prost(message, optional, tag="25")]
    pub context: ::core::option::Option<us_core_documentreference::Context>,
}
/// Nested message and enum types in `UsCoreDocumentreference`.
pub mod us_core_documentreference {
    /// Relationships to other documents
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RelatesTo {
        /// xml:id (or equivalent in JSON)
        #[prost(message, optional, tag="1")]
        pub id: ::core::option::Option<super::super::proto::String>,
        /// Additional Content defined by implementations
        #[prost(message, repeated, tag="2")]
        pub extension: ::prost::alloc::vec::Vec<super::super::proto::Extension>,
        /// Extensions that cannot be ignored
        #[prost(message, repeated, tag="3")]
        pub modifier_extension: ::prost::alloc::vec::Vec<super::super::proto::Extension>,
        /// replaces | transforms | signs | appends
        #[prost(message, optional, tag="4")]
        pub code: ::core::option::Option<super::super::proto::DocumentRelationshipTypeCode>,
        /// Target of the relationship
        #[prost(message, optional, tag="5")]
        pub target: ::core::option::Option<super::super::proto::Reference>,
    }
    /// Document referenced
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Content {
        /// xml:id (or equivalent in JSON)
        #[prost(message, optional, tag="1")]
        pub id: ::core::option::Option<super::super::proto::String>,
        /// Additional Content defined by implementations
        #[prost(message, repeated, tag="2")]
        pub extension: ::prost::alloc::vec::Vec<super::super::proto::Extension>,
        /// Extensions that cannot be ignored
        #[prost(message, repeated, tag="3")]
        pub modifier_extension: ::prost::alloc::vec::Vec<super::super::proto::Extension>,
        /// Where to access the document
        #[prost(message, optional, tag="4")]
        pub attachment: ::core::option::Option<super::super::proto::Attachment>,
        /// Format/content rules for the document
        #[prost(message, optional, tag="5")]
        pub format: ::core::option::Option<super::super::proto::Coding>,
    }
    /// Clinical context of document
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Context {
        /// xml:id (or equivalent in JSON)
        #[prost(message, optional, tag="1")]
        pub id: ::core::option::Option<super::super::proto::String>,
        /// Additional Content defined by implementations
        #[prost(message, repeated, tag="2")]
        pub extension: ::prost::alloc::vec::Vec<super::super::proto::Extension>,
        /// Extensions that cannot be ignored
        #[prost(message, repeated, tag="3")]
        pub modifier_extension: ::prost::alloc::vec::Vec<super::super::proto::Extension>,
        /// Context of the document  content
        #[prost(message, optional, tag="4")]
        pub encounter: ::core::option::Option<super::super::proto::Reference>,
        /// Main clinical acts documented
        #[prost(message, repeated, tag="5")]
        pub event: ::prost::alloc::vec::Vec<super::super::proto::CodeableConcept>,
        /// Time of service that is being documented
        #[prost(message, optional, tag="6")]
        pub period: ::core::option::Option<super::super::proto::Period>,
        /// Kind of facility where patient was seen
        #[prost(message, optional, tag="7")]
        pub facility_type: ::core::option::Option<super::super::proto::CodeableConcept>,
        /// Additional details about where the content was created (e.g. clinical
        /// specialty)
        #[prost(message, optional, tag="8")]
        pub practice_setting: ::core::option::Option<super::super::proto::CodeableConcept>,
        /// Patient demographics from source
        #[prost(message, optional, tag="9")]
        pub source_patient_info: ::core::option::Option<super::super::proto::Reference>,
        #[prost(message, repeated, tag="10")]
        pub related: ::prost::alloc::vec::Vec<context::Related>,
    }
    /// Nested message and enum types in `Context`.
    pub mod context {
        /// Related identifiers or resources
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Related {
            /// xml:id (or equivalent in JSON)
            #[prost(message, optional, tag="1")]
            pub id: ::core::option::Option<super::super::super::proto::String>,
            /// Additional Content defined by implementations
            #[prost(message, repeated, tag="2")]
            pub extension: ::prost::alloc::vec::Vec<super::super::super::proto::Extension>,
            /// Extensions that cannot be ignored
            #[prost(message, repeated, tag="3")]
            pub modifier_extension: ::prost::alloc::vec::Vec<super::super::super::proto::Extension>,
            /// Identifier of related objects or events
            #[prost(message, optional, tag="4")]
            pub identifier: ::core::option::Option<super::super::super::proto::Identifier>,
            /// Related Resource
            #[prost(message, optional, tag="5")]
            pub r#ref: ::core::option::Option<super::super::super::proto::Reference>,
        }
    }
}
/// Auto-generated from StructureDefinition for US Core Encounter Profile.
/// US Core Encounter Profile.
/// See <http://hl7.org/fhir/us/core/StructureDefinition/us-core-encounter>
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UsCoreEncounter {
    /// Logical id of this artifact
    #[prost(message, optional, tag="1")]
    pub id: ::core::option::Option<super::proto::Id>,
    /// Metadata about the resource
    #[prost(message, optional, tag="2")]
    pub meta: ::core::option::Option<super::proto::Meta>,
    /// A set of rules under which this content was created
    #[prost(message, optional, tag="3")]
    pub implicit_rules: ::core::option::Option<super::proto::Uri>,
    /// Language of the resource content
    #[prost(message, optional, tag="4")]
    pub language: ::core::option::Option<super::proto::LanguageCode>,
    /// Text summary of the resource, for human interpretation
    #[prost(message, optional, tag="5")]
    pub text: ::core::option::Option<super::proto::Narrative>,
    /// Contained, inline Resources
    #[prost(message, repeated, tag="6")]
    pub contained: ::prost::alloc::vec::Vec<super::proto::ContainedResource>,
    /// Additional Content defined by implementations
    #[prost(message, repeated, tag="7")]
    pub extension: ::prost::alloc::vec::Vec<super::proto::Extension>,
    /// Extensions that cannot be ignored
    #[prost(message, repeated, tag="8")]
    pub modifier_extension: ::prost::alloc::vec::Vec<super::proto::Extension>,
    /// Identifier(s) by which this encounter is known
    #[prost(message, repeated, tag="9")]
    pub identifier: ::prost::alloc::vec::Vec<super::proto::Identifier>,
    /// planned | arrived | triaged | in-progress | onleave | finished | cancelled
    /// +
    #[prost(message, optional, tag="10")]
    pub status: ::core::option::Option<super::proto::EncounterStatusCode>,
    #[prost(message, repeated, tag="11")]
    pub status_history: ::prost::alloc::vec::Vec<us_core_encounter::StatusHistory>,
    /// inpatient | outpatient | ambulatory | emergency +
    #[prost(message, optional, tag="12")]
    pub class_value: ::core::option::Option<super::proto::Coding>,
    #[prost(message, repeated, tag="13")]
    pub class_history: ::prost::alloc::vec::Vec<us_core_encounter::ClassHistory>,
    /// Specific type of encounter
    #[prost(message, repeated, tag="14")]
    pub r#type: ::prost::alloc::vec::Vec<super::proto::CodeableConcept>,
    /// Indicates the urgency of the encounter
    #[prost(message, optional, tag="15")]
    pub priority: ::core::option::Option<super::proto::CodeableConcept>,
    /// The patient ro group present at the encounter
    #[prost(message, optional, tag="16")]
    pub subject: ::core::option::Option<super::proto::Reference>,
    /// Episode(s) of care that this encounter should be recorded against
    #[prost(message, repeated, tag="17")]
    pub episode_of_care: ::prost::alloc::vec::Vec<super::proto::Reference>,
    /// The ReferralRequest that initiated this encounter
    #[prost(message, repeated, tag="18")]
    pub incoming_referral: ::prost::alloc::vec::Vec<super::proto::Reference>,
    #[prost(message, repeated, tag="19")]
    pub participant: ::prost::alloc::vec::Vec<us_core_encounter::Participant>,
    /// The appointment that scheduled this encounter
    #[prost(message, optional, tag="20")]
    pub appointment: ::core::option::Option<super::proto::Reference>,
    /// The start and end time of the encounter
    #[prost(message, optional, tag="21")]
    pub period: ::core::option::Option<super::proto::Period>,
    /// Quantity of time the encounter lasted (less time absent)
    #[prost(message, optional, tag="22")]
    pub length: ::core::option::Option<super::proto::Duration>,
    /// Reason the encounter takes place (code)
    #[prost(message, repeated, tag="23")]
    pub reason: ::prost::alloc::vec::Vec<super::proto::CodeableConcept>,
    #[prost(message, repeated, tag="24")]
    pub diagnosis: ::prost::alloc::vec::Vec<us_core_encounter::Diagnosis>,
    /// The set of accounts that may be used for billing for this Encounter
    #[prost(message, repeated, tag="25")]
    pub account: ::prost::alloc::vec::Vec<super::proto::Reference>,
    #[prost(message, optional, tag="26")]
    pub hospitalization: ::core::option::Option<us_core_encounter::Hospitalization>,
    #[prost(message, repeated, tag="27")]
    pub location: ::prost::alloc::vec::Vec<us_core_encounter::Location>,
    /// The custodian organization of this Encounter record
    #[prost(message, optional, tag="28")]
    pub service_provider: ::core::option::Option<super::proto::Reference>,
    /// Another Encounter this encounter is part of
    #[prost(message, optional, tag="29")]
    pub part_of: ::core::option::Option<super::proto::Reference>,
}
/// Nested message and enum types in `UsCoreEncounter`.
pub mod us_core_encounter {
    /// List of past encounter statuses
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct StatusHistory {
        /// xml:id (or equivalent in JSON)
        #[prost(message, optional, tag="1")]
        pub id: ::core::option::Option<super::super::proto::String>,
        /// Additional Content defined by implementations
        #[prost(message, repeated, tag="2")]
        pub extension: ::prost::alloc::vec::Vec<super::super::proto::Extension>,
        /// Extensions that cannot be ignored
        #[prost(message, repeated, tag="3")]
        pub modifier_extension: ::prost::alloc::vec::Vec<super::super::proto::Extension>,
        /// planned | arrived | triaged | in-progress | onleave | finished |
        /// cancelled +
        #[prost(message, optional, tag="4")]
        pub status: ::core::option::Option<super::super::proto::EncounterStatusCode>,
        /// The time that the episode was in the specified status
        #[prost(message, optional, tag="5")]
        pub period: ::core::option::Option<super::super::proto::Period>,
    }
    /// List of past encounter classes
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ClassHistory {
        /// xml:id (or equivalent in JSON)
        #[prost(message, optional, tag="1")]
        pub id: ::core::option::Option<super::super::proto::String>,
        /// Additional Content defined by implementations
        #[prost(message, repeated, tag="2")]
        pub extension: ::prost::alloc::vec::Vec<super::super::proto::Extension>,
        /// Extensions that cannot be ignored
        #[prost(message, repeated, tag="3")]
        pub modifier_extension: ::prost::alloc::vec::Vec<super::super::proto::Extension>,
        /// inpatient | outpatient | ambulatory | emergency +
        #[prost(message, optional, tag="4")]
        pub class_value: ::core::option::Option<super::super::proto::Coding>,
        /// The time that the episode was in the specified class
        #[prost(message, optional, tag="5")]
        pub period: ::core::option::Option<super::super::proto::Period>,
    }
    /// List of participants involved in the encounter
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Participant {
        /// xml:id (or equivalent in JSON)
        #[prost(message, optional, tag="1")]
        pub id: ::core::option::Option<super::super::proto::String>,
        /// Additional Content defined by implementations
        #[prost(message, repeated, tag="2")]
        pub extension: ::prost::alloc::vec::Vec<super::super::proto::Extension>,
        /// Extensions that cannot be ignored
        #[prost(message, repeated, tag="3")]
        pub modifier_extension: ::prost::alloc::vec::Vec<super::super::proto::Extension>,
        /// Role of participant in encounter
        #[prost(message, repeated, tag="4")]
        pub r#type: ::prost::alloc::vec::Vec<super::super::proto::CodeableConcept>,
        /// Period of time during the encounter that the participant participated
        #[prost(message, optional, tag="5")]
        pub period: ::core::option::Option<super::super::proto::Period>,
        /// Persons involved in the encounter other than the patient
        #[prost(message, optional, tag="6")]
        pub individual: ::core::option::Option<super::super::proto::Reference>,
    }
    /// The list of diagnosis relevant to this encounter
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Diagnosis {
        /// xml:id (or equivalent in JSON)
        #[prost(message, optional, tag="1")]
        pub id: ::core::option::Option<super::super::proto::String>,
        /// Additional Content defined by implementations
        #[prost(message, repeated, tag="2")]
        pub extension: ::prost::alloc::vec::Vec<super::super::proto::Extension>,
        /// Extensions that cannot be ignored
        #[prost(message, repeated, tag="3")]
        pub modifier_extension: ::prost::alloc::vec::Vec<super::super::proto::Extension>,
        /// Reason the encounter takes place (resource)
        #[prost(message, optional, tag="4")]
        pub condition: ::core::option::Option<super::super::proto::Reference>,
        /// Role that this diagnosis has within the encounter (e.g. admission,
        /// billing, discharge …)
        #[prost(message, optional, tag="5")]
        pub role: ::core::option::Option<super::super::proto::CodeableConcept>,
        /// Ranking of the diagnosis (for each role type)
        #[prost(message, optional, tag="6")]
        pub rank: ::core::option::Option<super::super::proto::PositiveInt>,
    }
    /// Details about the admission to a healthcare service
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Hospitalization {
        /// xml:id (or equivalent in JSON)
        #[prost(message, optional, tag="1")]
        pub id: ::core::option::Option<super::super::proto::String>,
        /// Additional Content defined by implementations
        #[prost(message, repeated, tag="2")]
        pub extension: ::prost::alloc::vec::Vec<super::super::proto::Extension>,
        /// Extensions that cannot be ignored
        #[prost(message, repeated, tag="3")]
        pub modifier_extension: ::prost::alloc::vec::Vec<super::super::proto::Extension>,
        /// Pre-admission identifier
        #[prost(message, optional, tag="4")]
        pub pre_admission_identifier: ::core::option::Option<super::super::proto::Identifier>,
        /// The location from which the patient came before admission
        #[prost(message, optional, tag="5")]
        pub origin: ::core::option::Option<super::super::proto::Reference>,
        /// From where patient was admitted (physician referral, transfer)
        #[prost(message, optional, tag="6")]
        pub admit_source: ::core::option::Option<super::super::proto::CodeableConcept>,
        /// The type of hospital re-admission that has occurred (if any). If the
        /// value is absent, then this is not identified as a readmission
        #[prost(message, optional, tag="7")]
        pub re_admission: ::core::option::Option<super::super::proto::CodeableConcept>,
        /// Diet preferences reported by the patient
        #[prost(message, repeated, tag="8")]
        pub diet_preference: ::prost::alloc::vec::Vec<super::super::proto::CodeableConcept>,
        /// Special courtesies (VIP, board member)
        #[prost(message, repeated, tag="9")]
        pub special_courtesy: ::prost::alloc::vec::Vec<super::super::proto::CodeableConcept>,
        /// Wheelchair, translator, stretcher, etc.
        #[prost(message, repeated, tag="10")]
        pub special_arrangement: ::prost::alloc::vec::Vec<super::super::proto::CodeableConcept>,
        /// Location to which the patient is discharged
        #[prost(message, optional, tag="11")]
        pub destination: ::core::option::Option<super::super::proto::Reference>,
        /// Category or kind of location after discharge
        #[prost(message, optional, tag="12")]
        pub discharge_disposition: ::core::option::Option<super::super::proto::CodeableConcept>,
    }
    /// List of locations where the patient has been
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Location {
        /// xml:id (or equivalent in JSON)
        #[prost(message, optional, tag="1")]
        pub id: ::core::option::Option<super::super::proto::String>,
        /// Additional Content defined by implementations
        #[prost(message, repeated, tag="2")]
        pub extension: ::prost::alloc::vec::Vec<super::super::proto::Extension>,
        /// Extensions that cannot be ignored
        #[prost(message, repeated, tag="3")]
        pub modifier_extension: ::prost::alloc::vec::Vec<super::super::proto::Extension>,
        /// Location the encounter takes place
        #[prost(message, optional, tag="4")]
        pub location: ::core::option::Option<super::super::proto::Reference>,
        /// planned | active | reserved | completed
        #[prost(message, optional, tag="5")]
        pub status: ::core::option::Option<super::super::proto::EncounterLocationStatusCode>,
        /// Time period during which the patient was present at the location
        #[prost(message, optional, tag="6")]
        pub period: ::core::option::Option<super::super::proto::Period>,
    }
}
/// Auto-generated from StructureDefinition for US Core ethnicity Extension.
/// US Core ethnicity Extension.
/// See <http://hl7.org/fhir/us/core/StructureDefinition/us-core-ethnicity>
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PatientUsCoreEthnicityExtension {
    /// xml:id (or equivalent in JSON)
    #[prost(message, optional, tag="1")]
    pub id: ::core::option::Option<super::proto::String>,
    /// Extension
    #[prost(message, repeated, tag="2")]
    pub extension: ::prost::alloc::vec::Vec<super::proto::Extension>,
    /// Hispanic or Latino|Not Hispanic or Latino
    #[prost(message, optional, tag="4")]
    pub omb_category: ::core::option::Option<super::proto::Coding>,
    /// Extended ethnicity codes
    #[prost(message, repeated, tag="5")]
    pub detailed: ::prost::alloc::vec::Vec<super::proto::Coding>,
    /// ethnicity Text
    #[prost(message, optional, tag="6")]
    pub text: ::core::option::Option<super::proto::String>,
}
/// Auto-generated from StructureDefinition for US Core Goal Profile.
/// US Core Goal Profile.
/// See <http://hl7.org/fhir/us/core/StructureDefinition/us-core-goal>
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UsCoreGoal {
    /// Logical id of this artifact
    #[prost(message, optional, tag="1")]
    pub id: ::core::option::Option<super::proto::Id>,
    /// Metadata about the resource
    #[prost(message, optional, tag="2")]
    pub meta: ::core::option::Option<super::proto::Meta>,
    /// A set of rules under which this content was created
    #[prost(message, optional, tag="3")]
    pub implicit_rules: ::core::option::Option<super::proto::Uri>,
    /// Language of the resource content
    #[prost(message, optional, tag="4")]
    pub language: ::core::option::Option<super::proto::LanguageCode>,
    /// Text summary of the resource, for human interpretation
    #[prost(message, optional, tag="5")]
    pub text: ::core::option::Option<super::proto::Narrative>,
    /// Contained, inline Resources
    #[prost(message, repeated, tag="6")]
    pub contained: ::prost::alloc::vec::Vec<super::proto::ContainedResource>,
    /// Additional Content defined by implementations
    #[prost(message, repeated, tag="7")]
    pub extension: ::prost::alloc::vec::Vec<super::proto::Extension>,
    /// Extensions that cannot be ignored
    #[prost(message, repeated, tag="8")]
    pub modifier_extension: ::prost::alloc::vec::Vec<super::proto::Extension>,
    /// External Ids for this goal
    #[prost(message, repeated, tag="9")]
    pub identifier: ::prost::alloc::vec::Vec<super::proto::Identifier>,
    /// proposed | accepted | planned | in-progress | on-target | ahead-of-target |
    /// behind-target | sustaining | achieved | on-hold | cancelled |
    /// entered-in-error | rejected
    #[prost(message, optional, tag="10")]
    pub status: ::core::option::Option<super::proto::GoalStatusCode>,
    /// E.g. Treatment, dietary, behavioral, etc.
    #[prost(message, repeated, tag="11")]
    pub category: ::prost::alloc::vec::Vec<super::proto::CodeableConcept>,
    /// high-priority | medium-priority | low-priority
    #[prost(message, optional, tag="12")]
    pub priority: ::core::option::Option<super::proto::CodeableConcept>,
    /// Code or text describing goal
    #[prost(message, optional, tag="13")]
    pub description: ::core::option::Option<super::proto::CodeableConcept>,
    /// Who this goal is intended for
    #[prost(message, optional, tag="14")]
    pub subject: ::core::option::Option<super::proto::Reference>,
    #[prost(message, optional, tag="15")]
    pub start: ::core::option::Option<us_core_goal::Start>,
    #[prost(message, optional, tag="16")]
    pub target: ::core::option::Option<us_core_goal::Target>,
    /// When goal status took effect
    #[prost(message, optional, tag="17")]
    pub status_date: ::core::option::Option<super::proto::Date>,
    /// Reason for current status
    #[prost(message, optional, tag="18")]
    pub status_reason: ::core::option::Option<super::proto::String>,
    /// Who's responsible for creating Goal?
    #[prost(message, optional, tag="19")]
    pub expressed_by: ::core::option::Option<super::proto::Reference>,
    /// Issues addressed by this goal
    #[prost(message, repeated, tag="20")]
    pub addresses: ::prost::alloc::vec::Vec<super::proto::Reference>,
    /// Comments about the goal
    #[prost(message, repeated, tag="21")]
    pub note: ::prost::alloc::vec::Vec<super::proto::Annotation>,
    /// What result was achieved regarding the goal?
    #[prost(message, repeated, tag="22")]
    pub outcome_code: ::prost::alloc::vec::Vec<super::proto::CodeableConcept>,
    /// Observation that resulted from goal
    #[prost(message, repeated, tag="23")]
    pub outcome_reference: ::prost::alloc::vec::Vec<super::proto::Reference>,
}
/// Nested message and enum types in `UsCoreGoal`.
pub mod us_core_goal {
    /// When goal pursuit begins
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Start {
        #[prost(oneof="start::Start", tags="1, 2")]
        pub start: ::core::option::Option<start::Start>,
    }
    /// Nested message and enum types in `Start`.
    pub mod start {
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Start {
            #[prost(message, tag="1")]
            Date(super::super::super::proto::Date),
            #[prost(message, tag="2")]
            CodeableConcept(super::super::super::proto::CodeableConcept),
        }
    }
    /// Target outcome for the goal
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Target {
        /// xml:id (or equivalent in JSON)
        #[prost(message, optional, tag="1")]
        pub id: ::core::option::Option<super::super::proto::String>,
        /// Additional Content defined by implementations
        #[prost(message, repeated, tag="2")]
        pub extension: ::prost::alloc::vec::Vec<super::super::proto::Extension>,
        /// Extensions that cannot be ignored
        #[prost(message, repeated, tag="3")]
        pub modifier_extension: ::prost::alloc::vec::Vec<super::super::proto::Extension>,
        /// The parameter whose value is being tracked
        #[prost(message, optional, tag="4")]
        pub measure: ::core::option::Option<super::super::proto::CodeableConcept>,
        #[prost(message, optional, tag="5")]
        pub detail: ::core::option::Option<target::Detail>,
        #[prost(message, optional, tag="6")]
        pub due: ::core::option::Option<target::Due>,
    }
    /// Nested message and enum types in `Target`.
    pub mod target {
        /// The target value to be achieved
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Detail {
            #[prost(oneof="detail::Detail", tags="1, 2, 3")]
            pub detail: ::core::option::Option<detail::Detail>,
        }
        /// Nested message and enum types in `Detail`.
        pub mod detail {
            #[derive(Clone, PartialEq, ::prost::Oneof)]
            pub enum Detail {
                #[prost(message, tag="1")]
                Quantity(super::super::super::super::proto::Quantity),
                #[prost(message, tag="2")]
                Range(super::super::super::super::proto::Range),
                #[prost(message, tag="3")]
                CodeableConcept(super::super::super::super::proto::CodeableConcept),
            }
        }
        /// Reach goal on or before
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Due {
            #[prost(oneof="due::Due", tags="1, 2")]
            pub due: ::core::option::Option<due::Due>,
        }
        /// Nested message and enum types in `Due`.
        pub mod due {
            #[derive(Clone, PartialEq, ::prost::Oneof)]
            pub enum Due {
                #[prost(message, tag="1")]
                Date(super::super::super::super::proto::Date),
                #[prost(message, tag="2")]
                Duration(super::super::super::super::proto::Duration),
            }
        }
    }
}
/// Auto-generated from StructureDefinition for US Core Immunization Profile.
/// US Core Immunization Profile.
/// See <http://hl7.org/fhir/us/core/StructureDefinition/us-core-immunization>
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UsCoreImmunization {
    /// Logical id of this artifact
    #[prost(message, optional, tag="1")]
    pub id: ::core::option::Option<super::proto::Id>,
    /// Metadata about the resource
    #[prost(message, optional, tag="2")]
    pub meta: ::core::option::Option<super::proto::Meta>,
    /// A set of rules under which this content was created
    #[prost(message, optional, tag="3")]
    pub implicit_rules: ::core::option::Option<super::proto::Uri>,
    /// Language of the resource content
    #[prost(message, optional, tag="4")]
    pub language: ::core::option::Option<super::proto::LanguageCode>,
    /// Text summary of the resource, for human interpretation
    #[prost(message, optional, tag="5")]
    pub text: ::core::option::Option<super::proto::Narrative>,
    /// Contained, inline Resources
    #[prost(message, repeated, tag="6")]
    pub contained: ::prost::alloc::vec::Vec<super::proto::ContainedResource>,
    /// Additional Content defined by implementations
    #[prost(message, repeated, tag="7")]
    pub extension: ::prost::alloc::vec::Vec<super::proto::Extension>,
    /// Extensions that cannot be ignored
    #[prost(message, repeated, tag="8")]
    pub modifier_extension: ::prost::alloc::vec::Vec<super::proto::Extension>,
    /// Business identifier
    #[prost(message, repeated, tag="9")]
    pub identifier: ::prost::alloc::vec::Vec<super::proto::Identifier>,
    /// completed | entered-in-error
    #[prost(message, optional, tag="10")]
    pub status: ::core::option::Option<super::proto::ImmunizationStatusCodesCode>,
    /// Flag for whether immunization was given
    #[prost(message, optional, tag="11")]
    pub not_given: ::core::option::Option<super::proto::Boolean>,
    /// Vaccine Product Type (bind to CVX)
    #[prost(message, optional, tag="12")]
    pub vaccine_code: ::core::option::Option<super::proto::CodeableConcept>,
    /// Who was immunized
    #[prost(message, optional, tag="13")]
    pub patient: ::core::option::Option<super::proto::Reference>,
    /// Encounter administered as part of
    #[prost(message, optional, tag="14")]
    pub encounter: ::core::option::Option<super::proto::Reference>,
    /// Vaccination administration date
    #[prost(message, optional, tag="15")]
    pub date: ::core::option::Option<super::proto::DateTime>,
    /// Indicates context the data was recorded in
    #[prost(message, optional, tag="16")]
    pub primary_source: ::core::option::Option<super::proto::Boolean>,
    /// Indicates the source of a secondarily reported record
    #[prost(message, optional, tag="17")]
    pub report_origin: ::core::option::Option<super::proto::CodeableConcept>,
    /// Where vaccination occurred
    #[prost(message, optional, tag="18")]
    pub location: ::core::option::Option<super::proto::Reference>,
    /// Vaccine manufacturer
    #[prost(message, optional, tag="19")]
    pub manufacturer: ::core::option::Option<super::proto::Reference>,
    /// Vaccine lot number
    #[prost(message, optional, tag="20")]
    pub lot_number: ::core::option::Option<super::proto::String>,
    /// Vaccine expiration date
    #[prost(message, optional, tag="21")]
    pub expiration_date: ::core::option::Option<super::proto::Date>,
    /// Body site vaccine  was administered
    #[prost(message, optional, tag="22")]
    pub site: ::core::option::Option<super::proto::CodeableConcept>,
    /// How vaccine entered body
    #[prost(message, optional, tag="23")]
    pub route: ::core::option::Option<super::proto::CodeableConcept>,
    /// Amount of vaccine administered
    #[prost(message, optional, tag="24")]
    pub dose_quantity: ::core::option::Option<super::proto::SimpleQuantity>,
    #[prost(message, repeated, tag="25")]
    pub practitioner: ::prost::alloc::vec::Vec<us_core_immunization::Practitioner>,
    /// Vaccination notes
    #[prost(message, repeated, tag="26")]
    pub note: ::prost::alloc::vec::Vec<super::proto::Annotation>,
    #[prost(message, optional, tag="27")]
    pub explanation: ::core::option::Option<us_core_immunization::Explanation>,
    #[prost(message, repeated, tag="28")]
    pub reaction: ::prost::alloc::vec::Vec<us_core_immunization::Reaction>,
    #[prost(message, repeated, tag="29")]
    pub vaccination_protocol: ::prost::alloc::vec::Vec<us_core_immunization::VaccinationProtocol>,
}
/// Nested message and enum types in `UsCoreImmunization`.
pub mod us_core_immunization {
    /// Who performed event
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Practitioner {
        /// xml:id (or equivalent in JSON)
        #[prost(message, optional, tag="1")]
        pub id: ::core::option::Option<super::super::proto::String>,
        /// Additional Content defined by implementations
        #[prost(message, repeated, tag="2")]
        pub extension: ::prost::alloc::vec::Vec<super::super::proto::Extension>,
        /// Extensions that cannot be ignored
        #[prost(message, repeated, tag="3")]
        pub modifier_extension: ::prost::alloc::vec::Vec<super::super::proto::Extension>,
        /// What type of performance was done
        #[prost(message, optional, tag="4")]
        pub role: ::core::option::Option<super::super::proto::CodeableConcept>,
        /// Individual who was performing
        #[prost(message, optional, tag="5")]
        pub actor: ::core::option::Option<super::super::proto::Reference>,
    }
    /// Administration/non-administration reasons
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Explanation {
        /// xml:id (or equivalent in JSON)
        #[prost(message, optional, tag="1")]
        pub id: ::core::option::Option<super::super::proto::String>,
        /// Additional Content defined by implementations
        #[prost(message, repeated, tag="2")]
        pub extension: ::prost::alloc::vec::Vec<super::super::proto::Extension>,
        /// Extensions that cannot be ignored
        #[prost(message, repeated, tag="3")]
        pub modifier_extension: ::prost::alloc::vec::Vec<super::super::proto::Extension>,
        /// Why immunization occurred
        #[prost(message, repeated, tag="4")]
        pub reason: ::prost::alloc::vec::Vec<super::super::proto::CodeableConcept>,
        /// Why immunization did not occur
        #[prost(message, repeated, tag="5")]
        pub reason_not_given: ::prost::alloc::vec::Vec<super::super::proto::CodeableConcept>,
    }
    /// Details of a reaction that follows immunization
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Reaction {
        /// xml:id (or equivalent in JSON)
        #[prost(message, optional, tag="1")]
        pub id: ::core::option::Option<super::super::proto::String>,
        /// Additional Content defined by implementations
        #[prost(message, repeated, tag="2")]
        pub extension: ::prost::alloc::vec::Vec<super::super::proto::Extension>,
        /// Extensions that cannot be ignored
        #[prost(message, repeated, tag="3")]
        pub modifier_extension: ::prost::alloc::vec::Vec<super::super::proto::Extension>,
        /// When reaction started
        #[prost(message, optional, tag="4")]
        pub date: ::core::option::Option<super::super::proto::DateTime>,
        /// Additional information on reaction
        #[prost(message, optional, tag="5")]
        pub detail: ::core::option::Option<super::super::proto::Reference>,
        /// Indicates self-reported reaction
        #[prost(message, optional, tag="6")]
        pub reported: ::core::option::Option<super::super::proto::Boolean>,
    }
    /// What protocol was followed
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct VaccinationProtocol {
        /// xml:id (or equivalent in JSON)
        #[prost(message, optional, tag="1")]
        pub id: ::core::option::Option<super::super::proto::String>,
        /// Additional Content defined by implementations
        #[prost(message, repeated, tag="2")]
        pub extension: ::prost::alloc::vec::Vec<super::super::proto::Extension>,
        /// Extensions that cannot be ignored
        #[prost(message, repeated, tag="3")]
        pub modifier_extension: ::prost::alloc::vec::Vec<super::super::proto::Extension>,
        /// Dose number within series
        #[prost(message, optional, tag="4")]
        pub dose_sequence: ::core::option::Option<super::super::proto::PositiveInt>,
        /// Details of vaccine protocol
        #[prost(message, optional, tag="5")]
        pub description: ::core::option::Option<super::super::proto::String>,
        /// Who is responsible for protocol
        #[prost(message, optional, tag="6")]
        pub authority: ::core::option::Option<super::super::proto::Reference>,
        /// Name of vaccine series
        #[prost(message, optional, tag="7")]
        pub series: ::core::option::Option<super::super::proto::String>,
        /// Recommended number of doses for immunity
        #[prost(message, optional, tag="8")]
        pub series_doses: ::core::option::Option<super::super::proto::PositiveInt>,
        /// Disease immunized against
        #[prost(message, repeated, tag="9")]
        pub target_disease: ::prost::alloc::vec::Vec<super::super::proto::CodeableConcept>,
        /// Indicates if dose counts towards immunity
        #[prost(message, optional, tag="10")]
        pub dose_status: ::core::option::Option<super::super::proto::CodeableConcept>,
        /// Why dose does (not) count
        #[prost(message, optional, tag="11")]
        pub dose_status_reason: ::core::option::Option<super::super::proto::CodeableConcept>,
    }
}
/// Auto-generated from StructureDefinition for US Core Location Profile.
/// US Core Location Profile.
/// See <http://hl7.org/fhir/us/core/StructureDefinition/us-core-location>
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UsCoreLocation {
    /// Logical id of this artifact
    #[prost(message, optional, tag="1")]
    pub id: ::core::option::Option<super::proto::Id>,
    /// Metadata about the resource
    #[prost(message, optional, tag="2")]
    pub meta: ::core::option::Option<super::proto::Meta>,
    /// A set of rules under which this content was created
    #[prost(message, optional, tag="3")]
    pub implicit_rules: ::core::option::Option<super::proto::Uri>,
    /// Language of the resource content
    #[prost(message, optional, tag="4")]
    pub language: ::core::option::Option<super::proto::LanguageCode>,
    /// Text summary of the resource, for human interpretation
    #[prost(message, optional, tag="5")]
    pub text: ::core::option::Option<super::proto::Narrative>,
    /// Contained, inline Resources
    #[prost(message, repeated, tag="6")]
    pub contained: ::prost::alloc::vec::Vec<super::proto::ContainedResource>,
    /// Additional Content defined by implementations
    #[prost(message, repeated, tag="7")]
    pub extension: ::prost::alloc::vec::Vec<super::proto::Extension>,
    /// Extensions that cannot be ignored
    #[prost(message, repeated, tag="8")]
    pub modifier_extension: ::prost::alloc::vec::Vec<super::proto::Extension>,
    /// Unique code or number identifying the location to its users
    #[prost(message, repeated, tag="9")]
    pub identifier: ::prost::alloc::vec::Vec<super::proto::Identifier>,
    /// active | suspended | inactive
    #[prost(message, optional, tag="10")]
    pub status: ::core::option::Option<super::proto::LocationStatusCode>,
    /// The Operational status of the location (typically only for a bed/room)
    #[prost(message, optional, tag="11")]
    pub operational_status: ::core::option::Option<super::proto::Coding>,
    /// Name of the location as used by humans
    #[prost(message, optional, tag="12")]
    pub name: ::core::option::Option<super::proto::String>,
    /// A list of alternate names that the location is known as, or was known as in
    /// the past
    #[prost(message, repeated, tag="13")]
    pub alias: ::prost::alloc::vec::Vec<super::proto::String>,
    /// Additional details about the location that could be displayed as further
    /// information to identify the location beyond its name
    #[prost(message, optional, tag="14")]
    pub description: ::core::option::Option<super::proto::String>,
    /// instance | kind
    #[prost(message, optional, tag="15")]
    pub mode: ::core::option::Option<super::proto::LocationModeCode>,
    /// Type of function performed
    #[prost(message, optional, tag="16")]
    pub r#type: ::core::option::Option<super::proto::CodeableConcept>,
    /// Contact details of the location
    #[prost(message, repeated, tag="17")]
    pub telecom: ::prost::alloc::vec::Vec<super::proto::ContactPoint>,
    /// Physical location
    #[prost(message, optional, tag="18")]
    pub address: ::core::option::Option<super::proto::Address>,
    /// Physical form of the location
    #[prost(message, optional, tag="19")]
    pub physical_type: ::core::option::Option<super::proto::CodeableConcept>,
    #[prost(message, optional, tag="20")]
    pub position: ::core::option::Option<us_core_location::Position>,
    /// Organization responsible for provisioning and upkeep
    #[prost(message, optional, tag="21")]
    pub managing_organization: ::core::option::Option<super::proto::Reference>,
    /// Another Location this one is physically part of
    #[prost(message, optional, tag="22")]
    pub part_of: ::core::option::Option<super::proto::Reference>,
    /// Technical endpoints providing access to services operated for the location
    #[prost(message, repeated, tag="23")]
    pub endpoint: ::prost::alloc::vec::Vec<super::proto::Reference>,
}
/// Nested message and enum types in `UsCoreLocation`.
pub mod us_core_location {
    /// The absolute geographic location
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Position {
        /// xml:id (or equivalent in JSON)
        #[prost(message, optional, tag="1")]
        pub id: ::core::option::Option<super::super::proto::String>,
        /// Additional Content defined by implementations
        #[prost(message, repeated, tag="2")]
        pub extension: ::prost::alloc::vec::Vec<super::super::proto::Extension>,
        /// Extensions that cannot be ignored
        #[prost(message, repeated, tag="3")]
        pub modifier_extension: ::prost::alloc::vec::Vec<super::super::proto::Extension>,
        /// Longitude with WGS84 datum
        #[prost(message, optional, tag="4")]
        pub longitude: ::core::option::Option<super::super::proto::Decimal>,
        /// Latitude with WGS84 datum
        #[prost(message, optional, tag="5")]
        pub latitude: ::core::option::Option<super::super::proto::Decimal>,
        /// Altitude with WGS84 datum
        #[prost(message, optional, tag="6")]
        pub altitude: ::core::option::Option<super::super::proto::Decimal>,
    }
}
/// Auto-generated from StructureDefinition for US Core Medication Profile.
/// US Core Medication Profile.
/// See <http://hl7.org/fhir/us/core/StructureDefinition/us-core-medication>
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UsCoreMedication {
    /// Logical id of this artifact
    #[prost(message, optional, tag="1")]
    pub id: ::core::option::Option<super::proto::Id>,
    /// Metadata about the resource
    #[prost(message, optional, tag="2")]
    pub meta: ::core::option::Option<super::proto::Meta>,
    /// A set of rules under which this content was created
    #[prost(message, optional, tag="3")]
    pub implicit_rules: ::core::option::Option<super::proto::Uri>,
    /// Language of the resource content
    #[prost(message, optional, tag="4")]
    pub language: ::core::option::Option<super::proto::LanguageCode>,
    /// Text summary of the resource, for human interpretation
    #[prost(message, optional, tag="5")]
    pub text: ::core::option::Option<super::proto::Narrative>,
    /// Contained, inline Resources
    #[prost(message, repeated, tag="6")]
    pub contained: ::prost::alloc::vec::Vec<super::proto::ContainedResource>,
    /// Additional Content defined by implementations
    #[prost(message, repeated, tag="7")]
    pub extension: ::prost::alloc::vec::Vec<super::proto::Extension>,
    /// Extensions that cannot be ignored
    #[prost(message, repeated, tag="8")]
    pub modifier_extension: ::prost::alloc::vec::Vec<super::proto::Extension>,
    /// Codes that identify this medication
    #[prost(message, optional, tag="9")]
    pub code: ::core::option::Option<super::proto::CodeableConcept>,
    /// active | inactive | entered-in-error
    #[prost(message, optional, tag="10")]
    pub status: ::core::option::Option<super::proto::MedicationStatusCode>,
    /// True if a brand
    #[prost(message, optional, tag="11")]
    pub is_brand: ::core::option::Option<super::proto::Boolean>,
    /// True if medication does not require a prescription
    #[prost(message, optional, tag="12")]
    pub is_over_the_counter: ::core::option::Option<super::proto::Boolean>,
    /// Manufacturer of the item
    #[prost(message, optional, tag="13")]
    pub manufacturer: ::core::option::Option<super::proto::Reference>,
    /// powder | tablets | capsule +
    #[prost(message, optional, tag="14")]
    pub form: ::core::option::Option<super::proto::CodeableConcept>,
    #[prost(message, repeated, tag="15")]
    pub ingredient: ::prost::alloc::vec::Vec<us_core_medication::Ingredient>,
    #[prost(message, optional, tag="16")]
    pub package_value: ::core::option::Option<us_core_medication::Package>,
    /// Picture of the medication
    #[prost(message, repeated, tag="17")]
    pub image: ::prost::alloc::vec::Vec<super::proto::Attachment>,
}
/// Nested message and enum types in `UsCoreMedication`.
pub mod us_core_medication {
    /// Active or inactive ingredient
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Ingredient {
        /// xml:id (or equivalent in JSON)
        #[prost(message, optional, tag="1")]
        pub id: ::core::option::Option<super::super::proto::String>,
        /// Additional Content defined by implementations
        #[prost(message, repeated, tag="2")]
        pub extension: ::prost::alloc::vec::Vec<super::super::proto::Extension>,
        /// Extensions that cannot be ignored
        #[prost(message, repeated, tag="3")]
        pub modifier_extension: ::prost::alloc::vec::Vec<super::super::proto::Extension>,
        #[prost(message, optional, tag="4")]
        pub item: ::core::option::Option<ingredient::Item>,
        /// Active ingredient indicator
        #[prost(message, optional, tag="5")]
        pub is_active: ::core::option::Option<super::super::proto::Boolean>,
        /// Quantity of ingredient present
        #[prost(message, optional, tag="6")]
        pub amount: ::core::option::Option<super::super::proto::Ratio>,
    }
    /// Nested message and enum types in `Ingredient`.
    pub mod ingredient {
        /// The product contained
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Item {
            #[prost(oneof="item::Item", tags="1, 2")]
            pub item: ::core::option::Option<item::Item>,
        }
        /// Nested message and enum types in `Item`.
        pub mod item {
            #[derive(Clone, PartialEq, ::prost::Oneof)]
            pub enum Item {
                #[prost(message, tag="1")]
                CodeableConcept(super::super::super::super::proto::CodeableConcept),
                #[prost(message, tag="2")]
                Reference(super::super::super::super::proto::Reference),
            }
        }
    }
    /// Details about packaged medications
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Package {
        /// xml:id (or equivalent in JSON)
        #[prost(message, optional, tag="1")]
        pub id: ::core::option::Option<super::super::proto::String>,
        /// Additional Content defined by implementations
        #[prost(message, repeated, tag="2")]
        pub extension: ::prost::alloc::vec::Vec<super::super::proto::Extension>,
        /// Extensions that cannot be ignored
        #[prost(message, repeated, tag="3")]
        pub modifier_extension: ::prost::alloc::vec::Vec<super::super::proto::Extension>,
        /// E.g. box, vial, blister-pack
        #[prost(message, optional, tag="4")]
        pub container: ::core::option::Option<super::super::proto::CodeableConcept>,
        #[prost(message, repeated, tag="5")]
        pub content: ::prost::alloc::vec::Vec<package::Content>,
        #[prost(message, repeated, tag="6")]
        pub batch: ::prost::alloc::vec::Vec<package::Batch>,
    }
    /// Nested message and enum types in `Package`.
    pub mod package {
        /// What is  in the package
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Content {
            /// xml:id (or equivalent in JSON)
            #[prost(message, optional, tag="1")]
            pub id: ::core::option::Option<super::super::super::proto::String>,
            /// Additional Content defined by implementations
            #[prost(message, repeated, tag="2")]
            pub extension: ::prost::alloc::vec::Vec<super::super::super::proto::Extension>,
            /// Extensions that cannot be ignored
            #[prost(message, repeated, tag="3")]
            pub modifier_extension: ::prost::alloc::vec::Vec<super::super::super::proto::Extension>,
            #[prost(message, optional, tag="4")]
            pub item: ::core::option::Option<content::Item>,
            /// Quantity present in the package
            #[prost(message, optional, tag="5")]
            pub amount: ::core::option::Option<super::super::super::proto::SimpleQuantity>,
        }
        /// Nested message and enum types in `Content`.
        pub mod content {
            /// The item in the package
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct Item {
                #[prost(oneof="item::Item", tags="1, 2")]
                pub item: ::core::option::Option<item::Item>,
            }
            /// Nested message and enum types in `Item`.
            pub mod item {
                #[derive(Clone, PartialEq, ::prost::Oneof)]
                pub enum Item {
                    #[prost(message, tag="1")]
                    CodeableConcept(super::super::super::super::super::proto::CodeableConcept),
                    #[prost(message, tag="2")]
                    Reference(super::super::super::super::super::proto::Reference),
                }
            }
        }
        /// Identifies a single production run
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Batch {
            /// xml:id (or equivalent in JSON)
            #[prost(message, optional, tag="1")]
            pub id: ::core::option::Option<super::super::super::proto::String>,
            /// Additional Content defined by implementations
            #[prost(message, repeated, tag="2")]
            pub extension: ::prost::alloc::vec::Vec<super::super::super::proto::Extension>,
            /// Extensions that cannot be ignored
            #[prost(message, repeated, tag="3")]
            pub modifier_extension: ::prost::alloc::vec::Vec<super::super::super::proto::Extension>,
            /// Identifier assigned to batch
            #[prost(message, optional, tag="4")]
            pub lot_number: ::core::option::Option<super::super::super::proto::String>,
            /// When batch will expire
            #[prost(message, optional, tag="5")]
            pub expiration_date: ::core::option::Option<super::super::super::proto::DateTime>,
        }
    }
}
/// Auto-generated from StructureDefinition for US Core Medication Request
/// Profile. US Core Medication Request Profile. See
/// <http://hl7.org/fhir/us/core/StructureDefinition/us-core-medicationrequest>
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UsCoreMedicationrequest {
    /// Logical id of this artifact
    #[prost(message, optional, tag="1")]
    pub id: ::core::option::Option<super::proto::Id>,
    /// Metadata about the resource
    #[prost(message, optional, tag="2")]
    pub meta: ::core::option::Option<super::proto::Meta>,
    /// A set of rules under which this content was created
    #[prost(message, optional, tag="3")]
    pub implicit_rules: ::core::option::Option<super::proto::Uri>,
    /// Language of the resource content
    #[prost(message, optional, tag="4")]
    pub language: ::core::option::Option<super::proto::LanguageCode>,
    /// Text summary of the resource, for human interpretation
    #[prost(message, optional, tag="5")]
    pub text: ::core::option::Option<super::proto::Narrative>,
    /// Contained, inline Resources
    #[prost(message, repeated, tag="6")]
    pub contained: ::prost::alloc::vec::Vec<super::proto::ContainedResource>,
    /// Additional Content defined by implementations
    #[prost(message, repeated, tag="7")]
    pub extension: ::prost::alloc::vec::Vec<super::proto::Extension>,
    /// Extensions that cannot be ignored
    #[prost(message, repeated, tag="8")]
    pub modifier_extension: ::prost::alloc::vec::Vec<super::proto::Extension>,
    /// External ids for this request
    #[prost(message, repeated, tag="9")]
    pub identifier: ::prost::alloc::vec::Vec<super::proto::Identifier>,
    /// Protocol or definition
    #[prost(message, repeated, tag="10")]
    pub definition: ::prost::alloc::vec::Vec<super::proto::Reference>,
    /// What request fulfills
    #[prost(message, repeated, tag="11")]
    pub based_on: ::prost::alloc::vec::Vec<super::proto::Reference>,
    /// Composite request this is part of
    #[prost(message, optional, tag="12")]
    pub group_identifier: ::core::option::Option<super::proto::Identifier>,
    /// active | on-hold | cancelled | completed | entered-in-error | stopped |
    /// draft | unknown
    #[prost(message, optional, tag="13")]
    pub status: ::core::option::Option<super::proto::MedicationRequestStatusCode>,
    /// proposal | plan | order | instance-order
    #[prost(message, optional, tag="14")]
    pub intent: ::core::option::Option<super::proto::MedicationRequestIntentCode>,
    /// Type of medication usage
    #[prost(message, optional, tag="15")]
    pub category: ::core::option::Option<super::proto::CodeableConcept>,
    /// routine | urgent | stat | asap
    #[prost(message, optional, tag="16")]
    pub priority: ::core::option::Option<super::proto::MedicationRequestPriorityCode>,
    #[prost(message, optional, tag="17")]
    pub medication: ::core::option::Option<us_core_medicationrequest::Medication>,
    /// Who or group medication request is for
    #[prost(message, optional, tag="18")]
    pub subject: ::core::option::Option<super::proto::Reference>,
    /// Created during encounter/admission/stay
    #[prost(message, optional, tag="19")]
    pub context: ::core::option::Option<super::proto::Reference>,
    /// Information to support ordering of the medication
    #[prost(message, repeated, tag="20")]
    pub supporting_information: ::prost::alloc::vec::Vec<super::proto::Reference>,
    /// When request was initially authored
    #[prost(message, optional, tag="21")]
    pub authored_on: ::core::option::Option<super::proto::DateTime>,
    #[prost(message, optional, tag="22")]
    pub requester: ::core::option::Option<us_core_medicationrequest::Requester>,
    /// Person who entered the request
    #[prost(message, optional, tag="23")]
    pub recorder: ::core::option::Option<super::proto::Reference>,
    /// Reason or indication for writing the prescription
    #[prost(message, repeated, tag="24")]
    pub reason_code: ::prost::alloc::vec::Vec<super::proto::CodeableConcept>,
    /// Condition or Observation that supports why the prescription is being
    /// written
    #[prost(message, repeated, tag="25")]
    pub reason_reference: ::prost::alloc::vec::Vec<super::proto::Reference>,
    /// Information about the prescription
    #[prost(message, repeated, tag="26")]
    pub note: ::prost::alloc::vec::Vec<super::proto::Annotation>,
    /// How the medication should be taken
    #[prost(message, repeated, tag="27")]
    pub dosage_instruction: ::prost::alloc::vec::Vec<super::proto::Dosage>,
    #[prost(message, optional, tag="28")]
    pub dispense_request: ::core::option::Option<us_core_medicationrequest::DispenseRequest>,
    #[prost(message, optional, tag="29")]
    pub substitution: ::core::option::Option<us_core_medicationrequest::Substitution>,
    /// An order/prescription that is being replaced
    #[prost(message, optional, tag="30")]
    pub prior_prescription: ::core::option::Option<super::proto::Reference>,
    /// Clinical Issue with action
    #[prost(message, repeated, tag="31")]
    pub detected_issue: ::prost::alloc::vec::Vec<super::proto::Reference>,
    /// A list of events of interest in the lifecycle
    #[prost(message, repeated, tag="32")]
    pub event_history: ::prost::alloc::vec::Vec<super::proto::Reference>,
}
/// Nested message and enum types in `UsCoreMedicationrequest`.
pub mod us_core_medicationrequest {
    /// Medication to be taken
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Medication {
        #[prost(oneof="medication::Medication", tags="1, 2")]
        pub medication: ::core::option::Option<medication::Medication>,
    }
    /// Nested message and enum types in `Medication`.
    pub mod medication {
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Medication {
            #[prost(message, tag="1")]
            CodeableConcept(super::super::super::proto::CodeableConcept),
            #[prost(message, tag="2")]
            Reference(super::super::super::proto::Reference),
        }
    }
    /// Who/What requested the Request
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Requester {
        /// xml:id (or equivalent in JSON)
        #[prost(message, optional, tag="1")]
        pub id: ::core::option::Option<super::super::proto::String>,
        /// Additional Content defined by implementations
        #[prost(message, repeated, tag="2")]
        pub extension: ::prost::alloc::vec::Vec<super::super::proto::Extension>,
        /// Extensions that cannot be ignored
        #[prost(message, repeated, tag="3")]
        pub modifier_extension: ::prost::alloc::vec::Vec<super::super::proto::Extension>,
        /// Who ordered the initial medication(s)
        #[prost(message, optional, tag="4")]
        pub agent: ::core::option::Option<super::super::proto::Reference>,
        /// Organization agent is acting for
        #[prost(message, optional, tag="5")]
        pub on_behalf_of: ::core::option::Option<super::super::proto::Reference>,
    }
    /// Medication supply authorization
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DispenseRequest {
        /// xml:id (or equivalent in JSON)
        #[prost(message, optional, tag="1")]
        pub id: ::core::option::Option<super::super::proto::String>,
        /// Additional Content defined by implementations
        #[prost(message, repeated, tag="2")]
        pub extension: ::prost::alloc::vec::Vec<super::super::proto::Extension>,
        /// Extensions that cannot be ignored
        #[prost(message, repeated, tag="3")]
        pub modifier_extension: ::prost::alloc::vec::Vec<super::super::proto::Extension>,
        /// Time period supply is authorized for
        #[prost(message, optional, tag="4")]
        pub validity_period: ::core::option::Option<super::super::proto::Period>,
        /// Number of refills authorized
        #[prost(message, optional, tag="5")]
        pub number_of_repeats_allowed: ::core::option::Option<super::super::proto::PositiveInt>,
        /// Amount of medication to supply per dispense
        #[prost(message, optional, tag="6")]
        pub quantity: ::core::option::Option<super::super::proto::SimpleQuantity>,
        /// Number of days supply per dispense
        #[prost(message, optional, tag="7")]
        pub expected_supply_duration: ::core::option::Option<super::super::proto::Duration>,
        /// Intended dispenser
        #[prost(message, optional, tag="8")]
        pub performer: ::core::option::Option<super::super::proto::Reference>,
    }
    /// Any restrictions on medication substitution
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Substitution {
        /// xml:id (or equivalent in JSON)
        #[prost(message, optional, tag="1")]
        pub id: ::core::option::Option<super::super::proto::String>,
        /// Additional Content defined by implementations
        #[prost(message, repeated, tag="2")]
        pub extension: ::prost::alloc::vec::Vec<super::super::proto::Extension>,
        /// Extensions that cannot be ignored
        #[prost(message, repeated, tag="3")]
        pub modifier_extension: ::prost::alloc::vec::Vec<super::super::proto::Extension>,
        /// Whether substitution is allowed or not
        #[prost(message, optional, tag="4")]
        pub allowed: ::core::option::Option<super::super::proto::Boolean>,
        /// Why should (not) substitution be made
        #[prost(message, optional, tag="5")]
        pub reason: ::core::option::Option<super::super::proto::CodeableConcept>,
    }
}
/// Auto-generated from StructureDefinition for US Core Medication Statement
/// Profile. US Core Medication Statement Profile. See
/// <http://hl7.org/fhir/us/core/StructureDefinition/us-core-medicationstatement>
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UsCoreMedicationstatement {
    /// Logical id of this artifact
    #[prost(message, optional, tag="1")]
    pub id: ::core::option::Option<super::proto::Id>,
    /// Metadata about the resource
    #[prost(message, optional, tag="2")]
    pub meta: ::core::option::Option<super::proto::Meta>,
    /// A set of rules under which this content was created
    #[prost(message, optional, tag="3")]
    pub implicit_rules: ::core::option::Option<super::proto::Uri>,
    /// Language of the resource content
    #[prost(message, optional, tag="4")]
    pub language: ::core::option::Option<super::proto::LanguageCode>,
    /// Text summary of the resource, for human interpretation
    #[prost(message, optional, tag="5")]
    pub text: ::core::option::Option<super::proto::Narrative>,
    /// Contained, inline Resources
    #[prost(message, repeated, tag="6")]
    pub contained: ::prost::alloc::vec::Vec<super::proto::ContainedResource>,
    /// Additional Content defined by implementations
    #[prost(message, repeated, tag="7")]
    pub extension: ::prost::alloc::vec::Vec<super::proto::Extension>,
    /// Extensions that cannot be ignored
    #[prost(message, repeated, tag="8")]
    pub modifier_extension: ::prost::alloc::vec::Vec<super::proto::Extension>,
    /// External identifier
    #[prost(message, repeated, tag="9")]
    pub identifier: ::prost::alloc::vec::Vec<super::proto::Identifier>,
    /// Fulfils plan, proposal or order
    #[prost(message, repeated, tag="10")]
    pub based_on: ::prost::alloc::vec::Vec<super::proto::Reference>,
    /// Part of referenced event
    #[prost(message, repeated, tag="11")]
    pub part_of: ::prost::alloc::vec::Vec<super::proto::Reference>,
    /// Encounter / Episode associated with MedicationStatement
    #[prost(message, optional, tag="12")]
    pub context: ::core::option::Option<super::proto::Reference>,
    /// active | completed | entered-in-error | intended | stopped | on-hold
    #[prost(message, optional, tag="13")]
    pub status: ::core::option::Option<super::proto::MedicationStatementStatusCode>,
    /// Type of medication usage
    #[prost(message, optional, tag="14")]
    pub category: ::core::option::Option<super::proto::CodeableConcept>,
    #[prost(message, optional, tag="15")]
    pub medication: ::core::option::Option<us_core_medicationstatement::Medication>,
    #[prost(message, optional, tag="16")]
    pub effective: ::core::option::Option<us_core_medicationstatement::Effective>,
    /// When the statement was asserted?
    #[prost(message, optional, tag="17")]
    pub date_asserted: ::core::option::Option<super::proto::DateTime>,
    /// Person or organization that provided the information about the taking of
    /// this medication
    #[prost(message, optional, tag="18")]
    pub information_source: ::core::option::Option<super::proto::Reference>,
    /// Who is/was taking  the medication
    #[prost(message, optional, tag="19")]
    pub subject: ::core::option::Option<super::proto::Reference>,
    /// Additional supporting information
    #[prost(message, repeated, tag="20")]
    pub derived_from: ::prost::alloc::vec::Vec<super::proto::Reference>,
    /// y | n | unk | na
    #[prost(message, optional, tag="21")]
    pub taken: ::core::option::Option<super::proto::MedicationStatementTakenCode>,
    /// True if asserting medication was not given
    #[prost(message, repeated, tag="22")]
    pub reason_not_taken: ::prost::alloc::vec::Vec<super::proto::CodeableConcept>,
    /// Reason for why the medication is being/was taken
    #[prost(message, repeated, tag="23")]
    pub reason_code: ::prost::alloc::vec::Vec<super::proto::CodeableConcept>,
    /// Condition or observation that supports why the medication is being/was
    /// taken
    #[prost(message, repeated, tag="24")]
    pub reason_reference: ::prost::alloc::vec::Vec<super::proto::Reference>,
    /// Further information about the statement
    #[prost(message, repeated, tag="25")]
    pub note: ::prost::alloc::vec::Vec<super::proto::Annotation>,
    /// Details of how medication is/was taken or should be taken
    #[prost(message, repeated, tag="26")]
    pub dosage: ::prost::alloc::vec::Vec<super::proto::Dosage>,
}
/// Nested message and enum types in `UsCoreMedicationstatement`.
pub mod us_core_medicationstatement {
    /// What medication was taken
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Medication {
        #[prost(oneof="medication::Medication", tags="1, 2")]
        pub medication: ::core::option::Option<medication::Medication>,
    }
    /// Nested message and enum types in `Medication`.
    pub mod medication {
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Medication {
            #[prost(message, tag="1")]
            CodeableConcept(super::super::super::proto::CodeableConcept),
            #[prost(message, tag="2")]
            Reference(super::super::super::proto::Reference),
        }
    }
    /// The date/time or interval when the medication was taken
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Effective {
        #[prost(oneof="effective::Effective", tags="1, 2")]
        pub effective: ::core::option::Option<effective::Effective>,
    }
    /// Nested message and enum types in `Effective`.
    pub mod effective {
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Effective {
            #[prost(message, tag="1")]
            DateTime(super::super::super::proto::DateTime),
            #[prost(message, tag="2")]
            Period(super::super::super::proto::Period),
        }
    }
}
/// Auto-generated from StructureDefinition for US Core Result Observation.
/// US Core Result Observation.
/// See
/// <http://hl7.org/fhir/us/core/StructureDefinition/us-core-observationresults>
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UsCoreObservationresults {
    /// Logical id of this artifact
    #[prost(message, optional, tag="1")]
    pub id: ::core::option::Option<super::proto::Id>,
    /// Metadata about the resource
    #[prost(message, optional, tag="2")]
    pub meta: ::core::option::Option<super::proto::Meta>,
    /// A set of rules under which this content was created
    #[prost(message, optional, tag="3")]
    pub implicit_rules: ::core::option::Option<super::proto::Uri>,
    /// Language of the resource content
    #[prost(message, optional, tag="4")]
    pub language: ::core::option::Option<super::proto::LanguageCode>,
    /// Text summary of the resource, for human interpretation
    #[prost(message, optional, tag="5")]
    pub text: ::core::option::Option<super::proto::Narrative>,
    /// Contained, inline Resources
    #[prost(message, repeated, tag="6")]
    pub contained: ::prost::alloc::vec::Vec<super::proto::ContainedResource>,
    /// Additional Content defined by implementations
    #[prost(message, repeated, tag="7")]
    pub extension: ::prost::alloc::vec::Vec<super::proto::Extension>,
    /// Extensions that cannot be ignored
    #[prost(message, repeated, tag="8")]
    pub modifier_extension: ::prost::alloc::vec::Vec<super::proto::Extension>,
    /// Business Identifier for observation
    #[prost(message, repeated, tag="9")]
    pub identifier: ::prost::alloc::vec::Vec<super::proto::Identifier>,
    /// Fulfills plan, proposal or order
    #[prost(message, repeated, tag="10")]
    pub based_on: ::prost::alloc::vec::Vec<super::proto::Reference>,
    /// registered | preliminary | final | amended +
    #[prost(message, optional, tag="11")]
    pub status: ::core::option::Option<super::proto::ObservationStatusCode>,
    /// Classification of  type of observation
    #[prost(message, repeated, tag="12")]
    pub category: ::prost::alloc::vec::Vec<super::proto::CodeableConcept>,
    /// Laboratory Test Name
    #[prost(message, optional, tag="13")]
    pub code: ::core::option::Option<super::proto::CodeableConcept>,
    /// Who and/or what this is about
    #[prost(message, optional, tag="14")]
    pub subject: ::core::option::Option<super::proto::Reference>,
    /// Healthcare event during which this observation is made
    #[prost(message, optional, tag="15")]
    pub context: ::core::option::Option<super::proto::Reference>,
    #[prost(message, optional, tag="16")]
    pub effective: ::core::option::Option<us_core_observationresults::Effective>,
    /// Date/Time this was made available
    #[prost(message, optional, tag="17")]
    pub issued: ::core::option::Option<super::proto::Instant>,
    /// Who is responsible for the observation
    #[prost(message, repeated, tag="18")]
    pub performer: ::prost::alloc::vec::Vec<super::proto::Reference>,
    #[prost(message, optional, tag="19")]
    pub value: ::core::option::Option<us_core_observationresults::Value>,
    /// Why the result is missing
    #[prost(message, optional, tag="20")]
    pub data_absent_reason: ::core::option::Option<super::proto::CodeableConcept>,
    /// High, low, normal, etc.
    #[prost(message, optional, tag="21")]
    pub interpretation: ::core::option::Option<super::proto::CodeableConcept>,
    /// Comments about result
    #[prost(message, optional, tag="22")]
    pub comment: ::core::option::Option<super::proto::String>,
    /// Observed body part
    #[prost(message, optional, tag="23")]
    pub body_site: ::core::option::Option<super::proto::CodeableConcept>,
    /// How it was done
    #[prost(message, optional, tag="24")]
    pub method: ::core::option::Option<super::proto::CodeableConcept>,
    /// Specimen used for this observation
    #[prost(message, optional, tag="25")]
    pub specimen: ::core::option::Option<super::proto::Reference>,
    /// (Measurement) Device
    #[prost(message, optional, tag="26")]
    pub device: ::core::option::Option<super::proto::Reference>,
    #[prost(message, repeated, tag="27")]
    pub reference_range: ::prost::alloc::vec::Vec<us_core_observationresults::ReferenceRange>,
    #[prost(message, repeated, tag="28")]
    pub related: ::prost::alloc::vec::Vec<us_core_observationresults::Related>,
    #[prost(message, repeated, tag="29")]
    pub component: ::prost::alloc::vec::Vec<us_core_observationresults::Component>,
}
/// Nested message and enum types in `UsCoreObservationresults`.
pub mod us_core_observationresults {
    /// Clinically relevant time/time-period for observation
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Effective {
        #[prost(oneof="effective::Effective", tags="1, 2")]
        pub effective: ::core::option::Option<effective::Effective>,
    }
    /// Nested message and enum types in `Effective`.
    pub mod effective {
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Effective {
            #[prost(message, tag="1")]
            DateTime(super::super::super::proto::DateTime),
            #[prost(message, tag="2")]
            Period(super::super::super::proto::Period),
        }
    }
    /// Result Value
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Value {
        #[prost(oneof="value::Value", tags="1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11")]
        pub value: ::core::option::Option<value::Value>,
    }
    /// Nested message and enum types in `Value`.
    pub mod value {
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Value {
            #[prost(message, tag="1")]
            Quantity(super::super::super::proto::Quantity),
            #[prost(message, tag="2")]
            CodeableConcept(super::super::super::proto::CodeableConcept),
            #[prost(message, tag="3")]
            StringValue(super::super::super::proto::String),
            #[prost(message, tag="4")]
            Boolean(super::super::super::proto::Boolean),
            #[prost(message, tag="5")]
            Range(super::super::super::proto::Range),
            #[prost(message, tag="6")]
            Ratio(super::super::super::proto::Ratio),
            #[prost(message, tag="7")]
            SampledData(super::super::super::proto::SampledData),
            #[prost(message, tag="8")]
            Attachment(super::super::super::proto::Attachment),
            #[prost(message, tag="9")]
            Time(super::super::super::proto::Time),
            #[prost(message, tag="10")]
            DateTime(super::super::super::proto::DateTime),
            #[prost(message, tag="11")]
            Period(super::super::super::proto::Period),
        }
    }
    /// Provides guide for interpretation
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ReferenceRange {
        /// xml:id (or equivalent in JSON)
        #[prost(message, optional, tag="1")]
        pub id: ::core::option::Option<super::super::proto::String>,
        /// Additional Content defined by implementations
        #[prost(message, repeated, tag="2")]
        pub extension: ::prost::alloc::vec::Vec<super::super::proto::Extension>,
        /// Extensions that cannot be ignored
        #[prost(message, repeated, tag="3")]
        pub modifier_extension: ::prost::alloc::vec::Vec<super::super::proto::Extension>,
        /// Low Range, if relevant
        #[prost(message, optional, tag="4")]
        pub low: ::core::option::Option<super::super::proto::SimpleQuantity>,
        /// High Range, if relevant
        #[prost(message, optional, tag="5")]
        pub high: ::core::option::Option<super::super::proto::SimpleQuantity>,
        /// Reference range qualifier
        #[prost(message, optional, tag="6")]
        pub r#type: ::core::option::Option<super::super::proto::CodeableConcept>,
        /// Reference range population
        #[prost(message, repeated, tag="7")]
        pub applies_to: ::prost::alloc::vec::Vec<super::super::proto::CodeableConcept>,
        /// Applicable age range, if relevant
        #[prost(message, optional, tag="8")]
        pub age: ::core::option::Option<super::super::proto::Range>,
        /// Text based reference range in an observation
        #[prost(message, optional, tag="9")]
        pub text: ::core::option::Option<super::super::proto::String>,
    }
    /// Resource related to this observation
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Related {
        /// xml:id (or equivalent in JSON)
        #[prost(message, optional, tag="1")]
        pub id: ::core::option::Option<super::super::proto::String>,
        /// Additional Content defined by implementations
        #[prost(message, repeated, tag="2")]
        pub extension: ::prost::alloc::vec::Vec<super::super::proto::Extension>,
        /// Extensions that cannot be ignored
        #[prost(message, repeated, tag="3")]
        pub modifier_extension: ::prost::alloc::vec::Vec<super::super::proto::Extension>,
        /// has-member | derived-from | sequel-to | replaces | qualified-by |
        /// interfered-by
        #[prost(message, optional, tag="4")]
        pub r#type: ::core::option::Option<super::super::proto::ObservationRelationshipTypeCode>,
        /// Resource that is related to this one
        #[prost(message, optional, tag="5")]
        pub target: ::core::option::Option<super::super::proto::Reference>,
    }
    /// Component results
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Component {
        /// xml:id (or equivalent in JSON)
        #[prost(message, optional, tag="1")]
        pub id: ::core::option::Option<super::super::proto::String>,
        /// Additional Content defined by implementations
        #[prost(message, repeated, tag="2")]
        pub extension: ::prost::alloc::vec::Vec<super::super::proto::Extension>,
        /// Extensions that cannot be ignored
        #[prost(message, repeated, tag="3")]
        pub modifier_extension: ::prost::alloc::vec::Vec<super::super::proto::Extension>,
        /// Type of component observation (code / type)
        #[prost(message, optional, tag="4")]
        pub code: ::core::option::Option<super::super::proto::CodeableConcept>,
        #[prost(message, optional, tag="5")]
        pub value: ::core::option::Option<component::Value>,
        /// Why the component result is missing
        #[prost(message, optional, tag="6")]
        pub data_absent_reason: ::core::option::Option<super::super::proto::CodeableConcept>,
        /// High, low, normal, etc.
        #[prost(message, optional, tag="7")]
        pub interpretation: ::core::option::Option<super::super::proto::CodeableConcept>,
        /// Provides guide for interpretation of component result
        #[prost(message, repeated, tag="8")]
        pub reference_range: ::prost::alloc::vec::Vec<ReferenceRange>,
    }
    /// Nested message and enum types in `Component`.
    pub mod component {
        /// Actual component result
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Value {
            #[prost(oneof="value::Value", tags="1, 2, 3, 4, 5, 6, 7, 8, 9, 10")]
            pub value: ::core::option::Option<value::Value>,
        }
        /// Nested message and enum types in `Value`.
        pub mod value {
            #[derive(Clone, PartialEq, ::prost::Oneof)]
            pub enum Value {
                #[prost(message, tag="1")]
                Quantity(super::super::super::super::proto::Quantity),
                #[prost(message, tag="2")]
                CodeableConcept(super::super::super::super::proto::CodeableConcept),
                #[prost(message, tag="3")]
                StringValue(super::super::super::super::proto::String),
                #[prost(message, tag="4")]
                Range(super::super::super::super::proto::Range),
                #[prost(message, tag="5")]
                Ratio(super::super::super::super::proto::Ratio),
                #[prost(message, tag="6")]
                SampledData(super::super::super::super::proto::SampledData),
                #[prost(message, tag="7")]
                Attachment(super::super::super::super::proto::Attachment),
                #[prost(message, tag="8")]
                Time(super::super::super::super::proto::Time),
                #[prost(message, tag="9")]
                DateTime(super::super::super::super::proto::DateTime),
                #[prost(message, tag="10")]
                Period(super::super::super::super::proto::Period),
            }
        }
    }
}
/// Auto-generated from StructureDefinition for US Core Organization Profile.
/// US Core Organization Profile.
/// See <http://hl7.org/fhir/us/core/StructureDefinition/us-core-organization>
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UsCoreOrganization {
    /// Logical id of this artifact
    #[prost(message, optional, tag="1")]
    pub id: ::core::option::Option<super::proto::Id>,
    /// Metadata about the resource
    #[prost(message, optional, tag="2")]
    pub meta: ::core::option::Option<super::proto::Meta>,
    /// A set of rules under which this content was created
    #[prost(message, optional, tag="3")]
    pub implicit_rules: ::core::option::Option<super::proto::Uri>,
    /// Language of the resource content
    #[prost(message, optional, tag="4")]
    pub language: ::core::option::Option<super::proto::LanguageCode>,
    /// Text summary of the resource, for human interpretation
    #[prost(message, optional, tag="5")]
    pub text: ::core::option::Option<super::proto::Narrative>,
    /// Contained, inline Resources
    #[prost(message, repeated, tag="6")]
    pub contained: ::prost::alloc::vec::Vec<super::proto::ContainedResource>,
    /// Additional Content defined by implementations
    #[prost(message, repeated, tag="7")]
    pub extension: ::prost::alloc::vec::Vec<super::proto::Extension>,
    /// Extensions that cannot be ignored
    #[prost(message, repeated, tag="8")]
    pub modifier_extension: ::prost::alloc::vec::Vec<super::proto::Extension>,
    /// Identifies this organization  across multiple systems
    #[prost(message, repeated, tag="9")]
    pub identifier: ::prost::alloc::vec::Vec<super::proto::Identifier>,
    /// Whether the organization's record is still in active use
    #[prost(message, optional, tag="10")]
    pub active: ::core::option::Option<super::proto::Boolean>,
    /// Kind of organization
    #[prost(message, repeated, tag="11")]
    pub r#type: ::prost::alloc::vec::Vec<super::proto::CodeableConcept>,
    /// Name used for the organization
    #[prost(message, optional, tag="12")]
    pub name: ::core::option::Option<super::proto::String>,
    /// A list of alternate names that the organization is known as, or was known
    /// as in the past
    #[prost(message, repeated, tag="13")]
    pub alias: ::prost::alloc::vec::Vec<super::proto::String>,
    /// A contact detail for the organization
    #[prost(message, repeated, tag="14")]
    pub telecom: ::prost::alloc::vec::Vec<super::proto::ContactPoint>,
    /// An address for the organization
    #[prost(message, repeated, tag="15")]
    pub address: ::prost::alloc::vec::Vec<super::proto::Address>,
    /// The organization of which this organization forms a part
    #[prost(message, optional, tag="16")]
    pub part_of: ::core::option::Option<super::proto::Reference>,
    #[prost(message, repeated, tag="17")]
    pub contact: ::prost::alloc::vec::Vec<us_core_organization::Contact>,
    /// Technical endpoints providing access to services operated for the
    /// organization
    #[prost(message, repeated, tag="18")]
    pub endpoint: ::prost::alloc::vec::Vec<super::proto::Reference>,
}
/// Nested message and enum types in `UsCoreOrganization`.
pub mod us_core_organization {
    /// Contact for the organization for a certain purpose
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Contact {
        /// xml:id (or equivalent in JSON)
        #[prost(message, optional, tag="1")]
        pub id: ::core::option::Option<super::super::proto::String>,
        /// Additional Content defined by implementations
        #[prost(message, repeated, tag="2")]
        pub extension: ::prost::alloc::vec::Vec<super::super::proto::Extension>,
        /// Extensions that cannot be ignored
        #[prost(message, repeated, tag="3")]
        pub modifier_extension: ::prost::alloc::vec::Vec<super::super::proto::Extension>,
        /// The type of contact
        #[prost(message, optional, tag="4")]
        pub purpose: ::core::option::Option<super::super::proto::CodeableConcept>,
        /// A name associated with the contact
        #[prost(message, optional, tag="5")]
        pub name: ::core::option::Option<super::super::proto::HumanName>,
        /// Contact details (telephone, email, etc.)  for a contact
        #[prost(message, repeated, tag="6")]
        pub telecom: ::prost::alloc::vec::Vec<super::super::proto::ContactPoint>,
        /// Visiting or postal addresses for the contact
        #[prost(message, optional, tag="7")]
        pub address: ::core::option::Option<super::super::proto::Address>,
    }
}
/// Auto-generated from StructureDefinition for US Core Patient Profile.
/// US Core Patient Profile.
/// See <http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient>
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UsCorePatient {
    /// Logical id of this artifact
    #[prost(message, optional, tag="1")]
    pub id: ::core::option::Option<super::proto::Id>,
    /// Metadata about the resource
    #[prost(message, optional, tag="2")]
    pub meta: ::core::option::Option<super::proto::Meta>,
    /// A set of rules under which this content was created
    #[prost(message, optional, tag="3")]
    pub implicit_rules: ::core::option::Option<super::proto::Uri>,
    /// Language of the resource content
    #[prost(message, optional, tag="4")]
    pub language: ::core::option::Option<super::proto::LanguageCode>,
    /// Text summary of the resource, for human interpretation
    #[prost(message, optional, tag="5")]
    pub text: ::core::option::Option<super::proto::Narrative>,
    /// Contained, inline Resources
    #[prost(message, repeated, tag="6")]
    pub contained: ::prost::alloc::vec::Vec<super::proto::ContainedResource>,
    /// Extension
    #[prost(message, repeated, tag="7")]
    pub extension: ::prost::alloc::vec::Vec<super::proto::Extension>,
    /// Extensions that cannot be ignored
    #[prost(message, repeated, tag="8")]
    pub modifier_extension: ::prost::alloc::vec::Vec<super::proto::Extension>,
    /// An identifier for this patient
    #[prost(message, repeated, tag="9")]
    pub identifier: ::prost::alloc::vec::Vec<super::proto::Identifier>,
    /// Whether this patient's record is in active use
    #[prost(message, optional, tag="10")]
    pub active: ::core::option::Option<super::proto::Boolean>,
    /// A name associated with the patient
    #[prost(message, repeated, tag="11")]
    pub name: ::prost::alloc::vec::Vec<super::proto::HumanName>,
    /// A contact detail for the individual
    #[prost(message, repeated, tag="12")]
    pub telecom: ::prost::alloc::vec::Vec<super::proto::ContactPoint>,
    /// male | female | other | unknown
    #[prost(message, optional, tag="13")]
    pub gender: ::core::option::Option<super::proto::AdministrativeGenderCode>,
    /// The date of birth for the individual
    #[prost(message, optional, tag="14")]
    pub birth_date: ::core::option::Option<super::proto::Date>,
    #[prost(message, optional, tag="15")]
    pub deceased: ::core::option::Option<us_core_patient::Deceased>,
    /// Addresses for the individual
    #[prost(message, repeated, tag="16")]
    pub address: ::prost::alloc::vec::Vec<super::proto::Address>,
    /// Marital (civil) status of a patient
    #[prost(message, optional, tag="17")]
    pub marital_status: ::core::option::Option<super::proto::CodeableConcept>,
    #[prost(message, optional, tag="18")]
    pub multiple_birth: ::core::option::Option<us_core_patient::MultipleBirth>,
    /// Image of the patient
    #[prost(message, repeated, tag="19")]
    pub photo: ::prost::alloc::vec::Vec<super::proto::Attachment>,
    #[prost(message, repeated, tag="20")]
    pub contact: ::prost::alloc::vec::Vec<us_core_patient::Contact>,
    #[prost(message, repeated, tag="22")]
    pub communication: ::prost::alloc::vec::Vec<us_core_patient::Communication>,
    /// Patient's nominated primary care provider
    #[prost(message, repeated, tag="23")]
    pub general_practitioner: ::prost::alloc::vec::Vec<super::proto::Reference>,
    /// Organization that is the custodian of the patient record
    #[prost(message, optional, tag="24")]
    pub managing_organization: ::core::option::Option<super::proto::Reference>,
    #[prost(message, repeated, tag="25")]
    pub link: ::prost::alloc::vec::Vec<us_core_patient::Link>,
    /// Extension
    #[prost(message, optional, tag="26")]
    pub race: ::core::option::Option<PatientUsCoreRaceExtension>,
    /// Extension
    #[prost(message, optional, tag="27")]
    pub ethnicity: ::core::option::Option<PatientUsCoreEthnicityExtension>,
    /// Extension
    #[prost(message, optional, tag="28")]
    pub birthsex: ::core::option::Option<UsCoreBirthSexCode>,
}
/// Nested message and enum types in `UsCorePatient`.
pub mod us_core_patient {
    /// Indicates if the individual is deceased or not
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Deceased {
        #[prost(oneof="deceased::Deceased", tags="1, 2")]
        pub deceased: ::core::option::Option<deceased::Deceased>,
    }
    /// Nested message and enum types in `Deceased`.
    pub mod deceased {
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Deceased {
            #[prost(message, tag="1")]
            Boolean(super::super::super::proto::Boolean),
            #[prost(message, tag="2")]
            DateTime(super::super::super::proto::DateTime),
        }
    }
    /// Whether patient is part of a multiple birth
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MultipleBirth {
        #[prost(oneof="multiple_birth::MultipleBirth", tags="1, 2")]
        pub multiple_birth: ::core::option::Option<multiple_birth::MultipleBirth>,
    }
    /// Nested message and enum types in `MultipleBirth`.
    pub mod multiple_birth {
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum MultipleBirth {
            #[prost(message, tag="1")]
            Boolean(super::super::super::proto::Boolean),
            #[prost(message, tag="2")]
            Integer(super::super::super::proto::Integer),
        }
    }
    /// A contact party (e.g. guardian, partner, friend) for the patient
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Contact {
        /// xml:id (or equivalent in JSON)
        #[prost(message, optional, tag="1")]
        pub id: ::core::option::Option<super::super::proto::String>,
        /// Additional Content defined by implementations
        #[prost(message, repeated, tag="2")]
        pub extension: ::prost::alloc::vec::Vec<super::super::proto::Extension>,
        /// Extensions that cannot be ignored
        #[prost(message, repeated, tag="3")]
        pub modifier_extension: ::prost::alloc::vec::Vec<super::super::proto::Extension>,
        /// The kind of relationship
        #[prost(message, repeated, tag="4")]
        pub relationship: ::prost::alloc::vec::Vec<super::super::proto::CodeableConcept>,
        /// A name associated with the contact person
        #[prost(message, optional, tag="5")]
        pub name: ::core::option::Option<super::super::proto::HumanName>,
        /// A contact detail for the person
        #[prost(message, repeated, tag="6")]
        pub telecom: ::prost::alloc::vec::Vec<super::super::proto::ContactPoint>,
        /// Address for the contact person
        #[prost(message, optional, tag="7")]
        pub address: ::core::option::Option<super::super::proto::Address>,
        /// male | female | other | unknown
        #[prost(message, optional, tag="8")]
        pub gender: ::core::option::Option<super::super::proto::AdministrativeGenderCode>,
        /// Organization that is associated with the contact
        #[prost(message, optional, tag="9")]
        pub organization: ::core::option::Option<super::super::proto::Reference>,
        /// The period during which this contact person or organization is valid to
        /// be contacted relating to this patient
        #[prost(message, optional, tag="10")]
        pub period: ::core::option::Option<super::super::proto::Period>,
    }
    /// A list of Languages which may be used to communicate with the patient about
    /// his or her health
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Communication {
        /// xml:id (or equivalent in JSON)
        #[prost(message, optional, tag="1")]
        pub id: ::core::option::Option<super::super::proto::String>,
        /// Additional Content defined by implementations
        #[prost(message, repeated, tag="2")]
        pub extension: ::prost::alloc::vec::Vec<super::super::proto::Extension>,
        /// Extensions that cannot be ignored
        #[prost(message, repeated, tag="3")]
        pub modifier_extension: ::prost::alloc::vec::Vec<super::super::proto::Extension>,
        /// The language which can be used to communicate with the patient about his
        /// or her health
        #[prost(message, optional, tag="4")]
        pub language: ::core::option::Option<super::super::proto::CodeableConcept>,
        /// Language preference indicator
        #[prost(message, optional, tag="5")]
        pub preferred: ::core::option::Option<super::super::proto::Boolean>,
    }
    /// Link to another patient resource that concerns the same actual person
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Link {
        /// xml:id (or equivalent in JSON)
        #[prost(message, optional, tag="1")]
        pub id: ::core::option::Option<super::super::proto::String>,
        /// Additional Content defined by implementations
        #[prost(message, repeated, tag="2")]
        pub extension: ::prost::alloc::vec::Vec<super::super::proto::Extension>,
        /// Extensions that cannot be ignored
        #[prost(message, repeated, tag="3")]
        pub modifier_extension: ::prost::alloc::vec::Vec<super::super::proto::Extension>,
        /// The other patient or related person resource that the link refers to
        #[prost(message, optional, tag="4")]
        pub other: ::core::option::Option<super::super::proto::Reference>,
        /// replaced-by | replaces | refer | seealso - type of link
        #[prost(message, optional, tag="5")]
        pub r#type: ::core::option::Option<super::super::proto::LinkTypeCode>,
    }
}
/// Auto-generated from StructureDefinition for US Core Practitioner.
/// US Core Practitioner.
/// See <http://hl7.org/fhir/us/core/StructureDefinition/us-core-practitioner>
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UsCorePractitioner {
    /// Logical id of this artifact
    #[prost(message, optional, tag="1")]
    pub id: ::core::option::Option<super::proto::Id>,
    /// Metadata about the resource
    #[prost(message, optional, tag="2")]
    pub meta: ::core::option::Option<super::proto::Meta>,
    /// A set of rules under which this content was created
    #[prost(message, optional, tag="3")]
    pub implicit_rules: ::core::option::Option<super::proto::Uri>,
    /// Language of the resource content
    #[prost(message, optional, tag="4")]
    pub language: ::core::option::Option<super::proto::LanguageCode>,
    /// Text summary of the resource, for human interpretation
    #[prost(message, optional, tag="5")]
    pub text: ::core::option::Option<super::proto::Narrative>,
    /// Contained, inline Resources
    #[prost(message, repeated, tag="6")]
    pub contained: ::prost::alloc::vec::Vec<super::proto::ContainedResource>,
    /// Additional Content defined by implementations
    #[prost(message, repeated, tag="7")]
    pub extension: ::prost::alloc::vec::Vec<super::proto::Extension>,
    /// Extensions that cannot be ignored
    #[prost(message, repeated, tag="8")]
    pub modifier_extension: ::prost::alloc::vec::Vec<super::proto::Extension>,
    /// A identifier for the person as this agent
    #[prost(message, repeated, tag="9")]
    pub identifier: ::prost::alloc::vec::Vec<super::proto::Identifier>,
    /// Whether this practitioner's record is in active use
    #[prost(message, optional, tag="10")]
    pub active: ::core::option::Option<super::proto::Boolean>,
    /// The name(s) associated with the practitioner
    #[prost(message, optional, tag="11")]
    pub name: ::core::option::Option<super::proto::HumanName>,
    /// A contact detail for the practitioner (that apply to all roles)
    #[prost(message, repeated, tag="12")]
    pub telecom: ::prost::alloc::vec::Vec<super::proto::ContactPoint>,
    /// Address(es) of the practitioner that are not role specific (typically home
    /// address)
    #[prost(message, repeated, tag="13")]
    pub address: ::prost::alloc::vec::Vec<super::proto::Address>,
    /// male | female | other | unknown
    #[prost(message, optional, tag="14")]
    pub gender: ::core::option::Option<super::proto::AdministrativeGenderCode>,
    /// The date  on which the practitioner was born
    #[prost(message, optional, tag="15")]
    pub birth_date: ::core::option::Option<super::proto::Date>,
    /// Image of the person
    #[prost(message, repeated, tag="16")]
    pub photo: ::prost::alloc::vec::Vec<super::proto::Attachment>,
    #[prost(message, repeated, tag="17")]
    pub qualification: ::prost::alloc::vec::Vec<us_core_practitioner::Qualification>,
    /// A language the practitioner is able to use in patient communication
    #[prost(message, repeated, tag="18")]
    pub communication: ::prost::alloc::vec::Vec<super::proto::CodeableConcept>,
}
/// Nested message and enum types in `UsCorePractitioner`.
pub mod us_core_practitioner {
    /// Qualifications obtained by training and certification
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Qualification {
        /// xml:id (or equivalent in JSON)
        #[prost(message, optional, tag="1")]
        pub id: ::core::option::Option<super::super::proto::String>,
        /// Additional Content defined by implementations
        #[prost(message, repeated, tag="2")]
        pub extension: ::prost::alloc::vec::Vec<super::super::proto::Extension>,
        /// Extensions that cannot be ignored
        #[prost(message, repeated, tag="3")]
        pub modifier_extension: ::prost::alloc::vec::Vec<super::super::proto::Extension>,
        /// An identifier for this qualification for the practitioner
        #[prost(message, repeated, tag="4")]
        pub identifier: ::prost::alloc::vec::Vec<super::super::proto::Identifier>,
        /// Coded representation of the qualification
        #[prost(message, optional, tag="5")]
        pub code: ::core::option::Option<super::super::proto::CodeableConcept>,
        /// Period during which the qualification is valid
        #[prost(message, optional, tag="6")]
        pub period: ::core::option::Option<super::super::proto::Period>,
        /// Organization that regulates and issues the qualification
        #[prost(message, optional, tag="7")]
        pub issuer: ::core::option::Option<super::super::proto::Reference>,
    }
}
/// Auto-generated from StructureDefinition for US Core PractitionerRole Profile.
/// US Core PractitionerRole Profile.
/// See <http://hl7.org/fhir/us/core/StructureDefinition/us-core-practitionerrole>
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UsCorePractitionerrole {
    /// Logical id of this artifact
    #[prost(message, optional, tag="1")]
    pub id: ::core::option::Option<super::proto::Id>,
    /// Metadata about the resource
    #[prost(message, optional, tag="2")]
    pub meta: ::core::option::Option<super::proto::Meta>,
    /// A set of rules under which this content was created
    #[prost(message, optional, tag="3")]
    pub implicit_rules: ::core::option::Option<super::proto::Uri>,
    /// Language of the resource content
    #[prost(message, optional, tag="4")]
    pub language: ::core::option::Option<super::proto::LanguageCode>,
    /// Text summary of the resource, for human interpretation
    #[prost(message, optional, tag="5")]
    pub text: ::core::option::Option<super::proto::Narrative>,
    /// Contained, inline Resources
    #[prost(message, repeated, tag="6")]
    pub contained: ::prost::alloc::vec::Vec<super::proto::ContainedResource>,
    /// Additional Content defined by implementations
    #[prost(message, repeated, tag="7")]
    pub extension: ::prost::alloc::vec::Vec<super::proto::Extension>,
    /// Extensions that cannot be ignored
    #[prost(message, repeated, tag="8")]
    pub modifier_extension: ::prost::alloc::vec::Vec<super::proto::Extension>,
    /// Business Identifiers that are specific to a role/location
    #[prost(message, repeated, tag="9")]
    pub identifier: ::prost::alloc::vec::Vec<super::proto::Identifier>,
    /// Whether this practitioner's record is in active use
    #[prost(message, optional, tag="10")]
    pub active: ::core::option::Option<super::proto::Boolean>,
    /// The period during which the practitioner is authorized to perform in these
    /// role(s)
    #[prost(message, optional, tag="11")]
    pub period: ::core::option::Option<super::proto::Period>,
    /// Practitioner that is able to provide the defined services for the
    /// organation
    #[prost(message, optional, tag="12")]
    pub practitioner: ::core::option::Option<super::proto::Reference>,
    /// Organization where the roles are available
    #[prost(message, optional, tag="13")]
    pub organization: ::core::option::Option<super::proto::Reference>,
    /// Roles which this practitioner may perform
    #[prost(message, optional, tag="14")]
    pub code: ::core::option::Option<super::proto::CodeableConcept>,
    /// Specific specialty of the practitioner
    #[prost(message, optional, tag="15")]
    pub specialty: ::core::option::Option<super::proto::CodeableConcept>,
    /// The location(s) at which this practitioner provides care
    #[prost(message, repeated, tag="16")]
    pub location: ::prost::alloc::vec::Vec<super::proto::Reference>,
    /// The list of healthcare services that this worker provides for this role's
    /// Organization/Location(s)
    #[prost(message, repeated, tag="17")]
    pub healthcare_service: ::prost::alloc::vec::Vec<super::proto::Reference>,
    /// Contact details that are specific to the role/location/service
    #[prost(message, repeated, tag="18")]
    pub telecom: ::prost::alloc::vec::Vec<super::proto::ContactPoint>,
    #[prost(message, repeated, tag="19")]
    pub available_time: ::prost::alloc::vec::Vec<us_core_practitionerrole::AvailableTime>,
    #[prost(message, repeated, tag="20")]
    pub not_available: ::prost::alloc::vec::Vec<us_core_practitionerrole::NotAvailable>,
    /// Description of availability exceptions
    #[prost(message, optional, tag="21")]
    pub availability_exceptions: ::core::option::Option<super::proto::String>,
    /// Technical endpoints providing access to services operated for the
    /// practitioner with this role
    #[prost(message, repeated, tag="22")]
    pub endpoint: ::prost::alloc::vec::Vec<super::proto::Reference>,
}
/// Nested message and enum types in `UsCorePractitionerrole`.
pub mod us_core_practitionerrole {
    /// Times the Service Site is available
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AvailableTime {
        /// xml:id (or equivalent in JSON)
        #[prost(message, optional, tag="1")]
        pub id: ::core::option::Option<super::super::proto::String>,
        /// Additional Content defined by implementations
        #[prost(message, repeated, tag="2")]
        pub extension: ::prost::alloc::vec::Vec<super::super::proto::Extension>,
        /// Extensions that cannot be ignored
        #[prost(message, repeated, tag="3")]
        pub modifier_extension: ::prost::alloc::vec::Vec<super::super::proto::Extension>,
        /// mon | tue | wed | thu | fri | sat | sun
        #[prost(message, repeated, tag="4")]
        pub days_of_week: ::prost::alloc::vec::Vec<super::super::proto::DaysOfWeekCode>,
        /// Always available? e.g. 24 hour service
        #[prost(message, optional, tag="5")]
        pub all_day: ::core::option::Option<super::super::proto::Boolean>,
        /// Opening time of day (ignored if allDay = true)
        #[prost(message, optional, tag="6")]
        pub available_start_time: ::core::option::Option<super::super::proto::Time>,
        /// Closing time of day (ignored if allDay = true)
        #[prost(message, optional, tag="7")]
        pub available_end_time: ::core::option::Option<super::super::proto::Time>,
    }
    /// Not available during this time due to provided reason
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct NotAvailable {
        /// xml:id (or equivalent in JSON)
        #[prost(message, optional, tag="1")]
        pub id: ::core::option::Option<super::super::proto::String>,
        /// Additional Content defined by implementations
        #[prost(message, repeated, tag="2")]
        pub extension: ::prost::alloc::vec::Vec<super::super::proto::Extension>,
        /// Extensions that cannot be ignored
        #[prost(message, repeated, tag="3")]
        pub modifier_extension: ::prost::alloc::vec::Vec<super::super::proto::Extension>,
        /// Reason presented to the user explaining why time not available
        #[prost(message, optional, tag="4")]
        pub description: ::core::option::Option<super::super::proto::String>,
        /// Service not availablefrom this date
        #[prost(message, optional, tag="5")]
        pub during: ::core::option::Option<super::super::proto::Period>,
    }
}
/// Auto-generated from StructureDefinition for US Core Procedure Profile.
/// US Core Procedure Profile.
/// See <http://hl7.org/fhir/us/core/StructureDefinition/us-core-procedure>
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UsCoreProcedure {
    /// Logical id of this artifact
    #[prost(message, optional, tag="1")]
    pub id: ::core::option::Option<super::proto::Id>,
    /// Metadata about the resource
    #[prost(message, optional, tag="2")]
    pub meta: ::core::option::Option<super::proto::Meta>,
    /// A set of rules under which this content was created
    #[prost(message, optional, tag="3")]
    pub implicit_rules: ::core::option::Option<super::proto::Uri>,
    /// Language of the resource content
    #[prost(message, optional, tag="4")]
    pub language: ::core::option::Option<super::proto::LanguageCode>,
    /// Text summary of the resource, for human interpretation
    #[prost(message, optional, tag="5")]
    pub text: ::core::option::Option<super::proto::Narrative>,
    /// Contained, inline Resources
    #[prost(message, repeated, tag="6")]
    pub contained: ::prost::alloc::vec::Vec<super::proto::ContainedResource>,
    /// Additional Content defined by implementations
    #[prost(message, repeated, tag="7")]
    pub extension: ::prost::alloc::vec::Vec<super::proto::Extension>,
    /// Extensions that cannot be ignored
    #[prost(message, repeated, tag="8")]
    pub modifier_extension: ::prost::alloc::vec::Vec<super::proto::Extension>,
    /// External Identifiers for this procedure
    #[prost(message, repeated, tag="9")]
    pub identifier: ::prost::alloc::vec::Vec<super::proto::Identifier>,
    /// Instantiates protocol or definition
    #[prost(message, repeated, tag="10")]
    pub definition: ::prost::alloc::vec::Vec<super::proto::Reference>,
    /// A request for this procedure
    #[prost(message, repeated, tag="11")]
    pub based_on: ::prost::alloc::vec::Vec<super::proto::Reference>,
    /// Part of referenced event
    #[prost(message, repeated, tag="12")]
    pub part_of: ::prost::alloc::vec::Vec<super::proto::Reference>,
    /// preparation | in-progress | suspended | aborted | completed |
    /// entered-in-error | unknown
    #[prost(message, optional, tag="13")]
    pub status: ::core::option::Option<super::proto::EventStatusCode>,
    /// True if procedure was not performed as scheduled
    #[prost(message, optional, tag="14")]
    pub not_done: ::core::option::Option<super::proto::Boolean>,
    /// Reason procedure was not performed
    #[prost(message, optional, tag="15")]
    pub not_done_reason: ::core::option::Option<super::proto::CodeableConcept>,
    /// Classification of the procedure
    #[prost(message, optional, tag="16")]
    pub category: ::core::option::Option<super::proto::CodeableConcept>,
    /// SNOMED-CT | ICD-10 | CPT-4
    #[prost(message, optional, tag="17")]
    pub code: ::core::option::Option<super::proto::CodeableConcept>,
    /// Who the procedure was performed on
    #[prost(message, optional, tag="18")]
    pub subject: ::core::option::Option<super::proto::Reference>,
    /// Encounter or episode associated with the procedure
    #[prost(message, optional, tag="19")]
    pub context: ::core::option::Option<super::proto::Reference>,
    #[prost(message, optional, tag="20")]
    pub performed: ::core::option::Option<us_core_procedure::Performed>,
    #[prost(message, repeated, tag="21")]
    pub performer: ::prost::alloc::vec::Vec<us_core_procedure::Performer>,
    /// Where the procedure happened
    #[prost(message, optional, tag="22")]
    pub location: ::core::option::Option<super::proto::Reference>,
    /// Coded reason procedure performed
    #[prost(message, repeated, tag="23")]
    pub reason_code: ::prost::alloc::vec::Vec<super::proto::CodeableConcept>,
    /// Condition that is the reason the procedure performed
    #[prost(message, repeated, tag="24")]
    pub reason_reference: ::prost::alloc::vec::Vec<super::proto::Reference>,
    /// Target body sites
    #[prost(message, repeated, tag="25")]
    pub body_site: ::prost::alloc::vec::Vec<super::proto::CodeableConcept>,
    /// The result of procedure
    #[prost(message, optional, tag="26")]
    pub outcome: ::core::option::Option<super::proto::CodeableConcept>,
    /// Any report resulting from the procedure
    #[prost(message, repeated, tag="27")]
    pub report: ::prost::alloc::vec::Vec<super::proto::Reference>,
    /// Complication following the procedure
    #[prost(message, repeated, tag="28")]
    pub complication: ::prost::alloc::vec::Vec<super::proto::CodeableConcept>,
    /// A condition that is a result of the procedure
    #[prost(message, repeated, tag="29")]
    pub complication_detail: ::prost::alloc::vec::Vec<super::proto::Reference>,
    /// Instructions for follow up
    #[prost(message, repeated, tag="30")]
    pub follow_up: ::prost::alloc::vec::Vec<super::proto::CodeableConcept>,
    /// Additional information about the procedure
    #[prost(message, repeated, tag="31")]
    pub note: ::prost::alloc::vec::Vec<super::proto::Annotation>,
    #[prost(message, repeated, tag="32")]
    pub focal_device: ::prost::alloc::vec::Vec<us_core_procedure::FocalDevice>,
    /// Items used during procedure
    #[prost(message, repeated, tag="33")]
    pub used_reference: ::prost::alloc::vec::Vec<super::proto::Reference>,
    /// Coded items used during the procedure
    #[prost(message, repeated, tag="34")]
    pub used_code: ::prost::alloc::vec::Vec<super::proto::CodeableConcept>,
}
/// Nested message and enum types in `UsCoreProcedure`.
pub mod us_core_procedure {
    /// Date/Period the procedure was performed
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Performed {
        #[prost(oneof="performed::Performed", tags="1, 2")]
        pub performed: ::core::option::Option<performed::Performed>,
    }
    /// Nested message and enum types in `Performed`.
    pub mod performed {
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Performed {
            #[prost(message, tag="1")]
            DateTime(super::super::super::proto::DateTime),
            #[prost(message, tag="2")]
            Period(super::super::super::proto::Period),
        }
    }
    /// The people who performed the procedure
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Performer {
        /// xml:id (or equivalent in JSON)
        #[prost(message, optional, tag="1")]
        pub id: ::core::option::Option<super::super::proto::String>,
        /// Additional Content defined by implementations
        #[prost(message, repeated, tag="2")]
        pub extension: ::prost::alloc::vec::Vec<super::super::proto::Extension>,
        /// Extensions that cannot be ignored
        #[prost(message, repeated, tag="3")]
        pub modifier_extension: ::prost::alloc::vec::Vec<super::super::proto::Extension>,
        /// The role the actor was in
        #[prost(message, optional, tag="4")]
        pub role: ::core::option::Option<super::super::proto::CodeableConcept>,
        /// The reference to the practitioner
        #[prost(message, optional, tag="5")]
        pub actor: ::core::option::Option<super::super::proto::Reference>,
        /// Organization the device or practitioner was acting for
        #[prost(message, optional, tag="6")]
        pub on_behalf_of: ::core::option::Option<super::super::proto::Reference>,
    }
    /// Device changed in procedure
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct FocalDevice {
        /// xml:id (or equivalent in JSON)
        #[prost(message, optional, tag="1")]
        pub id: ::core::option::Option<super::super::proto::String>,
        /// Additional Content defined by implementations
        #[prost(message, repeated, tag="2")]
        pub extension: ::prost::alloc::vec::Vec<super::super::proto::Extension>,
        /// Extensions that cannot be ignored
        #[prost(message, repeated, tag="3")]
        pub modifier_extension: ::prost::alloc::vec::Vec<super::super::proto::Extension>,
        /// Kind of change to device
        #[prost(message, optional, tag="4")]
        pub action: ::core::option::Option<super::super::proto::CodeableConcept>,
        /// Device that was changed
        #[prost(message, optional, tag="5")]
        pub manipulated: ::core::option::Option<super::super::proto::Reference>,
    }
}
/// Auto-generated from StructureDefinition for Profile-resource association
/// extension. Resource that this profile is based on. See
/// <http://hl7.org/fhir/us/core/StructureDefinition/us-core-profile-link>
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CapabilityStatementProfileResourceAssociationExtension {
    /// xml:id (or equivalent in JSON)
    #[prost(message, optional, tag="1")]
    pub id: ::core::option::Option<super::proto::String>,
    /// Additional Content defined by implementations
    #[prost(message, repeated, tag="2")]
    pub extension: ::prost::alloc::vec::Vec<super::proto::Extension>,
    /// Value of extension
    #[prost(message, optional, tag="3")]
    pub value_code: ::core::option::Option<super::proto::ResourceTypeCode>,
}
/// Auto-generated from StructureDefinition for US Core Race Extension.
/// US Core Race Extension.
/// See <http://hl7.org/fhir/us/core/StructureDefinition/us-core-race>
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PatientUsCoreRaceExtension {
    /// xml:id (or equivalent in JSON)
    #[prost(message, optional, tag="1")]
    pub id: ::core::option::Option<super::proto::String>,
    /// Extension
    #[prost(message, repeated, tag="2")]
    pub extension: ::prost::alloc::vec::Vec<super::proto::Extension>,
    /// American Indian or Alaska Native|Asian|Black or African American|Native
    /// Hawaiian or Other Pacific Islander|White
    #[prost(message, repeated, tag="4")]
    pub omb_category: ::prost::alloc::vec::Vec<super::proto::Coding>,
    /// Extended race codes
    #[prost(message, repeated, tag="5")]
    pub detailed: ::prost::alloc::vec::Vec<super::proto::Coding>,
    /// Race Text
    #[prost(message, optional, tag="6")]
    pub text: ::core::option::Option<super::proto::String>,
}
/// Auto-generated from StructureDefinition for US Core Smoking Status
/// Observation Profile. US Core Smoking Status Observation Profile. See
/// <http://hl7.org/fhir/us/core/StructureDefinition/us-core-smokingstatus>
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UsCoreSmokingstatus {
    /// Logical id of this artifact
    #[prost(message, optional, tag="1")]
    pub id: ::core::option::Option<super::proto::Id>,
    /// Metadata about the resource
    #[prost(message, optional, tag="2")]
    pub meta: ::core::option::Option<super::proto::Meta>,
    /// A set of rules under which this content was created
    #[prost(message, optional, tag="3")]
    pub implicit_rules: ::core::option::Option<super::proto::Uri>,
    /// Language of the resource content
    #[prost(message, optional, tag="4")]
    pub language: ::core::option::Option<super::proto::LanguageCode>,
    /// Text summary of the resource, for human interpretation
    #[prost(message, optional, tag="5")]
    pub text: ::core::option::Option<super::proto::Narrative>,
    /// Contained, inline Resources
    #[prost(message, repeated, tag="6")]
    pub contained: ::prost::alloc::vec::Vec<super::proto::ContainedResource>,
    /// Additional Content defined by implementations
    #[prost(message, repeated, tag="7")]
    pub extension: ::prost::alloc::vec::Vec<super::proto::Extension>,
    /// Extensions that cannot be ignored
    #[prost(message, repeated, tag="8")]
    pub modifier_extension: ::prost::alloc::vec::Vec<super::proto::Extension>,
    /// Business Identifier for observation
    #[prost(message, repeated, tag="9")]
    pub identifier: ::prost::alloc::vec::Vec<super::proto::Identifier>,
    /// Fulfills plan, proposal or order
    #[prost(message, repeated, tag="10")]
    pub based_on: ::prost::alloc::vec::Vec<super::proto::Reference>,
    /// registered | preliminary | final | amended +
    #[prost(message, optional, tag="11")]
    pub status: ::core::option::Option<super::proto::ObservationStatusCode>,
    /// Classification of  type of observation
    #[prost(message, repeated, tag="12")]
    pub category: ::prost::alloc::vec::Vec<super::proto::CodeableConcept>,
    /// Smoking Status: LOINC 72166-2  = Tobacco smoking status NHIS
    #[prost(message, optional, tag="13")]
    pub code: ::core::option::Option<super::proto::CodeableConcept>,
    /// Who and/or what this is about
    #[prost(message, optional, tag="14")]
    pub subject: ::core::option::Option<super::proto::Reference>,
    /// Healthcare event during which this observation is made
    #[prost(message, optional, tag="15")]
    pub context: ::core::option::Option<super::proto::Reference>,
    #[prost(message, optional, tag="16")]
    pub effective: ::core::option::Option<us_core_smokingstatus::Effective>,
    /// Date/Time this was made available
    #[prost(message, optional, tag="17")]
    pub issued: ::core::option::Option<super::proto::Instant>,
    /// Who is responsible for the observation
    #[prost(message, repeated, tag="18")]
    pub performer: ::prost::alloc::vec::Vec<super::proto::Reference>,
    #[prost(message, optional, tag="19")]
    pub value: ::core::option::Option<us_core_smokingstatus::Value>,
    /// Why the result is missing
    #[prost(message, optional, tag="20")]
    pub data_absent_reason: ::core::option::Option<super::proto::CodeableConcept>,
    /// High, low, normal, etc.
    #[prost(message, optional, tag="21")]
    pub interpretation: ::core::option::Option<super::proto::CodeableConcept>,
    /// Comments about result
    #[prost(message, optional, tag="22")]
    pub comment: ::core::option::Option<super::proto::String>,
    /// Observed body part
    #[prost(message, optional, tag="23")]
    pub body_site: ::core::option::Option<super::proto::CodeableConcept>,
    /// How it was done
    #[prost(message, optional, tag="24")]
    pub method: ::core::option::Option<super::proto::CodeableConcept>,
    /// Specimen used for this observation
    #[prost(message, optional, tag="25")]
    pub specimen: ::core::option::Option<super::proto::Reference>,
    /// (Measurement) Device
    #[prost(message, optional, tag="26")]
    pub device: ::core::option::Option<super::proto::Reference>,
    #[prost(message, repeated, tag="27")]
    pub reference_range: ::prost::alloc::vec::Vec<us_core_smokingstatus::ReferenceRange>,
    #[prost(message, repeated, tag="28")]
    pub related: ::prost::alloc::vec::Vec<us_core_smokingstatus::Related>,
    #[prost(message, repeated, tag="29")]
    pub component: ::prost::alloc::vec::Vec<us_core_smokingstatus::Component>,
}
/// Nested message and enum types in `UsCoreSmokingstatus`.
pub mod us_core_smokingstatus {
    /// Clinically relevant time/time-period for observation
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Effective {
        #[prost(oneof="effective::Effective", tags="1, 2")]
        pub effective: ::core::option::Option<effective::Effective>,
    }
    /// Nested message and enum types in `Effective`.
    pub mod effective {
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Effective {
            #[prost(message, tag="1")]
            DateTime(super::super::super::proto::DateTime),
            #[prost(message, tag="2")]
            Period(super::super::super::proto::Period),
        }
    }
    /// Coded Responses from Smoking Status Value Set
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Value {
        #[prost(oneof="value::Value", tags="2")]
        pub value: ::core::option::Option<value::Value>,
    }
    /// Nested message and enum types in `Value`.
    pub mod value {
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Value {
            #[prost(message, tag="2")]
            CodeableConcept(super::super::super::proto::CodeableConcept),
        }
    }
    /// Provides guide for interpretation
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ReferenceRange {
        /// xml:id (or equivalent in JSON)
        #[prost(message, optional, tag="1")]
        pub id: ::core::option::Option<super::super::proto::String>,
        /// Additional Content defined by implementations
        #[prost(message, repeated, tag="2")]
        pub extension: ::prost::alloc::vec::Vec<super::super::proto::Extension>,
        /// Extensions that cannot be ignored
        #[prost(message, repeated, tag="3")]
        pub modifier_extension: ::prost::alloc::vec::Vec<super::super::proto::Extension>,
        /// Low Range, if relevant
        #[prost(message, optional, tag="4")]
        pub low: ::core::option::Option<super::super::proto::SimpleQuantity>,
        /// High Range, if relevant
        #[prost(message, optional, tag="5")]
        pub high: ::core::option::Option<super::super::proto::SimpleQuantity>,
        /// Reference range qualifier
        #[prost(message, optional, tag="6")]
        pub r#type: ::core::option::Option<super::super::proto::CodeableConcept>,
        /// Reference range population
        #[prost(message, repeated, tag="7")]
        pub applies_to: ::prost::alloc::vec::Vec<super::super::proto::CodeableConcept>,
        /// Applicable age range, if relevant
        #[prost(message, optional, tag="8")]
        pub age: ::core::option::Option<super::super::proto::Range>,
        /// Text based reference range in an observation
        #[prost(message, optional, tag="9")]
        pub text: ::core::option::Option<super::super::proto::String>,
    }
    /// Resource related to this observation
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Related {
        /// xml:id (or equivalent in JSON)
        #[prost(message, optional, tag="1")]
        pub id: ::core::option::Option<super::super::proto::String>,
        /// Additional Content defined by implementations
        #[prost(message, repeated, tag="2")]
        pub extension: ::prost::alloc::vec::Vec<super::super::proto::Extension>,
        /// Extensions that cannot be ignored
        #[prost(message, repeated, tag="3")]
        pub modifier_extension: ::prost::alloc::vec::Vec<super::super::proto::Extension>,
        /// has-member | derived-from | sequel-to | replaces | qualified-by |
        /// interfered-by
        #[prost(message, optional, tag="4")]
        pub r#type: ::core::option::Option<super::super::proto::ObservationRelationshipTypeCode>,
        /// Resource that is related to this one
        #[prost(message, optional, tag="5")]
        pub target: ::core::option::Option<super::super::proto::Reference>,
    }
    /// Component results
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Component {
        /// xml:id (or equivalent in JSON)
        #[prost(message, optional, tag="1")]
        pub id: ::core::option::Option<super::super::proto::String>,
        /// Additional Content defined by implementations
        #[prost(message, repeated, tag="2")]
        pub extension: ::prost::alloc::vec::Vec<super::super::proto::Extension>,
        /// Extensions that cannot be ignored
        #[prost(message, repeated, tag="3")]
        pub modifier_extension: ::prost::alloc::vec::Vec<super::super::proto::Extension>,
        /// Type of component observation (code / type)
        #[prost(message, optional, tag="4")]
        pub code: ::core::option::Option<super::super::proto::CodeableConcept>,
        #[prost(message, optional, tag="5")]
        pub value: ::core::option::Option<component::Value>,
        /// Why the component result is missing
        #[prost(message, optional, tag="6")]
        pub data_absent_reason: ::core::option::Option<super::super::proto::CodeableConcept>,
        /// High, low, normal, etc.
        #[prost(message, optional, tag="7")]
        pub interpretation: ::core::option::Option<super::super::proto::CodeableConcept>,
        /// Provides guide for interpretation of component result
        #[prost(message, repeated, tag="8")]
        pub reference_range: ::prost::alloc::vec::Vec<ReferenceRange>,
    }
    /// Nested message and enum types in `Component`.
    pub mod component {
        /// Actual component result
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Value {
            #[prost(oneof="value::Value", tags="1, 2, 3, 4, 5, 6, 7, 8, 9, 10")]
            pub value: ::core::option::Option<value::Value>,
        }
        /// Nested message and enum types in `Value`.
        pub mod value {
            #[derive(Clone, PartialEq, ::prost::Oneof)]
            pub enum Value {
                #[prost(message, tag="1")]
                Quantity(super::super::super::super::proto::Quantity),
                #[prost(message, tag="2")]
                CodeableConcept(super::super::super::super::proto::CodeableConcept),
                #[prost(message, tag="3")]
                StringValue(super::super::super::super::proto::String),
                #[prost(message, tag="4")]
                Range(super::super::super::super::proto::Range),
                #[prost(message, tag="5")]
                Ratio(super::super::super::super::proto::Ratio),
                #[prost(message, tag="6")]
                SampledData(super::super::super::super::proto::SampledData),
                #[prost(message, tag="7")]
                Attachment(super::super::super::super::proto::Attachment),
                #[prost(message, tag="8")]
                Time(super::super::super::super::proto::Time),
                #[prost(message, tag="9")]
                DateTime(super::super::super::super::proto::DateTime),
                #[prost(message, tag="10")]
                Period(super::super::super::super::proto::Period),
            }
        }
    }
}
