use age::{Recipient, DecryptError, EncryptError, x25519::Identity};
use mpsc::TryRecvError;
use std::{
    io::{self, ErrorKind, Read, Write},
    net::TcpStream,
    sync::mpsc,
    thread,
    time::Duration,
    str::FromStr,
    iter,
};

// fonction pour créer une pause pour les threads
fn sleep() {
    thread::sleep(::std::time::Duration::from_millis(100));
}

// saisie d'un pseudonyme
fn saisir_pseudo() -> String {
    let mut pseudo = String::new();
    println!("Saisir votre pseudo : ");
    let _tmp = std::io::stdin().read_line(&mut pseudo)
        .unwrap_or(1);
    println!("Bonjour {} !", pseudo);

    pseudo
}

// saisie d'un message à envoyer
fn saisir_message() -> String {
     let mut message = String::new();
     let tampon = std::io::stdin().read_line(&mut message).unwrap_or(2);
 
     message
}

pub fn generation_des_cles( )-> Identity
{
    let key = age::x25519::Identity::generate();
    key
}

pub fn chiffrement_message(message:String,key_public:Box<dyn Recipient +Send>) -> String
{
    // Chiffre le message clair en message chiffré
    match age::Encryptor::with_recipients(vec![key_public]) {
        Some(encryptor) => {
            let mut encrypted = vec![];
            match encryptor.wrap_output(&mut encrypted) {
                Ok(mut writer) => {
                    match writer.write_all(message.as_bytes()) {
                        Ok(writer1) => {
                            match writer.finish() {
                                Ok(result) => {
                                    hex::encode(result);
                                },
                                Err(e) => {
                                    println!("Chiffrement impossible : {}", e);
                                }
                            }
                        },
                        Err(e) => {
                            println!("Chiffrement impossible : {}", e);
                        }
                    }
                },
                Err(e) => {
                    println!("Chiffrement impossible : {}", e);
                }
            }
            hex::encode(encrypted)
        },
        None => {
            println!("Erreur lors du chiffrement");
            let string = String::from("erreur");
            string
        }
    }
}

// déchiffre le message chiffré obtenu en message clair
pub fn dechiffrement_message(message:String, key_prive:Identity) -> Option<String>
{
    let message = hex::decode(message).unwrap_or(vec![4,0,4]);
    let decryptor = match age::Decryptor::new(&message[..]).expect("Impossible d'intialiser le Déchiffrement") {
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

fn main() -> std::io::Result<()> {
    let mut client = TcpStream::connect("localhost:25566")?;
    
    client.set_nonblocking(true)?;

    let (tx, rx) = mpsc::channel::<String>();
    let key = generation_des_cles();
    // L'affichage de la clé publique de l'utilisateur !
    println!("Voici la clé publique {} ",key.to_public());
    // Récuperation de la clé destinataire
    //println!("Saisir la clé publique destinataire !");
    // Stocke la clé en format string et on la converti en format Recipient
    let mut key_str = String::new();
    let mut key_dest = Err("empty");
    //let mut length = 0;
    while key_dest.is_err() {
        println!("Saisir la clé publique destinataire !");
        key_str = String::new();
        let input = std::io::stdin().read_line(&mut key_str).unwrap_or(3);
        println!("input {}", input);
        //length = input;
        // enlever le /n
        let key_str = &key_str[0..key_str.len()-1];
        key_dest = age::x25519::Recipient::from_str(&key_str);
        println!("key dest = {}", key_str);
    }

    // ajout d'un thread pour transmettre les sockets
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
                match key_dest {
                    Ok(ref key) => {
                        let message_chiffre = chiffrement_message(msg_string, Box::new(key.clone()));
                        client.write_all(&message_chiffre.as_bytes()).expect("writing to socket failed");
                    },
                    Err(e) => {
                        println!("Erreur {}", e)
                    }
                }
            }
            Err(TryRecvError::Empty) => (),
            Err(TryRecvError::Disconnected) => break,
        }
        sleep();
    });

    // saisie d'un pseudo
    let pseudo_client = saisir_pseudo();
    println!("{} > ", pseudo_client);
    pseudo_client.as_bytes();
    loop {
        // saisir un message en boucle dans le thread principal
        let mut buff = String::new();
        buff = saisir_message();

        let mut msg = buff.trim().to_string();
        // pour quitter proprement le programme ':quit'
        if msg == ":quit" || tx.send(msg).is_err() {
            break;
        }
    }
    println!("fin");

    // nécessaire pour utiliser ? (propagation d'erreur)
    Ok(())
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
        let message_dechiffre = dechiffrement_message(message_chiffre, key).unwrap_or("error".to_string());
        assert_eq!(message , message_dechiffre);
    }
}
