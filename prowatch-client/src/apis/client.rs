use std::rc::Rc;

use super::configuration::Configuration;

pub struct PWAPIClient {
    alarms_api: Box<dyn crate::apis::AlarmsApi>,
    areas_api: Box<dyn crate::apis::AreasApi>,
    assets_api: Box<dyn crate::apis::AssetsApi>,
    badge_fields_api: Box<dyn crate::apis::BadgeFieldsApi>,
    badging_api: Box<dyn crate::apis::BadgingApi>,
    channels_api: Box<dyn crate::apis::ChannelsApi>,
    clearcodes_api: Box<dyn crate::apis::ClearcodesApi>,
    companies_api: Box<dyn crate::apis::CompaniesApi>,
    configuration_api: Box<dyn crate::apis::ConfigurationApi>,
    events_api: Box<dyn crate::apis::EventsApi>,
    logdevs_api: Box<dyn crate::apis::LogdevsApi>,
    panels_api: Box<dyn crate::apis::PanelsApi>,
    sites_api: Box<dyn crate::apis::SitesApi>,
    subpanels_api: Box<dyn crate::apis::SubpanelsApi>,
    timezones_api: Box<dyn crate::apis::TimezonesApi>,
    users_api: Box<dyn crate::apis::UsersApi>,
    workstations_api: Box<dyn crate::apis::WorkstationsApi>,
}

impl PWAPIClient {
    pub fn new(configuration: Configuration) -> Self {
        let rc = Rc::new(configuration);

        PWAPIClient {
            alarms_api: Box::new(crate::apis::AlarmsApiClient::new(rc.clone())),
            areas_api: Box::new(crate::apis::AreasApiClient::new(rc.clone())),
            assets_api: Box::new(crate::apis::AssetsApiClient::new(rc.clone())),
            badge_fields_api: Box::new(crate::apis::BadgeFieldsApiClient::new(rc.clone())),
            badging_api: Box::new(crate::apis::BadgingApiClient::new(rc.clone())),
            channels_api: Box::new(crate::apis::ChannelsApiClient::new(rc.clone())),
            clearcodes_api: Box::new(crate::apis::ClearcodesApiClient::new(rc.clone())),
            companies_api: Box::new(crate::apis::CompaniesApiClient::new(rc.clone())),
            configuration_api: Box::new(crate::apis::ConfigurationApiClient::new(rc.clone())),
            events_api: Box::new(crate::apis::EventsApiClient::new(rc.clone())),
            logdevs_api: Box::new(crate::apis::LogdevsApiClient::new(rc.clone())),
            panels_api: Box::new(crate::apis::PanelsApiClient::new(rc.clone())),
            sites_api: Box::new(crate::apis::SitesApiClient::new(rc.clone())),
            subpanels_api: Box::new(crate::apis::SubpanelsApiClient::new(rc.clone())),
            timezones_api: Box::new(crate::apis::TimezonesApiClient::new(rc.clone())),
            users_api: Box::new(crate::apis::UsersApiClient::new(rc.clone())),
            workstations_api: Box::new(crate::apis::WorkstationsApiClient::new(rc.clone())),
        }
    }

    pub fn alarms_api(&self) -> &dyn crate::apis::AlarmsApi {
        self.alarms_api.as_ref()
    }

    pub fn areas_api(&self) -> &dyn crate::apis::AreasApi {
        self.areas_api.as_ref()
    }

    pub fn assets_api(&self) -> &dyn crate::apis::AssetsApi {
        self.assets_api.as_ref()
    }

    pub fn badge_fields_api(&self) -> &dyn crate::apis::BadgeFieldsApi {
        self.badge_fields_api.as_ref()
    }

    pub fn badging_api(&self) -> &dyn crate::apis::BadgingApi {
        self.badging_api.as_ref()
    }

    pub fn channels_api(&self) -> &dyn crate::apis::ChannelsApi {
        self.channels_api.as_ref()
    }

    pub fn clearcodes_api(&self) -> &dyn crate::apis::ClearcodesApi {
        self.clearcodes_api.as_ref()
    }

    pub fn companies_api(&self) -> &dyn crate::apis::CompaniesApi {
        self.companies_api.as_ref()
    }

    pub fn configuration_api(&self) -> &dyn crate::apis::ConfigurationApi {
        self.configuration_api.as_ref()
    }

    pub fn events_api(&self) -> &dyn crate::apis::EventsApi {
        self.events_api.as_ref()
    }

    pub fn logdevs_api(&self) -> &dyn crate::apis::LogdevsApi {
        self.logdevs_api.as_ref()
    }

    pub fn panels_api(&self) -> &dyn crate::apis::PanelsApi {
        self.panels_api.as_ref()
    }

    pub fn sites_api(&self) -> &dyn crate::apis::SitesApi {
        self.sites_api.as_ref()
    }

    pub fn subpanels_api(&self) -> &dyn crate::apis::SubpanelsApi {
        self.subpanels_api.as_ref()
    }

    pub fn timezones_api(&self) -> &dyn crate::apis::TimezonesApi {
        self.timezones_api.as_ref()
    }

    pub fn users_api(&self) -> &dyn crate::apis::UsersApi {
        self.users_api.as_ref()
    }

    pub fn workstations_api(&self) -> &dyn crate::apis::WorkstationsApi {
        self.workstations_api.as_ref()
    }
}
