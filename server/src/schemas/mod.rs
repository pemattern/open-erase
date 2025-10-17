use serde::Serialize;

use crate::error::ServiceResult;

pub mod token;
pub mod user;

fn json<T: Serialize>(val: T) -> ServiceResult<String> {
    Ok(serde_json::to_string_pretty(&val)?)
}
