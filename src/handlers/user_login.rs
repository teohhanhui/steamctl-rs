use std::sync::Arc;

use anyhow::Result;
use parking_lot::RwLock;
use secrecy::{ExposeSecret, SecretString};
use steamguard::{
    protobufs::steammessages_auth_steamclient::EAuthTokenPlatformType,
    transport::WebApiTransport,
    DeviceDetails,
};

use crate::api_responses::AllowedConfirmation;

pub fn _build_device_details() -> DeviceDetails {
    DeviceDetails {
        friendly_name: format!(
            "{} (steamctl)",
            gethostname::gethostname()
                .into_string()
                .expect("failed to get hostname")
        ),
        platform_type: EAuthTokenPlatformType::k_EAuthTokenPlatformType_MobileApp,
        os_type: -500,
        gaming_device_type: 528,
    }
}

pub fn begin_auth_via_credentials(
    _self: Arc<RwLock<steamguard::UserLogin<WebApiTransport>>>,
    username: String,
    password: SecretString,
) -> Result<Vec<AllowedConfirmation>, steamguard::LoginError> {
    let mut login = _self.write();
    let confirmation_methods =
        login.begin_auth_via_credentials(&username, password.expose_secret())?;
    let confirmation_methods = confirmation_methods
        .into_iter()
        .map(|method| AllowedConfirmation {
            confirmation_type: method.confirmation_type,
            associated_message: method.associated_messsage,
        })
        .collect();
    Ok(confirmation_methods)
}
