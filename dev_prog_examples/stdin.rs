use std::net::{TcpStream};
use std::io::{Read, Write};
use std::io;
use std::str::from_utf8;

fn main() {
    let mut message = String::new();
    println!("Saisir votre message :");
    let mut buffer = std::io::stdin().read_line(&mut message).unwrap();
    println!("Votre prénom est : {}", message);
    println!("Taille du buffer à lire : {}", buffer);
    let msg_octet = message.as_bytes();

    match TcpStream::connect("localhost:25566") {
        Ok(mut stream) => {
            println!("Conneté au port 25566");
            stream.write(msg_octet).unwrap();
            println!("Message envoyé, en attente d'une réponse...");

           let mut data = [0; 12]; // using 12 byte buffer
           match stream.read(&mut data) {
               Ok(size) => {
                   if &data == msg_octet {
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
        }, Err(e) => {
            println!("Impossible de se connecter au serveur !");
        }
    }
    println!("fin");
}