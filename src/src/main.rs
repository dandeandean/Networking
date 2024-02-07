use std::io::{Read,Write};
use std::net::{TcpListener,TcpStream};
use std::fs;

fn send_response(mut stream: TcpStream ) {

    let file_path = "./src/res.txt";
    let file_contents = fs::read_to_string(file_path).unwrap();
    let response = file_contents.as_bytes();
    // let response = "HTTP/1.1 200 OK".as_bytes();
    stream.write(response).expect("Failed to send");
    println!("Sending: {}",file_contents)
}

fn handle_client(mut stream: TcpStream){
    // From: https://www.youtube.com/watch?v=JiuouCJQzSQ
    let mut buffer = [0;1024];
    stream.read(&mut buffer).expect("Failed to read");
    let _request = String::from_utf8_lossy(&buffer[..]);
    // println!("This is what I got: {}",request);
    println!("Got Something!");
    send_response(stream);
}


fn main() {
    let target = "127.0.0.1:42069";
    let listener = TcpListener::bind(target).expect("Faled to bind");
    println!("Now listening to {}",target);
    for stream in listener.incoming(){
        match stream{
            Ok(stream) =>{
                println!("okay");
                handle_client(stream);
            }
            Err(_stream) =>{
                println!("something horrible happened");
            }
        }
    }

}
