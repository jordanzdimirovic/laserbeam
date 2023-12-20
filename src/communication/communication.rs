
use std::path::PathBuf;

use url::Url;

use crate::misc::get_current_timestamp;

use crate::types::UnixTimestamp;

#[derive(Debug)]
pub struct ConnectError {
    message: String
}

#[derive(Debug)]
pub struct SessionInformation {
    server_location: Url,
    active_since: UnixTimestamp
}

#[derive(Debug)]
pub struct UpSession {
    sess_info: SessionInformation,
    location: Box<PathBuf>,
    guid: String,
    passphrase: Option<String>,
    expiry: UnixTimestamp
}

pub fn establish_beam (server_address: &str, location: Box<PathBuf>) -> Result<UpSession, ConnectError> {
    // Parse the server address
    let server_url = Url::parse(server_address).map_err(|err| ConnectError {
        message: err.to_string()
    })?;

    // Send a request to the init endpoint
    // ...
    let resp_guid: String = "abcde".to_string();
    let resp_passphrase: Option<String> = Some ("defgh".to_string());

    // Get current timestamp
    let now = get_current_timestamp().expect("Something went wrong when getting the current timestamp");

    // Create an up session
    Ok(UpSession {
        expiry: now + (60 * 5), // Expires in 5 minutes
        guid: resp_guid,
        location: location,
        passphrase: resp_passphrase,
        sess_info: SessionInformation {
            active_since: now,
            server_location: server_url
        }
    })

}

// Implementations for connection
impl UpSession {
    fn start_beam() {
        // Todo
    }
}