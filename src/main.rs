use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, source::Source, Sink};
use std::io;
use glob::glob;
use std::env;
use std::process;
use std::thread;
use std::path::PathBuf;
use colored::Colorize;
use fsize::fsize;


fn internal_executor() {
    std::process::Command::new("clear").status().unwrap();
    let mut current_folder_: String = String::from("/home/path/to/your/playlist/folder/*.mp3"); // You can change format to wav or ogg.
    let mut songs_kiss_: Vec<PathBuf> = Vec::new();
    for path in glob(&current_folder_).unwrap().filter_map(Result::ok) {
        songs_kiss_.push(path.clone());
    }
    let ascii_art = r#"
       ___            ___     __                 ___  __  
| |\ |  |  |  | |\ | |__  __ |__) |     /\  \ / |__  |__) 
| | \|  |  \__/ | \| |___    |    |___ /~~\  |  |___ |  \ 
                                                          
Intunie-player | Version 1.0.r
Developer : https://github.com/kharkiv-io
"#;
    println!("{}", Colorize::white(ascii_art));
    let mut current_song: Option<Sink> = None;
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    loop {
        let mut current_command_: String = String::new();
        let mut current_volume_: String = String::new();
        println!("{}", Colorize::white("[ intune-player ]! "));
        io::stdin().read_line(&mut current_command_)
            .expect("Failure while read!");
        let current_command_ = current_command_.trim();
        match current_command_ {
            ":q" => {
                break;
            }
            ":pause" => {
                if let Some(sink) = &current_song {
                    sink.pause();
                } else {
                    println!("You're trying to pause nothing!");
                }
            }
            ":songs_loaded" => {
                if let Some(sink) = &current_song {
                    println!("{} songs in queue right now!", sink.len());
                } else {
                    println!("Failed to get songs in sink.");
                }
            }
            ":unpause" => {
                if let Some(sink) = &current_song {
                    sink.play();
                } else {
                    println!("You're trying to unpause nothing!");
                }
            }
            _ if current_command_.starts_with(":set_playback_speed ") => {
                let splits: Vec<&str> = current_command_.split_whitespace().collect();
                if splits.len() == 2 {
                    if let Ok(var) = splits[1].parse::<fsize>() {
                        if let Some(sink) = &current_song {
                            sink.set_speed(var as f32);
                        }
                    }
                }
            }
            ":kill_sink" => {
                if let Some(sink) = &current_song {
                    sink.clear();
                }
            }
            _ if current_command_.starts_with(":set_volume ") => {
                let splits: Vec<&str> = current_command_.split_whitespace().collect();
                if splits.len() == 2 {
                    if let Ok(var) = splits[1].parse::<fsize>() {
                        if let Some(sink) = &current_song {
                            sink.set_volume(var as f32);
                        }
                    } else {
                        println!("Wtf?");
                    }
                } else {
                    println!("Missed args.");
                }
            }
            ":next" => {
                if let Some(sink) = &current_song {
                    sink.skip_one();
                }
            }
            ":play" => {
                if let Some(sink) = current_song.take() {
                    sink.stop();
                }
                let sink = Sink::try_new(&stream_handle).unwrap();
                for i in 0..songs_kiss_.len() {
                    let mut song_file = File::open(&songs_kiss_[i]).unwrap();
                    let mut source = Decoder::new(BufReader::new(song_file)).unwrap();
                    sink.append(source);
                }
                sink.play();
                current_song = Some(sink);
            }
            _ => {
                println!("Incorrect command!");
            }
        }
    }
}

fn main() {
    internal_executor();
}
