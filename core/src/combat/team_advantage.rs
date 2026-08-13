//! Team advantage scoring for a live battle.
//!
//! Port of the `wows-toolkit` design notes (`TEAM_ADVANTAGE_SCORING.md`):
//! non-negative point tuples across score projection, fleet power and
//! strategic threat, plus the strength labels.

const CLASS_WEIGHTS: [(usize, f64); 5] = [
    (0, 1.5), // destroyer
    (1, 1.0), // cruiser
    (2, 1.0), // battleship
    (3, 1.3), // submarine
    (4, 1.2), // carrier
];

/// Combat state of one ship class.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct FleetClass {
    pub alive: u64,
    pub hp: f64,
    pub max_hp: f64,
}

impl FleetClass {
    fn power(&self, weight: f64) -> f64 {
        let fraction = if self.max_hp > 0.0 {
            (self.hp / self.max_hp).clamp(0.0, 1.0)
        } else {
            0.0
        };
        weight * self.alive as f64 * fraction
    }

    fn alive_count(&self) -> u64 {
        self.alive
    }
}

/// Combat state of one fleet.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct FleetState {
    pub destroyers: FleetClass,
    pub cruisers: FleetClass,
    pub battleships: FleetClass,
    pub submarines: FleetClass,
    pub carriers: FleetClass,
}

impl FleetState {
    fn classes(&self) -> [&FleetClass; 5] {
        [
            &self.destroyers,
            &self.cruisers,
            &self.battleships,
            &self.submarines,
            &self.carriers,
        ]
    }

    fn power(&self) -> f64 {
        self.classes()
            .iter()
            .zip(CLASS_WEIGHTS)
            .map(|(class, (_, weight))| class.power(weight))
            .sum()
    }

    fn total_alive(&self) -> u64 {
        self.classes().iter().map(|c| c.alive_count()).sum()
    }

    /// Number of classes with at least one ship alive (for diversity).
    fn classes_alive(&self) -> usize {
        self.classes().iter().filter(|c| c.alive > 0).count()
    }

    /// Destroyer + submarine survival score (`min(dd*1 + ss*0.8, 2.5)`).
    fn dd_ss_score(&self, time_weight: f64) -> f64 {
        (self.destroyers.alive as f64 + self.submarines.alive as f64 * 0.8)
            .min(2.5)
            * time_weight
    }
}

/// The state of a battle to score.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct BattleState {
    pub team0: FleetState,
    pub team1: FleetState,
    pub score0: f64,
    pub score1: f64,
    pub win_score: f64,
    /// Capture-point income in points per second for each team.
    pub cap_income0: f64,
    pub cap_income1: f64,
    /// Remaining match time in seconds.
    pub time_left: f64,
}

/// Strength label derived from the total point gap.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(Default)]
pub enum AdvantageLevel {
    #[default]
    Even,
    Weak,
    Moderate,
    Strong,
    Absolute,
}

/// The scoring result: non-negative tuples per team plus the verdict.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct TeamAdvantage {
    pub score_projection: (f64, f64),
    pub fleet_power: (f64, f64),
    pub strategic_threat: (f64, f64),
    pub total: (f64, f64),
    pub level: AdvantageLevel,
    pub team_eliminated: bool,
}

fn level_of(gap: f64) -> AdvantageLevel {
    if gap >= 10.0 {
        AdvantageLevel::Absolute
    } else if gap >= 6.0 {
        AdvantageLevel::Strong
    } else if gap >= 3.0 {
        AdvantageLevel::Moderate
    } else if gap >= 1.0 {
        AdvantageLevel::Weak
    } else {
        AdvantageLevel::Even
    }
}

/// Score projection (max 10): current gap (4), time-to-win (3), projected
/// final score (3).
fn score_projection(battle: &BattleState) -> (f64, f64) {
    let win_score = if battle.win_score > 0.0 {
        battle.win_score
    } else {
        return (0.0, 0.0);
    };
    let gap = (battle.score0 - battle.score1).abs();
    let mut team0 = 0.0;
    let mut team1 = 0.0;

    // Current score gap.
    if battle.score0 > battle.score1 {
        team0 += (gap / win_score * 4.0).min(4.0);
    } else if battle.score1 > battle.score0 {
        team1 += (gap / win_score * 4.0).min(4.0);
    }

    // Time-to-win from cap income alone.
    let time0 = if battle.cap_income0 > 0.0 {
        (win_score - battle.score0) / battle.cap_income0
    } else {
        f64::INFINITY
    };
    let time1 = if battle.cap_income1 > 0.0 {
        (win_score - battle.score1) / battle.cap_income1
    } else {
        f64::INFINITY
    };
    let can0 = time0 <= battle.time_left;
    let can1 = time1 <= battle.time_left;
    match (can0, can1) {
        (true, false) => team0 += 3.0,
        (false, true) => team1 += 3.0,
        (true, true) => {
            let diff = (time0 - time1).abs();
            let pts = if diff > 30.0 {
                3.0
            } else if diff > 10.0 {
                2.0
            } else if diff > 3.0 {
                1.0
            } else {
                0.0
            };
            if time0 < time1 {
                team0 += pts;
            } else if time1 < time0 {
                team1 += pts;
            }
        }
        (false, false) => {}
    }

    // Projected final score.
    let projected0 = battle.score0 + battle.cap_income0 * battle.time_left;
    let projected1 = battle.score1 + battle.cap_income1 * battle.time_left;
    let projected_gap = (projected0 - projected1).abs();
    let pts = if projected_gap >= 300.0 {
        3.0
    } else if projected_gap >= 150.0 {
        2.0
    } else if projected_gap >= 50.0 {
        1.0
    } else {
        0.0
    };
    if projected0 > projected1 {
        team0 += pts;
    } else if projected1 > projected0 {
        team1 += pts;
    }

    (team0, team1)
}

/// Fleet power (max 10): class-weighted HP split proportionally.
fn fleet_power(battle: &BattleState) -> (f64, f64) {
    let power0 = battle.team0.power();
    let power1 = battle.team1.power();
    let total = power0 + power1;
    if total <= 0.0 {
        return (0.0, 0.0);
    }
    (power0 / total * 10.0, power1 / total * 10.0)
}

/// Strategic threat (max 5): DD/SS survival, class diversity, CV advantage.
fn strategic_threat(battle: &BattleState) -> (f64, f64) {
    let time_weight = (battle.time_left / 300.0).clamp(0.2, 1.0);
    let diversity0 = match battle.team0.classes_alive() {
        0 | 1 => 0.0,
        2 => 0.5,
        3 => 1.0,
        _ => 1.5,
    };
    let diversity1 = match battle.team1.classes_alive() {
        0 | 1 => 0.0,
        2 => 0.5,
        3 => 1.0,
        _ => 1.5,
    };
    let (cv0, cv1) = match battle.team0.carriers.alive.cmp(&battle.team1.carriers.alive) {
        std::cmp::Ordering::Greater => (1.0, 0.0),
        std::cmp::Ordering::Less => (0.0, 1.0),
        std::cmp::Ordering::Equal => (0.0, 0.0),
    };
    (
        battle.team0.dd_ss_score(time_weight) + diversity0 + cv0,
        battle.team1.dd_ss_score(time_weight) + diversity1 + cv1,
    )
}

/// Evaluate which team holds the advantage.
#[must_use]
pub fn evaluate(battle: &BattleState) -> TeamAdvantage {
    let team0_dead = battle.team0.total_alive() == 0;
    let team1_dead = battle.team1.total_alive() == 0;
    if team0_dead || team1_dead {
        let (total0, total1) = if team0_dead { (0.0, 25.0) } else { (25.0, 0.0) };
        return TeamAdvantage {
            total: (total0, total1),
            level: AdvantageLevel::Absolute,
            team_eliminated: true,
            ..Default::default()
        };
    }

    let score_projection = score_projection(battle);
    let fleet_power = fleet_power(battle);
    let strategic_threat = strategic_threat(battle);
    let total = (
        score_projection.0 + fleet_power.0 + strategic_threat.0,
        score_projection.1 + fleet_power.1 + strategic_threat.1,
    );
    TeamAdvantage {
        score_projection,
        fleet_power,
        strategic_threat,
        total,
        level: level_of((total.0 - total.1).abs()),
        team_eliminated: false,
    }
}
