//! Ballistics engine tests.

#[cfg(test)]
mod tests {
    use super::super::model::BallisticShell;
    use super::super::sim::{overmatch_mm, penetration_curve, solve_for_range};

    fn montana_ap() -> BallisticShell {
        BallisticShell {
            mass_kg: 1225.0,
            calibre_mm: 406.0,
            muzzle_velocity: 701.0,
            drag: 0.352,
            krupp: 2598.0,
            normalization_deg: 0.0,
        }
    }

    #[test]
    fn muzzle_penetration_matches_wows_shell_model() {
        let shell = montana_ap();
        let point = solve_for_range(&shell, 0.0).expect("muzzle");
        // ~778 mm raw for Montana's Mk 8 at the muzzle.
        assert!(
            (740.0..840.0).contains(&point.raw_pen_mm),
            "raw pen {}",
            point.raw_pen_mm
        );
        assert_eq!(point.velocity, 701.0);
        assert!(point.belt_pen_mm <= point.raw_pen_mm);
        assert!(point.impact_angle_deg < 5.0, "flat at the muzzle");
    }

    #[test]
    fn penetration_decreases_with_range() {
        let shell = montana_ap();
        let curve = penetration_curve(&shell, 20_000.0, 6);
        assert!(curve.len() >= 5, "curve len {}", curve.len());
        assert!(curve[0].raw_pen_mm > curve.last().unwrap().raw_pen_mm);
        assert!(curve.last().unwrap().velocity < curve[0].velocity);
        assert!(
            curve.last().unwrap().impact_angle_deg > curve[0].impact_angle_deg,
            "steeper impact at long range"
        );
    }

    #[test]
    fn unreachable_range_returns_none() {
        let shell = montana_ap();
        assert!(solve_for_range(&shell, 100_000.0).is_none());
    }

    #[test]
    fn overmatch_rule() {
        assert!((overmatch_mm(406.0) - 28.4).abs() < 0.1);
        assert!((overmatch_mm(203.0) - 14.2).abs() < 0.1);
    }
}
