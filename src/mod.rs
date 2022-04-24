#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchParameter {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(enumeration="SearchParameterType", tag="2")]
    pub r#type: i32,
    #[prost(string, tag="3")]
    pub expression: ::prost::alloc::string::String,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum FhirVersion {
    Unknown = 0,
    Dstu2 = 1,
    Stu3 = 2,
    R4 = 4,
    R5 = 5,
}
/// TODO: Unify with StructureDefinitionKindCode
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum StructureDefinitionKindValue {
    KindUnknown = 0,
    KindPrimitiveType = 1,
    KindComplexType = 2,
    KindResource = 3,
    KindLogical = 4,
}
/// To annotate cardinality constraints.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Requirement {
    NotRequired = 0,
    RequiredByFhir = 1,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SearchParameterType {
    InvalidSearchParameterType = 0,
    Number = 1,
    Date = 2,
    String = 3,
    Token = 4,
    Reference = 5,
    Composite = 6,
    Quantity = 7,
    Uri = 8,
    Special = 9,
}
/// Metadata for generating FHIR packages
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PackageInfo {
    /// FHIR version for the package (e.g., STU3, R4, R5, etc).
    #[prost(enumeration="FhirVersion", tag="1")]
    pub fhir_version: i32,
    /// Base url to use for all profiles defined here.
    /// e.g., g.co/fhir/profiles
    #[prost(string, tag="2")]
    pub base_url: ::prost::alloc::string::String,
    /// Package for the generated protos for the resources.
    #[prost(string, tag="3")]
    pub proto_package: ::prost::alloc::string::String,
    /// Java package for the generated protos for the resources.
    #[prost(string, tag="4")]
    pub java_proto_package: ::prost::alloc::string::String,
    /// Go package for the generated protos for the resources.
    #[prost(string, tag="5")]
    pub go_proto_package: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub publisher: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub telcom_url: ::prost::alloc::string::String,
    #[prost(enumeration="package_info::License", tag="10")]
    pub license: i32,
    /// The copyright date to be included in the license text, which
    /// may simply be a year. The generator will use the current year
    /// if this is unset.
    #[prost(string, tag="11")]
    pub license_date: ::prost::alloc::string::String,
    /// If unset, defaults to TYPED_CONTAINED_RESOURCE for DSTU2 and STU3, and
    /// ANY for later versions.
    #[prost(enumeration="package_info::ContainedResourceBehavior", tag="12")]
    pub contained_resource_behavior: i32,
    /// If unset, defaults to no splitting
    #[prost(enumeration="package_info::FileSplittingBehavior", tag="13")]
    pub file_splitting_behavior: i32,
    /// Defines whether to generate a local ContainedResource proto or reuse one.
    /// If neither of these fields are set, use the core contained resource on
    /// all generated files.
    #[prost(oneof="package_info::ContainedResource", tags="6, 7")]
    pub contained_resource: ::core::option::Option<package_info::ContainedResource>,
}
/// Nested message and enum types in `PackageInfo`.
pub mod package_info {
    /// The license to include in the generated profile, if any.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum License {
        None = 0,
        Apache = 1,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ContainedResourceBehavior {
        /// See field below for default behavior.
        Default = 0,
        /// Uses protobuf.Any for contained resource fields.
        Any = 1,
        /// Use the corresponding resource protos
        TypedContainedResource = 2,
    }
    /// How the output messages should be divided into files.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum FileSplittingBehavior {
        /// Default is NO_SPLITTING
        DefaultSplittingBehavior = 0,
        /// All messages will be output into a single file,
        NoSplitting = 1,
        /// regardless of type
        ///
        /// All extensions will be put into a single file,
        SeparateExtensions = 2,
        /// and all other types will be put into a second
        /// file
        ///
        /// Like SEPARATE_EXTENSIONS, all extensions will
        SplitResources = 3,
    }
    /// Defines whether to generate a local ContainedResource proto or reuse one.
    /// If neither of these fields are set, use the core contained resource on
    /// all generated files.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ContainedResource {
        /// Generates a local ContainedResource
        #[prost(bool, tag="6")]
        LocalContainedResource(bool),
        /// for the profiles being created.
        ///
        /// A fully-qualified
        #[prost(string, tag="7")]
        ContainedResourcePackage(::prost::alloc::string::String),
    }
}
/// Config for definition of Profiles that are part of an Implementation
/// Guide to be generated
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Profiles {
    #[prost(message, repeated, tag="2")]
    pub profile: ::prost::alloc::vec::Vec<Profile>,
}
/// Config for definition of Extensions  that are part of an Implementation
/// Guide to be generated
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Extensions {
    #[prost(message, repeated, tag="2")]
    pub simple_extension: ::prost::alloc::vec::Vec<SimpleExtension>,
    #[prost(message, repeated, tag="3")]
    pub complex_extension: ::prost::alloc::vec::Vec<ComplexExtension>,
}
/// Config for definition of Terminologies (CodeSysems and ValueSets) that are
/// part of an Implementation Guide to be generated
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Terminologies {
    #[prost(message, repeated, tag="1")]
    pub code_system: ::prost::alloc::vec::Vec<CodeSystemConfig>,
    #[prost(message, repeated, tag="2")]
    pub value_set: ::prost::alloc::vec::Vec<ValueSetConfig>,
}
/// Defines a profile for a specific FHIR resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Profile {
    /// Description of the top-level element of the FHIR profile. For instance,
    /// this would be "Patient" in a patient profile.
    #[prost(message, optional, tag="1")]
    pub element_data: ::core::option::Option<ElementData>,
    /// Url of the Structure Definition being profiled
    /// e.g., <http://hl7.org/fhir/StructureDefinition/Patient>
    #[prost(string, tag="2")]
    pub base_url: ::prost::alloc::string::String,
    /// Element definitions to merge into the base definitions.
    /// Each ElementDefinition added here will be merged on top of the base
    /// definition for the element with the same ID, allowing adding new
    /// restrictions or documentation to the field. This cannot add any new fields,
    /// and it is invalid to supply an ElementDefinition whose id does not
    /// correspond to an id in the base definition.
    #[prost(message, repeated, tag="6")]
    pub element_definition: ::prost::alloc::vec::Vec<super::r4::core::ElementDefinition>,
    /// Extensions to be inlined as fields
    #[prost(message, repeated, tag="4")]
    pub extension_slice: ::prost::alloc::vec::Vec<ExtensionSlice>,
    /// CodeableConcept field to slice by Coding system uri.
    #[prost(message, repeated, tag="5")]
    pub codeable_concept_slice: ::prost::alloc::vec::Vec<CodeableConceptSlice>,
    /// Restrictions to existing fields (does not add new fields/slices)
    /// DEPRECATED - use `element_definition` instead
    #[prost(message, repeated, tag="3")]
    pub restriction: ::prost::alloc::vec::Vec<FieldRestriction>,
}
/// A restriction on a single field.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FieldRestriction {
    /// Id in the Structure Definition of the field to modify.
    /// e.g., Patient.birthDate
    #[prost(string, tag="1")]
    pub field_id: ::prost::alloc::string::String,
    /// Restriction on the size of the field.  Must be stricter than the
    /// original - the new size must be a valid subset of the original.
    #[prost(enumeration="SizeRestriction", tag="2")]
    pub size_restriction: i32,
    /// For reference fields, restricts what resources the reference
    /// can point to. Must be a subset of the original.
    #[prost(message, optional, tag="3")]
    pub reference_restriction: ::core::option::Option<ReferenceRestriction>,
    /// For choice fields, restricts what types the choice can have.
    /// Must be a subset of the original.
    #[prost(message, optional, tag="4")]
    pub choice_type_restriction: ::core::option::Option<ChoiceTypeRestriction>,
    #[prost(message, repeated, tag="5")]
    pub fhir_path_constraint: ::prost::alloc::vec::Vec<FhirPathConstraint>,
    /// For fields of type code, Coding, or CodeableConcept, binding to a ValueSet.
    /// See: <https://www.hl7.org/fhir/terminologies.html>
    #[prost(message, optional, tag="6")]
    pub value_set_binding: ::core::option::Option<ValueSetBinding>,
}
/// For Reference types, additional restrictions to apply
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReferenceRestriction {
    /// Allowed types to refer to.
    /// Must use unprofiled FHIR resource type name,
    /// e.g., Patient, Observation, Medication
    #[prost(string, repeated, tag="1")]
    pub allowed: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Which types are allowed for Choice types. These typically come from
/// the types at <https://www.hl7.org/fhir/valueset-defined-types.html>
/// but custom types ccan be used as well.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChoiceTypeRestriction {
    #[prost(string, repeated, tag="1")]
    pub allowed: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Structure used to create constraints as described in
/// <https://www.hl7.org/fhir/elementdefinition-definitions.html#ElementDefinition.constraint>
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FhirPathConstraint {
    #[prost(enumeration="fhir_path_constraint::Severity", tag="1")]
    pub severity: i32,
    /// Human-readable description of the constraint
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    /// The FHIRPath expression defining the constraint
    #[prost(string, tag="3")]
    pub expression: ::prost::alloc::string::String,
}
/// Nested message and enum types in `FhirPathConstraint`.
pub mod fhir_path_constraint {
    /// Constraint severity.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Severity {
        /// Generator defaults to error if unset
        Default = 0,
        Error = 1,
        Warning = 2,
    }
}
/// Declares an extension to add to a profile. Modeled as a slice because
/// extensions are always sliced by URL, per the FHIR spec.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExtensionSlice {
    /// Id in the Structure Definition of the field to add the extension to.
    /// e.g., Patient.birthDate
    /// Optional - if empty, defaults to top-level element.
    #[prost(string, tag="1")]
    pub field_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub element_data: ::core::option::Option<ElementData>,
    /// Must be an absolute url for the Structure Definition that defines this
    /// extension.
    /// e.g., <http://hl7.org/fhir/StructureDefinition/patient-birthTime>
    /// or <https://g.co/fhir/extensions/BinaryClassificationMetadata>
    #[prost(string, tag="3")]
    pub url: ::prost::alloc::string::String,
    /// Whether to set the "mustSupport" bit on the resulting ElementDefinition.
    /// See:
    /// <http://hl7.org/fhir/elementdefinition-definitions.html#ElementDefinition.mustSupport>
    #[prost(bool, tag="4")]
    pub must_support: bool,
}
/// Defines a FHIR slice on a codeable concept as described in
/// <https://www.hl7.org/fhir/profiling.html#slicing>
/// These are used in the protobuf representation of profiled resources
/// to create concrete fields for specific code systems.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CodeableConceptSlice {
    /// Id in the Structure Definition of the CodeableConcept to slice.
    /// e.g., Observation.code, or RiskAssessment.prediction.qualitativeRisk
    /// Note that this should be of type CodeableConcept, NOT type Coding.
    #[prost(string, tag="1")]
    pub field_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub coding_slice: ::prost::alloc::vec::Vec<codeable_concept_slice::CodingSlice>,
    /// Whether or not to allow Codings that do not fit into any of the above
    /// slices.
    /// Optional - defaults to OPEN
    #[prost(enumeration="super::r4::core::slicing_rules_code::Value", tag="4")]
    pub rules: i32,
}
/// Nested message and enum types in `CodeableConceptSlice`.
pub mod codeable_concept_slice {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CodingSlice {
        #[prost(message, optional, tag="1")]
        pub element_data: ::core::option::Option<super::ElementData>,
        #[prost(message, optional, tag="2")]
        pub code_data: ::core::option::Option<super::ValueSetBinding>,
    }
}
/// Definition for a complex extension, i.e. one that contains one or more nested
/// simple or complex extension fields.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ComplexExtension {
    #[prost(message, optional, tag="1")]
    pub element_data: ::core::option::Option<ElementData>,
    #[prost(bool, tag="2")]
    pub can_have_additional_extensions: bool,
    #[prost(message, repeated, tag="3")]
    pub simple_field: ::prost::alloc::vec::Vec<SimpleExtension>,
    #[prost(message, repeated, tag="4")]
    pub complex_field: ::prost::alloc::vec::Vec<ComplexExtension>,
}
/// Definition for a simple extension, i.e. one that contains
/// only a primitive data type.
/// TODO: support references with target profiles.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SimpleExtension {
    #[prost(message, optional, tag="1")]
    pub element_data: ::core::option::Option<ElementData>,
    /// Must be one of the types listed in the Extension.value\[x\] element here:
    /// <http://hl7.org/fhir/extension.profile.json>
    /// If this field has size > 1 it will generate a choice-type value
    /// For legacy reasons, if this is unset but `code_type` is set, this is
    /// assumed to be of type `code`.
    #[prost(string, repeated, tag="2")]
    pub r#type: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// For fields of type code, Coding, or CodeableConcept, binding to a ValueSet.
    /// See: <https://www.hl7.org/fhir/terminologies.html>
    #[prost(message, optional, tag="3")]
    pub code_type: ::core::option::Option<ValueSetBinding>,
    #[prost(bool, tag="4")]
    pub can_have_extensions: bool,
}
/// Description of a data element being profiled.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ElementData {
    /// Name for the element.
    /// For top-level elements:
    /// * This will be the name of the generated message
    /// * Must TitleCased and unique within all top-level elements in the package.
    /// * The url will be this name appended to Extensions#base_url.
    /// For subfields:
    /// * This will be the name of the field
    /// * Must jsonCased and unique within the containing message.
    /// * If this represents a container field, the generated message will be
    ///   this name in TitleCase.
    /// * The url will be equal to the name.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Size constraints for the element.
    #[prost(enumeration="SizeRestriction", tag="2")]
    pub size_restriction: i32,
    /// Human-readable description ofthe element.
    #[prost(string, tag="3")]
    pub description: ::prost::alloc::string::String,
    /// Short description.
    /// Optional - will re-use description if absent
    #[prost(string, tag="4")]
    pub short: ::prost::alloc::string::String,
    /// Free-form comment
    /// Optional
    #[prost(string, tag="5")]
    pub comment: ::prost::alloc::string::String,
    /// If set, provides an override for the structure definition URL.
    /// Should ONLY be used on top-level elements.
    #[prost(string, tag="6")]
    pub url_override: ::prost::alloc::string::String,
}
/// Describes ValueSet bindings for terminology fields.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValueSetBinding {
    /// Fixed ValueSet Url
    #[prost(string, tag="1")]
    pub system: ::prost::alloc::string::String,
    /// "Strength" of value set binding - i.e., whether to allow codes that
    /// are not in that value set.
    /// Optional - defaults to required.
    #[prost(enumeration="super::r4::core::binding_strength_code::Value", tag="2")]
    pub binding_strength: i32,
    /// Optional - if set, this code will have a fixed value, and will not be
    /// inlined as a field.
    #[prost(string, tag="3")]
    pub fixed_value: ::prost::alloc::string::String,
    /// Optional human-readable description of the binding.
    /// See:
    /// <https://www.hl7.org/fhir/elementdefinition-definitions.html#ElementDefinition.binding.description>
    #[prost(string, tag="4")]
    pub description: ::prost::alloc::string::String,
}
/// A compact representation of a code system used by the
/// TerminologyGenerator to build the FHIR equivalent.
/// See <https://www.hl7.org/fhir/codesystem.html> for
/// the FHIR resource this will generate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CodeSystemConfig {
    /// Sets the FHIR CodeSystem.name field.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Sets the FHIR CodeSystem.status field.
    #[prost(enumeration="super::r4::core::publication_status_code::Value", tag="2")]
    pub status: i32,
    /// Sets the FHIR CodeSystem.description field.
    #[prost(string, tag="3")]
    pub description: ::prost::alloc::string::String,
    /// Overrides the URL for the generated code system. The generator
    /// will use the PackageInfo.baseUrl/ValuesetConfig.name if this is unset.
    #[prost(string, tag="4")]
    pub url_override: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="5")]
    pub concept: ::prost::alloc::vec::Vec<code_system_config::Concept>,
}
/// Nested message and enum types in `CodeSystemConfig`.
pub mod code_system_config {
    /// Defines the CodeSystem.concept structure to be generated.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Concept {
        /// Sets the FHIR Codesystem.concept.code field.
        #[prost(string, tag="1")]
        pub code: ::prost::alloc::string::String,
        /// Sets the FHIR Codesystem.concept.display field.
        #[prost(string, tag="2")]
        pub display: ::prost::alloc::string::String,
        /// Sets the FHIR Codesystem.concept.definition field.
        #[prost(string, tag="3")]
        pub definition: ::prost::alloc::string::String,
        /// Flag indicating the code value is deprecated.
        #[prost(bool, tag="4")]
        pub deprecated: bool,
    }
}
/// A compact representation of a value set used by the
/// TerminologyGenerator to build the FHIR equivalent.
/// See <https://www.hl7.org/fhir/valueset.html> for the FHIR
/// resource this will generate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValueSetConfig {
    /// Sets the FHIR ValueSet.name field.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Sets the FHIR ValueSet.status field with the publication status.
    #[prost(enumeration="super::r4::core::publication_status_code::Value", tag="2")]
    pub status: i32,
    /// Sets the FHIR ValueSet.description field.
    #[prost(string, tag="3")]
    pub description: ::prost::alloc::string::String,
    /// Overrides the URL for the generated valueset. The generator
    /// will use the PackageInfo.baseUrl/ValuesetConfig.name if this is unset.
    #[prost(string, tag="4")]
    pub url_override: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="5")]
    pub system: ::prost::alloc::vec::Vec<value_set_config::System>,
}
/// Nested message and enum types in `ValueSetConfig`.
pub mod value_set_config {
    /// A code system and the subset of its values to include in the value set.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct System {
        /// The code system URL.
        #[prost(string, tag="1")]
        pub url: ::prost::alloc::string::String,
        // At most one of {include, exclude} fields should be set.
        // If neither is set, all codes from that system will included.

        /// Codes from this system to include.
        #[prost(string, repeated, tag="2")]
        pub include: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// Codes from this system to exclude.
        #[prost(string, repeated, tag="3")]
        pub exclude: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SizeRestriction {
    /// Does not set a size restriction on the field.
    Unset = 0,
    /// Field should have zero values, i.e. omitted from the record.
    Absent = 1,
    /// Field must have exactly one value.
    Required = 2,
    /// Field may have zero or one value.
    Optional = 3,
    /// Repeated field that should have at least one value.
    AtLeastOne = 4,
    /// Repeated field that may have zero or more values.
    Repeated = 5,
}
