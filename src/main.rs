use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    loop {
        sleep(Duration::from_secs(2));
        clear_console();
        println!("Welcome to Jvm Operating System");

        let result = Command::new("/jre/bin/java")
            .arg("-jar")
            .arg("/jos-engine.jar")
            .spawn();

        match result {
            Ok(mut child) => {
                let _w = child.wait();
            },
            Err(_err)  => println!("Error")
        }
    }
}

fn clear_console() {
    print!("\x1B[2J\x1B[1;1H");
}
