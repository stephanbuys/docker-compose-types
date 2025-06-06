// Service related structures and enums extracted from lib.rs

use derive_builder::*;
#[cfg(feature = "indexmap")]
use indexmap::IndexMap;
use serde::{Deserialize, Deserializer, Serialize};
#[cfg(not(feature = "indexmap"))]
use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt;
use std::str::FromStr;

use serde_yaml::Value;

use crate::{Labels, SingleValue, Volumes, MapOrEmpty, Secrets};

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
    pub extensions: IndexMap<crate::Extension, Value>,
    #[cfg(not(feature = "indexmap"))]
    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    pub extensions: HashMap<crate::Extension, Value>,
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

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub enum Group {
    Named(String),
    Gid(u32),
}

