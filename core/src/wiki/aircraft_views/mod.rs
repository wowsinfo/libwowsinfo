//! Carrier aircraft views for the wiki detail screen.
//! Split into domain submodules so every file stays small.

mod detail;
mod helpers;
mod slots;
mod support;

pub use detail::AircraftDetail;
pub use slots::{aircraft_slot_views, AircraftOptionView, AircraftSlotView};
pub use support::air_support_plane;

