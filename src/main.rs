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

fn internal_executor() {
    std::process::Command::new("clear").status().unwrap();
    let mut current_folder_: String = String::from("your/path/to/your/playlist/*.mp3"); // You can change format to wav or ogg.
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
    
    Commands : 
    :q - quit | :play <index> - start playing new song 
    :pause - pausing song | :unpause - resume play 
    :set_volume <var> - set volume ( 0.01 - 1.0 )
    "#;
    
    let mut current_song: Option<Sink> = None;
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    loop {
        println!("{}", ascii_art);
        let mut current_command_: String = String::new();
        let mut current_volume_: String = String::new();
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
                    std::process::Command::new("clear").status().unwrap();
                } else {
                    std::process::Command::new("clear").status().unwrap();
                    println!("You're trying to pause nothing!");
                }
            }
            ":unpause" => {
                if let Some(sink) = &current_song {
                    sink.play();
                    std::process::Command::new("clear").status().unwrap();
                }
            }
            _ if current_command_.starts_with(":set_volume ") => {
                let splits: Vec<&str> = current_command_.split_whitespace().collect();
                if splits.len() == 2 {
                    if let Ok(var) = splits[1].parse::<fsize>() {
                        if let Some(sink) = &current_song {
                            sink.set_volume(var as f32);
                            std::process::Command::new("clear").status().unwrap();
                        }
                    }
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
                            std::process::Command::new("clear").status().unwrap();
                            println!("Song playing -> {}", songs_kiss_[index-1].display());
                    } else {
                            std::process::Command::new("clear").status().unwrap();
                            println!("Song with ID : {} never founded!", current_command_); 
                        }
                    }
                }
            }
            _ => {
                std::process::Command::new("clear").status().unwrap();
                println!("Incorrect command!");
            }
        }
    }
}

fn main() {
    internal_executor();
}
