use std::io::{Read,Write};
use std::net::{TcpListener,TcpStream};

fn handle_client(mut stream: TcpStream){
    // From: https://www.youtube.com/watch?v=JiuouCJQzSQ
    let mut buffer = [0;1024];
    stream.read(&mut buffer).expect("Failed to read");
    let request = String::from_utf8_lossy(&buffer[..]);
    println!("This is what I got: {}",request);
    let response = "Hello You!".as_bytes();
    stream.write(response).expect("Failed to send");
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080")
        .expect("Faled to bind");
    println!("Now listening to 127.0.0.1:8080");
    for stream in listener.incoming(){
        match stream{
            Ok(stream) =>{
                println!("okay");
                std::thread::spawn(|| handle_client(stream));
            }
            Err(_stream) =>{
                println!("something horrible happened");
            }
        }
    }

}
