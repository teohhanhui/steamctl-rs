use std::cell::OnceCell;

use anyhow::Result;
use steamguard::transport::WebApiTransport;

pub fn _transport(transport: &OnceCell<WebApiTransport>) -> Result<&WebApiTransport> {
    transport.get_or_try_init(|| {
        let http_client = reqwest::blocking::Client::builder().build()?;
        let transport = WebApiTransport::new(http_client);
        Ok(transport)
    })
}
