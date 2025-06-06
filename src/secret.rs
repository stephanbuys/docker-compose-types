// Secret related structures extracted from lib.rs

#[cfg(feature = "indexmap")]
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
#[cfg(not(feature = "indexmap"))]
use std::collections::HashMap;

/// Container for secret definitions in a Compose file.
/// Maps secret names to their configuration settings.
#[cfg(feature = "indexmap")]
#[derive(Clone, Default, Debug, Serialize, Deserialize, PartialEq)]
pub struct ComposeSecrets(
    #[serde(with = "serde_yaml::with::singleton_map_recursive")]
    pub  IndexMap<String, Option<ComposeSecret>>,
);

/// Container for secret definitions in a Compose file.
/// Maps secret names to their configuration settings.
#[cfg(not(feature = "indexmap"))]
#[derive(Clone, Default, Debug, Serialize, Deserialize, PartialEq)]
pub struct ComposeSecrets(
    #[serde(with = "serde_yaml::with::singleton_map_recursive")]
    pub  HashMap<String, Option<ComposeSecret>>,
);

/// Represents a secret configuration in a Compose file.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub enum ComposeSecret {
    /// Secret sourced from a file.
    File(String),
    /// Secret sourced from an environment variable.
    Environment(String),
    /// Secret that is externally managed.
    #[serde(untagged)]
    External {
        /// Whether the secret is external.
        external: bool,
        /// Name of the external secret.
        name: String,
    },
}

/// Represents secret configurations for a service.
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
#[serde(untagged)]
pub enum Secrets {
    /// Simple list of secret names.
    Simple(Vec<String>),
    /// Advanced secret configurations with detailed settings.
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

/// Advanced secret configuration with detailed settings.
#[derive(Clone, Default, Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
#[serde(deny_unknown_fields)]
pub struct AdvancedSecrets {
    /// Name of the secret in the Compose file.
    pub source: String,
    /// Name of the file to mount in the container.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    /// UID of the secret file in the container.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
    /// GID of the secret file in the container.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gid: Option<String>,
    /// File mode of the secret file in the container (octal).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
}
