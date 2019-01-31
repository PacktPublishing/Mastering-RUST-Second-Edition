// super_player/src/media.rs

pub trait Playable {
    fn play(&self);
    fn pause() {
        println!("Paused");
    }
}
