use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Instantiate message
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct InstantiateMsg {}

/// Execute messages
#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    /// Register a new user with a shared secret key
    Register { secret_key: String },
    /// Authenticate using a TOTP code
    Authenticate { totp_code: String },
}

/// Query messages (not used in this example)
#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {}
