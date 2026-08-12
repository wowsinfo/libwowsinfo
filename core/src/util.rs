//! Port of the platform-independent helpers from `src/core/util/Util.ts`.

use std::{
    sync::atomic::{AtomicU64, Ordering},
    time::{SystemTime, UNIX_EPOCH},
};

use serde::{Serialize, de::DeserializeOwned};

/// `roundTo` in `Util.ts`: round to `digit` decimal places, returning `-1`
/// for `None`, NaN or infinity. Follows JS `toFixed` rounding (half away from
/// zero).
#[must_use]
pub fn round_to(num: Option<f64>, digit: u32) -> f64 {
    let Some(num) = num else { return -1.0 };
    if num.is_nan() || num.is_infinite() {
        return -1.0;
    }
    let factor = 10f64.powi(digit as i32);
    (num * factor).round() / factor
}

/// `dayDifference` in `Util.ts`: `ceil(|now - time| / 86400)`.
#[must_use]
pub fn day_difference(now_secs: i64, time_secs: i64) -> i64 {
    let diff = (now_secs - time_secs).unsigned_abs();
    ((diff + 86_399) / 86_400) as i64
}

/// Deterministic UTC rendering matching the shape of `humanTimeString` in
/// `Util.ts` (`YYYY.MM.DD HH:MM:SS`). The TypeScript version uses local time;
/// the shell can localise this string if desired.
#[must_use]
pub fn human_time_string(time_secs: i64) -> String {
    let (y, m, d, hh, mm, ss) = civil_from_unix(time_secs);
    format!("{y:04}.{m:02}.{d:02} {hh:02}:{mm:02}:{ss:02}")
}

/// `copy` in `Util.ts`: deep clone through JSON.
#[must_use]
pub fn copy<T>(value: &T) -> T
where
    T: Serialize + DeserializeOwned,
{
    serde_json::from_value(serde_json::to_value(value).expect("value must serialize"))
        .expect("copy")
}

static RNG_STATE: AtomicU64 = AtomicU64::new(0);

/// `random` in `Util.ts`: a small deterministic-ish PRNG for UI helpers.
/// Returns a value in `0..range`. `range` of zero returns 0.
#[must_use]
pub fn random(range: usize) -> usize {
    if range == 0 {
        return 0;
    }
    let mut state = RNG_STATE.load(Ordering::Relaxed);
    if state == 0 {
        state = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos() as u64)
            .unwrap_or(0x9E37_79B9_7F4A_7C15);
    }
    // xorshift64*
    state ^= state >> 12;
    state ^= state << 25;
    state ^= state >> 27;
    RNG_STATE.store(state, Ordering::Relaxed);
    ((state.wrapping_mul(0x2545_F491_4F6C_DD1D) >> 32) as usize) % range
}

/// Convert a Unix timestamp (seconds) to a UTC civil date.
/// Algorithm by Howard Hinnant, public domain.
#[must_use]
pub fn civil_from_unix(secs: i64) -> (i64, u32, u32, u32, u32, u32) {
    let days = secs.div_euclid(86_400);
    let rem = secs.rem_euclid(86_400);
    let (hh, mm, ss) = (rem / 3600, (rem % 3600) / 60, rem % 60);

    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    (y, m as u32, d as u32, hh as u32, mm as u32, ss as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_to_matches_javascript() {
        assert_eq!(round_to(Some(1.25), 1), 1.3);
        assert_eq!(round_to(Some(-1.25), 1), -1.3);
        assert_eq!(round_to(Some(1150.4), 0), 1150.0);
        assert_eq!(round_to(Some(f64::NAN), 0), -1.0);
        assert_eq!(round_to(Some(f64::INFINITY), 0), -1.0);
        assert_eq!(round_to(None, 0), -1.0);
        assert_eq!(round_to(Some(0.005), 2), 0.01);
    }

    #[test]
    fn day_difference_rounds_up_like_ceil() {
        assert_eq!(day_difference(1000, 0), 1);
        assert_eq!(day_difference(86_400, 0), 1);
        assert_eq!(day_difference(86_401, 0), 2);
    }

    #[test]
    fn human_time_is_deterministic() {
        assert_eq!(human_time_string(0), "1970.01.01 00:00:00");
        assert_eq!(human_time_string(1_700_000_000), "2023.11.14 22:13:20");
    }

    #[test]
    fn copy_round_trips() {
        let value = serde_json::json!({"a": [1, 2, {"b": null}]});
        assert_eq!(copy(&value), value);
    }

    #[test]
    fn random_is_bounded() {
        for _ in 0..100 {
            assert!(random(10) < 10);
        }
        assert_eq!(random(0), 0);
    }
}
