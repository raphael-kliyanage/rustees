// https://book.async.rs/tutorial/index.html

use std::net::{TcpStream};
use std::io::{Read, Write};
use std::thread;
use std::str::from_utf8;

// trait pour envoyer un message
// pub trait Message {
//     fn saisir_message(&self) -> u8;
//     fn envoyer_message(&self);
// }

fn saisir_pseudo() -> String {
    let mut pseudo = String::new();
    println!("Saisir votre pseudo : ");
    let _tmp = std::io::stdin().read_line(&mut pseudo).unwrap();
    println!("Bonjour {} !", pseudo);

    pseudo
}

fn saisir_message() -> String {
    let mut message = String::new();
    println!("Saisir votre message : ");
    let tampon = std::io::stdin().read_line(&mut message).unwrap();
    println!("Votre message est : {}", message);
    println!("Taille du message à lire : {}", tampon);
    
    message
}

fn main() {
    const BUFFER: usize = 512; // mem tampon à 512 octets
    let pseudo_client = saisir_pseudo();
    let pseudo_octet = pseudo_client.as_bytes();

    let mut socket = TcpStream::connect("localhost:25566");
    while match socket {
        Ok(ref mut socket) => {
            println!("Conneté au port 25566");
            let mut message_client = String::new();
            message_client = saisir_message();
            //let message_client = saisir_message();            
            let msg_octet = message_client.as_bytes();
            let tx: Vec<u8> = [msg_octet, b" > ", pseudo_octet].concat();
            socket.write(&tx).unwrap();
            println!("Message envoyé, en attente d'une réponse..."); 

            let mut trame = [0; BUFFER];
            match socket.read(&mut trame) {
                Ok(_size) => {
                    // code non pertinent pour le projet
                    // if &trame != msg_octet {
                    //     println!("Reply ok!");
                    // } else {
                    //     // remplacer le unwrap()
                    //     let msg_serveur = from_utf8(&trame).unwrap();
                    //     println!("Réponse innatendu : {}", msg_serveur);
                    // }
                    let rx = from_utf8(&trame).unwrap();
                    println!("Réponse serveur : {}", &rx);
                },
                Err(_e) => {
                    println!("Aucune réponse de reçu : {}", _e);
                }
            }
            true
        }, Err(ref _e) => {
            println!("Impossible de se connecter au serveur !");
            false
        }
    } {}
    // debug à supprimer à la fin du projet
    println!("fin");
}