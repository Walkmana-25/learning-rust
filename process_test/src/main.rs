
use std::{process::{Command, Stdio}, io::Read, thread::sleep, time::Duration};

fn main() {

  // Run echo add using /bin/sh
  let mut output = Command::new("/bin/sh")
    .arg("-c")
    .arg("touch tset")
    .stdin(Stdio::inherit())
    //.stdout(Stdio::piped())
    .spawn()
    .expect("failed to execute process");


    println!("{:?}", output);
    let time = Duration::from_secs(10);
    sleep(time);
    //output.wait().unwrap();
    println!("Ended");
    output.kill().unwrap();
}
