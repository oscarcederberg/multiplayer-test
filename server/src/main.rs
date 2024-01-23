use rand::Rng;
use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

const PEER_HOST: [u8; 4] = *b"HOST";
const PEER_JOIN: [u8; 4] = *b"JOIN";
const MAX_MSG_LEN: usize = 1024;
const CODE_LEN: usize = 5;

struct HolePunchServer {
    hosts: HashMap<[u8; CODE_LEN], TcpStream>,
}

impl HolePunchServer {
    fn send_response(&self, response: &[u8], stream: &mut TcpStream) -> bool {
        if let Err(error) = stream.write(response) {
            println!("Failed to send bytes: {}", error);
            false
        } else {
            println!(
                "Sent {} bytes: {:?}",
                response.len(),
                response
            );
            true
        }
    }

    fn add_host(&mut self, mut stream: TcpStream) {
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
        if self.send_response(&response, &mut stream) {
            self.hosts.insert(code, stream);
        }
    }

    fn join_host(&mut self, msg: &[u8], size: usize, mut stream: TcpStream) {
        if size < PEER_JOIN.len() + CODE_LEN + 1 {
            println!("Join request too short: {:?}", msg);
            self.send_response(b"Join request too short\n", &mut stream);
            return;
        }

        let code = &msg[(PEER_JOIN.len() + 1)..=(PEER_JOIN.len() + CODE_LEN)];

        if !code.iter().all(|v| {
            v.is_ascii_uppercase() && !matches!(v, b'A' | b'E' | b'I' | b'O' | b'U' | b'Y')
        }) {
            println!("Illegal join code: {:?}", code);
            self.send_response(b"Illegal join code\n", &mut stream);
            return;
        }

        if !self.hosts.contains_key(code) {
            println!("No host available with join code: {:?}", code);
            self.send_response(b"No host available with join code\n", &mut stream);
            return;
        }

        let mut host_stream = self.hosts.remove(code).unwrap();
        let host_address;
        if let Ok(address) = host_stream.peer_addr() {
            host_address = address;
        } else {
            println!("Failed to retreive host address from stream");
            return;
        }

        let peer_address;
        if let Ok(address) = stream.peer_addr() {
            peer_address = address;
        } else {
            println!("Failed to retreive peer address from stream");
            return;
        }

        if self.send_response(format!("Client: {:?}", peer_address).as_bytes(), &mut host_stream) {
            self.send_response(format!("Host: {:?}", host_address).as_bytes(), &mut stream);
        }
    }

    fn handle_stream(&mut self, mut stream: TcpStream) {
        let mut buffer = [0u8; MAX_MSG_LEN];
        let message;
        let message_size;
        if let Ok(size) = stream.read(&mut buffer) {
            message_size = size;
            message = &buffer[..size];
            println!("Received {} bytes: {:?}", message_size, message);
        } else {
            println!("Failed to receive bytes");
            return;
        }

        if message.starts_with(&PEER_HOST) {
            println!("HOST request received");
            self.add_host(stream);
        } else if message.starts_with(&PEER_JOIN) {
            println!("JOIN request received");
            self.join_host(message, message_size, stream);
        }
    }
}

fn main() {
    let port = 8777;
    let address = format!("127.0.0.1:{}", port);
    let listener = TcpListener::bind(&address).unwrap();
    println!("Hole Punching Server listening on {}", address);

    let hosts = HashMap::new();
    let mut server = HolePunchServer { hosts };

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => server.handle_stream(stream),
            _ => ()
        }
    }
}
