// Service related structures and enums extracted from lib.rs

use derive_builder::*;
#[cfg(feature = "indexmap")]
use indexmap::IndexMap;
use serde::{Deserialize, Deserializer, Serialize};
#[cfg(not(feature = "indexmap"))]
use std::collections::HashMap;

use serde_yaml::Value;

use crate::{MapOrEmpty, Secrets, SingleValue, Volumes};

/// Represents a service defined in the Compose file, mapping container configuration options.
#[derive(Builder, Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
#[builder(setter(into), default)]
pub struct Service {
    /// The hostname for the container ('hostname' in Compose).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    /// The domain name for the container ('domainname' in Compose).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domainname: Option<String>,
    /// Give extended privileges to this container ('privileged').
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub privileged: bool,
    /// Mount the container's root filesystem as read-only ('read_only').
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub read_only: bool,
    /// Healthcheck configuration for the service ('healthcheck').
    #[serde(skip_serializing_if = "Option::is_none")]
    pub healthcheck: Option<Healthcheck>,
    /// Deployment configuration options ('deploy').
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deploy: Option<Deploy>,
    /// Image to use for the service ('image').
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// Custom container name ('container_name').
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_name: Option<String>,
    /// Build configuration for the service ('build').
    #[serde(skip_serializing_if = "Option::is_none", rename = "build")]
    pub build_: Option<BuildStep>,
    /// PID namespace to use for the container ('pid').
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pid: Option<String>,
    /// Port mappings for the service ('ports').
    #[serde(default, skip_serializing_if = "Ports::is_empty")]
    pub ports: Ports,
    /// Environment variables for the container ('environment').
    #[serde(default, skip_serializing_if = "Environment::is_empty")]
    pub environment: Environment,
    /// Network mode for the service ('network_mode').
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_mode: Option<String>,
    /// Devices to expose to the container ('devices').
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub devices: Vec<String>,
    /// Restart policy for the service ('restart').
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restart: Option<String>,
    /// Labels for the service ('labels').
    #[serde(default, skip_serializing_if = "Labels::is_empty")]
    pub labels: Labels,
    /// Mounts a tmpfs mount into the container ('tmpfs').
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tmpfs: Option<Tmpfs>,
    /// Resource limit configurations ('ulimits').
    #[serde(default, skip_serializing_if = "Ulimits::is_empty")]
    pub ulimits: Ulimits,
    /// Volume configurations for the service ('volumes').
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub volumes: Vec<Volumes>,
    /// Network configurations for the service ('networks').
    #[serde(default, skip_serializing_if = "Networks::is_empty")]
    pub networks: Networks,
    /// Additional capabilities to add to the container ('cap_add').
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub cap_add: Vec<String>,
    /// Capabilities to drop from the container ('cap_drop').
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub cap_drop: Vec<String>,
    /// Dependencies for the service ('depends_on').
    #[serde(default, skip_serializing_if = "DependsOnOptions::is_empty")]
    pub depends_on: DependsOnOptions,
    /// Command to run in the container ('command').
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<Command>,
    /// Entrypoint for the container ('entrypoint').
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entrypoint: Option<Entrypoint>,
    /// Environment file to load ('env_file').
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env_file: Option<EnvFile>,
    /// Grace period for stopping the container ('stop_grace_period').
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_grace_period: Option<String>,
    /// Profiles the service is part of ('profiles').
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub profiles: Vec<String>,
    /// Linked services ('links').
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<String>,
    /// DNS servers for the container ('dns').
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dns: Vec<String>,
    /// IPC namespace to use for the container ('ipc').
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipc: Option<String>,
    /// Network to use for the container ('net').
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net: Option<String>,
    /// Signal to stop the container ('stop_signal').
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_signal: Option<String>,
    /// User to run the container as ('user').
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    /// User namespace to use ('userns_mode').
    #[serde(skip_serializing_if = "Option::is_none")]
    pub userns_mode: Option<String>,
    /// Working directory inside the container ('working_dir').
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_dir: Option<String>,
    /// Ports to expose from the container ('expose').
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub expose: Vec<String>,
    /// Volumes to inherit from the container ('volumes_from').
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
    /// Logging configuration for the service ('logging').
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging: Option<LoggingParameters>,
    /// Number of replicas for the service ('scale').
    #[serde(default, skip_serializing_if = "is_zero")]
    pub scale: i64,
    /// Enable init system inside the container ('init').
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub init: bool,
    /// Keep STDIN open even if not attached ('stdin_open').
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub stdin_open: bool,
    /// Size of /dev/shm ('shm_size').
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shm_size: Option<String>,
    #[cfg(feature = "indexmap")]
    #[serde(flatten, skip_serializing_if = "IndexMap::is_empty")]
    pub extensions: IndexMap<crate::Extension, Value>,
    #[cfg(not(feature = "indexmap"))]
    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    pub extensions: HashMap<crate::Extension, Value>,
    /// Additional hosts to add to the container's /etc/hosts ('extra_hosts').
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extra_hosts: Vec<String>,
    /// Groups to add the user to ('group_add').
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub group_add: Vec<Group>,
    /// Allocate a pseudo-TTY ('tty').
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub tty: bool,
    /// Sysctl options to set in the container ('sysctls').
    #[serde(default, skip_serializing_if = "SysCtls::is_empty")]
    pub sysctls: SysCtls,
    /// Security options to apply to the container ('security_opt').
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub security_opt: Vec<String>,
    /// Secrets to expose to the service ('secrets').
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets: Option<Secrets>,
    /// Image pull policy ('pull_policy').
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pull_policy: Option<PullPolicy>,
    /// Parent cgroup for the container ('cgroup_parent').
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cgroup_parent: Option<String>,
    /// Memory limit for the container ('mem_limit').
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mem_limit: Option<String>,
    /// Memory reservation for the container ('mem_reservation').
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mem_reservation: Option<String>,
    /// Memory swappiness for the container ('mem_swappiness').
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mem_swappiness: Option<u16>,
    /// Runtime to use for the container ('runtime').
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

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
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

/// Represents a build configuration for a service.
#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(untagged)]
pub enum BuildStep {
    /// Simple build configuration with just a context path.
    Simple(String),
    /// Advanced build configuration with detailed settings.
    Advanced(Box<AdvancedBuildStep>),
}

/// Advanced build configuration with detailed settings.
#[derive(Builder, Clone, Debug, Deserialize, Serialize, Eq, PartialEq, Default)]
#[serde(deny_unknown_fields)]
#[builder(setter(into), default)]
pub struct AdvancedBuildStep {
    /// Build context path.
    pub context: String,
    /// Path to the Dockerfile relative to the build context.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dockerfile: Option<String>,
    /// Build-time variables to pass to the build process.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub args: Option<BuildArgs>,
    /// Size of /dev/shm in bytes for the build container.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shm_size: Option<u64>,
    /// Target build stage to build.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    /// Network mode for the build container.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,
    /// Images to consider as cache sources.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub cache_from: Vec<String>,
    /// Metadata labels to apply to the built image.
    #[serde(default, skip_serializing_if = "Labels::is_empty")]
    pub labels: Labels,
}

/// Build arguments to pass to the build process.
#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(untagged)]
pub enum BuildArgs {
    /// Simple string build argument.
    Simple(String),
    /// List of build arguments.
    List(Vec<String>),
    /// Key-value pairs of build arguments.
    #[cfg(feature = "indexmap")]
    KvPair(IndexMap<String, String>),
    /// Key-value pairs of build arguments.
    #[cfg(not(feature = "indexmap"))]
    KvPair(HashMap<String, String>),
}

/// Advanced network configurations for a service.
#[cfg(feature = "indexmap")]
#[derive(Clone, Default, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct AdvancedNetworks(pub IndexMap<String, MapOrEmpty<AdvancedNetworkSettings>>);
/// Advanced network configurations for a service.
#[cfg(not(feature = "indexmap"))]
#[derive(Clone, Default, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct AdvancedNetworks(pub HashMap<String, MapOrEmpty<AdvancedNetworkSettings>>);

/// Detailed network settings for a service.
#[derive(Clone, Debug, Default, Serialize, Deserialize, Eq, PartialEq, Hash)]
#[serde(deny_unknown_fields)]
pub struct AdvancedNetworkSettings {
    /// IPv4 address for the container on this network.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv4_address: Option<String>,
    /// IPv6 address for the container on this network.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6_address: Option<String>,
    /// Network aliases for the container on this network.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub aliases: Vec<String>,
}

/// Sysctl options to set in the container.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum SysCtls {
    /// List of sysctl options as strings.
    List(Vec<String>),
    /// Map of sysctl option names to values.
    #[cfg(feature = "indexmap")]
    Map(IndexMap<String, Option<SingleValue>>),
    /// Map of sysctl option names to values.
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

/// Deployment configuration options for a service.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
#[serde(deny_unknown_fields)]
pub struct Deploy {
    /// Deployment mode (replicated or global).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    /// Number of container instances for the service.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i64>,
    /// Metadata labels for the deployed service.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub labels: Vec<String>,
    /// Configuration for how the service should be updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_config: Option<UpdateConfig>,
    /// Resource constraints for the service.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<Resources>,
    /// Restart policy for the service.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restart_policy: Option<RestartPolicy>,
    /// Placement constraints and preferences for the service.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement: Option<Placement>,
}

fn is_zero(val: &i64) -> bool {
    *val == 0
}

/// Healthcheck configuration for a service.
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
#[serde(deny_unknown_fields)]
pub struct Healthcheck {
    /// The test to perform to check container health.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test: Option<HealthcheckTest>,
    /// Time between running the check.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    /// Maximum time to wait for a check to complete.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,
    /// Number of consecutive failures needed to report unhealthy.
    #[serde(default, skip_serializing_if = "is_zero")]
    pub retries: i64,
    /// Start period for the container to initialize before counting retries.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_period: Option<String>,
    /// Time between running the check during the start period.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_interval: Option<String>,
    /// Disable the healthcheck.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub disable: bool,
}

/// Test to perform to check container health.
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
#[serde(untagged)]
pub enum HealthcheckTest {
    /// Single string command.
    Single(String),
    /// List of strings (command and its arguments).
    Multiple(Vec<String>),
}

/// Resource limits for a service.
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Default)]
#[serde(deny_unknown_fields)]
pub struct Limits {
    /// CPU limit for the service.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpus: Option<String>,
    /// Memory limit for the service.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<String>,
    /// Device limits for the service.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<Device>>,
}

/// Device configuration for a service.
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Default)]
#[serde(deny_unknown_fields)]
pub struct Device {
    /// Device driver to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver: Option<String>,
    /// Number of devices to allocate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u32>,
    /// List of device IDs to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_ids: Option<Vec<String>>,
    /// Device capabilities to enable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<Vec<String>>,
    /// Driver-specific options.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg(feature = "indexmap")]
    pub options: Option<IndexMap<String, Value>>,
    /// Driver-specific options.
    #[cfg(not(feature = "indexmap"))]
    pub options: Option<HashMap<String, Value>>,
}

/// Placement constraints and preferences for a service.
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Hash, Default)]
#[serde(deny_unknown_fields)]
pub struct Placement {
    /// Placement constraints for the service.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub constraints: Vec<String>,
    /// Placement preferences for the service.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub preferences: Vec<Preferences>,
}

/// Placement preference for a service.
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
#[serde(deny_unknown_fields)]
pub struct Preferences {
    /// Spread tasks across the given value.
    pub spread: String,
}

/// Resource constraints for a service.
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Default)]
#[serde(deny_unknown_fields)]
pub struct Resources {
    /// Hard resource limits for the service.
    pub limits: Option<Limits>,
    /// Resource reservations for the service.
    pub reservations: Option<Limits>,
}

/// Restart policy for a service.
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Hash, Default)]
#[serde(deny_unknown_fields)]
pub struct RestartPolicy {
    /// Condition for restarting the service (none, on-failure, any).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    /// Delay between restart attempts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delay: Option<String>,
    /// Maximum number of restart attempts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_attempts: Option<i64>,
    /// Time window to evaluate restart attempts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window: Option<String>,
}

/// Configuration for how a service should be updated.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
#[serde(deny_unknown_fields)]
pub struct UpdateConfig {
    /// Number of containers to update at a time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallelism: Option<i64>,
    /// Delay between updating groups of containers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delay: Option<String>,
    /// Action to take if an update fails (pause, continue, rollback).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_action: Option<String>,
    /// Duration to monitor updated tasks for failures.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitor: Option<String>,
    /// Failure rate to tolerate during an update.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_failure_ratio: Option<f64>,
}

/// Image pull policy for a service.
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum PullPolicy {
    /// Always pull the image.
    Always,
    /// Never pull the image.
    Never,
    /// Pull the image if it doesn't exist locally.
    #[serde(alias = "if_not_present")]
    Missing,
    /// Build the image from source.
    Build,
}

/// Command to run in the container.
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
#[serde(untagged)]
pub enum Command {
    /// Simple string command.
    Simple(String),
    /// Command with arguments as a list.
    Args(Vec<String>),
}

/// Entrypoint for the container.
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
#[serde(untagged)]
pub enum Entrypoint {
    /// Simple string entrypoint.
    Simple(String),
    /// Entrypoint as a list of strings.
    List(Vec<String>),
}

/// Group to add the user to.
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
#[serde(untagged)]
pub enum Group {
    /// Group name.
    Named(String),
    /// Group ID.
    Gid(u32),
}
