use timestamp_server::Config;

pub fn read_config(config_path: &str) -> std::io::Result<Config> {
    let content = std::fs::read_to_string(config_path)?;
    Ok(toml::from_str(&content)?)
}

pub fn build_address(protocol: &str, host: &str, endpoint: &str) -> String {
    format!(
        "{protocol}://{host}/{endpoint}",
        protocol = protocol,
        host = host,
        endpoint = endpoint
    )
}
