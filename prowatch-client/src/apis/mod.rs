use reqwest;
use serde_json;

#[derive(Debug)]
pub enum Error {
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
    Io(std::io::Error),
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::Serde(e)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

pub fn urlencode<T: AsRef<str>>(s: T) -> String {
    ::url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
}

mod alarms_api;
pub use self::alarms_api::{AlarmsApi, AlarmsApiClient};
mod areas_api;
pub use self::areas_api::{AreasApi, AreasApiClient};
mod assets_api;
pub use self::assets_api::{AssetsApi, AssetsApiClient};
mod badge_fields_api;
pub use self::badge_fields_api::{BadgeFieldsApi, BadgeFieldsApiClient};
mod badging_api;
pub use self::badging_api::{BadgingApi, BadgingApiClient};
mod channels_api;
pub use self::channels_api::{ChannelsApi, ChannelsApiClient};
mod clearcodes_api;
pub use self::clearcodes_api::{ClearcodesApi, ClearcodesApiClient};
mod companies_api;
pub use self::companies_api::{CompaniesApi, CompaniesApiClient};
mod configuration_api;
pub use self::configuration_api::{ConfigurationApi, ConfigurationApiClient};
mod events_api;
pub use self::events_api::{EventsApi, EventsApiClient};
mod logdevs_api;
pub use self::logdevs_api::{LogdevsApi, LogdevsApiClient};
mod panels_api;
pub use self::panels_api::{PanelsApi, PanelsApiClient};
mod sites_api;
pub use self::sites_api::{SitesApi, SitesApiClient};
mod subpanels_api;
pub use self::subpanels_api::{SubpanelsApi, SubpanelsApiClient};
mod timezones_api;
pub use self::timezones_api::{TimezonesApi, TimezonesApiClient};
mod users_api;
pub use self::users_api::{UsersApi, UsersApiClient};
mod workstations_api;
pub use self::workstations_api::{WorkstationsApi, WorkstationsApiClient};

pub mod client;
pub mod configuration;
