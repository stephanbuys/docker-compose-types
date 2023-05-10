use derive_builder::*;
#[cfg(feature = "indexmap")]
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use serde_yaml::Value;
#[cfg(not(feature = "indexmap"))]
use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt;
use std::str::FromStr;

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
    pub services: Option<Services>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes: Option<TopLevelVolumes>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub networks: Option<ComposeNetworks>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<Service>,
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
pub struct Service {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub privileged: bool,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<Environment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restart: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Labels>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tmpfs: Option<Tmpfs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ulimits: Option<Ulimits>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes: Option<Volumes>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub networks: Option<Networks>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cap_add: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depends_on: Option<DependsOnOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<Command>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entrypoint: Option<Entrypoint>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env_file: Option<EnvFile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_grace_period: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profiles: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_signal: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_dir: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub expose: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub volumes_from: Vec<String>,
    #[cfg(feature = "indexmap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extends: Option<IndexMap<String, String>>,
    #[cfg(not(feature = "indexmap"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extends: Option<HashMap<String, String>>,
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extra_hosts: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tty: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sysctls: Option<SysCtls>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_opt: Option<Vec<String>>,
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

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct DependsCondition {
    pub condition: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct LoggingParameters {
    pub driver: String,
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
    pub published: PublishedPort,
    pub protocol: String,
    pub mode: String,
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
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Services(pub IndexMap<String, Option<Service>>);
#[cfg(not(feature = "indexmap"))]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Services(pub HashMap<String, Option<Service>>);

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(untagged)]
pub enum Labels {
    List(Vec<String>),
    #[cfg(feature = "indexmap")]
    Map(IndexMap<String, String>),
    #[cfg(not(feature = "indexmap"))]
    Map(HashMap<String, String>),
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(untagged)]
pub enum Tmpfs {
    Simple(String),
    List(Vec<String>),
}

#[cfg(feature = "indexmap")]
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct Ulimits(pub IndexMap<String, Ulimit>);
#[cfg(not(feature = "indexmap"))]
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct Ulimits(pub HashMap<String, Ulimit>);

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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cache_from: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<Labels>,
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
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct AdvancedNetworks(pub IndexMap<String, MapOrEmpty<AdvancedNetworkSettings>>);
#[cfg(not(feature = "indexmap"))]
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct AdvancedNetworks(pub HashMap<String, MapOrEmpty<AdvancedNetworkSettings>>);

#[derive(Clone, Debug, Default, Serialize, Deserialize, Eq, PartialEq, Hash)]
#[serde(deny_unknown_fields)]
pub struct AdvancedNetworkSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv4_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Vec<String>>,
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

#[cfg(feature = "indexmap")]
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct ComposeVolumes(pub IndexMap<String, Option<IndexMap<String, String>>>);
#[cfg(not(feature = "indexmap"))]
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct ComposeVolumes(pub HashMap<String, Option<HashMap<String, String>>>);

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(untagged)]
pub enum TopLevelVolumes {
    CV(ComposeVolumes),
    Labelled(LabelledComposeVolumes),
}

#[cfg(feature = "indexmap")]
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct LabelledComposeVolumes(pub IndexMap<String, VolumeLabels>);
#[cfg(not(feature = "indexmap"))]
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct LabelledComposeVolumes(pub HashMap<String, VolumeLabels>);

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct VolumeLabels {
    #[cfg(feature = "indexmap")]
    pub labels: IndexMap<String, String>,
    #[cfg(not(feature = "indexmap"))]
    pub labels: HashMap<String, String>,
}

#[cfg(feature = "indexmap")]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ComposeNetworks(pub IndexMap<String, MapOrEmpty<NetworkSettings>>);

#[cfg(not(feature = "indexmap"))]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ComposeNetworks(pub HashMap<String, MapOrEmpty<NetworkSettings>>);

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver_opts: Option<IndexMap<String, Option<SingleValue>>>,
    #[cfg(not(feature = "indexmap"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver_opts: Option<HashMap<String, Option<SingleValue>>>,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub enable_ipv6: bool,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub internal: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external: Option<ComposeNetwork>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipam: Option<Ipam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Labels>,
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
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
#[serde(deny_unknown_fields)]
pub struct Deploy {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,
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
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub disable: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
#[serde(untagged)]
pub enum HealthcheckTest {
    Single(String),
    Multiple(Vec<String>),
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Hash, Default)]
#[serde(deny_unknown_fields)]
pub struct Limits {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpus: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<String>,
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

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Hash, Default)]
#[serde(deny_unknown_fields)]
pub struct Resources {
    pub limits: Option<Limits>,
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

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
#[serde(untagged)]
pub enum Volumes {
    Simple(Vec<String>),
    Advanced(Vec<AdvancedVolumes>),
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
    pub propagation: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Hash, Default)]
#[serde(deny_unknown_fields)]
pub struct Volume {
    pub nocopy: bool,
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
