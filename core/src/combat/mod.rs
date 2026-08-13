//! Combat calculations for advanced ship info and live battles.
//!
//! `ballistics` implements shell trajectory simulation with the
//! International Standard Atmosphere model, and `team_advantage` scores which
//! team holds the stronger position in a battle. Both are ports of the
//! reverse-engineering notes in `wows-toolkit/docs`.

mod ballistics;
mod team_advantage;
#[cfg(test)]
mod tests;

pub use ballistics::{
    air_density, simulate_trajectory, solve_for_range, Shell, TrajectoryPoint,
};
pub use team_advantage::{evaluate, AdvantageLevel, BattleState, FleetClass, FleetState, TeamAdvantage};
