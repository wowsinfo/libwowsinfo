//! Torpedo component shape.

use facet::Facet;
use serde::{Deserialize, Serialize};

use super::guns::WeaponInfo;

/// Torpedo component (`singleShot` + `launchers`).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Facet)]
pub struct TorpedoStats {
    pub single_shot: bool,
    pub launchers: Vec<WeaponInfo>,
}
