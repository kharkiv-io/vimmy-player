```
Intune-player
       ___            ___     __                 ___  __  
| |\ |  |  |  | |\ | |__  __ |__) |     /\  \ / |__  |__) 
| | \|  |  \__/ | \| |___    |    |___ /~~\  |  |___ |  \

Vim-like music player written in Rust

How to build and run 
1. Clone repository 
git clone https://github.com/kharkiv-io/vimmy-player
2. Install alsa-dev libraries
sudo apt-get install -y libasound2-dev
3. Build 
cargo build 
4. Run
cargo run 

Commands 
:q - exit from player and stop all playing songs 
:pause - pauses current playing song 
:unpause - unpause current song 
:songs_loaded - get number of loaded songs 
:kill_sink - clear all queue (sink) and pause playnig
:set_volume <float> - set volume percentage
:next - play next song that's loaded in sink 
:play - start playing

License 
GPLv3 (GNU General Public License version 3)
```
