use serde::{Deserialize, Serialize};
use derive_builder::*;
use indexmap::map::IndexMap;

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ComposeFile {
    V2Plus(Compose),
    V1(IndexMap<String,Service>),
    Single(SingleService),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SingleService {
    service: Service
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Compose {
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub services: Option<Services>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes: Option<TopLevelVolumes>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub networks: Option<ComposeNetworks>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<Service>,
}

impl Compose {
    pub fn new() -> Self {
        Self {
            version: None,
            service: None,
            services: None,
            volumes: None,
            networks: None,
        }
    }
}

#[derive(Builder, Clone, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[builder(setter(into), default)]
pub struct Service {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ports: Option<Vec<String>>,
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
    pub entrypoint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env_file: Option<EnvFile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_grace_period: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profiles: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<String>>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extends: Option<IndexMap<String,String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging: Option<LoggingParameters>,
    #[serde(default, skip_serializing_if = "is_zero")]
    pub scale: i64,
}

impl Service {
    pub fn image(&self) -> &str {
        if let Some(image) = &self.image {
            return image;
        }
        ""
    }

    pub fn network_mode(&self) -> &str {
        if let Some(network_mode) = &self.network_mode {
            return network_mode;
        }
        ""
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EnvFile {
    Simple(String),
    List(Vec<String>)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DependsOnOptions {
    Simple(Vec<String>),
    Conditional(IndexMap<String,DependsCondition>)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DependsCondition {
    pub condition: String,
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LoggingParameters {
    pub driver: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<LoggingParameterOptions>
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LoggingParameterOptions {
    #[serde(rename = "max-size")]
    pub max_size: String
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Environment {
    List(Vec<String>),
    KvPair(IndexMap<String,String>)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Services(pub IndexMap<String, Option<Service>>);

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Labels(pub IndexMap<String, String>);

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Ulimits {
    pub nofile: Nofile,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Nofile {
    pub soft: i64,
    pub hard: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Networks {
    Simple(Vec<String>),
    Advanced(AdvancedNetworks),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum BuildStep {
    Simple(String),
    Advanced(AdvancedBuildStep),
}

#[derive(Builder, Clone, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[builder(setter(into), default)]
pub struct AdvancedBuildStep {
    pub context: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dockerfile: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub args: Option<BuildArgs>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    shm_size: Option<u64>,
}

#[derive( Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum BuildArgs {
    Simple(String),
    List(Vec<String>),
    KvPair(IndexMap<String, String>)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AdvancedNetworks(pub IndexMap<String, Option<AdvancedNetworkSettings>>);

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AdvancedNetworkSettings {
    pub ipv4_address: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ComposeVolumes(pub IndexMap<String, Option<IndexMap<String, String>>>);

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum  TopLevelVolumes {
    CV(ComposeVolumes),
    Labelled(LabelledComposeVolumes)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LabelledComposeVolumes(pub IndexMap<String,VolumeLabels >);

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VolumeLabels {
    labels: IndexMap<String,String>
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ComposeNetworks(pub IndexMap<String, NetworkSettingsOptions>);

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NetworkSettingsOptions {
    External(ExternalNetworkSetting),
    Attachable(AttachableNetwork),
    Other(NetworkSettings),
    Empty(IndexMap<(),()>)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AttachableNetwork {
    pub attachable: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExternalNetworkSetting {
    pub external: ExternalNetworkSettingDetails,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExternalNetworkSettingDetails {
    pub name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NetworkSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver: Option<String>,
    pub ipam: Ipam,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Ipam {
    pub config: Vec<IpamConfig>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IpamConfig {
    pub subnet: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Deploy {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    pub replicas: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_config: Option<UpdateConfig>,
    pub resources: Resources,
    pub restart_policy: RestartPolicy,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement: Option<Placement>,
}

fn is_zero(val: &i64) -> bool {
    if *val == 0 {
        true
    } else {
        false
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
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
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub disable: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum HealthcheckTest {
    Single(String),
    Multiple(Vec<String>)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Limits {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpus: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Placement {
    pub constraints: Vec<String>,
    pub preferences: Vec<Preferences>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Preferences {
    pub spread: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Resources {
    pub limits: Limits,
    pub reservations: Limits,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RestartPolicy {
    pub condition: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delay: Option<String>,
    pub max_attempts: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UpdateConfig {
    pub parallelism: i64,
    pub delay: String,
    pub failure_action: String,
    pub monitor: String,
    pub max_failure_ratio: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Volumes {
    Simple(Vec<String>),
    Advanced(Vec<AdvancedVolumes>),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
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
    pub volume: Option<Volume>,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Volume {
    pub nocopy: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Command {
    Simple(String),
    Args(Vec<String>),
}

#[test]
fn volumes() {
    let v = r#"
volumes:
  - source: /host/path
    target: /container/path
    type: bind
    read_only: true
  - source: foobar
    type: volume
    target: /container/volumepath
  - type: volume
    target: /anonymous
  - type: volume
    source: foobar
    target: /container/volumepath2
    volume:
      nocopy: true
"#;

    #[derive(Deserialize)]
    #[allow(dead_code)]
    struct Container {
        volumes: Volumes
    }
    let _parsed : Container = serde_yaml::from_str(v).unwrap();
}