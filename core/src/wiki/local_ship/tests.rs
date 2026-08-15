//! Local ship view model tests.

#[cfg(test)]
mod tests {
    use crate::wiki::components::{BatteryStats, DispersionStats, GunStats, WeaponInfo};
    use super::super::dispersion::{horizontal_dispersion, vertical_dispersion};

    fn kawachi_dispersion() -> DispersionStats {
        DispersionStats {
            normal_distribution: true,
            taper_dist: 3000.0,
            delim: 0.5,
            ellipse_range_min: 50.0,
            ellipse_range_max: 250.0,
            radius_on_zero: 0.2,
            radius_on_delim: 0.6,
            radius_on_max: 0.8,
            ideal_distance: 1000.0,
            ideal_radius: 10.0,
            min_radius: 2.8,
            ..DispersionStats::default()
        }
    }

    #[test]
    fn dispersion_matches_shipbuilder_at_max_and_samples() {
        let disp = kawachi_dispersion();
        let max = 9880.0;
        let h = horizontal_dispersion(&disp, max);
        let v = vertical_dispersion(&disp, max, h, max);
        // ShipBuilder ground truth: 155.136 m horizontal, 124.1088 m vertical.
        assert!((h - 155.136).abs() < 1e-3, "horizontal at max: {h}");
        assert!((v - 124.1088).abs() < 1e-3, "vertical at max: {v}");

        // Short-range sample at 5 km (still beyond the 3 km taper distance).
        let h5 = horizontal_dispersion(&disp, 5000.0);
        let v5 = vertical_dispersion(&disp, max, h5, 5000.0);
        assert!((h5 - 120.0).abs() < 1e-6, "horizontal at 5 km: {h5}");
        assert!((v5 - 72.2915).abs() < 1e-3, "vertical at 5 km: {v5}");
    }

    #[test]
    fn dispersion_taper_branch_scales_radius() {
        // Within the taper distance the formula adds MinRadius * (x / taper).
        let disp = kawachi_dispersion();
        let h = horizontal_dispersion(&disp, 1500.0);
        let x = 1500.0 / 30.0;
        let expected = (x * (disp.ideal_radius - disp.min_radius) / disp.ideal_distance
            + disp.min_radius * (x / (disp.taper_dist / 30.0)))
            * 30.0;
        assert!((h - expected).abs() < 1e-9);
    }

    #[test]
    fn rotation_deg_s_prefers_battery_traverse() {
        let guns = GunStats {
            battery: Some(BatteryStats {
                traverse: vec![5.0, 5.0],
                ..BatteryStats::default()
            }),
            guns: vec![WeaponInfo {
                rotation: 36.0,
                ..WeaponInfo::default()
            }],
            ..GunStats::default()
        };
        assert_eq!(guns.rotation_deg_s(), 5.0);

        // Legacy fallback: legacy `rotation` is the 180° turn time.
        let legacy = GunStats {
            guns: vec![WeaponInfo {
                rotation: 36.0,
                ..WeaponInfo::default()
            }],
            ..GunStats::default()
        };
        assert!((legacy.rotation_deg_s() - 5.0).abs() < 1e-9);
    }
}
