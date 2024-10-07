use cosmwasm_std::{
    entry_point, Deps, DepsMut, Env, MessageInfo, Response, StdError,
    StdResult,
};

use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{UserInfo, USER_INFO};

use hmac::{Hmac, Mac};
use sha1::Sha1;
use base32::{Alphabet, decode as base32_decode};

type HmacSha1 = Hmac<Sha1>;


/// Number of seconds in a time step (30 seconds is standard)
const TIME_STEP: u64 = 30;

/// Number of digits in the TOTP code
const CODE_DIGITS: u32 = 6;

#[entry_point]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    Ok(Response::default())
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::Register { secret_key } => try_register(deps, info, secret_key),
        ExecuteMsg::Authenticate { totp_code } => try_authenticate(deps, env, info, totp_code),
    }
}

pub fn try_register(
    deps: DepsMut,
    info: MessageInfo,
    secret_key: String,
) -> StdResult<Response> {
    // Get the caller's canonical address
    let user_addr = deps.api.addr_canonicalize(info.sender.as_str())?;

    // Save the user's secret key securely
    let user_info = UserInfo { secret_key };
    USER_INFO.insert(deps.storage, &user_addr, &user_info)?;

    Ok(Response::new().add_attribute("action", "register"))
}

pub fn try_authenticate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    totp_code: String,
) -> StdResult<Response> {
    // Get the caller's canonical address
    let user_addr = deps.api.addr_canonicalize(info.sender.as_str())?;

    // Load the user's secret key
    let user_info = USER_INFO.get(deps.storage, &user_addr)
        .ok_or_else(|| StdError::generic_err("User not registered"))?;

    // Verify the TOTP code
    let is_valid = verify_totp(&user_info.secret_key, &totp_code, &env)?;

    if is_valid {
        Ok(Response::new()
            .add_attribute("action", "authenticate")
            .add_attribute("status", "success"))
    } else {
        Err(StdError::generic_err("Invalid TOTP code"))
    }
}

/// Verify the TOTP code
fn verify_totp(secret_key: &str, totp_code: &str, env: &Env) -> StdResult<bool> {
    // Decode the Base32 secret key
    let secret_bytes = base32_decode(Alphabet::RFC4648 { padding: false }, secret_key)
        .ok_or_else(|| StdError::generic_err("Invalid secret key encoding"))?;

    // Get the current Unix time from the block timestamp
    let current_time = env.block.time.seconds();

    // Compute the time counter
    let time_counter = current_time / TIME_STEP;

    // Check TOTP codes in a range (to account for time drift)
    // Steps: [previous, current, next] time steps
    let steps = [0u64, 1u64];

    // Check previous time step if possible
    if let Some(prev_counter) = time_counter.checked_sub(1) {
        let code = generate_totp_code(&secret_bytes, prev_counter)?;
        if code == totp_code {
            return Ok(true);
        }
    }

    // Check current and next time steps
    for step in steps.iter() {
        let counter = time_counter + *step;
        let code = generate_totp_code(&secret_bytes, counter)?;
        if code == totp_code {
            return Ok(true);
        }
    }

    Ok(false)
}


/// Generate TOTP code for a given time counter
fn generate_totp_code(secret: &[u8], counter: u64) -> StdResult<String> {
    // Convert counter to byte array (big-endian)
    let counter_bytes = counter.to_be_bytes();

    // Create HMAC-SHA1 instance
    let mut mac = HmacSha1::new_from_slice(secret)
        .map_err(|_| StdError::generic_err("HMAC error"))?;

    // Input the counter bytes
    mac.update(&counter_bytes);

    // Compute the HMAC digest
    let hmac_result = mac.finalize().into_bytes();

    // Perform dynamic truncation
    let offset = (hmac_result[19] & 0x0f) as usize;
    if offset + 4 > hmac_result.len() {
        return Err(StdError::generic_err("Invalid HMAC result length"));
    }
    let code = ((hmac_result[offset] & 0x7f) as u32) << 24
        | (hmac_result[offset + 1] as u32) << 16
        | (hmac_result[offset + 2] as u32) << 8
        | (hmac_result[offset + 3] as u32);

    // Compute the TOTP value
    let totp_value = code % 10u32.pow(CODE_DIGITS);

    // Format the code with leading zeros
    Ok(format!("{:0width$}", totp_value, width = CODE_DIGITS as usize))
}

#[entry_point]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<cosmwasm_std::Binary> {
    Ok(cosmwasm_std::Binary::default())
}
