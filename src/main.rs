use std::error::Error;
use rusty_audio::Audio;

fn main() -> Result <(), Box<dyn Error>>{
    let mut audio = Audio::new();
    for item in &["explode", "lose", "move", "pew", "startup", "win"] {
        audio.add(item, &format!("audio/{}.wav", item));
    }
    audio.play("startup");

    // Cleanup
    audio.wait();
    Ok(())
}

