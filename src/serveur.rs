// https://book.async.rs/tutorial/index.html
// https://thepacketgeek.com/rust/tcpstream/reading-and-writing/

use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
// use std::str::from_utf8; // pour voir ce que le serveur reçoit

fn handle_client(mut socket: TcpStream) {
    const BUFFER: usize = 512;  // mem tampon à 512 octets
    let mut data = [0 as u8; BUFFER];
    while match socket.read(&mut data) {
        Ok(size) => {
            // echo everything!
            // remplacer le unwrap()
            socket.write(&data[0..size]).unwrap();
            // voir ce que reçoit le serveur
            // let text = from_utf8(&data).unwrap();
            // println!("Client : {}", text);
            true
        },
        Err(_) => {
            println!("Une erreur est survenue, déconnexion du client : {}", 
                socket.peer_addr().unwrap());
            // remplacer le unwrap()
            socket.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

fn main() {
    // remplacer le unwrap()
    let listener1 = Vec::<TcpStream>::new();
    let listener = TcpListener::bind("0.0.0.0:25566").unwrap();
    // let listener_clone = listener.try_clone().unwrap();
    // accept connections and process them, spawning a new thread for each one
    println!("Serveur en écoute sur le port 25566");
    for socket in listener.incoming() {
        match socket {
            Ok(socket) => {
                // remplacer le unwrap()
                println!("Nouvelle connexion : {}",
                    socket.peer_addr().unwrap());
                /*let thread_socket = */thread::spawn(move || {
                    // connection succeeded
                    handle_client(socket)
                });
                // let res = thread_socket.join();
            }
            Err(e) => {
                println!("Echec de la connexion : {}", e);
            }
        }
    }
    drop(listener);
}
