use base64::{
    alphabet::{self},
    engine::{GeneralPurpose, GeneralPurposeConfig},
    Engine,
};
use chrono::{DateTime, Utc};
use crypto::{digest::Digest, sha1::Sha1};
use lazy_static::lazy_static;
use rand::{rngs::SmallRng, Rng, SeedableRng};
use std::{sync::Arc, time::SystemTime};
use tokio::sync::Mutex;

lazy_static! {
    pub static ref RNG: Arc<Mutex<SmallRng>> = Arc::new(Mutex::new(SmallRng::from_os_rng()));
    static ref SHA1: Mutex<Sha1> = Mutex::new(Sha1::new());
    pub static ref B64: Arc<GeneralPurpose> = Arc::new(base64::engine::GeneralPurpose::new(
        &alphabet::STANDARD,
        GeneralPurposeConfig::default()
    ));
}

pub async fn generate_realtime_auth_header(
    realtime_api_key: &str,
) -> Result<String, anyhow::Error> {
    let prefix = "TrAnSpErTh";
    let now = SystemTime::now();
    let now: DateTime<Utc> = now.into();
    let now_in_perth = now.with_timezone(&chrono_tz::Australia::Perth);
    let datetime = now_in_perth.format("%d%m%Y%H%M%S").to_string();

    let mut rng = RNG.lock().await;
    let nonce: String = (0..6)
        .map(|_| (rng.random_range(0f64..1f64) * 10f64).floor() as i8 % 10)
        .map(|i| i.to_string())
        .collect::<Vec<_>>()
        .join("");

    let nonce = B64.encode(format!("{}-{}", nonce, datetime));

    let token = format!(
        "{}-{}-{}",
        prefix,
        realtime_api_key.split('-').collect::<Vec<_>>().join(""),
        datetime
    );

    let mut hasher = SHA1.lock().await;
    hasher.input_str(&token);
    let mut bytes = vec![0; hasher.output_bytes()];
    hasher.result(&mut bytes);
    hasher.reset();

    let encoded_token = B64.encode(bytes);

    Ok(format!(
        "Custom Username=PhoneApp,Nonce={},Token={}",
        nonce, encoded_token
    ))
}
