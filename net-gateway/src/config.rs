use std::fmt::{Debug, Formatter};

use serde::{Deserialize, Serialize};
use toml::to_string;
use net_config::NetConfig;

#[allow(unused_imports)]
use std::env;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct BindAddress {
    pub addr: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct AllowedOrigin {
    pub addr: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct QuinClientAddress {
    pub(crate) addr: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct QuinReporter {
    pub host_name: String,
    pub port: String,
    pub addr: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct QuinInserter {
    pub host_name: String,
    pub port: String,
    pub addr: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct QuinServerApplication {
    pub(crate) app: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct FusionAuthServerAddress {
    pub(crate) addr: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct FusionAuthApiKey {
    pub(crate) key: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, NetConfig)]
pub struct Config {
    pub bind_address: BindAddress,
    pub allowed_origin: AllowedOrigin,
    pub(crate) quin_client_address: QuinClientAddress,
    pub quin_inserter: QuinInserter,
    pub quin_reporter: QuinReporter,
    pub(crate) quin_server_application: QuinServerApplication,
    pub(crate) fusion_auth_server_address: FusionAuthServerAddress,
    pub(crate) fusion_auth_api_key: FusionAuthApiKey,
}
