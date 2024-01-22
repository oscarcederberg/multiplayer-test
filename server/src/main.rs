use std::collections::HashMap;
use std::io;
use std::net::{SocketAddr, UdpSocket};
use rand::Rng;

const PEER_HOST: [u8; 4] = *b"HOST";
const PEER_JOIN: [u8; 4] = *b"JOIN";
const MAX_MSG_LEN: usize = 1024;
const CODE_LEN: usize = 5;

struct HolePunchServer {
    socket: UdpSocket,
    hosts: HashMap<[u8; CODE_LEN], SocketAddr>,
}

impl HolePunchServer {
    fn run(self) -> Result<(), io::Error> {
        let HolePunchServer { socket, mut hosts } = self;

        loop {
            // Receive messages from clients
            let mut buf = vec![0; MAX_MSG_LEN];

            let size;
            let peer_address;
            if let Ok((recv_size, recv_address)) = socket.recv_from(&mut buf) {
                size = recv_size;
                peer_address = recv_address;
            } else {
                println!("Failed to receive bytes");
                continue;
            }

            let msg = &buf[..size];

            println!("Received {} bytes from {}: {:?}", size, peer_address, msg);

            if msg.starts_with(&PEER_HOST) {
                println!("HOST request received");

                let mut code:[u8; CODE_LEN] = [0; CODE_LEN];
                let mut rng = rand::thread_rng();

                loop {
                    code.iter_mut().for_each(|v| {
                        *v = loop {
                            let character = rng.gen_range(b'A'..=b'Z');
                            if !matches!(character, b'A' | b'E'| b'I'| b'O'| b'U'| b'Y') {
                                break character;
                            }
                        }
                    });

                    if !hosts.contains_key(&code) {
                        break;
                    }
                }

                let mut response = code.to_vec();
                response.push(b'\n');

                if let Err(error) = socket.send_to(&response, peer_address) {
                    println!("Failed to receive bytes: {}", error);
                } else {
                    println!("Sent {} bytes to {}: {:?}",size, peer_address, response);
                    hosts.insert(code, peer_address);
                }
            } else if msg.starts_with(&PEER_JOIN) {
                todo!();
            }
        }
    }
}

fn main() {
    let port = 8777;
    let address = format!("127.0.0.1:{}", port);
    let socket = UdpSocket::bind(&address).unwrap();
    println!("Hole Punching Server listening on {}", address);

    let hosts = HashMap::new();
    let server = HolePunchServer { socket, hosts };

    let _ = server.run();
}