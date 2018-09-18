
mod media;
use media::Playable;

struct Audio(String);
struct Video(String);

impl Playable for Audio {
    fn play(&self) {
        println!("🎵 Now playing: {}", self.0);
    }
    fn pause() {
        println!("Paused");
    }
}

impl Playable for Video {
    fn play(&self) {
        println!("🎵 Now playing: {}", self.0);
    }
    fn pause() {
        println!("Paused");
    }
}

fn main() {
    println!("Music player!");
    let audio = Audio("ambient_music.mp3".to_string());
    audio.play();
}
