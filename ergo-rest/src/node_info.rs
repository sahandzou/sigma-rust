use std::cmp::Ordering;

use serde::{Deserialize, Serialize};

use crate::NodeResponse;

/// Node extended information from /info REST API endpoint
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[repr(C)]
pub struct NodeInfo {
    /// Node name
    pub name: String,
    /// Ergo node app version
    #[serde(rename = "appVersion")]
    pub app_version: String,
}

impl NodeInfo {
    /// Returns true iff the ergo node is at least v4.0.28. This is important since nipopow proofs
    /// only work correctly from this version onwards.
    pub fn is_at_least_version_4_0_28(&self) -> bool {
        let ord = self.app_version.cmp(&String::from("4.0.28"));
        ord == Ordering::Equal || ord == Ordering::Greater
    }
}

impl NodeResponse for NodeInfo {}
