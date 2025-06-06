// Secret related structures extracted from lib.rs

use serde::{Deserialize, Serialize};
#[cfg(feature = "indexmap")]
use indexmap::IndexMap;
#[cfg(not(feature = "indexmap"))]
use std::collections::HashMap;

#[cfg(feature = "indexmap")]
#[derive(Clone, Default, Debug, Serialize, Deserialize, PartialEq)]
pub struct ComposeSecrets(
    #[serde(with = "serde_yaml::with::singleton_map_recursive")]
    pub  IndexMap<String, Option<ComposeSecret>>,
);

#[cfg(not(feature = "indexmap"))]
#[derive(Clone, Default, Debug, Serialize, Deserialize, PartialEq)]
pub struct ComposeSecrets(
    #[serde(with = "serde_yaml::with::singleton_map_recursive")]
    pub  HashMap<String, Option<ComposeSecret>>,
);

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub enum ComposeSecret {
    File(String),
    Environment(String),
    #[serde(untagged)]
    External {
        external: bool,
        name: String,
    },
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
#[serde(untagged)]
pub enum Secrets {
    Simple(Vec<String>),
    Advanced(Vec<AdvancedSecrets>),
}

impl Default for Secrets {
    fn default() -> Self {
        Self::Simple(Vec::new())
    }
}

impl Secrets {
    pub fn is_empty(&self) -> bool {
        match self {
            Self::Simple(v) => v.is_empty(),
            Self::Advanced(v) => v.is_empty(),
        }
    }
}

#[derive(Clone, Default, Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
#[serde(deny_unknown_fields)]
pub struct AdvancedSecrets {
    pub source: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
}

