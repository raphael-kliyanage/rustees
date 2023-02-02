use std::{
    io::{ErrorKind, Read, Write},
    net::TcpListener,
    sync::mpsc,
    thread,
};

fn sleep() {
    thread::sleep(::std::time::Duration::from_millis(100));
}

fn main() -> std::io::Result<()> {
    const TAILLE_MSG: usize = 4096;  // mem tampon à 4096 octets
    let server = TcpListener::bind("localhost:25566")?;
    server
        .set_nonblocking(true)?;

    let mut clients = vec![];
    let (tx, rx) = mpsc::channel::<String>();
    loop {
        if let Ok((mut socket, addr)) = server.accept() {
            println!("Client {} connected", addr);

            let tx = tx.clone();
            clients.push(socket.try_clone()?);

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
    Ok(())
}
