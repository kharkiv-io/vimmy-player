use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, source::Source};
use std::io;
use glob::glob;

fn play_full(path_to: String) { // Play function 
    let (_stream, stream_handle) = OutputStream::try_default().unwrap(); // Get OutputStream
    let file = BufReader::new(File::open(path_to).unwrap()); // Open audiofile
    let source = Decoder::new(file).unwrap(); // Decode audiofile 
    let duration = source.total_duration().unwrap().as_secs_f32(); // Get duration of the audiofile
    let play_ = stream_handle.play_raw(source.convert_samples()); // Play audiofile 
    std::thread::sleep(std::time::Duration::from_secs_f32(duration)); // Keep alive thread 
}

fn main() {
    let current_folder_: String = String::from("/path/to/folder/contains/your/tracks/*.mp3");
    for path in glob(&current_folder_).unwrap().filter_map(Result::ok) {
        play_full(path.into_os_string().into_string().unwrap()); // Convert PathBuf to String 
        std::thread::sleep(std::time::Duration::from_secs(1)); // Delay after play  
    }
}
