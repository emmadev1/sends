use std::{process::{Command, Stdio}, io, env, thread, fs, path::Path};
use toml::Table;

pub mod gui;

#[derive(Debug, PartialEq)]
struct Config {
    dest: String,
    framerate: String,
    resolution: String,
    audio_source: String,
    mplayer: String,
    ffmpeg_binary: String,
    platform: String
}

fn main() {
    //thread::spawn(|| {gui::main_gui()});

    let mut configs: Config = read_config();

    //Argument checking loop, all variables that are affected by it must be declared before this
    let args: Vec<String> = env::args().collect();
    for i in &args {
        if i == "--local" || i == "-l" {
            configs.dest = String::from("local");
        }
        else if i == "-h" || i == "--help" {
            print_help();
            return
        }
    }

    if configs.dest.is_empty() { // We check if its empty again to ensure we have a destination
        println!("Destination is empty, defaulting to local");
        configs.dest = String::from("udp://127.0.0.1:9000");
    }

    if configs.dest == "local" {
        configs.dest = String::from("udp://127.0.0.1:9000");
    }

    configs.platform = String::from(env::consts::OS); // Moving this to another place to group it with the other definitions would be nice
    println!("{:?}", configs); // DEBUG

    if configs.resolution.is_empty() || configs.framerate.is_empty() || configs.audio_source.is_empty() {
        println!("Missing key configuration parameters, exiting");
        return
    }
    else {
        invoke_ffmpeg(configs);
        return
    }
}

fn invoke_ffmpeg(configs: Config) {
    let video_in;
    let audio_in;
    if configs.platform == "linux" {
        video_in = "x11grab";
        audio_in = "pulse";
    }
    else if configs.platform == "windows" {
        video_in = "dshow";
        audio_in = "dshow";
    }
    else {
        panic!("Platform unknown or not supported");
    }

    if configs.resolution.trim() == "native" {
        if configs.dest.trim() == "udp://127.0.0.1:9000" {
            Command::new(configs.mplayer).arg(&configs.dest).arg("-profile=low-latency").stdout(Stdio::null()).spawn().expect("Cannot open mpv");
        }
        Command::new(configs.ffmpeg_binary).arg("-f").arg(video_in)
            .arg("-framerate").arg(configs.framerate)
            .arg("-i").arg(":0")
            .arg("-f").arg(audio_in)
            .arg("-i").arg(configs.audio_source)
            .arg("-c:v").arg("libx264").arg("-preset").arg("ultrafast").arg("-tune").arg("zerolatency")
            .arg("-f").arg("mpegts").arg(&configs.dest)
            .status().expect("Cannot open ffmpeg");
    }
    else {
        if configs.dest.trim() == "udp://127.0.0.1:9000" {
            Command::new(configs.mplayer).arg(&configs.dest).arg("-profile=low-latency").stdout(Stdio::null()).spawn().expect("Cannot open mpv");
        }
        Command::new(configs.ffmpeg_binary).arg("-f").arg(video_in)
            .arg("-framerate").arg(configs.framerate)
            .arg("-s").arg(configs.resolution)
            .arg("-i").arg(":0")
            .arg("-f").arg(audio_in)
            .arg("-i").arg(configs.audio_source)
            .arg("-c:v").arg("libx264").arg("-preset").arg("ultrafast").arg("-tune").arg("zerolatency")
            .arg("-f").arg("mpegts").arg(&configs.dest)
            .status().expect("Cannot open ffmpeg");
    }
}

fn print_help() {
    println!("Sends, a simple application to stream video and audio to friends\n");
    println!(" -l, --local\t\tStream to udp://127.0.0.1:9000");
    println!(" -h, --help\t\tPrint this message");
}

fn read_config() -> Config {
    let mut configs: Config = Config {dest: String::new(),
        framerate: String::new(),
        resolution: String::new(),
        audio_source: String::new(),
        mplayer: String::new(),
        ffmpeg_binary: String::new(),
        platform: String::new()};

    let config_table;
    if Path::new("config.toml").is_file() == true {
        config_table = fs::read_to_string("config.toml").expect("Cannot read config file").parse::<Table>().unwrap();
    }
    else {
        println!("Config file is missing");
        configs.ffmpeg_binary = String::from("ffmpeg");
        configs.mplayer = String::from("mpv");
        return configs
    }

    if config_table.is_empty() == true {
        println!("Config file is empty");
        configs.ffmpeg_binary = String::from("ffmpeg");
        configs.mplayer = String::from("mpv");
        return configs
    }
    
    if config_table.get("config") != None {
        if config_table["config"].get("dest") != None {
            configs.dest = String::from(config_table["config"].get("dest").unwrap().as_str().unwrap())
        }

        if config_table["config"].get("framerate") != None {
            configs.framerate = config_table["config"].get("framerate").unwrap().as_integer().unwrap().to_string()
        }

        if config_table["config"].get("resolution") != None {
            configs.resolution = String::from(config_table["config"].get("resolution").unwrap().as_str().unwrap())
        }

        if config_table["config"].get("mplayer") != None {
            configs.mplayer = String::from(config_table["config"].get("mplayer").unwrap().as_str().unwrap())
        }

        if config_table["config"].get("ffmpeg_binary") != None {
            configs.ffmpeg_binary = String::from(config_table["config"].get("ffmpeg_binary").unwrap().as_str().unwrap())
        }

        if config_table["config"].get("audio_source") != None {
            configs.audio_source = String::from(config_table["config"].get("audio_source").unwrap().as_str().unwrap())
        }
        
}

    println!("{:?}", configs); // DEBUG
    return configs
}