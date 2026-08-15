//! Ship builder: module tree resolution and per-selection stats.
//!
//! The game data stores every ship as a module tree (`ships.<id>.modules`),
//! where each module option references component ids (`ships.<id>.components`).
//! This is a Rust port of the Flutter two app's `ShipModules` model, following
//! the wows-toolkit convention of one typed struct per component.
//! Split into domain submodules so every file stays small.

mod build;
mod helpers;
mod slots;
mod tests;
mod types;

pub use build::build_ship_build;
pub use slots::{module_option_delta, module_slots};
pub use types::{ModuleOption, ModuleSelection, ShipBuild};

