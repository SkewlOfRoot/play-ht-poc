use rodio::{Decoder, OutputStream, Source};
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::io::Cursor;

pub async fn speak(text: &str) -> Result<(), Box<dyn Error>> {
    let content_bytes = tts_bytes(text).await?;
    play_sound(content_bytes)?;
    Ok(())
}

fn play_sound(content_bytes: Vec<u8>) -> Result<(), Box<dyn Error>> {
    let cursor: Cursor<Vec<u8>> = Cursor::new(content_bytes.to_vec());

    let (_stream, handle) = OutputStream::try_default()?;

    let source = Decoder::new(cursor).unwrap();
    handle.play_raw(source.convert_samples())?;

    std::thread::sleep(std::time::Duration::from_secs(5));
    Ok(())
}

async fn tts_bytes(text: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let TtsCredentials {
        user_id,
        authorization,
    } = credentials()?;

    let body_map = body_map(text);

    let client = reqwest::Client::new();
    let response = client
        .post("https://api.play.ht/api/v2/tts/stream")
        .header("AUTHORIZATION", authorization)
        .header("X-USER-ID", user_id)
        .header("content-type", "application/json")
        .json(&body_map)
        .send()
        .await?;

    response.error_for_status_ref()?;

    let content_bytes = response.bytes().await?;
    Ok(content_bytes.to_vec())
}

fn body_map(text: &str) -> HashMap<&str, &str> {
    let mut map = HashMap::new();
    map.insert("text", text);
    map.insert("output_format", "mp3");
    map.insert(
        "voice",
        "s3://voice-cloning-zero-shot/d9ff78ba-d016-47f6-b0ef-dd630f59414e/female-cs/manifest.json",
    );
    map.insert("quality", "draft");
    map.insert("voice_engine", "PlayHT2.0-turbo");
    map.insert("emotion", "female_happy");
    map
}

fn credentials() -> Result<TtsCredentials, Box<dyn Error>> {
    let user_id = env::var("PLAYHT_USER_ID").expect("PLAYHT_USER_ID not found.");
    let authorization = env::var("PLAYHT_AUTHORIZATION").expect("PLAYHT_AUTHORIZATION not found.");

    Ok(TtsCredentials {
        user_id,
        authorization,
    })
}

struct TtsCredentials {
    user_id: String,
    authorization: String,
}
