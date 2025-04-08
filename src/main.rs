use std::env;
// use std::process::Command;
use reqwest;
use std::fs;
use std::io::{stdin, Write};

// fn cheater(path: &str) {
//     let resp = Command::new("wget")
//                 .arg(path)
//                 .output()
//                 .expect("wget command failed to start");

//     println!("status: {}", &resp.status);
//     println!("stdout: {}",  String::from_utf8_lossy(&resp.stdout));
//     println!("stderr: {}",  String::from_utf8_lossy(&resp.stderr));
// }

fn path_exists(path: &str) -> bool {
    fs::metadata(path).is_ok()
}

fn main() {
    let mut args: Vec<String> = env::args().collect();
    // dbg!(&args);

    if args.len() < 2 {
        println!("no url in arguments, please enter the url to fetch:");

        let mut inp_str = String::new();
        stdin().read_line(&mut inp_str)
            .expect("Failed to read input");
        
        args.push(inp_str[0..(inp_str.len()-1)].to_string());
    }

    let mut filename = String::new();

    // basically what i want to do here 
    // filename = 'index.html' if not ('/' in list(args[1]) else args[1].split('/')[1]

    // strip http/https if we have it
    if ["https:/", "http://"].contains(&&args[1][0..7]) {
        if &args[1][4..5] == "s" {
            filename = args[1][8..].to_string();
        } else {
            filename = args[1][7..].to_string();
        }
    } else {
        // if we don't have the header, add it to the url
        filename = args[1].clone();
        args[1] = "https://".to_owned() + &args[1];
    }

    // check for '/'
    println!("Filename: {}", &filename);
    if filename.contains("/") {
        // set filename to string contents after the last '/'
        for (i, c) in filename.chars().rev().enumerate() {
            if c == '/' {
                filename = filename[i..filename.len()].to_string();
                break;
            }
        }
    } else {
        filename = "index.html".to_string();
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
    
    // check if filename exists
    // i can feel the type errors already
    println!("using filename {}", &filename);
    let mut modded = false;
    let mut n = 2;
    while path_exists(&filename) {
        if !modded {
            filename = format!("{} ({})", filename, 1);
            modded = true;
        } else {
            filename = format!("{} ({})", filename[0..(filename.len()-4)].to_string(), n);
            n += 1;
        } 
        
        println!("filename taken, using filename {}", &filename);
    }

    println!("saving to '{}' ", &filename);

    let mut write_out = fs::File::create(filename)
                                .expect("cmon now");

    write_out.write_all(&body.as_bytes())
                .expect("sure hope that write worked");
}