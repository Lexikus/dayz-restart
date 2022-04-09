use serde::{Deserialize, Serialize};

pub mod restart {
    use super::*;

    pub fn post_endpoint(service_id: &str) -> String {
        format!("https://api.nitrado.net/services/{}/gameservers/restart", service_id)
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Response {
        pub status: String,
        pub message: String,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Parameter {
        pub message: String,
        #[serde(rename = "restart_message ")]
        pub restart_message: String,
    }

}
