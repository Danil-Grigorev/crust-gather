use clap::Parser;
use kube::CustomResource;
use serde::{Deserialize, Serialize};

use crate::cli::{Filters, GatherSettings};

/// This provides a config for fleet addon functionality
#[derive(CustomResource, Parser, Clone, Default, Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields)]
#[kube(schema = "disabled")]
#[kube(namespaced)]
#[kube(
    kind = "Gather",
    group = "crust.x-k8s.io",
    version = "v1alpha1"
)]
pub struct GatherSpec {
    #[command(flatten)]
    #[serde(default)]
    filter: Option<Filters>,

    #[command(flatten)]
    #[serde(default)]
    settings: GatherSettings,
}