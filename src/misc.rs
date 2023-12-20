

use std::time::{SystemTime, UNIX_EPOCH, SystemTimeError};
use crate::types::UnixTimestamp;

pub fn get_current_timestamp () -> Result<UnixTimestamp, SystemTimeError> { Ok(SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs()) }

