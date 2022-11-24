use serde::Deserialize;

use std::error::Error;
use std::net::{TcpListener, TcpStream};

#[derive(Deserialize, Debug)]
struct User {
    fingerprint: String,
    location: String,
}

fn read_user_from_stream(tcp_stream: TcpStream) -> Result<User, Box<dyn Error>> {
    let mut de = serde_json::Deserializer::from_reader(tcp_stream);
    let u = User::deserialize(&mut de)?;

    Ok(u)
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4000").unwrap();

    for stream in listener.incoming() {
        println!("{:#?}", read_user_from_stream(stream.unwrap()));
    }
}