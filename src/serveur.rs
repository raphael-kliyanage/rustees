use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::str::from_utf8;

// https://thepacketgeek.com/rust/tcpstream/reading-and-writing/

fn handle_client(mut stream: TcpStream) {
    let mut data = [0 as u8; 50]; // using 50 byte buffer
    while match stream.read(&mut data) {
        Ok(size) => {
            // echo everything!
            // remplacer le unwrap()
            stream.write(&data[0..size]).unwrap();
            let text = from_utf8(&data).unwrap();
            println!("Client : {}", text);
            true
        },
        Err(_) => {
            println!("An error occurred, terminating connection with {}", 
                stream.peer_addr().unwrap());
            // remplacer le unwrap()
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

fn main() {
    // remplacer le unwrap()
    let listener = TcpListener::bind("0.0.0.0:25566").unwrap();
    // accept connections and process them, spawning a new thread for each one
    println!("Serveur en écoute sur le port 25566");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // remplacer le unwrap()
                println!("Nouvelle connexion : {}",
                    stream.peer_addr().unwrap());
                thread::spawn(move|| {
                    // connection succeeded
                    handle_client(stream)
                });
            }
            Err(e) => {
                println!("Echec de la connexion : {}", e);
                /* connection failed */
            }
        }
    }
    // close the socket server
    drop(listener);
}
