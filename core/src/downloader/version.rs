//! Version helpers.

use crate::models::ApiResponse;

pub fn check_version_update(previous: &str, current: &str) -> bool {
    previous != current
}

/// `ApiResponse` helper: true when `status == "ok"` (SafeFetch's contract).
#[must_use]
pub fn is_ok<T>(response: &ApiResponse<T>) -> bool {
    response.status == "ok"
}
