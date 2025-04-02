use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use std::{env, process::Command, time::Duration};


const ASCII_ART: &str = r#"
      ⬤⬤⬤      ⬤  
     ⬤   ⬤     ⬤  
     ⬤   ⬤     ⬤  
     ⬤   ⬤     ⬤  
      ⬤⬤⬤      ⬤  

    -- 01Edu Docker Test --
"#;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    exercise: String,
    
    #[arg(short, long)]
    language: String,

}

fn main() {
    print!("{esc}c", esc = 27 as char);
    println!("{}", ASCII_ART);
    let args = Args::parse();

    let current_dir: String = env::current_dir()
        .expect("Failed to get current directory")
        .to_str()
        .unwrap()
        .to_string();

    let (docker_image, mount_path) = match args.language.as_str() {
        "java" => {

            let path = format!("{}/student:/app/student", current_dir);

             (
                "ghcr.io/01-edu/test-java:latest", 
                path
            )
        },
        "dart" => {
            let path = format!("{}/{}.dart:/jail/student/{}.dart", current_dir, args.exercise, args.exercise);

            (
                "ghcr.io/01-edu/test-dart:latest", 
                path
            )
        },
        _ => {
            println!("Unknown exercise selected");
            return;
        }
    };

    // Check if the image exists
    let image_exists = images_exist(docker_image);

    if !image_exists {
        println!("Image does not exist, pulling it...");
        pull_image(docker_image);
    }

    let status = Command::new("docker")
    .args(&[
        "run",
        "--rm",
        "-it",
        "-e",
        &format!("EXERCISE={}", args.exercise),
        "-v",
        &mount_path,
        docker_image,
    ])
    .status()
    .expect("Failed to execute command");

    if !status.success() {
        eprintln!("Error running Docker container");
    } else {
        println!("Docker container ran successfully");
    }
}

fn images_exist(image: &str) -> bool {
    let output = Command::new("docker")
        .args(&["images", "-q", image])
        .output()
        .expect("Failed to execute command");

    return !output.stdout.is_empty();
}   

fn pull_image(image: &str) {
    let pb = ProgressBar::new_spinner();
    pb.set_style(ProgressStyle::default_spinner()
        .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
        .template("{spinner} Downloading image... {msg}").unwrap());
    pb.enable_steady_tick(Duration::from_millis(100));
    
    let output = Command::new("docker")
        .args(&["pull", image])
        .output()
        .expect("Failed to execute command");
    
    pb.finish_with_message("Image pulled successfully!");
    
    if !output.status.success() {
        eprintln!("Error pulling image: {}", String::from_utf8_lossy(&output.stderr));
    } else {
        println!("Image pulled successfully");
    }
}