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
    let mut args: Vec<String> = env::args().collect();
    dbg!(&args);

    if args.len() < 2 {
        panic!("no url in arguments");
    }

    // println!("args prefix: {}", &args[1][0..7]);
    if !["https:/", "http://"].contains(&&args[1][0..7]) {
    //    panic!("bad url: missing prefix");
        args[1] = "https://".to_owned() + &args[1];
    }

    // println!("{}", &args[1]);

    // wget 2 (version 1)
    // cheats and just runs the wget command from terminal
    // cheater(&args[1]);

    let body = reqwest::blocking::get(&args[1])
                .expect("is this the error message")
                .text();
    
    println!("{}", body.expect("why am i doing this"));
}