use std::net::{TcpStream};
use std::io::{Read, Write};
use std::str::from_utf8;

fn main() {
    match TcpStream::connect("localhost:25566") {
        Ok(mut stream) => {
            println!("Conneté au port 25566");

            let msg = b"Hello!";

            stream.write(msg).unwrap();
            println!("Message envoyé, en attente d'une réponse...");

            let mut data = [0 as u8; 6]; // using 6 byte buffer
            match stream.read_exact(&mut data) {
                Ok(_) => {
                    if &data == msg {
                        println!("Reply ok!");
                    } else {
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
