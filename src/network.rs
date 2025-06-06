// Network related structures extracted from lib.rs

#[cfg(feature = "indexmap")]
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
#[cfg(not(feature = "indexmap"))]
use std::collections::HashMap;

use crate::{Labels, MapOrEmpty, SingleValue};

/// Container for network definitions in a Compose file.
/// Maps network names to their configuration settings.
#[cfg(feature = "indexmap")]
#[derive(Clone, Default, Debug, Serialize, Deserialize, PartialEq)]
pub struct ComposeNetworks(pub IndexMap<String, MapOrEmpty<NetworkSettings>>);

/// Container for network definitions in a Compose file.
/// Maps network names to their configuration settings.
#[cfg(not(feature = "indexmap"))]
#[derive(Clone, Default, Debug, Serialize, Deserialize, PartialEq)]
pub struct ComposeNetworks(pub HashMap<String, MapOrEmpty<NetworkSettings>>);

impl ComposeNetworks {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

/// Represents a network configuration in a Compose file.
/// Can be either a detailed configuration or a simple boolean value.
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
#[serde(untagged)]
pub enum ComposeNetwork {
    /// Detailed network configuration with specific settings.
    Detailed(ComposeNetworkSettingDetails),
    /// Simple boolean flag for enabling/disabling a network.
    Bool(bool),
}

/// Detailed configuration for a network in a Compose file.
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
#[serde(deny_unknown_fields)]
pub struct ComposeNetworkSettingDetails {
    /// Name of the network.
    pub name: String,
}

/// Boolean wrapper for external network settings.
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
#[serde(deny_unknown_fields)]
pub struct ExternalNetworkSettingBool(bool);

/// Configuration settings for a network in a Compose file.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
#[serde(deny_unknown_fields)]
pub struct NetworkSettings {
    /// Whether the network can be attached to by external containers.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub attachable: bool,
    /// Network driver to use for this network.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver: Option<String>,
    /// Driver-specific options for this network.
    #[cfg(feature = "indexmap")]
    #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
    pub driver_opts: IndexMap<String, Option<SingleValue>>,
    /// Driver-specific options for this network.
    #[cfg(not(feature = "indexmap"))]
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub driver_opts: HashMap<String, Option<SingleValue>>,
    /// Enable IPv6 networking on this network.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub enable_ipv6: bool,
    /// Create an internal network that is isolated from external networks.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub internal: bool,
    /// Specifies that this network is externally created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external: Option<ComposeNetwork>,
    /// IP Address Management configuration for this network.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipam: Option<Ipam>,
    /// Metadata labels for the network.
    #[serde(default, skip_serializing_if = "Labels::is_empty")]
    pub labels: Labels,
    /// Custom name for the network.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// IP Address Management configuration for a network.
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
#[serde(deny_unknown_fields)]
pub struct Ipam {
    /// IPAM driver to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver: Option<String>,
    /// List of IPAM configuration blocks.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub config: Vec<IpamConfig>,
}

/// Configuration block for IPAM settings.
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
#[serde(deny_unknown_fields)]
pub struct IpamConfig {
    /// Subnet in CIDR format.
    pub subnet: String,
    /// Gateway address for the subnet.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway: Option<String>,
}
