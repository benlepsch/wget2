use std::env;
use std::process::Command;
use reqwest;

fn cheater(path: &str) {
    let resp = Command::new("wget")
                .arg(path)
                .output()
                .expect("wget command failed to start");

    println!("status: {}", &resp.status);
    println!("stdout: {}",  String::from_utf8_lossy(&resp.stdout));
    println!("stderr: {}",  String::from_utf8_lossy(&resp.stderr));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    if args.len() < 2 {
        println!("requires url argument bozo");
        return
    }

    println!("{}", &args[1]);

    // wget 2 (version 1)
    // cheats and just runs the wget command from terminal
    // cheater(&args[1]);

    let body = reqwest::blocking::get(&args[1])
                .expect("is this the error message")
                .text();
    
    println!("{}", body.expect("why am i doing this"));
}