use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    loop {
        sleep(Duration::from_secs(2));
        clear_console();
        println!("Welcome to Jvm Operating System");

        let result = Command::new("/jre/bin/java")
            .arg("-Dlogback.configurationFile=/etc/logback.xml")
            .arg("--module-path")
            .arg("engine")
            .arg("-m")
            .arg("jos.core")
            .arg("ir.moke.jos.core.AppRunner")
            .spawn();

        match result {
            Ok(mut child) => {
                let _w = child.wait();
            },
            Err(_err)  => println!("{}", _err.to_string()),
        }
    }
}

fn clear_console() {
    print!("\x1B[2J\x1B[1;1H");
}
