//! Ballistic model types and constants.

use std::f64::consts::PI;

use facet::Facet;
use serde::{Deserialize, Serialize};

pub(super) const G: f64 = 9.8;
pub(super) const T0: f64 = 288.15;
pub(super) const L: f64 = 0.0065;
pub(super) const P0: f64 = 101_325.0;
pub(super) const R_GAS: f64 = 8.314_47;
pub(super) const M_AIR: f64 = 0.028_964_4;
pub(super) const GM_RL: f64 = (G * M_AIR) / (R_GAS * L);

/// Game-specific constants from the reverse-engineering notes.
pub(super) const TIME_MULTIPLIER: f64 = 2.75;
pub(super) const VELOCITY_POWER: f64 = 1.38;
pub(super) const DT: f64 = 0.02;
pub(super) const MAX_TIME: f64 = 200.0;
pub(super) const BISECT_TOLERANCE_M: f64 = 1.0;
pub(super) const BISECT_MAX_ITER: u32 = 60;

/// Shell parameters for the ballistic simulation and penetration model.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BallisticShell {
    /// Projectile mass in kilograms.
    pub mass_kg: f64,
    /// Calibre in millimetres.
    pub calibre_mm: f64,
    /// Muzzle velocity in metres per second.
    pub muzzle_velocity: f64,
    /// Air drag coefficient from the game data.
    pub drag: f64,
    /// Krupp hardness value (AP only).
    pub krupp: f64,
    /// Normalisation angle in degrees (0 when the data does not provide it).
    pub normalization_deg: f64,
}

impl BallisticShell {
    /// Precompute the drag and penetration coefficients.
    pub(super) fn params(&self) -> ShellParams {
        let calibre_m = self.calibre_mm / 1000.0;
        let r = calibre_m / 2.0;
        ShellParams {
            k: 0.5 * self.drag * r * r * PI / self.mass_kg,
            p_ppc: 1e-7
                * self.krupp
                * self.mass_kg.powf(0.69)
                * calibre_m.powf(-1.07),
        }
    }
}

pub(super) struct ShellParams {
    pub(super) k: f64,
    pub(super) p_ppc: f64,
}

/// One sample of the penetration-over-range chart.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Facet)]
pub struct PenetrationPoint {
    /// Horizontal range in metres.
    pub range_m: f64,
    /// Impact velocity magnitude in metres per second.
    pub velocity: f64,
    /// Time of flight in game seconds.
    pub time_s: f64,
    /// Raw penetration in millimetres.
    pub raw_pen_mm: f64,
    /// Effective penetration against a vertical (belt) plate.
    pub belt_pen_mm: f64,
    /// Effective penetration against a horizontal (deck) plate.
    pub deck_pen_mm: f64,
    /// Impact angle below the horizontal in degrees.
    pub impact_angle_deg: f64,
}
