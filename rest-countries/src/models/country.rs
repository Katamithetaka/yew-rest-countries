use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]

#[serde(rename_all = "camelCase")]
pub struct CountryNameModel {
    /*
            "name": {
                "common": "Germany",
                "nativeName": {
                    "deu": {
                        "common": "Deutschland",
                        "official": "Bundesrepublik Deutschland"
                    }
                },
                "official": "Federal Republic of Germany"
            },
    */
    pub common: String,
    pub official: String,
}

type NativeNameObject = HashMap<String, CountryNameModel>;

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]

#[serde(rename_all = "camelCase")]
struct CountryNameDetails {
    pub common: String,
    pub official: String,
    pub native_name: NativeNameObject,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]

#[serde(rename_all = "camelCase")]
pub struct FlagsDetails {
    pub png: String,
    pub svg: String,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CountryModel {
    pub region: String,
    pub name: CountryNameModel,
    pub capital: Option<Vec<String>>,
    pub population: u64,
    pub flags: FlagsDetails,
    pub cca2: String
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]

#[serde(rename_all = "camelCase")]
pub struct CurrencyDetails {
    pub name: String,
    pub symbol: Option<String>,
}

type CurrencyType = HashMap<String, CurrencyDetails>;
type LanguagesType = HashMap<String, String>;

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]

#[serde(rename_all = "camelCase")]
pub struct CountryDetailsModel {
    pub region: String,
    pub subregion: Option<String>,
    pub name: CountryNameModel,
    pub capital: Option<Vec<String>>,
    pub population: u64,
    pub tld: Option<Vec<String>>,
    pub currencies: Option<CurrencyType>,
    pub flags: FlagsDetails,
    pub languages: Option<LanguagesType>,
    pub cca2: String
}
