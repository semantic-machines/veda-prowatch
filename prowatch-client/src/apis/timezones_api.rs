/*
 * Pro-Watch API REST
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 771c6a63-9da8-4300-b275-33061d174776
 *
 * Generated by: https://openapi-generator.tech
 */

use std::borrow::Borrow;
#[allow(unused_imports)]
use std::option::Option;
use std::rc::Rc;

use reqwest;

use super::{configuration, Error};

pub struct TimezonesApiClient {
    configuration: Rc<configuration::Configuration>,
}

impl TimezonesApiClient {
    pub fn new(configuration: Rc<configuration::Configuration>) -> TimezonesApiClient {
        TimezonesApiClient {
            configuration,
        }
    }
}

pub trait TimezonesApi {
    fn timezones(&self) -> Result<Vec<crate::models::Array>, Error>;
    fn timezones1(&self, body: &str) -> Result<(), Error>;
    fn timezones_timezone_id(&self, timezone_id: &str, body: &str) -> Result<(), Error>;
}

impl TimezonesApi for TimezonesApiClient {
    fn timezones(&self) -> Result<Vec<crate::models::Array>, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/pwapi/timezones", configuration.base_path);
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref auth_conf) = configuration.basic_auth {
            req_builder = req_builder.basic_auth(auth_conf.0.to_owned(), auth_conf.1.to_owned());
        };

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn timezones1(&self, body: &str) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/pwapi/timezones", configuration.base_path);
        let mut req_builder = client.post(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref auth_conf) = configuration.basic_auth {
            req_builder = req_builder.basic_auth(auth_conf.0.to_owned(), auth_conf.1.to_owned());
        };
        req_builder = req_builder.json(&body);

        // send request
        let req = req_builder.build()?;

        client.execute(req)?.error_for_status()?;
        Ok(())
    }

    fn timezones_timezone_id(&self, timezone_id: &str, body: &str) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/pwapi/timezones/{timezoneID}", configuration.base_path, timezoneID = crate::apis::urlencode(timezone_id));
        let mut req_builder = client.put(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref auth_conf) = configuration.basic_auth {
            req_builder = req_builder.basic_auth(auth_conf.0.to_owned(), auth_conf.1.to_owned());
        };
        req_builder = req_builder.json(&body);

        // send request
        let req = req_builder.build()?;

        client.execute(req)?.error_for_status()?;
        Ok(())
    }
}