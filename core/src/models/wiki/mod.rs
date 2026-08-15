//! Ship and wiki models (`/wows/encyclopedia/*`).
//! Split into domain submodules so every file stays small.

mod armament;
mod consumables;
mod profile;
mod ships;
mod ship_wiki;
mod skills;
mod wiki;

pub use armament::{AntiAircraftProfile, AntiAircraftSlot, ArtilleryProfile, GunSlot, ShellInfo, TorpedoProfile};
pub use consumables::{Consumable, ConsumableProfile};
pub use profile::{EngineProfile, HullProfile, MinMax, ShipArmour, ShipConcealment, ShipMobility, ShipProfile, ShipWeaponry};
pub use ships::{EncyclopediaShip, Images, RawEncyclopediaShip};
pub use ship_wiki::ShipWiki;
pub use skills::{CommanderSkill, Perk};
pub use wiki::{CollectionCard, WikiCollection, WikiMap};

