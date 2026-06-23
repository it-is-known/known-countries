// This is free and unencumbered software released into the public domain.

use core::str::FromStr;

#[cfg(feature = "alloc")]
use alloc::{
    borrow::Cow,
    string::{String, ToString},
};

/// A country (based on ISO 3166-1).
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "borsh", derive(borsh::BorshSerialize, borsh::BorshDeserialize))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub enum Country {

    /// United Arab Emirates ("ae" in ISO 3166-1)
    UnitedArabEmirates,

    /// Argentina ("ar" in ISO 3166-1)
    Argentina,

    /// Australia ("au" in ISO 3166-1)
    Australia,

    /// Brazil ("br" in ISO 3166-1)
    Brazil,

    /// Canada ("ca" in ISO 3166-1)
    Canada,

    /// Switzerland ("ch" in ISO 3166-1)
    Switzerland,

    /// China ("cn" in ISO 3166-1)
    China,

    /// Germany ("de" in ISO 3166-1)
    Germany,

    /// Egypt ("eg" in ISO 3166-1)
    Egypt,

    /// Spain ("es" in ISO 3166-1)
    Spain,

    /// Finland ("fi" in ISO 3166-1)
    Finland,

    /// France ("fr" in ISO 3166-1)
    France,

    /// United Kingdom ("gb" in ISO 3166-1)
    UnitedKingdom,

    /// Greece ("gr" in ISO 3166-1)
    Greece,

    /// India ("in" in ISO 3166-1)
    India,

    /// Italy ("it" in ISO 3166-1)
    Italy,

    /// Japan ("jp" in ISO 3166-1)
    Japan,

    /// South Korea ("kr" in ISO 3166-1)
    SouthKorea,

    /// Mexico ("mx" in ISO 3166-1)
    Mexico,

    /// Netherlands ("nl" in ISO 3166-1)
    Netherlands,

    /// Norway ("no" in ISO 3166-1)
    Norway,

    /// New Zealand ("nz" in ISO 3166-1)
    NewZealand,

    /// Poland ("pl" in ISO 3166-1)
    Poland,

    /// Saudi Arabia ("sa" in ISO 3166-1)
    SaudiArabia,

    /// Sweden ("se" in ISO 3166-1)
    Sweden,

    /// Singapore ("sg" in ISO 3166-1)
    Singapore,

    /// Turkey ("tr" in ISO 3166-1)
    Turkey,

    /// Ukraine ("ua" in ISO 3166-1)
    Ukraine,

    /// United States ("us" in ISO 3166-1)
    #[default]
    UnitedStates,

    /// South Africa ("za" in ISO 3166-1)
    SouthAfrica,

    #[cfg(feature = "alloc")]
    Other(String),
}

impl Country {
    pub const ALL: &'static [Self] = &[];

    pub fn as_str(&self) -> &str {
        use Country::*;
        match self {
            UnitedArabEmirates => "ae",
            Argentina => "ar",
            Australia => "au",
            Brazil => "br",
            Canada => "ca",
            Switzerland => "ch",
            China => "cn",
            Germany => "de",
            Egypt => "eg",
            Spain => "es",
            Finland => "fi",
            France => "fr",
            UnitedKingdom => "gb",
            Greece => "gr",
            India => "in",
            Italy => "it",
            Japan => "jp",
            SouthKorea => "kr",
            Mexico => "mx",
            Netherlands => "nl",
            Norway => "no",
            NewZealand => "nz",
            Poland => "pl",
            SaudiArabia => "sa",
            Sweden => "se",
            Singapore => "sg",
            Turkey => "tr",
            Ukraine => "ua",
            UnitedStates => "us",
            SouthAfrica => "za",

            #[cfg(feature = "alloc")]
            Other(input) => input.as_str(),
        }
    }

    #[cfg(feature = "alloc")]
    pub fn into_string(self) -> String {
        self.as_str().into()
    }

    #[cfg(all(feature = "serde", feature = "alloc"))]
    pub fn to_json(&self) -> Option<serde_json::Value> {
        Some(self.clone().into_json())
    }

    #[cfg(all(feature = "serde", feature = "alloc"))]
    pub fn into_json(self) -> serde_json::Value {
        serde_json::Value::String(self.into_string())
    }

    #[cfg(all(feature = "bson", feature = "alloc"))]
    pub fn to_bson(&self) -> Option<bson::Bson> {
        Some(self.clone().into_bson())
    }

    #[cfg(all(feature = "bson", feature = "alloc"))]
    pub fn into_bson(self) -> bson::Bson {
        bson::Bson::String(self.into_string())
    }
}

impl FromStr for Country {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        use Country::*;
        Ok(match input {
            "ae" => UnitedArabEmirates,
            "ar" => Argentina,
            "au" => Australia,
            "br" => Brazil,
            "ca" => Canada,
            "ch" => Switzerland,
            "cn" => China,
            "de" => Germany,
            "eg" => Egypt,
            "es" => Spain,
            "fi" => Finland,
            "fr" => France,
            "gb" => UnitedKingdom,
            "gr" => Greece,
            "in" => India,
            "it" => Italy,
            "jp" => Japan,
            "kr" => SouthKorea,
            "mx" => Mexico,
            "nl" => Netherlands,
            "no" => Norway,
            "nz" => NewZealand,
            "pl" => Poland,
            "sa" => SaudiArabia,
            "se" => Sweden,
            "sg" => Singapore,
            "tr" => Turkey,
            "ua" => Ukraine,
            "us" => UnitedStates,
            "za" => SouthAfrica,
            _ => return Err(()),
        })
    }
}

impl core::fmt::Display for Country {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl AsRef<str> for Country {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl<T> From<&T> for Country
where
    T: Clone + Into<Self>,
{
    fn from(t: &T) -> Self {
        t.clone().into()
    }
}

impl From<&str> for Country {
    fn from(input: &str) -> Self {
        match input.parse() {
            Ok(output) => output,

            #[cfg(feature = "alloc")]
            Err(_) => Self::Other(input.into()),

            #[cfg(not(feature = "alloc"))]
            Err(_) => unimplemented!("unknown country: {}", input),
        }
    }
}

#[cfg(feature = "alloc")]
impl<'a> From<Cow<'a, str>> for Country {
    fn from(input: Cow<'a, str>) -> Self {
        input
            .parse()
            .unwrap_or_else(|_| Self::Other(input.into_owned()))
    }
}

#[cfg(feature = "alloc")]
impl From<String> for Country {
    fn from(input: String) -> Self {
        input.parse().unwrap_or_else(|_| Self::Other(input))
    }
}

#[cfg(all(feature = "serde", feature = "alloc"))]
impl TryFrom<serde_json::Value> for Country {
    type Error = ();

    fn try_from(input: serde_json::Value) -> Result<Self, Self::Error> {
        use serde_json::Value;
        match input {
            Value::String(input) => Ok(input.parse().unwrap_or_else(|_| Self::Other(input))),
            _ => Err(()),
        }
    }
}

impl From<&'static Country> for &str {
    fn from(input: &'static Country) -> Self {
        input.as_str()
    }
}

#[cfg(feature = "alloc")]
impl From<Country> for String {
    fn from(input: Country) -> Self {
        input.to_string()
    }
}
