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
    let mut current_folder_: String = String::from("/full/path/to/playlist/*.mp3"); // You can change format to wav or ogg.
    let mut songs_kiss_: Vec<PathBuf> = Vec::new();
    for path in glob(&current_folder_).unwrap().filter_map(Result::ok) {
        songs_kiss_.push(path.clone());
    }
    let ascii_art = r#"

 ▄ ▄▄▄▄     ■  █  ▐▌▄▄▄▄  ▗▞▀▚▖▄▄▄▄  █ ▗▞▀▜▌▄   ▄ ▗▞▀▚▖ ▄▄▄ 
 ▄ █   █ ▗▄▟▙▄▖▀▄▄▞▘█   █ ▐▛▀▀▘█   █ █ ▝▚▄▟▌█   █ ▐▛▀▀▘█    
 █ █   █   ▐▌       █   █ ▝▚▄▄▖█▄▄▄▀ █       ▀▀▀█ ▝▚▄▄▖█    
 █         ▐▌                  █     █      ▄   █           
           ▐▌                  ▀             ▀▀▀            

Intune-player | Version 1.0.b 
Developer : https://github.com/kharkiv-io

"#;
    println!("{}", ascii_art);
    let mut current_song: Option<Sink> = None;
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    loop {
        let mut current_command_: String = String::new();
        let mut current_volume_: String = String::new();
        println!("[ intune-player ]");
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
            ":unpause" => {
                if let Some(sink) = &current_song {
                    sink.play();
                } else {
                    println!("You're trying to unpause nothing!");
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
            _ if current_command_.starts_with(":play ") => {
                let splits: Vec<&str> = current_command_.split_whitespace().collect();
                if splits.len() == 2 {
                    if let Ok(index) = splits[1].parse::<usize>() {
                        if index > 0 && index <= songs_kiss_.len() {
                            if let Some(sink) = current_song.take() {
                                sink.stop();
                            }
                            let song_file = File::open(&songs_kiss_[index - 1]).unwrap();
                            let source = Decoder::new(BufReader::new(song_file)).unwrap();
                            let sink = Sink::try_new(&stream_handle).unwrap();
                            sink.append(source);
                            sink.play();
                            current_song = Some(sink);
                        } else {
                            println!("Song never founded!"); 
                        }
                    }
                }
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
