#[derive(serde::Deserialize, Clone)]
pub struct ApplicationConfig {
    pub redis_connection_string: String,
    pub realtime_api_key: String,
    pub reference_data_api_key: String,
}
