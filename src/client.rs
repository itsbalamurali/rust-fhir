//Inspiration: https://github.com/Vermonster/fhir-kit-client/blob/master/lib/client.js

use anyhow::anyhow;
use regex::Regex;
use reqwest::header::HeaderMap;
use reqwest::Response;
use serde::Deserialize;

const SMART_OAUTH_URL: &str =
    "http://fhir-registry.smarthealthit.org/StructureDefinition/oauth-uris";

pub struct FhirRestClient<'a> {
    options: &'a ClientOptions,
    http_client: &'a reqwest::Client,
}

pub struct RequestOptions {
    pub cert: String,
    pub key: String,
    pub ca: String,
}

pub struct ClientOptions {
    pub base_url: String,
    pub bearer_token: Option<String>,
    pub custom_headers: Option<HeaderMap>,
    pub request_options: Option<RequestOptions>,
}

pub struct SmartAuthMetaData {
    pub authorize_url: String,
    pub token_url: String,
    pub register_url: Option<String>,
    pub manage_url: Option<String>,
}

#[derive(Deserialize)]
struct SmartConfiguration {
    pub issuer: Option<String>,
    #[serde(rename = "jwks_uri")]
    pub jwks_uri: Option<String>,
    #[serde(rename = "authorization_endpoint")]
    pub authorization_endpoint: String,
    #[serde(rename = "token_endpoint")]
    pub token_endpoint: String,
    #[serde(rename = "token_endpoint_auth_methods_supported")]
    pub token_endpoint_auth_methods_supported: Option<Vec<String>>,
    #[serde(rename = "grant_types_supported")]
    pub grant_types_supported: Vec<String>,
    #[serde(rename = "registration_endpoint")]
    pub registration_endpoint: Option<String>,
    #[serde(rename = "scopes_supported")]
    pub scopes_supported: Option<Vec<String>>,
    #[serde(rename = "response_types_supported")]
    pub response_types_supported: Option<Vec<String>>,
    #[serde(rename = "management_endpoint")]
    pub management_endpoint: Option<String>,
    #[serde(rename = "introspection_endpoint")]
    pub introspection_endpoint: Option<String>,
    #[serde(rename = "revocation_endpoint")]
    pub revocation_endpoint: Option<String>,
    #[serde(rename = "code_challenge_methods_supported")]
    pub code_challenge_methods_supported: Vec<String>,
    pub capabilities: Vec<String>,
}
#[derive(Deserialize)]
struct OpenIdConfiguration {}

impl FhirRestClient<'_> {
    /// Create a new FHIR client.
    fn new(options: ClientOptions) -> Self {
        let mut http_client = reqwest::Client::builder();
        let mut headers = HeaderMap::new();
        if options.bearer_token.is_some() {
            headers.insert(
                "Authorization",
                format!("Bearer {}", options.bearer_token.unwrap())
                    .parse()
                    .unwrap(),
            );
        }
        if options.custom_headers.is_some() {
            for (key, value) in options.custom_headers.unwrap().iter() {
                headers.insert(key, value.to_owned());
            }
        }
        http_client = http_client.default_headers(headers);
        http_client = http_client.user_agent("Rust FHIR Client (https://crates.io/fhir)");
        return Self {
            options: &options,
            http_client: &http_client.build().unwrap(),
        };
    }

    /// Get the base url.
    fn base_url(&self) -> String {
        return format!("{}/", self.options.base_url.trim_end_matches("/"));
    }

    /// Get a resource by FHIR id.
    pub async fn read(self, resource_type: String, id: String) -> Result<Response, anyhow::Error> {
        self.validate_resource_type(resource_type);
        return match self
            .http_client
            .get(format!("{}/{}/{}", self.base_url(), resource_type, id))
            .send()
            .await
        {
            Ok(resp) => Ok(resp),
            Err(err) => Err(anyhow!(err)),
        };
    }

    /// Get a resource by FHIR id and version.
    pub async fn vread(
        self,
        resource_type: String,
        id: String,
        version: String,
    ) -> Result<Response, anyhow::Error> {
        self.validate_resource_type(resource_type).unwrap();
        return match self
            .http_client
            .get(format!(
                "{}/{}/{}/_history/{}",
                self.base_url(),
                resource_type,
                id,
                version
            ))
            .send()
            .await
        {
            Ok(resp) => Ok(resp),
            Err(err) => Err(anyhow!(err)),
        };
    }

    /// Create a new FHIR resource.
    pub async fn create(
        self,
        resource_type: String,
        body: String,
    ) -> Result<Response, anyhow::Error> {
        self.validate_resource_type(resource_type);
        return match self
            .http_client
            .post(format!("{}/{}", self.base_url(), resource_type))
            .body(body)
            .send()
            .await
        {
            Ok(resp) => Ok(resp),
            Err(err) => Err(anyhow!(err)),
        };
    }

    /// Delete a resource by FHIR id.
    pub async fn delete(
        self,
        resource_type: String,
        id: String,
    ) -> Result<Response, anyhow::Error> {
        self.validate_resource_type(resource_type);
        return match self
            .http_client
            .delete(format!("{}/{}/{}", self.base_url(), resource_type, id))
            .send()
            .await
        {
            Ok(resp) => Ok(resp),
            Err(err) => Err(anyhow!(err)),
        };
    }

    /// Update a resource by FHIR id or search params.
    pub async fn update(
        self,
        resource_type: String,
        id: String,
        search_params: Option<String>,
        body: String,
    ) -> Result<Response, anyhow::Error> {
        self.validate_resource_type(resource_type);
        let mut url = format!("{}/{}", self.base_url(), resource_type);
        if search_params.is_some() {
            url = format!("{}?{}", url, search_params.unwrap())
        } else {
            url = format!("{}/{}", url, id)
        }
        return match self.http_client.put(url).body(body).send().await {
            Ok(resp) => Ok(resp),
            Err(err) => Err(anyhow!(err)),
        };
    }

    /// Patch a resource by FHIR id.
    pub async fn patch(
        self,
        resource_type: String,
        id: String,
        patch: String,
    ) -> Result<Response, anyhow::Error> {
        self.validate_resource_type(resource_type);
        let url = format!("{}/{}/{}", self.base_url(), resource_type, id);
        return match self.http_client.put(url).body(patch).send().await {
            Ok(resp) => Ok(resp),
            Err(err) => Err(anyhow!(err)),
        };
    }

    /// Perform batch operations on resource by FHIR id.
    pub async fn batch(
        self,
        resource_type: String,
        request_bundle: String,
    ) -> Result<Response, anyhow::Error> {
        self.validate_resource_type(resource_type);
        return match self
            .http_client
            .post(self.base_url())
            .body(request_bundle)
            .send()
            .await
        {
            Ok(resp) => Ok(resp),
            Err(err) => Err(anyhow!(err)),
        };
    }

    // TODO
    pub async fn operation(self) {}

    // TODO: Pagination
    pub fn next_page(self) {}

    // TODO: Pagination
    pub fn prev_page(self) {}

    // TODO: Search
    // pub async fn search(self, resource_type: String, compartment: String, search_params: BTreeMap<String,String>) -> Result<Response, reqwest::Error>  {
    //     self.validate_resource_type(resource_type);
    //     if !compartment.is_empty() && !resource_type.is_empty() {
    //         return self.compartment_search(&resource_type, compartment, search_params);
    //     }
    //     if !resourceType.is_empty() {
    //         return self.resource_search(&resource_type, search_params);
    //     }
    //     if !search_params.is_empty() {
    //         return this.system_search(search_params);
    //     }
    // }

    // Get the FHIR System History.
    pub async fn system_history(self) -> Result<Response, anyhow::Error> {
        return match self
            .http_client
            .get(format!("{}/{}", self.base_url(), "_history"))
            .send()
            .await
        {
            Ok(resp) => Ok(resp),
            Err(err) => Err(anyhow!(err)),
        };
    }

    fn validate_resource_type(&self, resource_type: String) -> Result<(), anyhow::Error> {
        const FHIR_REFERENCE_REGEX: &str = r#"/^((http|https):\/\/([A-Za-z0-9\\.:%$]*\/)*)?(Account|ActivityDefinition|AdverseEvent|AllergyIntolerance|Appointment|AppointmentResponse|AuditEvent|Basic|Binary|BiologicallyDerivedProduct|BodySite|BodyStructure|Bundle|CapabilityStatement|CarePlan|CareTeam|CatalogEntry|ChargeItem|ChargeItemDefinition|Claim|ClaimResponse|ClinicalImpression|CodeSystem|Communication|CommunicationRequest|CompartmentDefinition|Composition|ConceptMap|Condition|Conformance|Consent|Contract|Coverage|CoverageEligibilityRequest|CoverageEligibilityResponse|DataElement|DecisionSupportRule|DecisionSupportServiceModule|DetectedIssue|Device|DeviceComponent|DeviceDefinition|DeviceMetric|DeviceRequest|DeviceUseRequest|DeviceUseStatement|DiagnosticOrder|DiagnosticReport|DiagnosticRequest|DocumentManifest|DocumentReference|EffectEvidenceSynthesis|EligibilityRequest|EligibilityResponse|Encounter|Endpoint|EnrollmentRequest|EnrollmentResponse|EntryDefinition|EpisodeOfCare|EventDefinition|Evidence|EvidenceVariable|ExampleScenario|ExpansionProfile|ExplanationOfBenefit|FamilyMemberHistory|Flag|Goal|GraphDefinition|Group|GuidanceRequest|GuidanceResponse|HealthcareService|ImagingExcerpt|ImagingManifest|ImagingObjectSelection|ImagingStudy|Immunization|ImmunizationEvaluation|ImmunizationRecommendation|ImplementationGuide|ImplementationGuideInput|ImplementationGuideOutput|InsurancePlan|Invoice|ItemInstance|Library|Linkage|List|Location|Measure|MeasureReport|Media|Medication|MedicationAdministration|MedicationDispense|MedicationKnowledge|MedicationOrder|MedicationRequest|MedicationStatement|MedicinalProduct|MedicinalProductAuthorization|MedicinalProductClinicals|MedicinalProductContraindication|MedicinalProductDeviceSpec|MedicinalProductIndication|MedicinalProductIngredient|MedicinalProductInteraction|MedicinalProductManufactured|MedicinalProductPackaged|MedicinalProductPharmaceutical|MedicinalProductUndesirableEffect|MessageDefinition|MessageHeader|ModuleDefinition|ModuleMetadata|MolecularSequence|NamingSystem|NutritionOrder|NutritionRequest|Observation|ObservationDefinition|OccupationalData|OperationDefinition|OperationOutcome|Order|OrderResponse|OrderSet|Organization|OrganizationAffiliation|OrganizationRole|Patient|PaymentNotice|PaymentReconciliation|Person|PlanDefinition|Practitioner|PractitionerRole|Procedure|ProcedureRequest|ProcessRequest|ProcessResponse|ProductPlan|Protocol|Provenance|Questionnaire|QuestionnaireResponse|ReferralRequest|RelatedPerson|RequestGroup|ResearchDefinition|ResearchElementDefinition|ResearchStudy|ResearchSubject|RiskAssessment|RiskEvidenceSynthesis|Schedule|SearchParameter|Sequence|ServiceDefinition|ServiceRequest|Slot|Specimen|SpecimenDefinition|StructureDefinition|StructureMap|Subscription|Substance|SubstanceNucleicAcid|SubstancePolymer|SubstanceProtein|SubstanceReferenceInformation|SubstanceSourceMaterial|SubstanceSpecification|SupplyDelivery|SupplyRequest|Task|TerminologyCapabilities|TestReport|TestScript|UserSession|ValueSet|VerificationResult|VisionPrescription)\/[A-Za-z0-9\-.]{1,256}(\/_history\/[A-Za-z0-9\-.]{1,256})?$/,"#;
        let re = Regex::new(FHIR_REFERENCE_REGEX).unwrap();
        if !resource_type.is_empty() && re.is_match(resource_type.as_str()) {
            return Ok(());
        }
        return Err(anyhow!("Invalid resourceType: {}", resource_type));
    }

    ///  Obtain the SMART OAuth URLs from the Capability Statement, or any of the .well-known addresses.
    ///  See: http://docs.smarthealthit.org/authorization/conformance-statement/
    pub async fn smart_auth_metadata(self) -> Result<SmartAuthMetaData, anyhow::Error> {
        // Try
        let smart_config = match self
            .http_client
            .get(format!(
                "{}{}",
                self.base_url(),
                ".well-known/smart-configuration"
            ))
            .header("Accept", "application/fhir+json,application/json")
            .send()
            .await
        {
            Ok(resp) => resp.json::<SmartConfiguration>().await?,
            Err(e) => Err(anyhow!(e)),
        };
        //TODO: Try Capability statement
        //TODO: Try OpenID Config
        let openid_config = match self
            .http_client
            .get(format!(
                "{}{}",
                self.base_url(),
                ".well-known/openid-configuration"
            ))
            .send()
            .await
        {
            Ok(resp) => resp.json::<OpenIdConfiguration>().await?,
            Err(e) => Err(anyhow!(e)),
        };

        return Ok(SmartAuthMetaData {
            authorize_url: smart_config.authorization_endpoint,
            token_url: smart_config.token_endpoint,
            register_url: smart_config.registration_endpoint,
            manage_url: smart_config.management_endpoint,
        });
    }
}
