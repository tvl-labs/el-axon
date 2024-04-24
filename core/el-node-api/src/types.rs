use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NodeVersion {
    pub node_name:    String,
    pub spec_version: String,
    pub node_version: String,
}

impl Default for NodeVersion {
    fn default() -> Self {
        Self {
            node_name:    "EigenLayer-AVS-AXON".to_string(),
            spec_version: "v0.1.0".to_string(),
            node_version: "v0.1.0".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct ServiceStatus {
    pub id:          String,
    pub name:        String,
    pub description: String,
    pub status:      String,
}
