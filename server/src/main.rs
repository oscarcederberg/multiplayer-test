use rand::Rng;
use std::collections::HashMap;
use std::net::{SocketAddr, UdpSocket};

const PEER_HOST: [u8; 4] = *b"HOST";
const PEER_JOIN: [u8; 4] = *b"JOIN";
const MAX_MSG_LEN: usize = 1024;
const CODE_LEN: usize = 5;


struct HolePunchServer {
    socket: UdpSocket,
    hosts: HashMap<[u8; CODE_LEN], SocketAddr>,
}

impl HolePunchServer {
    fn send_response(&self, response: &[u8], address: &SocketAddr) -> bool {
        if let Err(error) = self.socket.send_to(response, address) {
            println!("Failed to send bytes: {}", error);
            false
        } else {
            println!("Sent {} bytes to {}: {:?}", response.len(), address, response);
            true
        }
    }

    fn add_host(&mut self, address: &SocketAddr) {
        let mut code: [u8; CODE_LEN] = [0; CODE_LEN];
        let mut rng = rand::thread_rng();

        loop {
            code.iter_mut().for_each(|v| {
                *v = loop {
                    let character = rng.gen_range(b'A'..=b'Z');
                    if !matches!(character, b'A' | b'E' | b'I' | b'O' | b'U' | b'Y') {
                        break character;
                    }
                }
            });

            if !self.hosts.contains_key(&code) {
                break;
            }
        }

        let mut response = code.to_vec();
        response.push(b'\n');
        if self.send_response(&response, address) {
            self.hosts.insert(code, *address);
        }
    }

    fn join_host(&mut self, msg: &[u8], size: usize, address: &SocketAddr) {
        if size < PEER_JOIN.len() + CODE_LEN + 1 {
            println!("Join request too short: {:?}", msg);
            self.send_response(b"Join request too short\n", address);
            return;
        }

        let code = &msg[(PEER_JOIN.len() + 1)..=(PEER_JOIN.len() + CODE_LEN)];

        if !code.iter().all(|v| {
            v.is_ascii_uppercase() && !matches!(v, b'A' | b'E' | b'I' | b'O' | b'U' | b'Y')
        }) {
            println!("Illegal join code: {:?}", code);
            self.send_response(b"Illegal join code\n", address);
            return;
        }

        if !self.hosts.contains_key(code) {
            println!("No host available with join code: {:?}", code);
            self.send_response(b"No host available with join code\n", address);
            return;
        }

        let host = self.hosts.remove(code).unwrap();

        if self.send_response(format!("Client: {:?}", address).as_bytes(), &host) {
            self.send_response(format!("Host: {:?}", host).as_bytes(), address);
        }
    }

    fn run(mut self) {
        loop {
            // Receive messages from clients
            let mut buf = vec![0; MAX_MSG_LEN];

            let size;
            let peer_address;
            if let Ok((recv_size, recv_address)) = self.socket.recv_from(&mut buf) {
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
                self.add_host(&peer_address);
            } else if msg.starts_with(&PEER_JOIN) {
                println!("JOIN request received");
                self.join_host(msg, size, &peer_address);
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

    server.run();
}
