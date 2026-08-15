//! Local ship wiki view model (split into domain submodules).

mod airstrike;
mod armor;
mod battery;
mod depth_charge;
mod dispersion;
mod module;
mod pen;
mod shell;
mod ship;
mod special;
mod torpedo;

pub use battery::MainBatteryView;
pub use module::{ModuleOptionView, ModuleSlotView};
pub use pen::PenCurveView;
pub use shell::ShellView;
pub use ship::{build_local_ship_wiki, LocalShipWiki};
pub use torpedo::TorpedoView;

#[cfg(test)]
mod tests;

