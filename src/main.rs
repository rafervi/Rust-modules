mod player;
use crate::archive::arch::arch_file as arc;
use rand::{Rng, thread_rng};
mod archive;
use rand::distributions::Alphanumeric;

fn main() {
    player::play_movie("snatch.mp4");
    player::play_audio("rhcp.mp3");
    clean::perform_clean();
    clean::files::clean_files();
    arc("something.txt");
    let mut rng = rand::thread_rng();
    let a:i32 = rng.gen();
    println!("{}",a);
    let mut rng = rand::thread_rng();
    let i: i32 = rng.gen();
    println!("{}",i);

    println!("bounded int: {}", rng.gen_range(0,160));
    println!("bounded float: {}", rng.gen_range(0.0, 100.0));
    println!("bounded float: {}", rng.gen_range(0.0, 100.0));

    let rand_string: String = thread_rng()
        .sample_iter(Alphanumeric)
        .take(30)
        .collect();
    println!("Gen String: {} ", rand_string)
;
}

mod clean {
    pub fn perform_clean() {
        println!("Cleaning mod");
    }
    pub mod files {
        pub fn clean_files() {
            println!("Removing unused files");
        }
    }
}
