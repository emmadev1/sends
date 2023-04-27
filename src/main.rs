use std::{process::{Command, Stdio}, io, env, thread};

pub mod gui;

fn main() {
    thread::spawn(|| {gui::main_gui()});

    let mut dest: String = String::new();
    println!("Choose destination");
    io::stdin().read_line(&mut dest).expect("no");

    if dest.trim() == "q" {
        return
    }
    else if dest.trim() == "local" {
        dest = String::from("udp://127.0.0.1:9000");
    }

    let platform: String = String::from(env::consts::OS);

    loop {
        let mut input_mode: String = String::new();
        println!("Press h for help");
        io::stdin().read_line(&mut input_mode).expect("no");

        if input_mode.trim() == "h" {
            println!("p1 (Preset 1): Native resolution at 60 fps \np2 (Preset 2): 720p resolution at 30 fps \np3 (Preset 3): Native resolution at 30 fps");
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