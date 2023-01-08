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
    let tampon = std::io::stdin().read_line(&mut message).unwrap();
    println!("Votre prénom est : {}", message);
    println!("Taille du tampon à lire : {}", tampon);
    let msg_octet = message.as_bytes();

    match TcpStream::connect("localhost:25566") {
        Ok(mut socket) => {
            println!("Conneté au port 25566");
            socket.write(msg_octet).unwrap();
            println!("Message envoyé, en attente d'une réponse...");

           let mut trame = [0; 512]; // mem tampon/buffer à 512 octets
           match socket.read(&mut trame) {
                // [bug] si le message ne fait pas 12 octets de buffer
                // alors c'est le else qui est pris en compte
                // mais le serveur spam le réponse
                Ok(_size) => {
                    // if &trame != msg_octet {
                    //     println!("Reply ok!");
                    // } else {
                    //     // remplacer le unwrap()
                    //     let msg_serveur = from_utf8(&trame).unwrap();
                    //     println!("Réponse innatendu : {}", msg_serveur);
                    // }
                    let reponse_serveur = from_utf8(&trame).unwrap();
                    println!("Réponse serveur : {}", reponse_serveur);
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