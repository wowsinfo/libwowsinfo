//! AP penetration curve view model.

use facet::Facet;
use serde::{Deserialize, Serialize};

use super::shell::ShellView;
use crate::wiki::gamedata::GameData;
use crate::wiki::penetration::{penetration_curve, BallisticShell};

/// One AP penetration curve for the chart.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct PenCurveView {
    pub shell_key: String,
    pub shell_name: String,
    pub points: Vec<crate::wiki::penetration::PenetrationPoint>,
}

pub(super) fn pen_curve(data: &GameData, shell: &ShellView, max_range_m: f64) -> Option<PenCurveView> {
    let ap = data.projectiles.get(&shell.key)?.ap.clone()?;
    let ballistics = BallisticShell {
        mass_kg: ap.weight_kg.max(shell.weight),
        calibre_mm: shell.calibre_mm,
        muzzle_velocity: ap.velocity,
        drag: ap.drag,
        krupp: ap.krupp,
        normalization_deg: 0.0,
    };
    // Dense enough that the app's range slider can interpolate smoothly.
    let points = penetration_curve(&ballistics, max_range_m, 101);
    Some(PenCurveView {
        shell_key: shell.key.clone(),
        shell_name: shell.name.clone(),
        points,
    })
}
