use std::env;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    if args.len() < 2 {
        println!("requires url argument bozo");
        return
    }

    println!("{}", &args[1]);

    let resp = Command::new("wget")
                .arg(args[1].clone())
                .output()
                .expect("wget command failed to start");

    println!("status: {}", &resp.status);
    println!("stdout: {}",  String::from_utf8_lossy(&resp.stdout));
    println!("stderr: {}",  String::from_utf8_lossy(&resp.stderr));
}
