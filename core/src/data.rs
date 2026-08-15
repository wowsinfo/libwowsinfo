//! Port of `src/value/data.ts`: app constants, storage keys and server helpers.

use facet::Facet;
use serde::{Deserialize, Serialize};

/// App information (from `APP` in `value/data.ts`).
pub const APP_VERSION: &str = "1.7.0";
pub const IOS_VERSION: &str = "1.7.0";
pub const GAME_VERSION: &str = "12.7.0.0";
pub const GITHUB: &str = "https://github.com/wowsinfo/react-native-app";
pub const APP_STORE: &str = "https://itunes.apple.com/app/id1202750166";
pub const GOOGLE_PLAY: &str =
    "https://play.google.com/store/apps/details?id=com.yihengquan.wowsinfo";
pub const DEVELOPER: &str = "mailto:development.henryquan@gmail.com?subject=[WoWs Info 1.7.0] ";
pub const PATREON: &str = "https://www.patreon.com/henryquan";
pub const PAYPAL: &str = "https://www.paypal.me/YihengQuan";
pub const PERSONAL_RATING_URL: &str = "https://wows-numbers.com/personal/rating";
pub const LATEST_RELEASE: &str = "https://github.com/wowsinfo/react-native-app/releases/latest";

/// The four Wargaming servers, ordered exactly like the `SERVER` array in
/// `value/data.ts` (`["ru", "eu", "com", "asia"]`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default, Facet)]
#[repr(u8)]
pub enum Server {
    Ru = 0,
    Eu = 1,
    Com = 2,
    #[default]
    Asia = 3,
}

pub const SERVER: [Server; 4] = [Server::Ru, Server::Eu, Server::Com, Server::Asia];

impl Server {
    /// Domain used in Wargaming API URLs, e.g. `api.worldofwarships.eu`.
    #[must_use]
    pub fn domain(self) -> &'static str {
        ["ru", "eu", "com", "asia"][self as usize]
    }

    /// Like `getPrefix` in `value/data.ts`: `com` maps to `na`.
    #[must_use]
    pub fn prefix(self) -> &'static str {
        if self == Self::Com {
            "na"
        } else {
            self.domain()
        }
    }

    #[must_use]
    pub fn from_index(index: usize) -> Option<Self> {
        SERVER.get(index).copied()
    }
}

/// User preferences / local settings keys (`LOCAL` in `value/data.ts`).
pub mod local {
    pub const FRIEND_LIST: &str = "@WoWs_Info:playerList";
    pub const USER_INFO: &str = "@WoWs_Info:userInfo";
    pub const USER_DATA: &str = "@WoWs_Info:userData";
    pub const USER_SERVER: &str = "@WoWs_Info:currServer";
    pub const APP_VERSION: &str = "@WoWs_Info:currVersion";
    pub const GAME_VERSION: &str = "@WoWs_Info:gameVersion";
    pub const DATE: &str = "@WoWs_Info:currDate";
    pub const LAST_UPDATE: &str = "@WoWs_Info:lastUpdate";
    pub const THEME: &str = "@WoWs_Info:themeColour";
    pub const DARK_MODE: &str = "@WoWs_Info:darkMode";
    pub const SWAP_BUTTON: &str = "@WoWs_Info:swapButton";
    pub const NO_IMAGE_MODE: &str = "@WoWs_Info:noImageMode";
    pub const FIRST_LAUNCH: &str = "@WoWs_Info:firstLaunch";
    pub const API_LANGUAGE: &str = "@WoWs_Info:apiLanguage";
    pub const USER_LANGUAGE: &str = "@WoWs_Info:userLanguage";
    pub const LAST_LOCATION: &str = "@WoWs_Info:lastLocation";
    pub const PRO_VERSION: &str = "@WoWs_Info:proVersion";
    pub const RS_IP: &str = "@WoWs_Info:rsIP";
    pub const SHOW_BANNER: &str = "@WoWs_Info:banner_ads";
    pub const SHOW_FULLSCREEN: &str = "@WoWs_Info:fullscreen_ads";
}

/// Cached Wargaming data keys (`SAVED` in `value/data.ts`).
pub mod saved {
    pub const LANGUAGE: &str = "@Data:language";
    pub const ENCYCLOPEDIA: &str = "@Data:encyclopedia";
    pub const ACHIEVEMENT: &str = "@Data:achievement";
    pub const COMMANDER_SKILL: &str = "@Data:commander_skill";
    pub const COLLECTION: &str = "@Data:collection";
    pub const WARSHIP: &str = "@Data:warship";
    pub const MAP: &str = "@Data:gameMap";
    pub const CONSUMABLE: &str = "@Data:consumable";
    pub const PR: &str = "@Data:personal_rating";
}

/// Default preference values used by `DataLoader.loadLocal`.
pub const DEFAULT_SERVER: Server = Server::Asia;
pub const DEFAULT_API_LANGUAGE: &str = "en";
pub const DEFAULT_USER_LANGUAGE: &str = "en";
pub const DEFAULT_SWAP_BUTTON: bool = false;
pub const DEFAULT_FIRST_LAUNCH: bool = true;
pub const DEFAULT_PRO_VERSION: bool = false;

/// Number of seconds in a day.
const SECONDS_PER_DAY: i64 = 86_400;

/// `shouldUpdateWithCycle` in `value/data.ts`: update when at least 7 days
/// separate `last_update` from the current date.
#[must_use]
pub fn should_update_with_cycle(curr_secs: i64, last_update_secs: i64) -> bool {
    days_between(curr_secs, last_update_secs) >= 7
}

/// `dayDifference` in `core/util/Util.ts`: absolute day difference, ceil.
#[must_use]
pub fn days_between(a_secs: i64, b_secs: i64) -> i64 {
    let diff = (a_secs - b_secs).unsigned_abs();
    diff.div_ceil(SECONDS_PER_DAY as u64) as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn server_domains_and_prefixes_match_typescript() {
        assert_eq!(Server::Ru.domain(), "ru");
        assert_eq!(Server::Eu.domain(), "eu");
        assert_eq!(Server::Com.domain(), "com");
        assert_eq!(Server::Asia.domain(), "asia");
        assert_eq!(Server::Com.prefix(), "na");
        assert_eq!(Server::Asia.prefix(), "asia");
    }

    #[test]
    fn default_server_is_asia() {
        assert_eq!(Server::default(), Server::Asia);
        assert_eq!(Server::from_index(3), Some(Server::Asia));
    }

    #[test]
    fn update_cycle_is_seven_days() {
        let now = 1_700_000_000;
        assert!(!should_update_with_cycle(now, now - 6 * SECONDS_PER_DAY));
        assert!(should_update_with_cycle(now, now - 7 * SECONDS_PER_DAY));
        assert!(should_update_with_cycle(now, now - 30 * SECONDS_PER_DAY));
    }

    #[test]
    fn day_difference_rounds_up() {
        assert_eq!(days_between(0, 86_399), 1);
        assert_eq!(days_between(0, 86_400), 1);
        assert_eq!(days_between(0, 86_401), 2);
    }
}
