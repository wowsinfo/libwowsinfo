//! Shell trajectory simulation.
//!
//! Port of the `wows-toolkit` reverse-engineering notes (`BALLISTICS.md`):
//! International Standard Atmosphere air density, quadratic drag and RK4
//! integration in the (x, y) plane.

/// Sea-level pressure (Pa).
const P0: f64 = 101_325.0;
/// Temperature lapse rate (K/m).
const L: f64 = 0.0065;
/// Sea-level temperature (K).
const T0: f64 = 288.15;
/// Gravitational acceleration (m/s^2).
const G: f64 = 9.8;
/// Molar mass of air (kg/mol).
const M_AIR: f64 = 0.028_964_4;
/// Ideal gas constant (J/(mol K)).
const R_GAS: f64 = 8.314_47;

/// Shell properties needed by the drag model.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Shell {
    pub mass_kg: f64,
    pub diameter_m: f64,
    pub drag_coefficient: f64,
    pub muzzle_velocity: f64,
}

/// One simulated trajectory sample.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TrajectoryPoint {
    pub x: f64,
    pub y: f64,
    pub speed: f64,
    pub time: f64,
}

/// Air density at altitude `y` (m) using the ISA model — algebraically
/// identical to the game's formula.
#[must_use]
pub fn air_density(y: f64) -> f64 {
    let t = T0 - L * y;
    if t <= 0.0 {
        return 0.0;
    }
    let pressure = P0 * (t / T0).powf(G * M_AIR / (R_GAS * L));
    M_AIR * pressure / (R_GAS * t)
}

/// Per-step acceleration from drag and gravity for the given velocity.
fn acceleration(k: f64, y: f64, vx: f64, vy: f64) -> (f64, f64) {
    let rho = air_density(y);
    let speed = (vx * vx + vy * vy).sqrt();
    (-k * rho * vx * speed, -k * rho * vy * speed - G)
}

/// Drag coefficient `k = (pi/8) * cd * d^2 / mass` from the ballistics notes.
fn drag_coefficient(shell: &Shell) -> f64 {
    std::f64::consts::PI / 8.0 * shell.drag_coefficient * shell.diameter_m.powi(2) / shell.mass_kg
}

/// Simulate a shell fired at `angle_deg` from `height_m` with RK4 integration.
/// Stops when the shell falls back to the start height or `max_time` elapses.
#[must_use]
pub fn simulate_trajectory(
    shell: &Shell,
    angle_deg: f64,
    height_m: f64,
    dt: f64,
    max_time: f64,
) -> Vec<TrajectoryPoint> {
    let k = drag_coefficient(shell);
    let angle = angle_deg.to_radians();
    let mut vx = shell.muzzle_velocity * angle.cos();
    let mut vy = shell.muzzle_velocity * angle.sin();
    let mut x = 0.0;
    let mut y = height_m;
    let mut time = 0.0;
    let mut points = vec![TrajectoryPoint {
        x,
        y,
        speed: shell.muzzle_velocity,
        time,
    }];

    while time < max_time && y >= height_m - 1e-6 {
        let (ax1, ay1) = acceleration(k, y, vx, vy);
        let (ax2, ay2) = acceleration(k, y + dt / 2.0 * ay1, vx + dt / 2.0 * ax1, vy + dt / 2.0 * ay1);
        let (ax3, ay3) = acceleration(k, y + dt / 2.0 * ay2, vx + dt / 2.0 * ax2, vy + dt / 2.0 * ay2);
        let (ax4, ay4) = acceleration(k, y + dt * ay3, vx + dt * ax3, vy + dt * ay3);

        let new_vx = vx + dt / 6.0 * (ax1 + 2.0 * ax2 + 2.0 * ax3 + ax4);
        let new_vy = vy + dt / 6.0 * (ay1 + 2.0 * ay2 + 2.0 * ay3 + ay4);
        x += dt / 2.0 * (vx + new_vx);
        y += dt / 2.0 * (vy + new_vy);
        vx = new_vx;
        vy = new_vy;
        time += dt;

        points.push(TrajectoryPoint {
            x,
            y,
            speed: (vx * vx + vy * vy).sqrt(),
            time,
        });
    }
    points
}

/// Range (horizontal distance) reached by a shot fired at `angle_deg`.
fn range_for_angle(shell: &Shell, angle_deg: f64, dt: f64) -> f64 {
    simulate_trajectory(shell, angle_deg, 0.0, dt, 300.0)
        .last()
        .map(|point| point.x)
        .unwrap_or(0.0)
}

/// Find the launch angle (degrees) that lands at `range` metres using
/// bisection over the monotonic 0..45 degree branch. Returns `None` when the
/// range is unreachable or `range <= 0`.
#[must_use]
pub fn solve_for_range(shell: &Shell, range: f64, dt: f64) -> Option<f64> {
    if range <= 0.0 || dt <= 0.0 {
        return None;
    }
    let mut low = 0.0_f64;
    let mut high = 45.0_f64;
    if range_for_angle(shell, high, dt) < range {
        return None;
    }
    for _ in 0..64 {
        let mid = (low + high) / 2.0;
        if range_for_angle(shell, mid, dt) < range {
            low = mid;
        } else {
            high = mid;
        }
    }
    Some((low + high) / 2.0)
}
