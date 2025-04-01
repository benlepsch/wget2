use std::env;
// use std::process::Command;
use reqwest;
use std::fs::File;
use std::io::prelude::*;

// fn cheater(path: &str) {
//     let resp = Command::new("wget")
//                 .arg(path)
//                 .output()
//                 .expect("wget command failed to start");

//     println!("status: {}", &resp.status);
//     println!("stdout: {}",  String::from_utf8_lossy(&resp.stdout));
//     println!("stderr: {}",  String::from_utf8_lossy(&resp.stderr));
// }

fn main() {
    let mut args: Vec<String> = env::args().collect();
    // dbg!(&args);

    if args.len() < 2 {
        panic!("no url in arguments");
    }

    let filename;

    // println!("args prefix: {}", &args[1][0..7]);
    if !["https:/", "http://"].contains(&&args[1][0..7]) {
    //    panic!("bad url: missing prefix");
        filename = args[1].clone();
        args[1] = "https://".to_owned() + &args[1];
    } else {
        // strip http or https
        if &args[1][4..5] == "s" {
            filename = args[1][8..].to_string();
        } else {
            filename = args[1][7..].to_string();
        }
    }

    // println!("{}", &args[1]);

    // wget 2 (version 1)
    // cheats and just runs the wget command from terminal
    // cheater(&args[1]);

    println!("connecting to {}", &args[1]);
    let webpage = reqwest::blocking::get(&args[1])
                .expect("is this the error message");

    // dbg!(&webpage);
    
    let body = webpage.text()
                .expect("if this fails im fucked");
    
    

    println!("saving to '{}' ", &filename);

    let mut write_out = File::create(filename)
                                .expect("cmon now");

    write_out.write_all(&body.as_bytes())
                .expect("sure hope that write worked");
}