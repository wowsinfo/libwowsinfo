//! Special ability (F / rage mode) view model.

use facet::Facet;
use serde::{Deserialize, Serialize};

use crate::wiki::components::SpecialStats;
use crate::wiki::loadouts::modifier_lines;
use crate::wiki::LangMap;

/// Resolved special ability (F / rage mode) view.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct SpecialAbilityView {
    /// Humanised mode name (e.g. "Main Gun Accuracy").
    pub name: String,
    /// Raw mode key (e.g. `main_gun_accuracy`).
    pub mode: String,
    pub duration_s: f64,
    pub preparation_s: f64,
    pub progress_per_action: f64,
    pub progress_name: String,
    pub required_count: i64,
    pub sub_ribbons: Vec<i64>,
    pub time_limit_s: f64,
    pub separate_tracking: bool,
    pub start_enabled: bool,
    pub inactivity_delay_s: f64,
    pub progress_loss_interval_s: f64,
    pub progress_loss_per_interval: f64,
    pub auto_usage: bool,
    /// Friendly modifier lines ("Secondary reload -25%", ...).
    pub modifiers: Vec<String>,
}

pub(super) fn special_ability_view(lang: &LangMap, stats: &SpecialStats) -> SpecialAbilityView {
    SpecialAbilityView {
        name: humanize_mode(&stats.mode),
        mode: stats.mode.clone(),
        duration_s: stats.boost_duration,
        preparation_s: stats.boost_preparation,
        progress_per_action: stats.progress_per_action,
        progress_name: stats.progress_name.clone(),
        required_count: stats.required_count,
        sub_ribbons: stats.sub_ribbons.clone(),
        time_limit_s: stats.time_limit,
        separate_tracking: stats.separate_tracking,
        start_enabled: stats.start_enabled,
        inactivity_delay_s: stats.decrement_delay,
        progress_loss_interval_s: stats.decrement_period,
        progress_loss_per_interval: stats.decrement_count,
        auto_usage: stats.auto_usage,
        modifiers: modifier_lines(lang, &stats.modifiers),
    }
}

/// Humanise a raw rage-mode key (`main_gun_accuracy` -> "Main Gun Accuracy").
fn humanize_mode(mode: &str) -> String {
    if mode.is_empty() {
        return "Special Ability".to_string();
    }
    mode.split('_')
        .filter(|word| !word.is_empty() && *word != "te")
        .map(|word| match word {
            "atba" => "Secondary".to_string(),
            _ => {
                let mut chars = word.chars();
                match chars.next() {
                    Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                    None => String::new(),
                }
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

