//! Aircraft parsing (`wowsinfo.json -> aircrafts`).
//! Split into domain submodules so every file stays small.

mod helpers;
mod parse;
mod tests;
mod types;

pub use parse::parse_aircrafts;
pub use types::AircraftInfo;

