use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
// use std::str::from_utf8; // pour voir ce que le serveur reçoit

// https://thepacketgeek.com/rust/tcpstream/reading-and-writing/

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
    let listener = TcpListener::bind("0.0.0.0:25566").unwrap();
    // accept connections and process them, spawning a new thread for each one
    println!("Serveur en écoute sur le port 25566");
    for socket in listener.incoming() {
        match socket {
            Ok(socket) => {
                // remplacer le unwrap()
                println!("Nouvelle connexion : {}",
                    socket.peer_addr().unwrap());
                thread::spawn(move|| {
                    // connection succeeded
                    handle_client(socket)
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
