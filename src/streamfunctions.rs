use std::process::Command;
use std::process::Output;
use std::process::Stdio;
use std::io::Write;
use std::io::Read;
use image::io::Reader as ImageReader;
use image::DynamicImage;


//create a macro that's like "unwrap or continue"
//so that you can do something like
//let x = unwrap_or_continue!(somefunction());

macro_rules! try_unwrap_or_continue {
    ($expr:expr) => {
        match $expr {
            Ok(val) => val,
            Err(_) => continue,
        }
    }
}


use rand::Rng;


pub async fn get_average_audio_levels( stream: &str, seconds: f32) -> f32{

    let output: std::process::Output = Command::new("ffmpeg")
        .arg("-i")
        .arg(stream)
        .arg("-t")
        .arg( format!("{}",seconds) )
        .arg("-filter:a")
        .arg("volumedetect")
        .arg("-f")
        .arg("null")
        .arg("/dev/null")
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    // println!("STDERR: {}", stderr);

    // ] mean_volume: -13.5 dB

    let regex = regex::Regex::new(r"mean_volume: -(\d+.\d+) dB").unwrap();

    let caps = regex.captures(&stderr).unwrap();

    let volume = caps.get(1).unwrap().as_str();

    println!("volume {}", volume);

    volume.parse::<f32>().unwrap()

}


//also train
pub async fn get_stream_as_images( ip: &str, seconds: f32, fps: f32 ) -> Vec<DynamicImage>{


    std::fs::create_dir(&"./screenshots/");


    // Generate a random string
    let random_string: String = rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(10)
        .map(char::from)
        .collect();

    // Print the random string
    println!("Generated random string: {}", random_string);


    let output_folder = format!("./screenshots/{}/", random_string);


    // Use the random string to create a directory
    match std::fs::create_dir(&output_folder) {
        Ok(_) => println!("Created a directory with the name: {}", random_string),
        Err(err) => println!("Failed to create a directory: {}", err),
    }




    let video_file = "test.webm";




    let status = Command::new("ffmpeg")
        .arg("-i")
        .arg(video_file)
        .arg("-t")
        .arg( format!("{}",seconds) )
        .arg("-vf")
        .arg(format!("fps={}", fps))
        .arg(format!("{}out%04d.png", output_folder))
        .status()
        .expect("Failed to execute command");

    println!("Process exited with status: {}", status);

    //async wait 1 second   
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;


    let mut toreturn = Vec::new();

    //get a vector of the filenames in the directory
    let filesindirectory = std::fs::read_dir(output_folder).unwrap();
    let mut filesindirectory = filesindirectory.map(|x| x.unwrap().path() ).collect::<Vec<_>>();
    //sort the vector of filenames
    filesindirectory.sort_by(|a, b| a.file_name().cmp(&b.file_name()));

    for path in filesindirectory {
        let path_str = path.to_str().unwrap();

        let img = try_unwrap_or_continue!( try_unwrap_or_continue!( ImageReader::open( path_str ) ).decode() );

        toreturn.push(img);
    }

    return toreturn;
}