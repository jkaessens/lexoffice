#![doc = "The countries endpoint provides read access to the list of countries known to lexoffice."]
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum TaxClassification {
    #[serde(rename = "de")]
    De,
    #[serde(rename = "intraCommunity")]
    IntraCommunity,
    #[serde(rename = "thirdPartyCountry")]
    ThirdPartyCountry,
}
impl std::str::FromStr for TaxClassification {
    type Err = serde_plain::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_plain::from_str::<Self>(s)
    }
}
#[derive(Debug, Clone, PartialEq, TypedBuilder, Serialize, Deserialize)]
#[builder(doc)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Country {
    #[doc = "The country's code. See [our FAQ](https://developers.lexoffice.io/docs/#faq-country-codes) for specification."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub country_code: Option<crate::types::CountryCode>,
    #[doc = "Country name (English)"]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub country_name_en: Option<String>,
    #[doc = "Country name (German translation)"]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub country_name_de: Option<String>,
    #[doc = "Tax classification. Possible values are **de** (*Germany*), **intraCommunity** (eligible for *Innergemeinschaftliche Lieferung*), and **thirdPartyCountry** (other). See [below](https://developers.lexoffice.io/docs/#countries-endpoint-country-tax-classification)"]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub tax_classification: Option<TaxClassification>,
}
