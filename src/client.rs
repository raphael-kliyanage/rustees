use std::net::{TcpStream};
use std::io::{Read, Write};
use std::str::from_utf8;

// trait pour envoyer un message
// pub trait Message {
//     fn saisir_message(&self) -> u8;
//     fn envoyer_message(&self);
// }

fn main() {
    let mut message = String::new();
    println!("Saisir votre message :");
    let buffer = std::io::stdin().read_line(&mut message).unwrap();
    println!("Votre prénom est : {}", message);
    println!("Taille du buffer à lire : {}", buffer);
    let msg_octet = message.as_bytes();

    match TcpStream::connect("localhost:25566") {
        Ok(mut stream) => {
            println!("Conneté au port 25566");
            stream.write(msg_octet).unwrap();
            println!("Message envoyé, en attente d'une réponse...");

           let mut data = [0; 512]; // using 12 byte buffer
           match stream.read(&mut data) {
                // [bug] si le message ne fait pas 12 octets de buffer
                // alors c'est le else qui est pris en compte
                // mais le serveur spam le réponse
                Ok(_size) => {
                    if &data == msg_octet {
                        println!("Reply ok!");
                    } else {
                        // remplacer le unwrap()
                        let text = from_utf8(&data).unwrap();
                        println!("Réponse innatendu : {}", text);
                    }
                },
                Err(_e) => {
                    println!("Aucune réponse de reçu : {}", _e);
                }
            }
        }, Err(_e) => {
            println!("Impossible de se connecter au serveur !");
        }
    }
    println!("fin");
}