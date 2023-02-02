// https://book.async.rs/tutorial/index.html

// use std::net::{TcpStream};
// use std::io::{Read, Write, ErrorKind};
// use std::sync::mpsc;
// use mpsc::TryRecvError;
// use std::thread;
use thiserror::Error;
use std::time::Duration as Dur;
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use std::process::exit;
use std::path::Path;
use serde::{Deserialize, Serialize};
use serde_json;
use age::{Recipient, DecryptError};
use std::str::FromStr;
use std::fs::File;
use std::iter;
use mpsc::TryRecvError;
use age;
use age::x25519::Identity;
use std::{
    io::{self, Read, Write},
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


// gestion propore des erreurs !

#[derive(Error, Debug)]
pub enum ClientError 
{
    #[error("impossible de trouver la bonne clé ! ")]
    NoMatchingKey,
    #[error("Impossible de déchiffrer le message ! ")]
    CantDecrypte,
    #[error("Impossible de chiffrer le message !")]
    CantEncrypte,
}

// Stocke les  types messages recu et envoyes

#[derive(Serialize, Deserialize)]
pub enum TypeDeMessage
{
 MessageEnvoye,
 MessageRecu,

}

// la base de donnée stocké dans le JSON
#[derive(Serialize, Deserialize)]

 pub struct BaseDeDonneesJson {
     messages:Vec<(String , TypeDeMessage)>
 }

// lire le fichier  qui contient  les message et le cree la de donneé
pub fn recupere_message(fichier : &str) -> Result<BaseDeDonneesJson, String>
{
    match  File::open(fichier) {
        Ok(fichier)=>{
            match serde_json::from_reader(fichier) {
                Ok(bdd)=> {
                    return Ok(bdd);
                },
                Err(_)=> {
                    return Err(String::from("Impossible de déserialiser le fichier !"));
                }
            }
        },
        Err(_)=> {
            return Err(String::from(" Impossible de lire le fichier"));
        }
    }
}

// enregistre  le message et le stocke dans le fichier JSON
pub fn enregistre_message(base_de_donne: BaseDeDonneesJson, fichier : &str) -> Option<String>
{
    match File::create(fichier)
    {
        Ok(fichier_ecriture) => {
            match serde_json::to_writer(fichier_ecriture,&base_de_donne)
             {
                 Ok(_) =>  {
                     return None;
                 },
                 Err(_)=> {
                     return Some(String::from("Impossible de sérialiser la base de donnée !"));
                 }
            }
        },
        Err(_) =>
        {
             return Some(String::from("Impossible d'ecrire le fichier !"));
        }
    }
}


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

pub fn chiffrement_message(message:String,key_public:Box<dyn Recipient +Send>) -> Result<String,ClientError>
{
    // Chiffre le message clair  en message chiffré

        let Some(encryptor) = age::Encryptor::with_recipients(vec![key_public])
        else {
            return Err(ClientError::CantEncrypte)
        };
            

        let mut encrypted = vec![];
        let Ok (mut writer) = encryptor.wrap_output(&mut encrypted)
        else  {
            return Err(ClientError::CantEncrypte)
        };

        if writer.write_all(message.as_bytes()).is_err() {
            return Err(ClientError::CantEncrypte)
        }
        if writer.finish().is_err() {
            return Err(ClientError::CantEncrypte)

        }

        Ok(hex::encode(encrypted))

}

// déchiffre le message chiffré obtenu en message clair
pub fn dechiffrement_message(message:String, key_prive:Identity) -> Result<String,ClientError>
{
    let message = hex::decode(message).unwrap();
    let decryptor = match age::Decryptor::new(&message[..])
    {
        Ok(decrypte) => {
            match decrypte {
                age::Decryptor::Recipients(d) => d,
                age::Decryptor::Passphrase(_) => unreachable!(),
            }
        },
        Err(_) => return Err(ClientError::CantDecrypte),
    };

    let mut decrypted = vec![];
    let mut reader = match decryptor.decrypt(iter::once(&key_prive as &dyn age::Identity)) {
        Ok(data) => data,
        Err(e) => {
            match e {
                DecryptError::NoMatchingKeys => return Err(ClientError::NoMatchingKey),
                _ => panic!("{}", e)
            }
        }
    };
    if reader.read_to_end(&mut decrypted).is_err() 
    {
        return Err(ClientError::CantDecrypte)
    }


     match  std::str::from_utf8(&decrypted) 
     {
        Ok(msg) => Ok(msg.to_string()),
        Err(_) => Err(ClientError::CantDecrypte),
    }
    
}



fn main() {

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
    let stop_db = Arc::new(AtomicBool::new(false));
    let stop_db_clone = stop_db.clone();




    thread::spawn(move ||  {
        // Stocke le resulat dans la var result_bdd
        let  mut result_bdd = match Path::new("bdd.json").exists()
        {
            true =>  {
                match recupere_message("bdd.json")
                {
                    Ok(bdd)=> bdd,
                    Err(err) => {
                        println!("Err: {} " ,err );
                        exit(1);
                    }

                }
            },
            false => {
                 BaseDeDonneesJson {
                     messages:Vec::<(String , TypeDeMessage)>::new()
                }
            }

        };

        // affichage des message a partir de la base de donnée
        for (message, type_msg) in &result_bdd.messages
        {
            match  type_msg {
                TypeDeMessage::MessageEnvoye => {
                    println!("message envoyé ==>  {} ", message);
                },
                TypeDeMessage::MessageRecu => {
                    println!("message recu <==  {}",message);
                }
            }
        }

        loop  {
            if stop_db_clone.load(Ordering::Relaxed) == true {
                // Save server database
               match  enregistre_message(result_bdd, "bdd.json") {
                Some(err) => {
                    println!("Err:{}" , err);
                    exit(1);
                },
                None => exit(0),
                
            }
            
                
            }

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
                    Ok(msg) =>
                    {
                        result_bdd.messages.push(
                            (msg.clone() , TypeDeMessage::MessageRecu)
                        );
                        println!("message recv {:?}", msg);
                    },
                    Err(erreur) => 
                    {
                        match erreur {
                            ClientError::NoMatchingKey => (),
                            ClientError::CantDecrypte => {
                                println!("{}",erreur);
                                exit(1);
                            }
                            ClientError::CantEncrypte => 
                            {
                                println!("{}",erreur);
                                exit(1);
                            }

                        }

                    }
                }
            }

            // Chiffre et envoie le message au serveur
            match rx.try_recv() {
                Ok(msg) => {
                    let buff = msg.clone().into_bytes();
                    //Convertir le vecteur en format string pour le chiffrement !!
                    let msg_string = std::str::from_utf8(&buff).expect("Impossible de convertir le vecteur en string").to_string();
                    result_bdd.messages.push(
                        (msg_string.clone(), TypeDeMessage::MessageEnvoye)
                    );
                    let message_chiffre =   match chiffrement_message(msg_string, Box::new(key_dest.clone())) 
                    {
                        Ok(msg) => msg,
                        Err(erreur) => 
                        {
                            println!("{}",erreur);
                            exit(1);

                        }
                    
                    };
                    client.write_all(&message_chiffre.as_bytes()).expect("writing to socket failed");
                }
                Err(TryRecvError::Empty) => (),
                Err(TryRecvError::Disconnected) => break,
            }

            thread::sleep(Duration::from_millis(100));
    }});


    let pseudo_client = saisir_pseudo();
    println!("{} > ", pseudo_client);
  //  pseudo_client.as_bytes();
    loop {
        let mut buff = String::new();
        io::stdin()
            .read_line(&mut buff)
            .expect("reading from stdin failed");

        let msg = buff.trim().to_string();
        if msg == ":quit" || tx.send(msg).is_err() {
            stop_db.store(true, Ordering::Relaxed);
            thread::sleep(Dur::from_secs(2));
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
        let message_chiffre = match chiffrement_message(message.clone(),Box::new(key.to_public())) {
            Ok(msg) => msg,
            Err(erreur) => {
                println!("{}",erreur);
                exit(1);
            },
        };
        let message_dechiffre = dechiffrement_message(message_chiffre, key).unwrap();
        assert_eq!(message , message_dechiffre);
    }

    #[test]
    fn test_enregistre_message_success() {
        let base_de_donnees = BaseDeDonneesJson {
            messages: vec![],
        };
        let result = enregistre_message(base_de_donnees, "test_file.json");
        assert_eq!(result, None);
    }

    #[test]
    fn test_enregistre_message_echec_creation_fichier() {
        let base_de_donnees = BaseDeDonneesJson {
            messages: vec![],
        };
        let result = enregistre_message(base_de_donnees, "/this/file/path/does/not/exist/test_file.json");
        assert_eq!(result, Some(String::from("Impossible d'ecrire le fichier !")));
    }
/*
    #[test]
    fn test_enregistre_message_echec_serialisation() {
        let base_de_donnees = BaseDeDonneesJson {
            messages: vec![],
        };
        let result = enregistre_message(base_de_donnees, "/root/test_file.json");
        assert_eq!(result, Some(String::from("Impossible de sérialiser la base de donnée !")));
    }*/
}

