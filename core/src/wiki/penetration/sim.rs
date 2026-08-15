//! Drag-based trajectory simulation and penetration (ShipBuilder physics).

use super::model::{
    BallisticShell, PenetrationPoint, ShellParams, BISECT_MAX_ITER, BISECT_TOLERANCE_M, DT, G,
    GM_RL, L, M_AIR, MAX_TIME, P0, R_GAS, T0, TIME_MULTIPLIER, VELOCITY_POWER,
};
use std::f64::consts::PI;


/// Air density at altitude `y` (m) using the ISA model.
pub(super) fn air_density(altitude: f64) -> f64 {
    let t = T0 - L * altitude;
    if t <= 0.0 {
        return 0.0;
    }
    let p = P0 * (t / T0).powf(GM_RL);
    (M_AIR * p) / (R_GAS * t)
}

pub(super) fn acceleration(k: f64, vx: f64, vy: f64, y: f64) -> (f64, f64) {
    let rho = air_density(y);
    let speed = (vx * vx + vy * vy).sqrt();
    let k_rho = k * rho;
    (-k_rho * vx * speed, -G - k_rho * vy * speed)
}

/// Simulate a trajectory with RK4 integration.
/// Returns `(range, vx, vy, time)` at the point the shell returns to `y = 0`.
pub(super) fn simulate_trajectory(params: &ShellParams, v0: f64, angle: f64) -> Option<(f64, f64, f64, f64)> {
    let mut x = 0.0;
    let mut y = 0.0;
    let mut vx = v0 * angle.cos();
    let mut vy = v0 * angle.sin();
    let mut t = 0.0;
    let k = params.k;

    while t < MAX_TIME {
        let (ax1, ay1) = acceleration(k, vx, vy, y);
        let vx2 = vx + ax1 * DT * 0.5;
        let vy2 = vy + ay1 * DT * 0.5;
        let y2 = y + vy * DT * 0.5;
        let (ax2, ay2) = acceleration(k, vx2, vy2, y2);
        let vx3 = vx + ax2 * DT * 0.5;
        let vy3 = vy + ay2 * DT * 0.5;
        let y3 = y + vy2 * DT * 0.5;
        let (ax3, ay3) = acceleration(k, vx3, vy3, y3);
        let vx4 = vx + ax3 * DT;
        let vy4 = vy + ay3 * DT;
        let y4 = y + vy3 * DT;
        let (ax4, ay4) = acceleration(k, vx4, vy4, y4);

        let dx = (vx + 2.0 * vx2 + 2.0 * vx3 + vx4) / 6.0 * DT;
        let dy = (vy + 2.0 * vy2 + 2.0 * vy3 + vy4) / 6.0 * DT;
        let dvx = (ax1 + 2.0 * ax2 + 2.0 * ax3 + ax4) / 6.0 * DT;
        let dvy = (ay1 + 2.0 * ay2 + 2.0 * ay3 + ay4) / 6.0 * DT;

        let new_y = y + dy;

        // Linear interpolation back to the exact ground crossing.
        if new_y < 0.0 && t > DT {
            let frac = y / (y - new_y);
            let final_x = x + dx * frac;
            let final_vx = vx + dvx * frac;
            let final_vy = vy + dvy * frac;
            let final_t = t + DT * frac;
            return Some((final_x, final_vx, final_vy, final_t));
        }

        x += dx;
        y = new_y;
        vx += dvx;
        vy += dvy;
        t += DT;
    }
    None
}

/// Find the maximum horizontal range by scanning launch angles 5..=60 degrees.
pub(super) fn max_range(params: &ShellParams, v0: f64) -> Option<f64> {
    let mut best = 0.0;
    for deg in 5..=60 {
        let angle = (deg as f64).to_radians();
        if let Some((dist, _, _, _)) = simulate_trajectory(params, v0, angle)
            && dist > best
        {
            best = dist;
        }
    }
    (best > 0.0).then_some(best)
}

/// Build an impact result from a simulated landing.
pub(super) fn build_impact(params: &ShellParams, distance: f64, vx: f64, vy: f64, time: f64) -> PenetrationPoint {
    let velocity = (vx * vx + vy * vy).sqrt();
    let horizontal = (vy / vx.max(1e-9)).atan().abs();
    let impact_angle_deg = horizontal.to_degrees();
    let deck = PI / 2.0 - horizontal;
    let raw = params.p_ppc * velocity.powf(VELOCITY_POWER);
    let belt = raw * horizontal.cos();
    let deck_pen = raw * deck.cos();
    PenetrationPoint {
        range_m: distance,
        velocity,
        time_s: time / TIME_MULTIPLIER,
        raw_pen_mm: raw,
        belt_pen_mm: belt,
        deck_pen_mm: deck_pen,
        impact_angle_deg,
    }
}

/// Solve for the low-angle trajectory that lands at `range_m`.
/// Returns `None` when the range is unreachable.
#[must_use]
pub fn solve_for_range(shell: &BallisticShell, range_m: f64) -> Option<PenetrationPoint> {
    let params = shell.params();
    let v0 = shell.muzzle_velocity;
    if range_m <= 0.0 {
        return Some(build_impact(&params, 0.0, v0, 0.0, 0.0));
    }
    let max_r = max_range(&params, v0)?;
    if range_m > max_r {
        return None;
    }

    let mut low = 0.001_f64.to_radians();
    let mut high = 45.0_f64.to_radians();
    let mut best: Option<PenetrationPoint> = None;
    for _ in 0..BISECT_MAX_ITER {
        let mid = (low + high) / 2.0;
        if let Some((dist, vx, vy, t)) = simulate_trajectory(&params, v0, mid) {
            best = Some(build_impact(&params, dist, vx, vy, t));
            let err = dist - range_m;
            if err.abs() < BISECT_TOLERANCE_M {
                return best;
            }
            if err > 0.0 {
                high = mid;
            } else {
                low = mid;
            }
        } else {
            break;
        }
    }
    best
}

/// Sample the penetration curve from the muzzle out to `max_range_m`.
#[must_use]
pub fn penetration_curve(shell: &BallisticShell, max_range_m: f64, steps: usize) -> Vec<PenetrationPoint> {
    let steps = steps.max(2);
    (0..steps)
        .filter_map(|i| {
            let range = max_range_m * (i as f64) / (steps as f64 - 1.0);
            solve_for_range(shell, range)
        })
        .collect()
}

/// Armour thickness (mm) that this calibre overmatches (`calibre / 14.3`).
#[must_use]
pub fn overmatch_mm(calibre_mm: f64) -> f64 {
    calibre_mm / 14.3
}
