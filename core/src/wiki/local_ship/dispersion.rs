//! Dispersion model (ShipBuilder formulas).

use facet::Facet;
use serde::{Deserialize, Serialize};

use crate::wiki::components::DispersionStats;

/// One dispersion sample (ellipse radii in meters).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct DispersionPointView {
    pub range_m: f64,
    pub horizontal_m: f64,
    pub vertical_m: f64,
}

/// Resolved dispersion model for a battery, computed in Rust.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct DispersionView {
    pub normal_distribution: bool,
    pub taper_dist_m: f64,
    pub delim_dist_m: f64,
    /// Ellipse at the battery's maximum range.
    pub at_max: DispersionPointView,
    /// Samples at 5 km / 10 km (only when below max range).
    pub samples: Vec<DispersionPointView>,
    /// Horizontal formula, `X` = range in km, result in meters.
    pub formula_horizontal: String,
    /// Vertical-coefficient formula at long range, `X` = range in km.
    pub formula_vertical: String,
    /// Vertical-coefficient formula below the delimiter distance.
    pub formula_vertical_short: String,
}

/// Horizontal dispersion in meters at `range_m` (port of ShipBuilder's
/// `Dispersion.CalculateHorizontalDispersion`, modifier = 1.0 for base stats).
pub(super) fn horizontal_dispersion(disp: &DispersionStats, range_m: f64) -> f64 {
    if range_m <= 0.0 {
        return 0.0;
    }
    let x = range_m / 30.0;
    let effective_taper = disp.taper_dist / 30.0;
    if effective_taper > 0.0 && x <= effective_taper {
        (x * (disp.ideal_radius - disp.min_radius) / disp.ideal_distance
            + disp.min_radius * (x / effective_taper))
            * 30.0
    } else {
        (x * (disp.ideal_radius - disp.min_radius) / disp.ideal_distance + disp.min_radius) * 30.0
    }
}

/// Vertical dispersion in meters at `range_m` (port of ShipBuilder's
/// `Dispersion.CalculateVerticalDispersion`).
pub(super) fn vertical_dispersion(disp: &DispersionStats, max_range_m: f64, horizontal_m: f64, range_m: f64) -> f64 {
    if max_range_m <= 0.0 {
        return 0.0;
    }
    let max_range_bw = max_range_m / 30.0;
    let x = range_m / 30.0;
    let delim_dist = max_range_bw * disp.delim;
    let coeff = if x < delim_dist {
        if delim_dist > f64::EPSILON {
            disp.radius_on_zero + (disp.radius_on_delim - disp.radius_on_zero) * (x / delim_dist)
        } else {
            disp.radius_on_zero
        }
    } else {
        let denom = max_range_bw - delim_dist;
        if denom.abs() > f64::EPSILON {
            disp.radius_on_delim + (disp.radius_on_max - disp.radius_on_delim) * (x - delim_dist) / denom
        } else {
            disp.radius_on_max
        }
    };
    horizontal_m * coeff
}

/// Build the resolved dispersion model (base stats, no modifiers).
pub(super) fn dispersion_view(disp: &DispersionStats, max_range_m: f64) -> DispersionView {
    let at_max_h = horizontal_dispersion(disp, max_range_m);
    let at_max = DispersionPointView {
        range_m: max_range_m,
        horizontal_m: at_max_h,
        vertical_m: vertical_dispersion(disp, max_range_m, at_max_h, max_range_m),
    };
    let mut samples = Vec::new();
    for range in [5_000.0, 10_000.0] {
        if range < max_range_m {
            let h = horizontal_dispersion(disp, range);
            samples.push(DispersionPointView {
                range_m: range,
                horizontal_m: h,
                vertical_m: vertical_dispersion(disp, max_range_m, h, range),
            });
        }
    }

    let max_range_bw = max_range_m / 30.0;
    let delim_dist = max_range_bw * disp.delim;
    let v_radius_coeff = if max_range_bw > 0.0 && (1.0 - disp.delim).abs() > f64::EPSILON {
        (disp.radius_on_max - disp.radius_on_delim) / (max_range_bw * (1.0 - disp.delim))
    } else {
        0.0
    };
    let h_coeff = if disp.ideal_distance > 0.0 {
        (disp.ideal_radius - disp.min_radius) / disp.ideal_distance * 1000.0
    } else {
        0.0
    };
    let formula_horizontal =
        format!("X * {} + {}", fmt_disp(h_coeff), fmt_disp(30.0 * disp.min_radius));
    let formula_vertical = format!(
        "(X * {} + {})",
        fmt_disp((v_radius_coeff / 30.0) * 1000.0),
        fmt_disp((-max_range_bw * disp.delim * v_radius_coeff) + disp.radius_on_delim),
    );
    let formula_vertical_short = if max_range_bw > 0.0 && delim_dist > f64::EPSILON {
        format!(
            "(X * {} + {})",
            fmt_disp(((disp.radius_on_delim - disp.radius_on_zero) / delim_dist / 30.0) * 1000.0),
            fmt_disp(disp.radius_on_zero),
        )
    } else {
        formula_vertical.clone()
    };

    DispersionView {
        normal_distribution: disp.normal_distribution,
        taper_dist_m: disp.taper_dist,
        delim_dist_m: max_range_m * disp.delim,
        at_max,
        samples,
        formula_horizontal,
        formula_vertical,
        formula_vertical_short,
    }
}

/// Format a dispersion-formula coefficient like C# `Math.Round(x, 4)`
/// (trailing zeros trimmed).
fn fmt_disp(value: f64) -> String {
    let s = format!("{value:.4}");
    let trimmed = s.trim_end_matches('0').trim_end_matches('.');
    if trimmed.is_empty() || trimmed == "-0" {
        "0".to_string()
    } else {
        trimmed.to_string()
    }
}
