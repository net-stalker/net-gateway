use std::fmt::{Debug, Formatter};

use serde::{Deserialize, Serialize};
use toml::to_string;
use net_config::NetConfig;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct BindAddres {
    pub addr: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct AllowedOrigin {
    pub addr: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct QuinClientAddres {
    pub(crate) addr: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct QuinServerAddres {
    pub(crate) addr: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct QuinServerApplication {
    pub(crate) app: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct FusionAuthServerAddres {
    pub(crate) addr: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct FusionAuthApiKey {
    pub(crate) key: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, NetConfig)]
pub struct Config {
    pub bind_addres: BindAddres,
    pub allowed_origin: AllowedOrigin,
    pub(crate) quin_client_addres: QuinClientAddres,
    pub(crate) quin_server_addres: QuinServerAddres,
    pub(crate) quin_server_application: QuinServerApplication,
    pub(crate) fusion_auth_server_addres: FusionAuthServerAddres,
    pub(crate) fusion_auth_api_key: FusionAuthApiKey,
}
