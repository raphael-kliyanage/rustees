// https://book.async.rs/tutorial/index.html

// use std::net::{TcpStream};
// use std::io::{Read, Write, ErrorKind};
// use std::sync::mpsc;
// use mpsc::TryRecvError;
// use std::time::Duration;
// use std::thread;
use age::{Recipient, DecryptError, EncryptError};
use std::str::FromStr;
use std::iter;
use mpsc::TryRecvError;
use age;
use age::x25519::Identity;
use std::{
    io::{self, ErrorKind, Read, Write},
    net::TcpStream,
    sync::mpsc,
    thread,
    time::Duration,
};

// trait pour envoyer un message
// pub trait Message {
//     fn saisir_message(&self) -> u8;
//     fn envoyer_message(&self);
// }

fn sleep() {
    thread::sleep(::std::time::Duration::from_millis(100));
}

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



pub fn generation_des_cles( )-> Identity
{
    let key = age::x25519::Identity::generate();
    key
}

pub fn chiffrement_message(message:String,key_public:Box<dyn Recipient +Send>) -> String
{
    // Chiffre le message clair  en message chiffré

        let encryptor = age::Encryptor::with_recipients(vec![key_public])
            .expect("we provided a recipient");

        let mut encrypted = vec![];
        let mut writer = encryptor.wrap_output(&mut encrypted).expect("Chiffrement Impossible");
        writer.write_all(message.as_bytes()).expect("Impossible d'ecrire le message !");
        writer.finish().expect("Impossible de finaliser le Chiffrement");

        hex::encode(encrypted)
}

// déchiffre le message chiffré obtenu en message clair
pub fn dechiffrement_message(message:String, key_prive:Identity) -> Option<String>
{
    let message = hex::decode(message).unwrap();
    let decryptor = match age::Decryptor::new(&message[..]).expect("Impossible d'intialiser le Déchiffrement"){
        age::Decryptor::Recipients(d) => d,
        _ => unreachable!(),
    };

    let mut decrypted = vec![];
    let mut reader = match decryptor.decrypt(iter::once(&key_prive as &dyn age::Identity)) {
        Ok(data) => data,
        Err(e) => {
            match e {
                DecryptError::NoMatchingKeys => return None,
                _ => panic!("{}", e)
            }
        }
    };
    reader.read_to_end(&mut decrypted).expect("Impossible de lire le contenu du message! ");
    Some(std::str::from_utf8(&decrypted).expect("Impossible de convertir le vecteur en string").to_string())
}





fn main() {
//    const BUFFER: usize = 512; // mem tampon à 512 octets
//    let pseudo_client = saisir_pseudo();
//    let pseudo_octet = pseudo_client.as_bytes();

//    let mut socket = TcpStream::connect("localhost:25566");
    let mut client = TcpStream::connect("localhost:25566")
        .expect("Stream failed to connect");
    client
        .set_nonblocking(true)
        .expect("failed to initiate non-blocking");

    let (tx, rx) = mpsc::channel::<String>();
    let key = generation_des_cles();
    // L'affichage de la clé publique de l'utilisateur !
    println!("Voici la clé publique {} ",key.to_public());
    // Récuperation de la clé destinataire
    println!("Saisir la clé publique destinataire !");
    // Stocke la clé en format string et on la converti en format Recipient
    let mut key_str = String::new();
    std::io::stdin().read_line(&mut key_str).unwrap();
    // enlever le /n
    let key_str = &key_str[0..key_str.len()-1];
    let key_dest = age::x25519::Recipient::from_str(&key_str).unwrap();

    thread::spawn(move || loop {
        const BUFF_SIZE: usize = 4096;
        // Buffer temporaire (morceaux du message)
        let mut buff = vec![0; BUFF_SIZE];
        // Message chiffré final
        let mut encrypted_msg: Vec<u8> = Vec::new();
        let mut skip: bool = true;
        // Lire les paquets tant qu'on a pas le message complet
        loop {
            match client.read(&mut buff) {
                Ok(msg_len) => {
                    if msg_len == BUFF_SIZE {
                        encrypted_msg.append(&mut buff);
                        skip = false;
                        break;
                    } else if msg_len != 0 {
                        encrypted_msg.append(&mut buff[..msg_len].to_vec());
                        skip = false;
                        break;
                    } else {
                        break;
                    }
                },
                Err(_) => break
            }
        }

        // Lancement du déchiffrement du message
        if skip == false {
            let msg = std::str::from_utf8(&encrypted_msg)
                .expect("Impossible de convertir le vecteur en string")
                .to_string()
                .trim_matches(char::from(0))
                .to_string();
            match dechiffrement_message(msg,key.clone()) {
                Some(msg) => println!("message recv {:?}", msg),
                None => ()
            }
        }

        // Chiffre et envoie le message au serveur
        match rx.try_recv() {
            Ok(msg) => {
                let buff = msg.clone().into_bytes();
                let msg_string = std::str::from_utf8(&buff).expect("Impossible de convertir le vecteur en string").to_string();
                let message_chiffre = chiffrement_message(msg_string, Box::new(key_dest.clone()));
                client.write_all(&message_chiffre.as_bytes()).expect("writing to socket failed");
            }
            Err(TryRecvError::Empty) => (),
            Err(TryRecvError::Disconnected) => break,
        }

        thread::sleep(Duration::from_millis(100));
    });


    let mut pseudo_client = saisir_pseudo();
    println!("{} > ", pseudo_client);
    pseudo_client.as_bytes();
    loop {
        let mut buff = String::new();
        io::stdin()
            .read_line(&mut buff)
            .expect("reading from stdin failed");

        let msg = buff.trim().to_string();
        if msg == ":quit" || tx.send(msg).is_err() {
            break;
        }
    }
    println!("Bye bye");
//    while match socket {
//        Ok(ref mut socket) => {
//            println!("Conneté au port 25566");
//            // let mut message_client = String::new();
//            // message_client = saisir_message();
//            //let message_client = saisir_message();
//            // let msg_octet = message_client.as_bytes();
//            // let tx: Vec<u8> = [msg_octet, b" > ", pseudo_octet].concat();
//            // socket.write(&tx).unwrap();
//            // println!("Message envoyé, en attente d'une réponse...");
//
//            // let mut trame = vec![0; BUFFER];
//            match socket.read(&mut trame) {
//                Ok(_size) => {
//                    // code non pertinent pour le projet
//                    // if &trame != msg_octet {
//                    //     println!("Reply ok!");
//                    // } else {
//                    //     // remplacer le unwrap()
//                    //     let msg_serveur = from_utf8(&trame).unwrap();
//                    //     println!("Réponse innatendu : {}", msg_serveur);
//                    // }
//                    let rx = from_utf8(&trame).unwrap();
//                    println!("Réponse serveur : {}", &rx);
//                },
//                Err(_e) => {
//                    println!("Aucune réponse de reçu : {}", _e);
//                }
//            }
//            true
//        }, Err(ref _e) => {
//            println!("Impossible de se connecter au serveur !");
//            false
//        }
//    } {}
    // debug à supprimer à la fin du projet
    println!("fin");
}

#[cfg(test)]
mod test
{
    use super::*;
    #[test]
    fn test_chiffrement_message()
    {
        let key = generation_des_cles();
        let message = " test le chiffement des message!".to_string();
        let message_chiffre = chiffrement_message(message.clone(),Box::new(key.to_public()));
        let message_dechiffre = dechiffrement_message(message_chiffre, key).unwrap();
        assert_eq!(message , message_dechiffre);
    }

}
