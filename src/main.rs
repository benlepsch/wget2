use std::net::TcpStream;

use std::io::prelude::*;
use std::io::{Result, stdin, Write};

use std::fs;
use std::env;

#[allow(non_snake_case)]
mod HttpRequest;

fn path_exists(path: &str) -> bool {
    fs::metadata(path).is_ok()
}

fn fetch_data(url: String, path: String) -> Result<String> {
    let ip_addr = format!("{}{}", url, ":80");
    println!("using addr {}", ip_addr);
    let mut stream = TcpStream::connect(&ip_addr)?;   
    println!("connected to {ip_addr}");

    println!("Building GET request");
    let req = HttpRequest::HttpRequest::new(
        HttpRequest::MethodKind::GET, path, None);

    println!("Sending request to server");
    let _ = stream.write(&req.serialize());

    println!("Reading reply");
    /*
        headers = []
        while last two bytes are not "\r\n" {
            current = ''
            while last two bytes are not "\r\n" {
                push onto current
            }
            push to headers
            read out the next two bytes
        }
    */
    let mut headers = Vec::new();
    let mut tmp = [0; 1];
    let mut last: u8;
    
    stream.read(&mut tmp).expect("someting wrong");
    last = tmp[0];
    stream.read(&mut tmp).expect("somethign wrong");

    // "\r\n" = 0x0d 0x0a
    while last != 13 && tmp[0] != 10 {
        let mut current = String::new();

        while last != 13 && tmp[0] != 10 {
            current.push(last as char);
            last = tmp[0];
            stream.read(&mut tmp).expect("something wrong");
        }

        headers.push(current);

        // 0x0d 0x0a 0x.. 0x.. 0x.. 
        // last tmp 
        stream.read(&mut tmp).expect("something wrong");
        last = tmp[0];
        stream.read(&mut tmp).expect("somethg wrong");
    }
    
    // println!("done");
    // dbg!(&headers);

    let _http_resp = headers[0].clone();
    headers.remove(0);

    let header_map: std::collections::HashMap<&str, &str> = headers.iter()
        .map(|header| {
            let mut split = header.split(": ");
            (split.next().unwrap(), split.next().unwrap())
        })
        .collect();

    // dbg!(&header_map);
    let msg_length = header_map["Content-Length"].parse::<usize>().unwrap();
    let mut buf = vec![0; msg_length];
    stream.read(&mut buf).expect("somethign wrong (last)");

    let to_str = std::str::from_utf8(&buf).unwrap().to_string();
    Ok(to_str)
}

fn main() {
    let mut args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("no url in arguments, please enter the url to fetch:");

        let mut inp_str = String::new();
        stdin().read_line(&mut inp_str)
            .expect("Failed to read input");
        
        args.push(inp_str[0..(inp_str.len()-1)].to_string());
    }

    let mut filename = args[1].clone();

    if filename.contains("/") {
        // set filename to string contents after the last '/'
        for (i, c) in filename.chars().rev().enumerate() {
            if c == '/' {
                filename = filename[(i+1)..filename.len()].to_string();
                filename = format!("{}.html", filename);
                break;
            }
        }
    } else {
        filename = "index.html".to_string();
    }

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

    // separate url from path
    let mut url = String::new();
    let mut path = "".to_string();
    let mut done_url = false;

    for part in args[1].split("/") {
        if !done_url {
            url = part.to_string();
            done_url = true;
        } else {
            path = format!("{}/{}", path, part);
        }
    }

    let html = fetch_data(url, path).unwrap();

    println!("saving to '{}' ", &filename);

    let mut write_out = fs::File::create(filename)
                                .expect("cmon now");

    write_out.write_all(&html.as_bytes())
                .expect("sure hope that write worked");
}
