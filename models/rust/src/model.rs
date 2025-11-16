use crate::helpers::{CordisDate, CordisDateTime};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct Identifiers {
    pub grant_doi: Option<String>,
    pub issn: Option<String>,
    pub doi: Option<String>,
    pub isbn: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct Validation {
    pub euro_sci_voc: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct Category {
    #[serde(rename = "@classification")]
    pub classification: String,
    #[serde(rename = "@type")]
    pub _type: String,
    #[serde(rename = "@weight")]
    pub weight: Option<u64>,
    #[serde(rename = "@order")]
    pub order: Option<u64>,
    pub language: String,
    pub available_languages: String,
    pub code: String,
    pub title: String,
    pub description: Option<String>,
    pub display_code: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Categories {
    pub category: Vec<Category>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    pub street: Option<String>,
    pub city: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
    pub post_box: Option<String>,
    pub url: Option<String>,
    pub geolocation: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub enum AssociationType {
    AssociatedPartner,
    Coordinator,
    IsRegisteredBy,
    Participant,
    ThirdParty,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct Organization {
    #[serde(rename = "@netEcContribution")]
    pub net_ec_contribution: Option<f64>,
    #[serde(rename = "@totalCost")]
    pub total_cost: Option<f64>,
    #[serde(rename = "@source")]
    pub source: String,
    #[serde(rename = "@order")]
    pub order: Option<u64>,
    #[serde(rename = "@ecContribution")]
    pub ec_contribution: Option<f64>,
    #[serde(rename = "@terminated")]
    pub terminated: Option<bool>,
    #[serde(rename = "@sme")]
    pub sme: Option<bool>, // small-medium enterprise?
    #[serde(rename = "@type")]
    pub _type: AssociationType,
    pub available_languages: String,
    pub rcn: String,
    pub id: String,
    pub vat_number: Option<String>,
    pub legal_name: String,
    pub short_name: Option<String>,
    pub address: Address,
    pub department_name: Option<String>,
    pub relations: Relations,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct Article {
    #[serde(rename = "@source")]
    pub source: String,
    #[serde(rename = "@type")]
    pub _type: String,
    #[serde(rename = "@order")]
    pub order: Option<u64>,
    #[serde(rename = "@section")]
    pub section: Option<String>,
    pub available_languages: String,
    pub rcn: String,
    pub id: Option<String>,
    pub title: String,
    pub content_update_date: Option<CordisDateTime>,
    pub archived_date: Option<CordisDateTime>,
    pub relations: Relations,
    pub teaser: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct Call {
    #[serde(rename = "@source")]
    pub source: String,
    #[serde(rename = "@type")]
    pub _type: String,
    pub rcn: u64,
    pub title: String,
    pub identifier: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct Programme {
    #[serde(rename = "@type")]
    pub _type: Option<String>,
    #[serde(rename = "@source")]
    pub source: Option<String>,
    #[serde(rename = "@uniqueProgrammePart")]
    pub unique_programme_part: Option<bool>,
    pub available_languages: Option<String>,
    pub rcn: String,
    pub id: Option<String>,
    pub code: String,
    pub framework_programme: Option<String>,
    pub pga: Option<String>,
    pub title: Option<String>,
    pub parent: Option<Programmes>,
    pub relations: Option<Relations>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct Result {
    #[serde(rename = "@type")]
    pub _type: Option<String>,
    #[serde(rename = "@source")]
    pub source: String,
    pub available_languages: String,
    pub rcn: u64,
    pub id: String,
    pub title: Option<String>,
    pub details: Option<ResultDetails>,
    pub identifiers: Option<Identifiers>,
    pub description: Option<String>,
    pub teaser: Option<String>,
    pub source_update_date: Option<CordisDateTime>,
    pub content_update_date: Option<CordisDateTime>,
    pub relations: Relations,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct ResultDetails {
    pub authors: Option<String>,
    pub journal_number: Option<String>,
    pub journal_title: Option<String>,
    pub published_pages: Option<String>,
    pub published_year: Option<String>,
    pub publisher: Option<String>,
    pub ipr_awarded: Option<String>,
    pub ipr_number: Option<String>,
    pub ipr_date: Option<CordisDate>,
    pub ipr_prefix: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct Programmes {
    pub programme: Vec<Programme>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct WebItem {
    #[serde(rename = "@type")]
    pub _type: String,
    #[serde(rename = "@order")]
    pub order: Option<u64>,
    #[serde(rename = "@source")]
    pub source: Option<String>,
    #[serde(rename = "@represents")]
    pub represents: Option<String>,
    pub title: Option<String>,
    pub language: String,
    pub available_languages: String,
    pub uri: String,
    pub alternative_text: Option<String>,
    pub mimetype: Option<String>,
    pub size: u64,
    pub hash_value: Option<String>,
    pub copyright: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct WebLink {
    #[serde(rename = "@source")]
    pub source: String,
    #[serde(rename = "@type")]
    pub _type: String,
    #[serde(rename = "@represents")]
    pub represents: Option<String>,

    pub title: Option<String>,
    pub language: String,
    pub available_languages: String,
    pub id: String,
    pub phys_url: String,
    pub default_language: String,
    pub archived_date: Option<CordisDateTime>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct Associatons {
    pub article: Option<Vec<Article>>,
    pub call: Option<Vec<Call>>,
    pub organization: Option<Vec<Organization>>,
    pub programme: Option<Vec<Programme>>,
    pub result: Option<Vec<Result>>,
    pub web_item: Option<Vec<WebItem>>,
    pub web_link: Option<Vec<WebLink>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct Region {
    #[serde(rename = "@type")]
    pub _type: Option<String>,
    pub name: Option<String>,
    pub rcn: u64,
    pub nuts_code: Option<String>,
    pub parents: Option<Regions>,
    pub eu_code: Option<String>,
    pub iso_code: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct Regions {
    pub region: Vec<Region>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Relations {
    pub associations: Option<Associatons>,
    pub categories: Option<Categories>,
    pub regions: Option<Vec<Regions>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
#[serde(rename = "project")]
pub struct Project {
    pub language: String,
    pub available_languages: String,
    pub rcn: u64,
    pub id: u64,
    pub acronym: String,
    pub teaser: String,
    pub objective: String,
    pub title: String,
    pub total_cost: f64,
    pub ec_max_contribution: f64,
    pub start_date: CordisDate,
    pub end_date: CordisDate,
    pub ec_signature_date: Option<CordisDate>,
    pub duration: f64,
    pub status: String,
    pub keywords: Option<String>,
    pub identifiers: Option<Identifiers>,
    pub validation: Option<Validation>,
    pub source_update_date: CordisDateTime,
    pub content_creation_date: CordisDateTime,
    pub content_update_date: CordisDateTime,
    pub last_update_date: CordisDateTime,
    pub termination_date: Option<CordisDateTime>,
    pub relations: Relations,
}
