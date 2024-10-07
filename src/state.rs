use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use secret_toolkit_storage::{Keymap};
use cosmwasm_std::CanonicalAddr;

/// User information stored securely
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct UserInfo {
    pub secret_key: String, // Base32-encoded secret key
}

/// Storage for user information
pub static USER_INFO: Keymap<CanonicalAddr, UserInfo> = Keymap::new(b"user_info");
