use std::net::{TcpStream};
use std::io::{Read, Write};
use std::str::from_utf8;

fn main() {
    match TcpStream::connect("localhost:25566") {
        Ok(mut stream) => {
            println!("Conneté au port 25566");

            let msg = b"Hello world!";

            // remplacer le unwrap()
            stream.write(msg).unwrap();
            println!("Message envoyé, en attente d'une réponse...");

            // augmenter la taille des données transmis
            let mut data = [0 as u8; 12]; // using 12 byte buffer
            match stream.read_exact(&mut data) {
                Ok(()) => {
                    if &data == msg {
                        println!("Reply ok!");
                    } else {
                        // remplacer le unwrap()
                        let text = from_utf8(&data).unwrap();
                        println!("Réponse innatendu : {}", text);
                    }
                },
                Err(e) => {
                    println!("Aucune réponse de reçu : {}", e);
                }
            }
        },
        Err(e) => {
            println!("Impossible de se connecter au port 25566 : {}", e);
        }
    }
    println!("Terminé");
}
