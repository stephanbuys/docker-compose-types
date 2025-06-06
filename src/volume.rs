// Volume related structures extracted from lib.rs

use serde::{Deserialize, Serialize};
#[cfg(feature = "indexmap")]
use indexmap::IndexMap;
#[cfg(not(feature = "indexmap"))]
use std::collections::HashMap;

use crate::{Labels, SingleValue, MapOrEmpty};

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

