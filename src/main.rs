use std::{process::{Command, Stdio}, io, env};

fn main() {
    let mut input_mode: String = String::new();
    println!("Press h for help");
    io::stdin().read_line(&mut input_mode).expect("no");

    if input_mode.trim() == "h" {
        println!("p1 (Preset 1): Native resolution at 60 fps \np2 (Preset 2): 720p resolution at 30 fps \np3 (Preset 3): Native resolution at 30 fps");
        main();
    }
    else if input_mode.trim() == "p1" {
        invoke_ffmpeg(&String::from("native"), &String::from("60"), &String::from("test.mp4"));
    }
    else if input_mode.trim() == "p2" {
        invoke_ffmpeg(&String::from("1280x720"), &String::from("30"), &String::from("test.mp4"));
    }
    else if input_mode.trim() == "p3" {
        invoke_ffmpeg(&String::from("native"), &String::from("30"), &String::from("test.mp4"));
    }
    else if input_mode.trim() == "c" {
        query_args();
    }
    else if input_mode.trim() == "q" {
        println!("Quitting");
    }
    else {
        println!("Choose a valid mode of operation");
    }
}

fn query_args() {
    let mut resolution: String = String::new();
    let mut framerate: String = String::new();
    let mut destination: String = String::new();
    println!("Choose a resolution");
    io::stdin().read_line(&mut resolution).expect("no");
    println!("Choose a framerate");
    io::stdin().read_line(&mut framerate).expect("no");
    println!("Choose a destination");
    io::stdin().read_line(&mut destination).expect("no");
    invoke_ffmpeg(&resolution, &framerate, &destination);
}

fn invoke_ffmpeg(resolution: &String, framerate: &String, destination: &String) {
    if resolution.trim() == "native" {
        Command::new("ffmpeg").arg("-f").arg("x11grab")
            .arg("-framerate").arg(framerate)
            .arg("-i").arg(":0")
            .arg("-c:v").arg("libx264").arg("-preset").arg("ultrafast").arg("-tune").arg("zerolatency")
            .arg("-f").arg("flv").arg(destination)
            .status().expect("Cannot open ffmpeg");
    }
    else {
        Command::new("ffmpeg").arg("-f").arg("x11grab")
            .arg("-framerate").arg(framerate)
            .arg("-i").arg(":0")
            .arg("-vf").arg(String::from("resize=") + resolution)
            .arg("-c:v").arg("libx264").arg("-preset").arg("ultrafast").arg("-tune").arg("zerolatency")
            .arg("-f").arg("flv").arg(destination)
            .status().expect("Cannot open ffmpeg");
    }
}