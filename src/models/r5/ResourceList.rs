#![allow(unused_imports, non_camel_case_types)]

use crate::models::r5::Account::Account;
use crate::models::r5::ActivityDefinition::ActivityDefinition;
use crate::models::r5::AdministrableProductDefinition::AdministrableProductDefinition;
use crate::models::r5::AdverseEvent::AdverseEvent;
use crate::models::r5::AllergyIntolerance::AllergyIntolerance;
use crate::models::r5::Appointment::Appointment;
use crate::models::r5::AppointmentResponse::AppointmentResponse;
use crate::models::r5::ArtifactAssessment::ArtifactAssessment;
use crate::models::r5::AuditEvent::AuditEvent;
use crate::models::r5::Basic::Basic;
use crate::models::r5::Binary::Binary;
use crate::models::r5::BiologicallyDerivedProduct::BiologicallyDerivedProduct;
use crate::models::r5::BodyStructure::BodyStructure;
use crate::models::r5::Bundle::Bundle;
use crate::models::r5::CapabilityStatement::CapabilityStatement;
use crate::models::r5::CapabilityStatement2::CapabilityStatement2;
use crate::models::r5::CarePlan::CarePlan;
use crate::models::r5::CareTeam::CareTeam;
use crate::models::r5::ChargeItem::ChargeItem;
use crate::models::r5::ChargeItemDefinition::ChargeItemDefinition;
use crate::models::r5::Citation::Citation;
use crate::models::r5::Claim::Claim;
use crate::models::r5::ClaimResponse::ClaimResponse;
use crate::models::r5::ClinicalImpression::ClinicalImpression;
use crate::models::r5::ClinicalUseDefinition::ClinicalUseDefinition;
use crate::models::r5::ClinicalUseIssue::ClinicalUseIssue;
use crate::models::r5::CodeSystem::CodeSystem;
use crate::models::r5::Communication::Communication;
use crate::models::r5::CommunicationRequest::CommunicationRequest;
use crate::models::r5::CompartmentDefinition::CompartmentDefinition;
use crate::models::r5::Composition::Composition;
use crate::models::r5::ConceptMap::ConceptMap;
use crate::models::r5::ConceptMap2::ConceptMap2;
use crate::models::r5::Condition::Condition;
use crate::models::r5::ConditionDefinition::ConditionDefinition;
use crate::models::r5::Consent::Consent;
use crate::models::r5::Contract::Contract;
use crate::models::r5::Coverage::Coverage;
use crate::models::r5::CoverageEligibilityRequest::CoverageEligibilityRequest;
use crate::models::r5::CoverageEligibilityResponse::CoverageEligibilityResponse;
use crate::models::r5::DetectedIssue::DetectedIssue;
use crate::models::r5::Device::Device;
use crate::models::r5::DeviceDefinition::DeviceDefinition;
use crate::models::r5::DeviceDispense::DeviceDispense;
use crate::models::r5::DeviceMetric::DeviceMetric;
use crate::models::r5::DeviceRequest::DeviceRequest;
use crate::models::r5::DeviceUsage::DeviceUsage;
use crate::models::r5::DiagnosticReport::DiagnosticReport;
use crate::models::r5::DocumentManifest::DocumentManifest;
use crate::models::r5::DocumentReference::DocumentReference;
use crate::models::r5::Encounter::Encounter;
use crate::models::r5::Endpoint::Endpoint;
use crate::models::r5::EnrollmentRequest::EnrollmentRequest;
use crate::models::r5::EnrollmentResponse::EnrollmentResponse;
use crate::models::r5::EpisodeOfCare::EpisodeOfCare;
use crate::models::r5::EventDefinition::EventDefinition;
use crate::models::r5::Evidence::Evidence;
use crate::models::r5::EvidenceReport::EvidenceReport;
use crate::models::r5::EvidenceVariable::EvidenceVariable;
use crate::models::r5::ExampleScenario::ExampleScenario;
use crate::models::r5::ExplanationOfBenefit::ExplanationOfBenefit;
use crate::models::r5::FamilyMemberHistory::FamilyMemberHistory;
use crate::models::r5::Flag::Flag;
use crate::models::r5::Goal::Goal;
use crate::models::r5::GraphDefinition::GraphDefinition;
use crate::models::r5::Group::Group;
use crate::models::r5::GuidanceResponse::GuidanceResponse;
use crate::models::r5::HealthcareService::HealthcareService;
use crate::models::r5::ImagingSelection::ImagingSelection;
use crate::models::r5::ImagingStudy::ImagingStudy;
use crate::models::r5::Immunization::Immunization;
use crate::models::r5::ImmunizationEvaluation::ImmunizationEvaluation;
use crate::models::r5::ImmunizationRecommendation::ImmunizationRecommendation;
use crate::models::r5::ImplementationGuide::ImplementationGuide;
use crate::models::r5::Ingredient::Ingredient;
use crate::models::r5::InsurancePlan::InsurancePlan;
use crate::models::r5::InventoryReport::InventoryReport;
use crate::models::r5::Invoice::Invoice;
use crate::models::r5::Library::Library;
use crate::models::r5::Linkage::Linkage;
use crate::models::r5::List::List;
use crate::models::r5::Location::Location;
use crate::models::r5::ManufacturedItemDefinition::ManufacturedItemDefinition;
use crate::models::r5::Measure::Measure;
use crate::models::r5::MeasureReport::MeasureReport;
use crate::models::r5::Medication::Medication;
use crate::models::r5::MedicationAdministration::MedicationAdministration;
use crate::models::r5::MedicationDispense::MedicationDispense;
use crate::models::r5::MedicationKnowledge::MedicationKnowledge;
use crate::models::r5::MedicationRequest::MedicationRequest;
use crate::models::r5::MedicationUsage::MedicationUsage;
use crate::models::r5::MedicinalProductDefinition::MedicinalProductDefinition;
use crate::models::r5::MessageDefinition::MessageDefinition;
use crate::models::r5::MessageHeader::MessageHeader;
use crate::models::r5::MolecularSequence::MolecularSequence;
use crate::models::r5::NamingSystem::NamingSystem;
use crate::models::r5::NutritionIntake::NutritionIntake;
use crate::models::r5::NutritionOrder::NutritionOrder;
use crate::models::r5::NutritionProduct::NutritionProduct;
use crate::models::r5::Observation::Observation;
use crate::models::r5::ObservationDefinition::ObservationDefinition;
use crate::models::r5::OperationDefinition::OperationDefinition;
use crate::models::r5::OperationOutcome::OperationOutcome;
use crate::models::r5::Organization::Organization;
use crate::models::r5::OrganizationAffiliation::OrganizationAffiliation;
use crate::models::r5::PackagedProductDefinition::PackagedProductDefinition;
use crate::models::r5::Parameters::Parameters;
use crate::models::r5::Patient::Patient;
use crate::models::r5::PaymentNotice::PaymentNotice;
use crate::models::r5::PaymentReconciliation::PaymentReconciliation;
use crate::models::r5::Permission::Permission;
use crate::models::r5::Person::Person;
use crate::models::r5::PlanDefinition::PlanDefinition;
use crate::models::r5::Practitioner::Practitioner;
use crate::models::r5::PractitionerRole::PractitionerRole;
use crate::models::r5::Procedure::Procedure;
use crate::models::r5::Provenance::Provenance;
use crate::models::r5::Questionnaire::Questionnaire;
use crate::models::r5::QuestionnaireResponse::QuestionnaireResponse;
use crate::models::r5::RegulatedAuthorization::RegulatedAuthorization;
use crate::models::r5::RelatedPerson::RelatedPerson;
use crate::models::r5::RequestGroup::RequestGroup;
use crate::models::r5::ResearchStudy::ResearchStudy;
use crate::models::r5::ResearchSubject::ResearchSubject;
use crate::models::r5::RiskAssessment::RiskAssessment;
use crate::models::r5::Schedule::Schedule;
use crate::models::r5::SearchParameter::SearchParameter;
use crate::models::r5::ServiceRequest::ServiceRequest;
use crate::models::r5::Slot::Slot;
use crate::models::r5::Specimen::Specimen;
use crate::models::r5::SpecimenDefinition::SpecimenDefinition;
use crate::models::r5::StructureDefinition::StructureDefinition;
use crate::models::r5::StructureMap::StructureMap;
use crate::models::r5::Subscription::Subscription;
use crate::models::r5::SubscriptionStatus::SubscriptionStatus;
use crate::models::r5::SubscriptionTopic::SubscriptionTopic;
use crate::models::r5::Substance::Substance;
use crate::models::r5::SubstanceDefinition::SubstanceDefinition;
use crate::models::r5::SubstanceNucleicAcid::SubstanceNucleicAcid;
use crate::models::r5::SubstancePolymer::SubstancePolymer;
use crate::models::r5::SubstanceProtein::SubstanceProtein;
use crate::models::r5::SubstanceReferenceInformation::SubstanceReferenceInformation;
use crate::models::r5::SubstanceSourceMaterial::SubstanceSourceMaterial;
use crate::models::r5::SupplyDelivery::SupplyDelivery;
use crate::models::r5::SupplyRequest::SupplyRequest;
use crate::models::r5::Task::Task;
use crate::models::r5::TerminologyCapabilities::TerminologyCapabilities;
use crate::models::r5::TestReport::TestReport;
use crate::models::r5::TestScript::TestScript;
use crate::models::r5::ValueSet::ValueSet;
use crate::models::r5::VerificationResult::VerificationResult;
use crate::models::r5::VisionPrescription::VisionPrescription;
use serde_json::value::Value;
use std::borrow::Cow;

#[derive(Debug)]
pub struct ResourceList<'a> {
    pub(crate) value: Cow<'a, Value>,
}

#[derive(Debug)]
pub enum ResourceListEnum<'a> {
    ResourceAccount(Account<'a>),
    ResourceActivityDefinition(ActivityDefinition<'a>),
    ResourceAdministrableProductDefinition(AdministrableProductDefinition<'a>),
    ResourceAdverseEvent(AdverseEvent<'a>),
    ResourceAllergyIntolerance(AllergyIntolerance<'a>),
    ResourceAppointment(Appointment<'a>),
    ResourceAppointmentResponse(AppointmentResponse<'a>),
    ResourceArtifactAssessment(ArtifactAssessment<'a>),
    ResourceAuditEvent(AuditEvent<'a>),
    ResourceBasic(Basic<'a>),
    ResourceBinary(Binary<'a>),
    ResourceBiologicallyDerivedProduct(BiologicallyDerivedProduct<'a>),
    ResourceBodyStructure(BodyStructure<'a>),
    ResourceBundle(Bundle<'a>),
    ResourceCapabilityStatement(CapabilityStatement<'a>),
    ResourceCapabilityStatement2(CapabilityStatement2<'a>),
    ResourceCarePlan(CarePlan<'a>),
    ResourceCareTeam(CareTeam<'a>),
    ResourceChargeItem(ChargeItem<'a>),
    ResourceChargeItemDefinition(ChargeItemDefinition<'a>),
    ResourceCitation(Citation<'a>),
    ResourceClaim(Claim<'a>),
    ResourceClaimResponse(ClaimResponse<'a>),
    ResourceClinicalImpression(ClinicalImpression<'a>),
    ResourceClinicalUseDefinition(ClinicalUseDefinition<'a>),
    ResourceClinicalUseIssue(ClinicalUseIssue<'a>),
    ResourceCodeSystem(CodeSystem<'a>),
    ResourceCommunication(Communication<'a>),
    ResourceCommunicationRequest(CommunicationRequest<'a>),
    ResourceCompartmentDefinition(CompartmentDefinition<'a>),
    ResourceComposition(Composition<'a>),
    ResourceConceptMap(ConceptMap<'a>),
    ResourceConceptMap2(ConceptMap2<'a>),
    ResourceCondition(Condition<'a>),
    ResourceConditionDefinition(ConditionDefinition<'a>),
    ResourceConsent(Consent<'a>),
    ResourceContract(Contract<'a>),
    ResourceCoverage(Coverage<'a>),
    ResourceCoverageEligibilityRequest(CoverageEligibilityRequest<'a>),
    ResourceCoverageEligibilityResponse(CoverageEligibilityResponse<'a>),
    ResourceDetectedIssue(DetectedIssue<'a>),
    ResourceDevice(Device<'a>),
    ResourceDeviceDefinition(DeviceDefinition<'a>),
    ResourceDeviceDispense(DeviceDispense<'a>),
    ResourceDeviceMetric(DeviceMetric<'a>),
    ResourceDeviceRequest(DeviceRequest<'a>),
    ResourceDeviceUsage(DeviceUsage<'a>),
    ResourceDiagnosticReport(DiagnosticReport<'a>),
    ResourceDocumentManifest(DocumentManifest<'a>),
    ResourceDocumentReference(DocumentReference<'a>),
    ResourceEncounter(Encounter<'a>),
    ResourceEndpoint(Endpoint<'a>),
    ResourceEnrollmentRequest(EnrollmentRequest<'a>),
    ResourceEnrollmentResponse(EnrollmentResponse<'a>),
    ResourceEpisodeOfCare(EpisodeOfCare<'a>),
    ResourceEventDefinition(EventDefinition<'a>),
    ResourceEvidence(Evidence<'a>),
    ResourceEvidenceReport(EvidenceReport<'a>),
    ResourceEvidenceVariable(EvidenceVariable<'a>),
    ResourceExampleScenario(ExampleScenario<'a>),
    ResourceExplanationOfBenefit(ExplanationOfBenefit<'a>),
    ResourceFamilyMemberHistory(FamilyMemberHistory<'a>),
    ResourceFlag(Flag<'a>),
    ResourceGoal(Goal<'a>),
    ResourceGraphDefinition(GraphDefinition<'a>),
    ResourceGroup(Group<'a>),
    ResourceGuidanceResponse(GuidanceResponse<'a>),
    ResourceHealthcareService(HealthcareService<'a>),
    ResourceImagingSelection(ImagingSelection<'a>),
    ResourceImagingStudy(ImagingStudy<'a>),
    ResourceImmunization(Immunization<'a>),
    ResourceImmunizationEvaluation(ImmunizationEvaluation<'a>),
    ResourceImmunizationRecommendation(ImmunizationRecommendation<'a>),
    ResourceImplementationGuide(ImplementationGuide<'a>),
    ResourceIngredient(Ingredient<'a>),
    ResourceInsurancePlan(InsurancePlan<'a>),
    ResourceInventoryReport(InventoryReport<'a>),
    ResourceInvoice(Invoice<'a>),
    ResourceLibrary(Library<'a>),
    ResourceLinkage(Linkage<'a>),
    ResourceList(List<'a>),
    ResourceLocation(Location<'a>),
    ResourceManufacturedItemDefinition(ManufacturedItemDefinition<'a>),
    ResourceMeasure(Measure<'a>),
    ResourceMeasureReport(MeasureReport<'a>),
    ResourceMedication(Medication<'a>),
    ResourceMedicationAdministration(MedicationAdministration<'a>),
    ResourceMedicationDispense(MedicationDispense<'a>),
    ResourceMedicationKnowledge(MedicationKnowledge<'a>),
    ResourceMedicationRequest(MedicationRequest<'a>),
    ResourceMedicationUsage(MedicationUsage<'a>),
    ResourceMedicinalProductDefinition(MedicinalProductDefinition<'a>),
    ResourceMessageDefinition(MessageDefinition<'a>),
    ResourceMessageHeader(MessageHeader<'a>),
    ResourceMolecularSequence(MolecularSequence<'a>),
    ResourceNamingSystem(NamingSystem<'a>),
    ResourceNutritionIntake(NutritionIntake<'a>),
    ResourceNutritionOrder(NutritionOrder<'a>),
    ResourceNutritionProduct(NutritionProduct<'a>),
    ResourceObservation(Observation<'a>),
    ResourceObservationDefinition(ObservationDefinition<'a>),
    ResourceOperationDefinition(OperationDefinition<'a>),
    ResourceOperationOutcome(OperationOutcome<'a>),
    ResourceOrganization(Organization<'a>),
    ResourceOrganizationAffiliation(OrganizationAffiliation<'a>),
    ResourcePackagedProductDefinition(PackagedProductDefinition<'a>),
    ResourceParameters(Parameters<'a>),
    ResourcePatient(Patient<'a>),
    ResourcePaymentNotice(PaymentNotice<'a>),
    ResourcePaymentReconciliation(PaymentReconciliation<'a>),
    ResourcePermission(Permission<'a>),
    ResourcePerson(Person<'a>),
    ResourcePlanDefinition(PlanDefinition<'a>),
    ResourcePractitioner(Practitioner<'a>),
    ResourcePractitionerRole(PractitionerRole<'a>),
    ResourceProcedure(Procedure<'a>),
    ResourceProvenance(Provenance<'a>),
    ResourceQuestionnaire(Questionnaire<'a>),
    ResourceQuestionnaireResponse(QuestionnaireResponse<'a>),
    ResourceRegulatedAuthorization(RegulatedAuthorization<'a>),
    ResourceRelatedPerson(RelatedPerson<'a>),
    ResourceRequestGroup(RequestGroup<'a>),
    ResourceResearchStudy(ResearchStudy<'a>),
    ResourceResearchSubject(ResearchSubject<'a>),
    ResourceRiskAssessment(RiskAssessment<'a>),
    ResourceSchedule(Schedule<'a>),
    ResourceSearchParameter(SearchParameter<'a>),
    ResourceServiceRequest(ServiceRequest<'a>),
    ResourceSlot(Slot<'a>),
    ResourceSpecimen(Specimen<'a>),
    ResourceSpecimenDefinition(SpecimenDefinition<'a>),
    ResourceStructureDefinition(StructureDefinition<'a>),
    ResourceStructureMap(StructureMap<'a>),
    ResourceSubscription(Subscription<'a>),
    ResourceSubscriptionStatus(SubscriptionStatus<'a>),
    ResourceSubscriptionTopic(SubscriptionTopic<'a>),
    ResourceSubstance(Substance<'a>),
    ResourceSubstanceDefinition(SubstanceDefinition<'a>),
    ResourceSubstanceNucleicAcid(SubstanceNucleicAcid<'a>),
    ResourceSubstancePolymer(SubstancePolymer<'a>),
    ResourceSubstanceProtein(SubstanceProtein<'a>),
    ResourceSubstanceReferenceInformation(SubstanceReferenceInformation<'a>),
    ResourceSubstanceSourceMaterial(SubstanceSourceMaterial<'a>),
    ResourceSupplyDelivery(SupplyDelivery<'a>),
    ResourceSupplyRequest(SupplyRequest<'a>),
    ResourceTask(Task<'a>),
    ResourceTerminologyCapabilities(TerminologyCapabilities<'a>),
    ResourceTestReport(TestReport<'a>),
    ResourceTestScript(TestScript<'a>),
    ResourceValueSet(ValueSet<'a>),
    ResourceVerificationResult(VerificationResult<'a>),
    ResourceVisionPrescription(VisionPrescription<'a>),
}

impl ResourceList<'_> {
    pub fn new(value: &Value) -> ResourceList {
        ResourceList {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    pub fn resource(&self) -> Option<ResourceListEnum> {
        let fhir_type = self.value["resourceType"].as_str().unwrap();
        match fhir_type {
            "Account" => Some(ResourceListEnum::ResourceAccount(Account {
                value: self.value.clone(),
            })),
            "ActivityDefinition" => Some(ResourceListEnum::ResourceActivityDefinition(
                ActivityDefinition {
                    value: self.value.clone(),
                },
            )),
            "AdministrableProductDefinition" => {
                Some(ResourceListEnum::ResourceAdministrableProductDefinition(
                    AdministrableProductDefinition {
                        value: self.value.clone(),
                    },
                ))
            }
            "AdverseEvent" => Some(ResourceListEnum::ResourceAdverseEvent(AdverseEvent {
                value: self.value.clone(),
            })),
            "AllergyIntolerance" => Some(ResourceListEnum::ResourceAllergyIntolerance(
                AllergyIntolerance {
                    value: self.value.clone(),
                },
            )),
            "Appointment" => Some(ResourceListEnum::ResourceAppointment(Appointment {
                value: self.value.clone(),
            })),
            "AppointmentResponse" => Some(ResourceListEnum::ResourceAppointmentResponse(
                AppointmentResponse {
                    value: self.value.clone(),
                },
            )),
            "ArtifactAssessment" => Some(ResourceListEnum::ResourceArtifactAssessment(
                ArtifactAssessment {
                    value: self.value.clone(),
                },
            )),
            "AuditEvent" => Some(ResourceListEnum::ResourceAuditEvent(AuditEvent {
                value: self.value.clone(),
            })),
            "Basic" => Some(ResourceListEnum::ResourceBasic(Basic {
                value: self.value.clone(),
            })),
            "Binary" => Some(ResourceListEnum::ResourceBinary(Binary {
                value: self.value.clone(),
            })),
            "BiologicallyDerivedProduct" => Some(
                ResourceListEnum::ResourceBiologicallyDerivedProduct(BiologicallyDerivedProduct {
                    value: self.value.clone(),
                }),
            ),
            "BodyStructure" => Some(ResourceListEnum::ResourceBodyStructure(BodyStructure {
                value: self.value.clone(),
            })),
            "Bundle" => Some(ResourceListEnum::ResourceBundle(Bundle {
                value: self.value.clone(),
            })),
            "CapabilityStatement" => Some(ResourceListEnum::ResourceCapabilityStatement(
                CapabilityStatement {
                    value: self.value.clone(),
                },
            )),
            "CapabilityStatement2" => Some(ResourceListEnum::ResourceCapabilityStatement2(
                CapabilityStatement2 {
                    value: self.value.clone(),
                },
            )),
            "CarePlan" => Some(ResourceListEnum::ResourceCarePlan(CarePlan {
                value: self.value.clone(),
            })),
            "CareTeam" => Some(ResourceListEnum::ResourceCareTeam(CareTeam {
                value: self.value.clone(),
            })),
            "ChargeItem" => Some(ResourceListEnum::ResourceChargeItem(ChargeItem {
                value: self.value.clone(),
            })),
            "ChargeItemDefinition" => Some(ResourceListEnum::ResourceChargeItemDefinition(
                ChargeItemDefinition {
                    value: self.value.clone(),
                },
            )),
            "Citation" => Some(ResourceListEnum::ResourceCitation(Citation {
                value: self.value.clone(),
            })),
            "Claim" => Some(ResourceListEnum::ResourceClaim(Claim {
                value: self.value.clone(),
            })),
            "ClaimResponse" => Some(ResourceListEnum::ResourceClaimResponse(ClaimResponse {
                value: self.value.clone(),
            })),
            "ClinicalImpression" => Some(ResourceListEnum::ResourceClinicalImpression(
                ClinicalImpression {
                    value: self.value.clone(),
                },
            )),
            "ClinicalUseDefinition" => Some(ResourceListEnum::ResourceClinicalUseDefinition(
                ClinicalUseDefinition {
                    value: self.value.clone(),
                },
            )),
            "ClinicalUseIssue" => Some(ResourceListEnum::ResourceClinicalUseIssue(
                ClinicalUseIssue {
                    value: self.value.clone(),
                },
            )),
            "CodeSystem" => Some(ResourceListEnum::ResourceCodeSystem(CodeSystem {
                value: self.value.clone(),
            })),
            "Communication" => Some(ResourceListEnum::ResourceCommunication(Communication {
                value: self.value.clone(),
            })),
            "CommunicationRequest" => Some(ResourceListEnum::ResourceCommunicationRequest(
                CommunicationRequest {
                    value: self.value.clone(),
                },
            )),
            "CompartmentDefinition" => Some(ResourceListEnum::ResourceCompartmentDefinition(
                CompartmentDefinition {
                    value: self.value.clone(),
                },
            )),
            "Composition" => Some(ResourceListEnum::ResourceComposition(Composition {
                value: self.value.clone(),
            })),
            "ConceptMap" => Some(ResourceListEnum::ResourceConceptMap(ConceptMap {
                value: self.value.clone(),
            })),
            "ConceptMap2" => Some(ResourceListEnum::ResourceConceptMap2(ConceptMap2 {
                value: self.value.clone(),
            })),
            "Condition" => Some(ResourceListEnum::ResourceCondition(Condition {
                value: self.value.clone(),
            })),
            "ConditionDefinition" => Some(ResourceListEnum::ResourceConditionDefinition(
                ConditionDefinition {
                    value: self.value.clone(),
                },
            )),
            "Consent" => Some(ResourceListEnum::ResourceConsent(Consent {
                value: self.value.clone(),
            })),
            "Contract" => Some(ResourceListEnum::ResourceContract(Contract {
                value: self.value.clone(),
            })),
            "Coverage" => Some(ResourceListEnum::ResourceCoverage(Coverage {
                value: self.value.clone(),
            })),
            "CoverageEligibilityRequest" => Some(
                ResourceListEnum::ResourceCoverageEligibilityRequest(CoverageEligibilityRequest {
                    value: self.value.clone(),
                }),
            ),
            "CoverageEligibilityResponse" => {
                Some(ResourceListEnum::ResourceCoverageEligibilityResponse(
                    CoverageEligibilityResponse {
                        value: self.value.clone(),
                    },
                ))
            }
            "DetectedIssue" => Some(ResourceListEnum::ResourceDetectedIssue(DetectedIssue {
                value: self.value.clone(),
            })),
            "Device" => Some(ResourceListEnum::ResourceDevice(Device {
                value: self.value.clone(),
            })),
            "DeviceDefinition" => Some(ResourceListEnum::ResourceDeviceDefinition(
                DeviceDefinition {
                    value: self.value.clone(),
                },
            )),
            "DeviceDispense" => Some(ResourceListEnum::ResourceDeviceDispense(DeviceDispense {
                value: self.value.clone(),
            })),
            "DeviceMetric" => Some(ResourceListEnum::ResourceDeviceMetric(DeviceMetric {
                value: self.value.clone(),
            })),
            "DeviceRequest" => Some(ResourceListEnum::ResourceDeviceRequest(DeviceRequest {
                value: self.value.clone(),
            })),
            "DeviceUsage" => Some(ResourceListEnum::ResourceDeviceUsage(DeviceUsage {
                value: self.value.clone(),
            })),
            "DiagnosticReport" => Some(ResourceListEnum::ResourceDiagnosticReport(
                DiagnosticReport {
                    value: self.value.clone(),
                },
            )),
            "DocumentManifest" => Some(ResourceListEnum::ResourceDocumentManifest(
                DocumentManifest {
                    value: self.value.clone(),
                },
            )),
            "DocumentReference" => Some(ResourceListEnum::ResourceDocumentReference(
                DocumentReference {
                    value: self.value.clone(),
                },
            )),
            "Encounter" => Some(ResourceListEnum::ResourceEncounter(Encounter {
                value: self.value.clone(),
            })),
            "Endpoint" => Some(ResourceListEnum::ResourceEndpoint(Endpoint {
                value: self.value.clone(),
            })),
            "EnrollmentRequest" => Some(ResourceListEnum::ResourceEnrollmentRequest(
                EnrollmentRequest {
                    value: self.value.clone(),
                },
            )),
            "EnrollmentResponse" => Some(ResourceListEnum::ResourceEnrollmentResponse(
                EnrollmentResponse {
                    value: self.value.clone(),
                },
            )),
            "EpisodeOfCare" => Some(ResourceListEnum::ResourceEpisodeOfCare(EpisodeOfCare {
                value: self.value.clone(),
            })),
            "EventDefinition" => Some(ResourceListEnum::ResourceEventDefinition(EventDefinition {
                value: self.value.clone(),
            })),
            "Evidence" => Some(ResourceListEnum::ResourceEvidence(Evidence {
                value: self.value.clone(),
            })),
            "EvidenceReport" => Some(ResourceListEnum::ResourceEvidenceReport(EvidenceReport {
                value: self.value.clone(),
            })),
            "EvidenceVariable" => Some(ResourceListEnum::ResourceEvidenceVariable(
                EvidenceVariable {
                    value: self.value.clone(),
                },
            )),
            "ExampleScenario" => Some(ResourceListEnum::ResourceExampleScenario(ExampleScenario {
                value: self.value.clone(),
            })),
            "ExplanationOfBenefit" => Some(ResourceListEnum::ResourceExplanationOfBenefit(
                ExplanationOfBenefit {
                    value: self.value.clone(),
                },
            )),
            "FamilyMemberHistory" => Some(ResourceListEnum::ResourceFamilyMemberHistory(
                FamilyMemberHistory {
                    value: self.value.clone(),
                },
            )),
            "Flag" => Some(ResourceListEnum::ResourceFlag(Flag {
                value: self.value.clone(),
            })),
            "Goal" => Some(ResourceListEnum::ResourceGoal(Goal {
                value: self.value.clone(),
            })),
            "GraphDefinition" => Some(ResourceListEnum::ResourceGraphDefinition(GraphDefinition {
                value: self.value.clone(),
            })),
            "Group" => Some(ResourceListEnum::ResourceGroup(Group {
                value: self.value.clone(),
            })),
            "GuidanceResponse" => Some(ResourceListEnum::ResourceGuidanceResponse(
                GuidanceResponse {
                    value: self.value.clone(),
                },
            )),
            "HealthcareService" => Some(ResourceListEnum::ResourceHealthcareService(
                HealthcareService {
                    value: self.value.clone(),
                },
            )),
            "ImagingSelection" => Some(ResourceListEnum::ResourceImagingSelection(
                ImagingSelection {
                    value: self.value.clone(),
                },
            )),
            "ImagingStudy" => Some(ResourceListEnum::ResourceImagingStudy(ImagingStudy {
                value: self.value.clone(),
            })),
            "Immunization" => Some(ResourceListEnum::ResourceImmunization(Immunization {
                value: self.value.clone(),
            })),
            "ImmunizationEvaluation" => Some(ResourceListEnum::ResourceImmunizationEvaluation(
                ImmunizationEvaluation {
                    value: self.value.clone(),
                },
            )),
            "ImmunizationRecommendation" => Some(
                ResourceListEnum::ResourceImmunizationRecommendation(ImmunizationRecommendation {
                    value: self.value.clone(),
                }),
            ),
            "ImplementationGuide" => Some(ResourceListEnum::ResourceImplementationGuide(
                ImplementationGuide {
                    value: self.value.clone(),
                },
            )),
            "Ingredient" => Some(ResourceListEnum::ResourceIngredient(Ingredient {
                value: self.value.clone(),
            })),
            "InsurancePlan" => Some(ResourceListEnum::ResourceInsurancePlan(InsurancePlan {
                value: self.value.clone(),
            })),
            "InventoryReport" => Some(ResourceListEnum::ResourceInventoryReport(InventoryReport {
                value: self.value.clone(),
            })),
            "Invoice" => Some(ResourceListEnum::ResourceInvoice(Invoice {
                value: self.value.clone(),
            })),
            "Library" => Some(ResourceListEnum::ResourceLibrary(Library {
                value: self.value.clone(),
            })),
            "Linkage" => Some(ResourceListEnum::ResourceLinkage(Linkage {
                value: self.value.clone(),
            })),
            "List" => Some(ResourceListEnum::ResourceList(List {
                value: self.value.clone(),
            })),
            "Location" => Some(ResourceListEnum::ResourceLocation(Location {
                value: self.value.clone(),
            })),
            "ManufacturedItemDefinition" => Some(
                ResourceListEnum::ResourceManufacturedItemDefinition(ManufacturedItemDefinition {
                    value: self.value.clone(),
                }),
            ),
            "Measure" => Some(ResourceListEnum::ResourceMeasure(Measure {
                value: self.value.clone(),
            })),
            "MeasureReport" => Some(ResourceListEnum::ResourceMeasureReport(MeasureReport {
                value: self.value.clone(),
            })),
            "Medication" => Some(ResourceListEnum::ResourceMedication(Medication {
                value: self.value.clone(),
            })),
            "MedicationAdministration" => Some(ResourceListEnum::ResourceMedicationAdministration(
                MedicationAdministration {
                    value: self.value.clone(),
                },
            )),
            "MedicationDispense" => Some(ResourceListEnum::ResourceMedicationDispense(
                MedicationDispense {
                    value: self.value.clone(),
                },
            )),
            "MedicationKnowledge" => Some(ResourceListEnum::ResourceMedicationKnowledge(
                MedicationKnowledge {
                    value: self.value.clone(),
                },
            )),
            "MedicationRequest" => Some(ResourceListEnum::ResourceMedicationRequest(
                MedicationRequest {
                    value: self.value.clone(),
                },
            )),
            "MedicationUsage" => Some(ResourceListEnum::ResourceMedicationUsage(MedicationUsage {
                value: self.value.clone(),
            })),
            "MedicinalProductDefinition" => Some(
                ResourceListEnum::ResourceMedicinalProductDefinition(MedicinalProductDefinition {
                    value: self.value.clone(),
                }),
            ),
            "MessageDefinition" => Some(ResourceListEnum::ResourceMessageDefinition(
                MessageDefinition {
                    value: self.value.clone(),
                },
            )),
            "MessageHeader" => Some(ResourceListEnum::ResourceMessageHeader(MessageHeader {
                value: self.value.clone(),
            })),
            "MolecularSequence" => Some(ResourceListEnum::ResourceMolecularSequence(
                MolecularSequence {
                    value: self.value.clone(),
                },
            )),
            "NamingSystem" => Some(ResourceListEnum::ResourceNamingSystem(NamingSystem {
                value: self.value.clone(),
            })),
            "NutritionIntake" => Some(ResourceListEnum::ResourceNutritionIntake(NutritionIntake {
                value: self.value.clone(),
            })),
            "NutritionOrder" => Some(ResourceListEnum::ResourceNutritionOrder(NutritionOrder {
                value: self.value.clone(),
            })),
            "NutritionProduct" => Some(ResourceListEnum::ResourceNutritionProduct(
                NutritionProduct {
                    value: self.value.clone(),
                },
            )),
            "Observation" => Some(ResourceListEnum::ResourceObservation(Observation {
                value: self.value.clone(),
            })),
            "ObservationDefinition" => Some(ResourceListEnum::ResourceObservationDefinition(
                ObservationDefinition {
                    value: self.value.clone(),
                },
            )),
            "OperationDefinition" => Some(ResourceListEnum::ResourceOperationDefinition(
                OperationDefinition {
                    value: self.value.clone(),
                },
            )),
            "OperationOutcome" => Some(ResourceListEnum::ResourceOperationOutcome(
                OperationOutcome {
                    value: self.value.clone(),
                },
            )),
            "Organization" => Some(ResourceListEnum::ResourceOrganization(Organization {
                value: self.value.clone(),
            })),
            "OrganizationAffiliation" => Some(ResourceListEnum::ResourceOrganizationAffiliation(
                OrganizationAffiliation {
                    value: self.value.clone(),
                },
            )),
            "PackagedProductDefinition" => Some(
                ResourceListEnum::ResourcePackagedProductDefinition(PackagedProductDefinition {
                    value: self.value.clone(),
                }),
            ),
            "Parameters" => Some(ResourceListEnum::ResourceParameters(Parameters {
                value: self.value.clone(),
            })),
            "Patient" => Some(ResourceListEnum::ResourcePatient(Patient {
                value: self.value.clone(),
            })),
            "PaymentNotice" => Some(ResourceListEnum::ResourcePaymentNotice(PaymentNotice {
                value: self.value.clone(),
            })),
            "PaymentReconciliation" => Some(ResourceListEnum::ResourcePaymentReconciliation(
                PaymentReconciliation {
                    value: self.value.clone(),
                },
            )),
            "Permission" => Some(ResourceListEnum::ResourcePermission(Permission {
                value: self.value.clone(),
            })),
            "Person" => Some(ResourceListEnum::ResourcePerson(Person {
                value: self.value.clone(),
            })),
            "PlanDefinition" => Some(ResourceListEnum::ResourcePlanDefinition(PlanDefinition {
                value: self.value.clone(),
            })),
            "Practitioner" => Some(ResourceListEnum::ResourcePractitioner(Practitioner {
                value: self.value.clone(),
            })),
            "PractitionerRole" => Some(ResourceListEnum::ResourcePractitionerRole(
                PractitionerRole {
                    value: self.value.clone(),
                },
            )),
            "Procedure" => Some(ResourceListEnum::ResourceProcedure(Procedure {
                value: self.value.clone(),
            })),
            "Provenance" => Some(ResourceListEnum::ResourceProvenance(Provenance {
                value: self.value.clone(),
            })),
            "Questionnaire" => Some(ResourceListEnum::ResourceQuestionnaire(Questionnaire {
                value: self.value.clone(),
            })),
            "QuestionnaireResponse" => Some(ResourceListEnum::ResourceQuestionnaireResponse(
                QuestionnaireResponse {
                    value: self.value.clone(),
                },
            )),
            "RegulatedAuthorization" => Some(ResourceListEnum::ResourceRegulatedAuthorization(
                RegulatedAuthorization {
                    value: self.value.clone(),
                },
            )),
            "RelatedPerson" => Some(ResourceListEnum::ResourceRelatedPerson(RelatedPerson {
                value: self.value.clone(),
            })),
            "RequestGroup" => Some(ResourceListEnum::ResourceRequestGroup(RequestGroup {
                value: self.value.clone(),
            })),
            "ResearchStudy" => Some(ResourceListEnum::ResourceResearchStudy(ResearchStudy {
                value: self.value.clone(),
            })),
            "ResearchSubject" => Some(ResourceListEnum::ResourceResearchSubject(ResearchSubject {
                value: self.value.clone(),
            })),
            "RiskAssessment" => Some(ResourceListEnum::ResourceRiskAssessment(RiskAssessment {
                value: self.value.clone(),
            })),
            "Schedule" => Some(ResourceListEnum::ResourceSchedule(Schedule {
                value: self.value.clone(),
            })),
            "SearchParameter" => Some(ResourceListEnum::ResourceSearchParameter(SearchParameter {
                value: self.value.clone(),
            })),
            "ServiceRequest" => Some(ResourceListEnum::ResourceServiceRequest(ServiceRequest {
                value: self.value.clone(),
            })),
            "Slot" => Some(ResourceListEnum::ResourceSlot(Slot {
                value: self.value.clone(),
            })),
            "Specimen" => Some(ResourceListEnum::ResourceSpecimen(Specimen {
                value: self.value.clone(),
            })),
            "SpecimenDefinition" => Some(ResourceListEnum::ResourceSpecimenDefinition(
                SpecimenDefinition {
                    value: self.value.clone(),
                },
            )),
            "StructureDefinition" => Some(ResourceListEnum::ResourceStructureDefinition(
                StructureDefinition {
                    value: self.value.clone(),
                },
            )),
            "StructureMap" => Some(ResourceListEnum::ResourceStructureMap(StructureMap {
                value: self.value.clone(),
            })),
            "Subscription" => Some(ResourceListEnum::ResourceSubscription(Subscription {
                value: self.value.clone(),
            })),
            "SubscriptionStatus" => Some(ResourceListEnum::ResourceSubscriptionStatus(
                SubscriptionStatus {
                    value: self.value.clone(),
                },
            )),
            "SubscriptionTopic" => Some(ResourceListEnum::ResourceSubscriptionTopic(
                SubscriptionTopic {
                    value: self.value.clone(),
                },
            )),
            "Substance" => Some(ResourceListEnum::ResourceSubstance(Substance {
                value: self.value.clone(),
            })),
            "SubstanceDefinition" => Some(ResourceListEnum::ResourceSubstanceDefinition(
                SubstanceDefinition {
                    value: self.value.clone(),
                },
            )),
            "SubstanceNucleicAcid" => Some(ResourceListEnum::ResourceSubstanceNucleicAcid(
                SubstanceNucleicAcid {
                    value: self.value.clone(),
                },
            )),
            "SubstancePolymer" => Some(ResourceListEnum::ResourceSubstancePolymer(
                SubstancePolymer {
                    value: self.value.clone(),
                },
            )),
            "SubstanceProtein" => Some(ResourceListEnum::ResourceSubstanceProtein(
                SubstanceProtein {
                    value: self.value.clone(),
                },
            )),
            "SubstanceReferenceInformation" => {
                Some(ResourceListEnum::ResourceSubstanceReferenceInformation(
                    SubstanceReferenceInformation {
                        value: self.value.clone(),
                    },
                ))
            }
            "SubstanceSourceMaterial" => Some(ResourceListEnum::ResourceSubstanceSourceMaterial(
                SubstanceSourceMaterial {
                    value: self.value.clone(),
                },
            )),
            "SupplyDelivery" => Some(ResourceListEnum::ResourceSupplyDelivery(SupplyDelivery {
                value: self.value.clone(),
            })),
            "SupplyRequest" => Some(ResourceListEnum::ResourceSupplyRequest(SupplyRequest {
                value: self.value.clone(),
            })),
            "Task" => Some(ResourceListEnum::ResourceTask(Task {
                value: self.value.clone(),
            })),
            "TerminologyCapabilities" => Some(ResourceListEnum::ResourceTerminologyCapabilities(
                TerminologyCapabilities {
                    value: self.value.clone(),
                },
            )),
            "TestReport" => Some(ResourceListEnum::ResourceTestReport(TestReport {
                value: self.value.clone(),
            })),
            "TestScript" => Some(ResourceListEnum::ResourceTestScript(TestScript {
                value: self.value.clone(),
            })),
            "ValueSet" => Some(ResourceListEnum::ResourceValueSet(ValueSet {
                value: self.value.clone(),
            })),
            "VerificationResult" => Some(ResourceListEnum::ResourceVerificationResult(
                VerificationResult {
                    value: self.value.clone(),
                },
            )),
            "VisionPrescription" => Some(ResourceListEnum::ResourceVisionPrescription(
                VisionPrescription {
                    value: self.value.clone(),
                },
            )),
            _ => None,
        }
    }

    pub fn validate(&self) -> bool {
        if let Some(resource) = self.resource() {
            match resource {
                ResourceListEnum::ResourceAccount(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceActivityDefinition(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceAdministrableProductDefinition(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceAdverseEvent(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceAllergyIntolerance(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceAppointment(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceAppointmentResponse(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceArtifactAssessment(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceAuditEvent(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceBasic(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceBinary(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceBiologicallyDerivedProduct(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceBodyStructure(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceBundle(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceCapabilityStatement(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceCapabilityStatement2(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceCarePlan(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceCareTeam(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceChargeItem(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceChargeItemDefinition(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceCitation(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceClaim(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceClaimResponse(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceClinicalImpression(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceClinicalUseDefinition(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceClinicalUseIssue(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceCodeSystem(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceCommunication(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceCommunicationRequest(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceCompartmentDefinition(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceComposition(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceConceptMap(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceConceptMap2(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceCondition(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceConditionDefinition(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceConsent(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceContract(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceCoverage(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceCoverageEligibilityRequest(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceCoverageEligibilityResponse(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceDetectedIssue(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceDevice(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceDeviceDefinition(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceDeviceDispense(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceDeviceMetric(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceDeviceRequest(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceDeviceUsage(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceDiagnosticReport(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceDocumentManifest(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceDocumentReference(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceEncounter(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceEndpoint(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceEnrollmentRequest(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceEnrollmentResponse(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceEpisodeOfCare(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceEventDefinition(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceEvidence(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceEvidenceReport(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceEvidenceVariable(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceExampleScenario(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceExplanationOfBenefit(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceFamilyMemberHistory(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceFlag(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceGoal(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceGraphDefinition(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceGroup(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceGuidanceResponse(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceHealthcareService(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceImagingSelection(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceImagingStudy(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceImmunization(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceImmunizationEvaluation(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceImmunizationRecommendation(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceImplementationGuide(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceIngredient(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceInsurancePlan(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceInventoryReport(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceInvoice(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceLibrary(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceLinkage(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceList(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceLocation(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceManufacturedItemDefinition(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceMeasure(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceMeasureReport(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceMedication(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceMedicationAdministration(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceMedicationDispense(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceMedicationKnowledge(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceMedicationRequest(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceMedicationUsage(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceMedicinalProductDefinition(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceMessageDefinition(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceMessageHeader(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceMolecularSequence(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceNamingSystem(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceNutritionIntake(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceNutritionOrder(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceNutritionProduct(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceObservation(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceObservationDefinition(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceOperationDefinition(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceOperationOutcome(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceOrganization(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceOrganizationAffiliation(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourcePackagedProductDefinition(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceParameters(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourcePatient(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourcePaymentNotice(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourcePaymentReconciliation(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourcePermission(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourcePerson(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourcePlanDefinition(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourcePractitioner(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourcePractitionerRole(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceProcedure(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceProvenance(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceQuestionnaire(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceQuestionnaireResponse(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceRegulatedAuthorization(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceRelatedPerson(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceRequestGroup(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceResearchStudy(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceResearchSubject(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceRiskAssessment(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceSchedule(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceSearchParameter(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceServiceRequest(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceSlot(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceSpecimen(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceSpecimenDefinition(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceStructureDefinition(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceStructureMap(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceSubscription(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceSubscriptionStatus(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceSubscriptionTopic(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceSubstance(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceSubstanceDefinition(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceSubstanceNucleicAcid(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceSubstancePolymer(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceSubstanceProtein(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceSubstanceReferenceInformation(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceSubstanceSourceMaterial(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceSupplyDelivery(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceSupplyRequest(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceTask(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceTerminologyCapabilities(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceTestReport(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceTestScript(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceValueSet(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceVerificationResult(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceVisionPrescription(val) => {
                    return val.validate();
                }
            }
        }
        return false;
    }
}
