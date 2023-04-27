use std::{process::{Command, Stdio}, io, env, thread, fs, mem};
use toml::Table;

pub mod gui;

#[derive(Debug, PartialEq)]
struct Config {
    dest: String,
    framerate: i64,
    resolution: String,
}

fn main() {
    //thread::spawn(|| {gui::main_gui()});

    let mut dest: String = String::new();

    let configs: Option<Config> = read_config();
    if configs == None {
        drop(configs);
    }

    //Argument checking loop, all variables that are affected by it must be declared before this
    let args: Vec<String> = env::args().collect();
    for i in &args {
        if i == "--local" || i == "-l" {
            dest = String::from("local");
        }
        else if i == "-h" || i == "--help" {
            print_help();
            return
        }
    }

    if dest.trim() == "local" {
        dest = String::from("udp://127.0.0.1:9000");
    }
    else if dest.is_empty() {
        println!("Choose destination");
        io::stdin().read_line(&mut dest).expect("no");
    }
    else {
        panic!();
    }

    if dest.trim() == "q" {
        return
    }

    let platform: String = String::from(env::consts::OS);
    println!("Press h for help");

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
            invoke_ffmpeg(&platform, &String::from("native"), &String::from("60"), &dest.trim().to_string());
            break
        }
        else if input_mode.trim() == "p2" {
            invoke_ffmpeg(&platform, &String::from("1280x720"), &String::from("30"), &dest.trim().to_string());
            break
        }
        else if input_mode.trim() == "p3" {
            invoke_ffmpeg(&platform, &String::from("native"), &String::from("30"), &dest.trim().to_string());
            break
        }
        else if input_mode.trim() == "c" {
            query_args(&dest, &platform);
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

fn query_args(destination: &String, platform: &String) {
    let mut resolution: String = String::new();
    let mut framerate: String = String::new();

    println!("Choose a resolution");
    io::stdin().read_line(&mut resolution).expect("no");
    println!("Choose a framerate");
    io::stdin().read_line(&mut framerate).expect("no");
    invoke_ffmpeg(&platform, &resolution, &framerate, &destination);
}

fn invoke_ffmpeg(platform: &String, resolution: &String, framerate: &String, destination: &String) {
    let video_in;
    let binary;
    let mplayer;
    if platform == "linux" {
        video_in = "x11grab";
        binary = "ffmpeg";
        mplayer = "mpv";
    }
    else if platform == "windows" {
        video_in = "dshow";
        binary = "ffmpeg.exe";
        mplayer = "mpv.exe";
    }
    else {
        panic!("Platform unknown or not supported");
    }

    if resolution.trim() == "native" {
        if destination.trim() == "udp://127.0.0.1:9000" {
            Command::new(mplayer).arg(destination).arg("-profile=low-latency").stdout(Stdio::null()).spawn().expect("Cannot open mpv");
        }
        Command::new(binary).arg("-f").arg(video_in)
            .arg("-framerate").arg(framerate)
            .arg("-i").arg(":0")
            .arg("-c:v").arg("libx264").arg("-preset").arg("ultrafast").arg("-tune").arg("zerolatency")
            .arg("-f").arg("mpegts").arg(destination)
            .status().expect("Cannot open ffmpeg");
    }
    else {
        if destination.trim() == "udp://127.0.0.1:9000" {
            Command::new(mplayer).arg(destination).arg("-profile=low-latency").stdout(Stdio::null()).spawn().expect("Cannot open mpv");
        }
        Command::new(binary).arg("-f").arg(video_in)
            .arg("-framerate").arg(framerate)
            .arg("-i").arg(":0")
            .arg("-s").arg(resolution)
            .arg("-c:v").arg("libx264").arg("-preset").arg("ultrafast").arg("-tune").arg("zerolatency")
            .arg("-f").arg("mpegts").arg(destination)
            .status().expect("Cannot open ffmpeg");
    }
}

fn print_help() {
    println!("Sends, a simple application to stream video and audio to friends\n");
    println!(" -l, --local\t\tStream to udp://127.0.0.1:9000");
    println!(" -h, --help\t\tPrint this message");
}

fn read_config() -> Option<Config> {
    let config_table = fs::read_to_string("config.toml").expect("Cannot read config file").parse::<Table>().unwrap();

    if config_table.is_empty() == true {
        println!("Config file empty");
        return None
    }

    let mut configs: Config = Config {dest: String::new(),
        framerate: 0,
        resolution: String::new()};

    if config_table["config"].get("dest") != None {
        configs.dest = String::from(config_table["config"]["dest"].as_str()?)
    }
    if config_table["config"].get("framerate") != None {
        configs.framerate = config_table["config"]["framerate"].as_integer().unwrap()
    }
    if config_table["config"].get("resolution") != None {
        configs.resolution = String::from(config_table["config"]["resolution"].as_str()?)
    }

    println!("{:?}", configs);
    return Some(configs)
}