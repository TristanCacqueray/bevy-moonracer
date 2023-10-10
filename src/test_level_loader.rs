use bevy_moonracer::level_loader;

fn main() {
    for level in level_loader::load() {
        println!("{:?}", level);
    }
}
