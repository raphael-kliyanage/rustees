// https://book.async.rs/tutorial/index.html
// https://thepacketgeek.com/rust/tcpstream/reading-and-writing/

//use std::thread;
//use std::net::{TcpListener, TcpStream, Shutdown};
//use std::io::{Read, Write};
// use std::str::from_utf8; // pour voir ce que le serveur reçoit
use std::{
    io::{ErrorKind, Read, Write},
    net::TcpListener,
    sync::mpsc,
    thread,
};

fn sleep() {
    thread::sleep(::std::time::Duration::from_millis(100));
}

// fn handle_client(mut socket: Vec::<TcpStream>) {
//     const BUFFER: usize = 512;  // mem tampon à 512 octets
//     let mut data = [0 as u8; BUFFER];
//     for n in 0..socket.len() {
//         while match socket[n].read(&mut data) {
//             Ok(size) => {
//                 // echo everything!
//                 // remplacer le unwrap()
//                 socket[n].write(&data[0..size]).unwrap();
//                 // voir ce que reçoit le serveur
//                 // let text = from_utf8(&data).unwrap();
//                 // println!("Client : {}", text);
//                 true
//             },
//             Err(_) => {
//                 println!("Une erreur est survenue, déconnexion du client : {}",
//                     socket[n].peer_addr().unwrap());
//                 // remplacer le unwrap()
//                 socket[n].shutdown(Shutdown::Both).unwrap();
//                 false
//             }
//         } {}
//     }
// }

fn main() {
    const TAILLE_MSG: usize = 4096;  // mem tampon à 4096 octets
    let server = TcpListener::bind("localhost:25566")
        .expect("Listener failed to bind");
    server
        .set_nonblocking(true)
        .expect("failed to initialize non-blocking");

    let mut clients = vec![];
    let (tx, rx) = mpsc::channel::<String>();
    loop {
        if let Ok((mut socket, addr)) = server.accept() {
            println!("Client {} connected", addr);

            let tx = tx.clone();
            clients.push(socket.try_clone().expect("failed to clone client"));

            thread::spawn(move || loop {
                // Buffer temporaire (morceaux du message)
                let mut buff = vec![0; TAILLE_MSG];
                // Message chiffré final
                let mut encrypted_msg: Vec<u8> = Vec::new();
                let mut skip: bool = false;
                // Lire les paquets tant qu'on a pas le message complet
                loop {
                    match socket.read(&mut buff) {
                        Ok(msg_len) => {
                            if msg_len == TAILLE_MSG {
                                encrypted_msg.append(&mut buff);
                            } else if msg_len != 0 {
                                encrypted_msg.append(&mut buff[..msg_len].to_vec());
                                break;
                            } else {
                                skip = true;
                                break;
                            }
                        },
                        Err(_) => break
                    }
                }
                if skip == false {
                    let msg = std::str::from_utf8(&encrypted_msg).expect("Impossible de convertir le vecteur en string").to_string();
                    println!("Rcv <== {:?}", socket.peer_addr());
                    println!("{} {:?} len={}", addr, msg, msg.len());
                    tx.send(msg).expect("failed to send message to rx");
                }
                sleep();
            });
        }

        if let Ok(msg) = rx.try_recv() {
            clients = clients
                .into_iter()
                .filter_map(|mut client| {
                    let mut buff = msg.clone().into_bytes();
                    buff.resize(TAILLE_MSG, 0);
                    println!("Send ==> {:?}", client.peer_addr());
                    client.write_all(&buff).map(|_| client).ok()
                })
                .collect::<Vec<_>>();
        }

        sleep();
    }

//    // remplacer le unwrap()
//    let listener = TcpListener::bind("0.0.0.0:25566").unwrap();
//    //let listener_clone = listener.try_clone().unwrap();
//    // accept connections and process them, spawning a new thread for each one
//    println!("Serveur en écoute sur le port 25566");
//    for socket in listener.incoming() {
//        match socket {
//            Ok(socket) => {
//                // remplacer le unwrap()
//                let socket_clone = socket.try_clone().expect("Impossible de cloner");
//                let mut vec_socket = Vec::<TcpStream>::new();
//                vec_socket.push(socket_clone);
//                println!("Nouvelle connexion : {}",
//                    socket.peer_addr().unwrap());
//                    //listener1.push(socket);
//                /*let thread_socket = */thread::spawn(move || {
//                    // connection succeeded
//                    handle_client(vec_socket)
//                });
//                // let res = thread_socket.join();
//            }
//            Err(e) => {
//                println!("Echec de la connexion : {}", e);
//            }
//        }
//    }
//    drop(listener);
}
