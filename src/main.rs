use std::{process::{Command, Stdio}, io, env, thread, fs, path::Path};
use toml::Table;

#[derive(Debug, PartialEq)]
struct Config {
    platform: String,
    config_path: String,
    dest: String,
    framerate: String,
    resolution: String,
    audio_source: String,
    mplayer: String,
    ffmpeg_binary: String,
    record_video: bool,
    record_audio: bool,
    default_pulse_sink: String,
    enable_pulse_hack: bool,
    playback: bool,
}

fn main() {
    //thread::spawn(|| {gui::main_gui()});
    let mut configs_init: Config = Config {platform: String::from(env::consts::OS),
        config_path: String::new(),
        dest: String::new(),
        framerate: String::new(),
        resolution: String::new(),
        audio_source: String::new(),
        mplayer: String::new(),
        ffmpeg_binary: String::new(),
        record_audio: true,
        record_video: true,
        default_pulse_sink: String::new(),
        enable_pulse_hack: false,
        playback: false};

    let args: Vec<String> = env::args().collect();
    let mut e: usize = 0;

    //Argument checking loop, all variables that are affected by it must be declared before this
    for i in &args {
        if i == "--local" || i == "-l" {
            configs_init.dest = String::from("local");
        }
        else if i == "-h" || i == "--help" {
            print_help();
            return
        }
        else if i == "-p" || i == "--pulse" {
            configs_init.enable_pulse_hack = true;
        }
        else if i == "-c" || i == "--config" {
            if args.get(e + 1) != None {
                configs_init.config_path = String::from(&args[e + 1]);
            }
        }
        e += 1;
    }

    let mut configs: Config = read_config(configs_init);

    if configs.enable_pulse_hack == true || configs.platform == "Linux" {
        pulse_setup(&configs);
    }


    if configs.dest.is_empty() { // We check if its empty again to ensure we have a destination
        println!("Destination is missing, exiting");
        return
    }

    if configs.dest == "local" {
        configs.dest = String::from("udp://127.0.0.1:9000");
    }

    println!("{:?}", configs); // DEBUG

    if configs.resolution.is_empty() || configs.framerate.is_empty() || configs.audio_source.is_empty() {
        println!("Missing key configuration parameters, exiting");
        return
    }
    else {
        if configs.platform == "linux" {
            invoke_ffmpeg_linux(configs);
        }
        else if configs.platform == "windows" {
            invoke_ffmpeg_windows(configs);
        }
        else {
            panic!("Unknown or unsupported platform");
        }
    }
}

fn pulse_setup(configs: &Config) {
    let sink = &configs.default_pulse_sink;
    let mut init = String::from("slaves=");
    init += &sink;
    Command::new("pactl").arg("load-module").arg("module-combine-sink").arg("sink_name=sends").arg(init).spawn().expect("Cannot run pactl");
}

fn invoke_ffmpeg_linux(configs: Config) {
    if configs.resolution.trim() == "native" {
        if configs.dest.trim() == "udp://127.0.0.1:9000" || configs.playback == true {
            Command::new(configs.mplayer).arg(&configs.dest).arg("-profile=low-latency").stdout(Stdio::null()).spawn().expect("Cannot open {configs.mplayer}");
        }
        Command::new(configs.ffmpeg_binary).arg("-f").arg("x11grab")
            .arg("-framerate").arg(configs.framerate)
            .arg("-i").arg(":0")
            .arg("-f").arg("pulse")
            .arg("-i").arg(configs.audio_source)
            .arg("-c:v").arg("libx264").arg("-preset").arg("ultrafast").arg("-tune").arg("zerolatency")
            .arg("-f").arg("mpegts").arg(&configs.dest)
            .status().expect("Cannot open ffmpeg");
    }
    else {
        if configs.dest.trim() == "udp://127.0.0.1:9000" {
            Command::new(configs.mplayer).arg(&configs.dest).arg("-profile=low-latency").stdout(Stdio::null()).spawn().expect("Cannot open {configs.mplayer}");
        }
        Command::new(configs.ffmpeg_binary).arg("-f").arg("x11grab")
            .arg("-framerate").arg(configs.framerate)
            .arg("-s").arg(configs.resolution)
            .arg("-i").arg(":0")
            .arg("-f").arg("pulse")
            .arg("-i").arg(configs.audio_source)
            .arg("-c:v").arg("libx264").arg("-preset").arg("ultrafast").arg("-tune").arg("zerolatency")
            .arg("-f").arg("mpegts").arg(&configs.dest)
            .status().expect("Cannot open ffmpeg");
    }
}

fn invoke_ffmpeg_windows(configs: Config) {
    println!("not yet");
}

fn print_help() {
    println!("Sends, a simple application to stream video and audio to friends\n");
    println!(" -l, --local\t\tStream to udp://127.0.0.1:9000");
    println!(" -c, --config\t\tPath to config file");
    println!(" -p, --pulse\t\tUse pulseauduio's module-combine-sink");
    println!(" -h, --help\t\tPrint this message");
}

fn read_config(mut configs: Config) -> Config {
    let config_table;

    if Path::new(&configs.config_path).is_file() == true {
        config_table = fs::read_to_string(&configs.config_path).expect("Cannot read config file").parse::<Table>().unwrap();
    }
    else if Path::new("config.toml").is_file() == true {
        config_table = fs::read_to_string("config.toml").expect("Cannot read config file").parse::<Table>().unwrap();
    } 
    else if Path::new("config/config.toml").is_file() == true {
        config_table = fs::read_to_string("config/config.toml").expect("Cannot read config file").parse::<Table>().unwrap();
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
        if config_table["config"].get("dest") != None && configs.dest.is_empty() == true {
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
        if config_table["config"].get("default_pulse_sink") != None {
            configs.default_pulse_sink = String::from(config_table["config"].get("default_pulse_sink").unwrap().as_str().unwrap())
        }
    }

    println!("{:?}", configs); // DEBUG
    return configs
}
