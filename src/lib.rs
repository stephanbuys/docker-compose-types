use derive_builder::*;
#[cfg(feature = "indexmap")]
use indexmap::IndexMap;
use serde::{Deserialize, Deserializer, Serialize};
#[cfg(feature = "yml")]
use serde_yml as serde_yaml;
#[cfg(not(feature = "indexmap"))]
use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt;
use std::str::FromStr;

use serde_yaml::Value;

#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ComposeFile {
    V2Plus(Compose),
    #[cfg(feature = "indexmap")]
    V1(IndexMap<String, Service>),
    #[cfg(not(feature = "indexmap"))]
    V1(HashMap<String, Service>),
    Single(SingleService),
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct SingleService {
    pub service: Service,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct Compose {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub includes: Option<Includes>,
    #[serde(default, skip_serializing_if = "Services::is_empty")]
    pub services: Services,
    #[serde(default, skip_serializing_if = "TopLevelVolumes::is_empty")]
    pub volumes: TopLevelVolumes,
    #[serde(default, skip_serializing_if = "ComposeNetworks::is_empty")]
    pub networks: ComposeNetworks,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<Service>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets: Option<ComposeSecrets>,
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

#[derive(Builder, Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
#[builder(setter(into), default)]
pub struct Include {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_directory: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env_file: Option<EnvFile>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum Includes {
    Short(Vec<String>),
    Long(Vec<Include>),
}

impl Includes {
    pub fn is_empty(&self) -> bool {
        match self {
            Includes::Short(xs) => xs.is_empty(),
            Includes::Long(xs) => xs.is_empty(),
        }
    }
}

#[derive(Builder, Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
#[builder(setter(into), default)]
pub struct Service {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domainname: Option<String>,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub privileged: bool,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub read_only: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub healthcheck: Option<Healthcheck>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deploy: Option<Deploy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "build")]
    pub build_: Option<BuildStep>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pid: Option<String>,
    #[serde(default, skip_serializing_if = "Ports::is_empty")]
    pub ports: Ports,
    #[serde(default, skip_serializing_if = "Environment::is_empty")]
    pub environment: Environment,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_mode: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub devices: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restart: Option<String>,
    #[serde(default, skip_serializing_if = "Labels::is_empty")]
    pub labels: Labels,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tmpfs: Option<Tmpfs>,
    #[serde(default, skip_serializing_if = "Ulimits::is_empty")]
    pub ulimits: Ulimits,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub volumes: Vec<Volumes>,
    #[serde(default, skip_serializing_if = "Networks::is_empty")]
    pub networks: Networks,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub cap_add: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub cap_drop: Vec<String>,
    #[serde(default, skip_serializing_if = "DependsOnOptions::is_empty")]
    pub depends_on: DependsOnOptions,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<Command>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entrypoint: Option<Entrypoint>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env_file: Option<EnvFile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_grace_period: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub profiles: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dns: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_signal: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub userns_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_dir: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub expose: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub volumes_from: Vec<String>,
    #[cfg(feature = "indexmap")]
    #[serde(
        default,
        deserialize_with = "de_extends_indexmap",
        skip_serializing_if = "IndexMap::is_empty"
    )]
    pub extends: IndexMap<String, String>,
    #[cfg(not(feature = "indexmap"))]
    #[serde(
        default,
        deserialize_with = "de_extends_hashmap",
        skip_serializing_if = "HashMap::is_empty"
    )]
    pub extends: HashMap<String, String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging: Option<LoggingParameters>,
    #[serde(default, skip_serializing_if = "is_zero")]
    pub scale: i64,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub init: bool,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub stdin_open: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shm_size: Option<String>,
    #[cfg(feature = "indexmap")]
    #[serde(flatten, skip_serializing_if = "IndexMap::is_empty")]
    pub extensions: IndexMap<Extension, Value>,
    #[cfg(not(feature = "indexmap"))]
    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    pub extensions: HashMap<Extension, Value>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extra_hosts: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub group_add: Vec<Group>,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub tty: bool,
    #[serde(default, skip_serializing_if = "SysCtls::is_empty")]
    pub sysctls: SysCtls,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub security_opt: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets: Option<Secrets>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pull_policy: Option<PullPolicy>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cgroup_parent: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mem_limit: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mem_reservation: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mem_swappiness: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime: Option<String>,
}

#[cfg(feature = "indexmap")]
fn de_extends_indexmap<'de, D>(deserializer: D) -> Result<IndexMap<String, String>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Value::deserialize(deserializer)?;
    if let Some(value_str) = value.as_str() {
        let mut map = IndexMap::new();
        map.insert("service".to_string(), value_str.to_string());
        return Ok(map);
    }

    if let Some(value_map) = value.as_mapping() {
        let mut map = IndexMap::new();
        for (k, v) in value_map {
            if !k.is_string() || !v.is_string() {
                return Err(serde::de::Error::custom(
                    "extends must must have string type for both Keys and Values".to_string(),
                ));
            }
            //Should be safe due to previous check
            map.insert(
                k.as_str().unwrap().to_string(),
                v.as_str().unwrap().to_string(),
            );
        }
        return Ok(map);
    }

    Err(serde::de::Error::custom(
        "extends must either be a map or a string".to_string(),
    ))
}

#[cfg(not(feature = "indexmap"))]
fn de_extends_hashmap<'de, D>(deserializer: D) -> Result<HashMap<String, String>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Value::deserialize(deserializer)?;
    if let Some(value_str) = value.as_str() {
        let mut map = HashMap::new();
        map.insert("service".to_string(), value_str.to_string());
        return Ok(map);
    }

    if let Some(value_map) = value.as_mapping() {
        let mut map = HashMap::new();
        for (k, v) in value_map {
            if !k.is_string() || !v.is_string() {
                return Err(serde::de::Error::custom(
                    "extends must must have string type for both Keys and Values".to_string(),
                ));
            }
            //Should be safe due to previous check
            map.insert(
                k.as_str().unwrap().to_string(),
                v.as_str().unwrap().to_string(),
            );
        }
        return Ok(map);
    }

    Err(serde::de::Error::custom(
        "extends must either be a map or a string".to_string(),
    ))
}

impl Service {
    pub fn image(&self) -> &str {
        self.image.as_deref().unwrap_or_default()
    }

    pub fn network_mode(&self) -> &str {
        self.network_mode.as_deref().unwrap_or_default()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
#[serde(untagged)]
pub enum EnvFile {
    Simple(String),
    List(Vec<String>),
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(untagged)]
pub enum DependsOnOptions {
    Simple(Vec<String>),
    #[cfg(feature = "indexmap")]
    Conditional(IndexMap<String, DependsCondition>),
    #[cfg(not(feature = "indexmap"))]
    Conditional(HashMap<String, DependsCondition>),
}

impl Default for DependsOnOptions {
    fn default() -> Self {
        Self::Simple(Vec::new())
    }
}

impl DependsOnOptions {
    pub fn is_empty(&self) -> bool {
        match self {
            Self::Simple(v) => v.is_empty(),
            Self::Conditional(m) => m.is_empty(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct DependsCondition {
    pub condition: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct LoggingParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver: Option<String>,
    #[cfg(feature = "indexmap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<IndexMap<String, SingleValue>>,
    #[cfg(not(feature = "indexmap"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<HashMap<String, SingleValue>>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum Ports {
    Short(Vec<String>),
    Long(Vec<Port>),
}

impl Default for Ports {
    fn default() -> Self {
        Self::Short(Vec::default())
    }
}

impl Ports {
    pub fn is_empty(&self) -> bool {
        match self {
            Self::Short(v) => v.is_empty(),
            Self::Long(v) => v.is_empty(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Port {
    pub target: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_ip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published: Option<PublishedPort>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum PublishedPort {
    Single(u16),
    Range(String),
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum Environment {
    List(Vec<String>),
    #[cfg(feature = "indexmap")]
    KvPair(IndexMap<String, Option<SingleValue>>),
    #[cfg(not(feature = "indexmap"))]
    KvPair(HashMap<String, Option<SingleValue>>),
}

impl Default for Environment {
    fn default() -> Self {
        Self::List(Vec::new())
    }
}

impl Environment {
    pub fn is_empty(&self) -> bool {
        match self {
            Self::List(v) => v.is_empty(),
            Self::KvPair(m) => m.is_empty(),
        }
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

#[cfg(feature = "indexmap")]
#[derive(Clone, Default, Debug, Serialize, Deserialize, PartialEq)]
pub struct Services(pub IndexMap<String, Option<Service>>);
#[cfg(not(feature = "indexmap"))]
#[derive(Clone, Default, Debug, Serialize, Deserialize, PartialEq)]
pub struct Services(pub HashMap<String, Option<Service>>);

impl Services {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(untagged)]
pub enum Labels {
    List(Vec<String>),
    #[cfg(feature = "indexmap")]
    Map(IndexMap<String, String>),
    #[cfg(not(feature = "indexmap"))]
    Map(HashMap<String, String>),
}

impl Default for Labels {
    fn default() -> Self {
        Self::List(Vec::new())
    }
}

impl Labels {
    pub fn is_empty(&self) -> bool {
        match self {
            Self::List(v) => v.is_empty(),
            Self::Map(m) => m.is_empty(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(untagged)]
pub enum Tmpfs {
    Simple(String),
    List(Vec<String>),
}

#[cfg(feature = "indexmap")]
#[derive(Clone, Default, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct Ulimits(pub IndexMap<String, Ulimit>);
#[cfg(not(feature = "indexmap"))]
#[derive(Clone, Default, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct Ulimits(pub HashMap<String, Ulimit>);

impl Ulimits {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(untagged)]
pub enum Ulimit {
    Single(i64),
    SoftHard { soft: i64, hard: i64 },
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(untagged)]
pub enum Networks {
    Simple(Vec<String>),
    Advanced(AdvancedNetworks),
}

impl Default for Networks {
    fn default() -> Self {
        Self::Simple(Vec::new())
    }
}

impl Networks {
    pub fn is_empty(&self) -> bool {
        match self {
            Self::Simple(n) => n.is_empty(),
            Self::Advanced(n) => n.0.is_empty(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(untagged)]
pub enum BuildStep {
    Simple(String),
    Advanced(AdvancedBuildStep),
}

#[derive(Builder, Clone, Debug, Deserialize, Serialize, Eq, PartialEq, Default)]
#[serde(deny_unknown_fields)]
#[builder(setter(into), default)]
pub struct AdvancedBuildStep {
    pub context: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dockerfile: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub args: Option<BuildArgs>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shm_size: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub cache_from: Vec<String>,
    #[serde(default, skip_serializing_if = "Labels::is_empty")]
    pub labels: Labels,
}

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(untagged)]
pub enum BuildArgs {
    Simple(String),
    List(Vec<String>),
    #[cfg(feature = "indexmap")]
    KvPair(IndexMap<String, String>),
    #[cfg(not(feature = "indexmap"))]
    KvPair(HashMap<String, String>),
}

#[cfg(feature = "indexmap")]
#[derive(Clone, Default, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct AdvancedNetworks(pub IndexMap<String, MapOrEmpty<AdvancedNetworkSettings>>);
#[cfg(not(feature = "indexmap"))]
#[derive(Clone, Default, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct AdvancedNetworks(pub HashMap<String, MapOrEmpty<AdvancedNetworkSettings>>);

#[derive(Clone, Debug, Default, Serialize, Deserialize, Eq, PartialEq, Hash)]
#[serde(deny_unknown_fields)]
pub struct AdvancedNetworkSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv4_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6_address: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub aliases: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum SysCtls {
    List(Vec<String>),
    #[cfg(feature = "indexmap")]
    Map(IndexMap<String, Option<SingleValue>>),
    #[cfg(not(feature = "indexmap"))]
    Map(HashMap<String, Option<SingleValue>>),
}

impl Default for SysCtls {
    fn default() -> Self {
        Self::List(Vec::new())
    }
}

impl SysCtls {
    pub fn is_empty(&self) -> bool {
        match self {
            Self::List(v) => v.is_empty(),
            Self::Map(m) => m.is_empty(),
        }
    }
}

#[cfg(feature = "indexmap")]
#[derive(Clone, Default, Debug, Serialize, Deserialize, PartialEq)]
pub struct TopLevelVolumes(pub IndexMap<String, MapOrEmpty<ComposeVolume>>);
#[cfg(not(feature = "indexmap"))]
#[derive(Clone, Default, Debug, Serialize, Deserialize, PartialEq)]
pub struct TopLevelVolumes(pub HashMap<String, MapOrEmpty<ComposeVolume>>);

impl TopLevelVolumes {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ComposeVolume {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver: Option<String>,
    #[cfg(feature = "indexmap")]
    #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
    pub driver_opts: IndexMap<String, Option<SingleValue>>,
    #[cfg(not(feature = "indexmap"))]
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub driver_opts: HashMap<String, Option<SingleValue>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external: Option<ExternalVolume>,
    #[serde(default, skip_serializing_if = "Labels::is_empty")]
    pub labels: Labels,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ExternalVolume {
    Bool(bool),
    Name { name: String },
}

#[cfg(feature = "indexmap")]
#[derive(Clone, Default, Debug, Serialize, Deserialize, PartialEq)]
pub struct ComposeNetworks(pub IndexMap<String, MapOrEmpty<NetworkSettings>>);

#[cfg(not(feature = "indexmap"))]
#[derive(Clone, Default, Debug, Serialize, Deserialize, PartialEq)]
pub struct ComposeNetworks(pub HashMap<String, MapOrEmpty<NetworkSettings>>);

impl ComposeNetworks {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
#[serde(untagged)]
pub enum ComposeNetwork {
    Detailed(ComposeNetworkSettingDetails),
    Bool(bool),
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
#[serde(deny_unknown_fields)]
pub struct ComposeNetworkSettingDetails {
    pub name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
#[serde(deny_unknown_fields)]
pub struct ExternalNetworkSettingBool(bool);

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
#[serde(deny_unknown_fields)]
pub struct NetworkSettings {
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub attachable: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver: Option<String>,
    #[cfg(feature = "indexmap")]
    #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
    pub driver_opts: IndexMap<String, Option<SingleValue>>,
    #[cfg(not(feature = "indexmap"))]
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub driver_opts: HashMap<String, Option<SingleValue>>,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub enable_ipv6: bool,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub internal: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external: Option<ComposeNetwork>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipam: Option<Ipam>,
    #[serde(default, skip_serializing_if = "Labels::is_empty")]
    pub labels: Labels,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
#[serde(deny_unknown_fields)]
pub struct Ipam {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub config: Vec<IpamConfig>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
#[serde(deny_unknown_fields)]
pub struct IpamConfig {
    pub subnet: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
#[serde(deny_unknown_fields)]
pub struct Deploy {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i64>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub labels: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_config: Option<UpdateConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<Resources>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restart_policy: Option<RestartPolicy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement: Option<Placement>,
}

fn is_zero(val: &i64) -> bool {
    *val == 0
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
#[serde(deny_unknown_fields)]
pub struct Healthcheck {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test: Option<HealthcheckTest>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,
    #[serde(default, skip_serializing_if = "is_zero")]
    pub retries: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_period: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_interval: Option<String>,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub disable: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
#[serde(untagged)]
pub enum HealthcheckTest {
    Single(String),
    Multiple(Vec<String>),
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Default)]
#[serde(deny_unknown_fields)]
pub struct Limits {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpus: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<Device>>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Default)]
#[serde(deny_unknown_fields)]
pub struct Device {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg(feature = "indexmap")]
    pub options: Option<IndexMap<String, Value>>,
    #[cfg(not(feature = "indexmap"))]
    pub options: Option<HashMap<String, Value>>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Hash, Default)]
#[serde(deny_unknown_fields)]
pub struct Placement {
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub constraints: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub preferences: Vec<Preferences>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
#[serde(deny_unknown_fields)]
pub struct Preferences {
    pub spread: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Default)]
#[serde(deny_unknown_fields)]
pub struct Resources {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limits: Option<Limits>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reservations: Option<Limits>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Hash, Default)]
#[serde(deny_unknown_fields)]
pub struct RestartPolicy {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delay: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_attempts: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
#[serde(deny_unknown_fields)]
pub struct UpdateConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallelism: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delay: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_action: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_failure_ratio: Option<f64>,
}

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

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum PullPolicy {
    Always,
    Never,
    #[serde(alias = "if_not_present")]
    Missing,
    Build,
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
#[serde(untagged)]
pub enum Volumes {
    Simple(String),
    Advanced(AdvancedVolumes),
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
#[serde(deny_unknown_fields)]
pub struct AdvancedVolumes {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    pub target: String,
    #[serde(rename = "type")]
    pub _type: String,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub read_only: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bind: Option<Bind>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub volume: Option<Volume>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tmpfs: Option<TmpfsSettings>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Hash, Default)]
#[serde(deny_unknown_fields)]
pub struct Bind {
    pub propagation: Option<String>,
    pub create_host_path: Option<bool>,
    pub selinux: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Hash, Default)]
#[serde(deny_unknown_fields)]
pub struct Volume {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nocopy: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subpath: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Hash, Default)]
#[serde(deny_unknown_fields)]
pub struct TmpfsSettings {
    pub size: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
#[serde(untagged)]
pub enum Command {
    Simple(String),
    Args(Vec<String>),
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
#[serde(untagged)]
pub enum Entrypoint {
    Simple(String),
    List(Vec<String>),
}

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

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
#[serde(untagged)]
pub enum Group {
    Named(String),
    Gid(u32),
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
