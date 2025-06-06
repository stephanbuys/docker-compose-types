#[cfg(feature = "indexmap")]
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
#[cfg(feature = "yml")]
use serde_yml as serde_yaml;
#[cfg(not(feature = "indexmap"))]
use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt;
use std::str::FromStr;

use serde_yaml::Value;

mod network;
mod secret;
mod service;
mod volume;

pub use network::*;
pub use secret::*;
pub use service::*;
pub use volume::*;

/// Represents a Docker Compose document, supporting different formats and versions.
#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ComposeFile {
    /// Version 2 and above compose file structure.
    V2Plus(Compose),
    #[cfg(feature = "indexmap")]
    /// Legacy v1 service definitions as a map of service names to `Service`.
    V1(IndexMap<String, Service>),
    #[cfg(not(feature = "indexmap"))]
    /// Legacy v1 service definitions as a standard `HashMap`.
    V1(HashMap<String, Service>),
    /// Single service definition format, wrapping one `Service` instance.
    Single(SingleService),
}

/// Wrapper for a single `Service` when using the single service format.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct SingleService {
    /// The single service defined in the document ('service').
    pub service: Service,
}

/// Core Compose model containing version, services, volumes, networks, and extensions.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct Compose {
    /// Compose specification version string (e.g., '"3.8"').
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// Optional project name override ('name').
    pub name: Option<String>,
    /// Service definitions map ('services').
    #[serde(default, skip_serializing_if = "Services::is_empty")]
    pub services: Services,
    /// Top-level volume definitions ('volumes').
    #[serde(default, skip_serializing_if = "TopLevelVolumes::is_empty")]
    pub volumes: TopLevelVolumes,
    /// Network definitions ('networks').
    #[serde(default, skip_serializing_if = "ComposeNetworks::is_empty")]
    pub networks: ComposeNetworks,
    /// Optional single service inline support ('service').
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<Service>,
    /// Top-level secret definitions ('secrets').
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets: Option<ComposeSecrets>,
    /// Extension fields (keys starting with 'x-') flattened into a map.
    #[cfg(feature = "indexmap")]
    #[serde(flatten, skip_serializing_if = "IndexMap::is_empty")]
    pub extensions: IndexMap<Extension, Value>,
    #[cfg(not(feature = "indexmap"))]
    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    pub extensions: HashMap<Extension, Value>,
}

impl Compose {
    pub fn new() -> Self {
        Default::default()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Hash, Default, Ord, PartialOrd)]
#[serde(try_from = "String")]
pub struct Extension(String);

impl FromStr for Extension {
    type Err = ExtensionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let owned = s.to_owned();
        Extension::try_from(owned)
    }
}

impl TryFrom<String> for Extension {
    type Error = ExtensionParseError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        if s.starts_with("x-") {
            Ok(Self(s))
        } else {
            Err(ExtensionParseError(s))
        }
    }
}

/// The result of a failed TryFrom<String> conversion for [`Extension`]
///
/// Contains the string that was being converted
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct ExtensionParseError(pub String);

impl fmt::Display for ExtensionParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "unknown attribute {:?}, extensions must start with 'x-' (see https://docs.docker.com/compose/compose-file/#extension)", self.0)
    }
}

impl std::error::Error for ExtensionParseError {}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, PartialOrd)]
#[serde(untagged)]
pub enum SingleValue {
    String(String),
    Bool(bool),
    Unsigned(u64),
    Signed(i64),
    Float(f64),
}

impl fmt::Display for SingleValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::String(s) => f.write_str(s),
            Self::Bool(b) => write!(f, "{b}"),
            Self::Unsigned(u) => write!(f, "{u}"),
            Self::Signed(i) => write!(f, "{i}"),
            Self::Float(fl) => write!(f, "{fl}"),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Hash)]
#[serde(untagged)]
pub enum MapOrEmpty<T> {
    Map(T),
    Empty,
}

impl<T> Default for MapOrEmpty<T> {
    fn default() -> Self {
        Self::Empty
    }
}

impl<T> From<MapOrEmpty<T>> for Option<T> {
    fn from(value: MapOrEmpty<T>) -> Self {
        match value {
            MapOrEmpty::Map(t) => Some(t),
            MapOrEmpty::Empty => None,
        }
    }
}

impl<T> Serialize for MapOrEmpty<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::Map(t) => t.serialize(serializer),
            Self::Empty => {
                use serde::ser::SerializeMap;
                serializer.serialize_map(None)?.end()
            }
        }
    }
}
