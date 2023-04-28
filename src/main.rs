use std::{process::{Command, Stdio}, io, env, thread, fs, path::Path};
use toml::Table;

pub mod gui;

#[derive(Debug, PartialEq)]
struct Config {
    dest: String,
    framerate: String,
    resolution: String,
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

    if configs.dest.is_empty() {
        println!("Choose destination");
        let mut dest: String = String::new();
        io::stdin().read_line(&mut dest).expect("no");
        configs.dest = String::from(dest.trim());
    }

    if configs.dest.is_empty() { // We check if its empty again to ensure we have a destination
        println!("Destination is empty, defaulting to local");
        configs.dest = String::from("udp://127.0.0.1:9000");
    }

    if configs.dest.trim() == "q" {
        println!("Quitting");
        return
    } else if configs.dest.trim() == "local" {
        configs.dest = String::from("udp://127.0.0.1:9000");
    }

    configs.platform = String::from(env::consts::OS); // Moving this to another place to group it with the other definitions would be nice
    println!("Press h for help");
    println!("{:?}", configs); // DEBUG

    loop {
        let mut input_mode: String = String::new();
        io::stdin().read_line(&mut input_mode).expect("no");

        if input_mode.trim() == "h" {
            println!("p1 (Preset 1): Native resolution at 60 fps");
            println!("p2 (Preset 2): 720p resolution at 30 fps");
            println!("p3 (Preset 3): Native resolution at 30 fps");
            println!("c (Custom): Custom settings for resolution and framerate");
        }
        else if input_mode.trim() == "p1" {
            configs.resolution = String::from("native");
            configs.framerate = String::from("60");
            println!("{:?}", configs); // DEBUG
            invoke_ffmpeg(configs);
            break
        }
        else if input_mode.trim() == "p2" {
            configs.resolution = String::from("1280x720");
            configs.framerate = String::from("30");
            println!("{:?}", configs); // DEBUG
            invoke_ffmpeg(configs);
            break
        }
        else if input_mode.trim() == "p3" {
            configs.resolution = String::from("native");
            configs.framerate = String::from("30");
            println!("{:?}", configs); // DEBUG
            invoke_ffmpeg(configs);
            break
        }
        else if input_mode.trim() == "c" {
            query_args(configs);
            break
        }
        else if input_mode.trim() == "q" {
            println!("Quitting");
            break
        }
        else {
            println!("Choose a valid mode of operation");
        }
    }
}

fn query_args(configs: Config) {
    let mut resolution: String = String::new();
    let mut framerate: String = String::new();
    let mut configs = configs;

    println!("Choose a resolution");
    io::stdin().read_line(&mut resolution).expect("no");
    configs.resolution = resolution;
    println!("Choose a framerate");
    io::stdin().read_line(&mut framerate).expect("no");
    configs.framerate = framerate;
    invoke_ffmpeg(configs);
}

fn invoke_ffmpeg(configs: Config) {
    let video_in;
    if configs.platform == "linux" {
        video_in = "x11grab";
    }
    else if configs.platform == "windows" {
        video_in = "dshow";
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
            .arg("-i").arg(":0")
            .arg("-s").arg(configs.resolution)
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
        
}

    println!("{:?}", configs); // DEBUG
    return configs
}