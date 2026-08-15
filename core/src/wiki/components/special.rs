//! Special ability (F / rage mode) component.

use facet::Facet;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::wiki::modifiers::{parse_modifiers, ModifierSet};
use super::helpers::{as_f64, bool_field};

/// Special ability (F / rage mode) component, v15.7 `specialAbility.rage`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct SpecialStats {
    /// Raw mode key (e.g. `main_gun_accuracy`, `survivability`).
    pub mode: String,
    pub boost_duration: f64,
    pub boost_preparation: f64,
    /// Progress gained per triggering action (`progressPerAction`).
    pub progress_per_action: f64,
    pub progress_name: String,
    /// Required trigger count (`requiredCount`).
    pub required_count: i64,
    pub sub_ribbons: Vec<i64>,
    pub time_limit: f64,
    pub separate_tracking: bool,
    pub start_enabled: bool,
    pub decrement_delay: f64,
    pub decrement_period: f64,
    /// Progress lost per interval after the inactivity delay.
    pub decrement_count: f64,
    pub auto_usage: bool,
    pub modifiers: ModifierSet,
}

pub(crate) fn parse_special(json: &Value) -> Option<SpecialStats> {
    let rage = json
        .get("specialAbility")
        .and_then(|sa| sa.get("rage"))
        .filter(|v| v.is_object())
        .or_else(|| json.get("rageMode").filter(|v| v.is_object()))?;
    Some(SpecialStats {
        mode: rage
            .get("name")
            .and_then(Value::as_str)
            .or_else(|| rage.get("rageModeName").and_then(Value::as_str))
            .unwrap_or("")
            .to_string(),
        boost_duration: as_f64(rage, "duration").max(as_f64(rage, "boostDuration")),
        boost_preparation: as_f64(rage, "preparation").max(as_f64(rage, "boostPreparation")),
        progress_per_action: as_f64(rage, "progressPerAction"),
        progress_name: rage
            .get("progressName")
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_string(),
        required_count: as_f64(rage, "requiredCount").round() as i64,
        sub_ribbons: rage
            .get("subRibbons")
            .and_then(Value::as_array)
            .map(|arr| arr.iter().filter_map(Value::as_i64).collect())
            .unwrap_or_default(),
        time_limit: as_f64(rage, "timeLimit"),
        separate_tracking: bool_field(rage, "separateTracking"),
        start_enabled: bool_field(rage, "startEnabled"),
        decrement_delay: as_f64(rage, "inactivityDelay").max(as_f64(rage, "decrementDelay")),
        decrement_period: as_f64(rage, "progressLossInterval").max(as_f64(rage, "decrementPeriod")),
        decrement_count: as_f64(rage, "progressLossPerInterval").max(as_f64(rage, "decrementCount")),
        auto_usage: bool_field(rage, "autoUsage") || bool_field(rage, "isAutoUsage"),
        modifiers: rage
            .get("modifiers")
            .map(parse_modifiers)
            .unwrap_or_default(),
    })
}
