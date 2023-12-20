

pub mod connect {
    use url::Url;

    struct LaserbeamConnectError (&str);

    struct LaserbeamActiveConnection {
        location: Url,
        active_since: i64
    }
    
    fn connect_to_server (server_address: &str) -> Result<LaserbeamActiveConnection, LaserbeamConnectError> {
        let url = Url::parse(server_address).unwrap_or_else(|err| return Err);
        return Ok(LaserbeamActiveConnection {
            active_since: 0i64,
            location: url
        });
    }
}

