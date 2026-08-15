//! Drag-based AP penetration engine (wows-toolkit ballistic model).
//! Split into domain submodules so every file stays small.

mod model;
mod sim;
#[cfg(test)]
mod tests;

pub use model::{BallisticShell, PenetrationPoint};
pub use sim::{overmatch_mm, penetration_curve, solve_for_range};

