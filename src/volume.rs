// Volume related structures extracted from lib.rs

#[cfg(feature = "indexmap")]
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
#[cfg(not(feature = "indexmap"))]
use std::collections::HashMap;

use crate::{Labels, MapOrEmpty, SingleValue};

/// Container for volume definitions in a Compose file.
/// Maps volume names to their configuration settings.
#[cfg(feature = "indexmap")]
#[derive(Clone, Default, Debug, Serialize, Deserialize, PartialEq)]
pub struct TopLevelVolumes(pub IndexMap<String, MapOrEmpty<ComposeVolume>>);
/// Container for volume definitions in a Compose file.
/// Maps volume names to their configuration settings.
#[cfg(not(feature = "indexmap"))]
#[derive(Clone, Default, Debug, Serialize, Deserialize, PartialEq)]
pub struct TopLevelVolumes(pub HashMap<String, MapOrEmpty<ComposeVolume>>);

impl TopLevelVolumes {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

/// Configuration for a volume in a Compose file.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ComposeVolume {
    /// Volume driver to use for this volume.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver: Option<String>,
    /// Driver-specific options for this volume.
    #[cfg(feature = "indexmap")]
    #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
    pub driver_opts: IndexMap<String, Option<SingleValue>>,
    /// Driver-specific options for this volume.
    #[cfg(not(feature = "indexmap"))]
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub driver_opts: HashMap<String, Option<SingleValue>>,
    /// Specifies that this volume is externally created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external: Option<ExternalVolume>,
    /// Metadata labels for the volume.
    #[serde(default, skip_serializing_if = "Labels::is_empty")]
    pub labels: Labels,
    /// Custom name for the volume.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Represents an external volume configuration in a Compose file.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ExternalVolume {
    /// Simple boolean flag for external volumes.
    Bool(bool),
    /// Named external volume with a specific name.
    Name { name: String },
}

/// Represents a volume configuration in a service.
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
#[serde(untagged)]
pub enum Volumes {
    /// Simple string volume specification (e.g., "./host:/container").
    Simple(String),
    /// Advanced volume configuration with detailed settings.
    Advanced(AdvancedVolumes),
}

/// Advanced volume configuration with detailed settings.
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
#[serde(deny_unknown_fields)]
pub struct AdvancedVolumes {
    /// Source path or volume name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// Mount path inside the container.
    pub target: String,
    /// Mount type (bind, volume, tmpfs, etc.).
    #[serde(rename = "type")]
    pub _type: String,
    /// Whether the volume is read-only.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub read_only: bool,
    /// Bind mount specific options.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bind: Option<Bind>,
    /// Named volume specific options.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub volume: Option<Volume>,
    /// Tmpfs specific options.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tmpfs: Option<TmpfsSettings>,
}

/// Configuration options for bind mounts.
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Hash, Default)]
#[serde(deny_unknown_fields)]
pub struct Bind {
    /// Propagation mode for the bind mount.
    pub propagation: Option<String>,
    /// Whether to create the host path if it doesn't exist.
    pub create_host_path: Option<bool>,
    /// SELinux context for the bind mount.
    pub selinux: Option<String>,
}

/// Configuration options for named volumes.
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Hash, Default)]
#[serde(deny_unknown_fields)]
pub struct Volume {
    /// Disable copying data from the container when a volume is created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nocopy: Option<bool>,
    /// Subpath within the volume to mount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subpath: Option<String>,
}

/// Configuration options for tmpfs mounts.
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Hash, Default)]
#[serde(deny_unknown_fields)]
pub struct TmpfsSettings {
    /// Size of the tmpfs mount in bytes.
    pub size: u64,
}
