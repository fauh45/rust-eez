use std::{
    collections::HashMap,
    io::Write,
    net::{TcpListener, TcpStream},
    sync::{Arc, RwLock},
};

use rust_eez::{commands::commands::handle_commands, resp::RespType, storage::StorageType};

fn handle_tcp_stream(
    mut stream: TcpStream,
    storage: Arc<RwLock<HashMap<String, StorageType>>>,
) -> std::io::Result<()> {
    // NOTE: Might not be the best way to do it? Clone might be VERY expensive here!
    let resp_result = RespType::deserialize(stream.try_clone()?);

    match resp_result {
        Ok((resp, mut stream)) => {
            println!("[Main Handler] Parsed TCP packet: `{:?}`", resp);
            let mut response = RespType::Error("WRONGTYPE array was expected".into());

            if let RespType::Array(commands) = resp {
                response = handle_commands(commands, storage);
            }

            println!("[Main Handler] Responding with `{:#?}`", response);

            stream.write(&response.serialize())?;
        }
        // TODO: Somehow make the error also return the stream?
        Err(err) => {
            println!("Parsing error: `{:?}`", err);

            stream.write(
                &RespType::Error("ERR Could not deserialized command(s)".into()).serialize(),
            )?;
        }
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    let bind_address = "0.0.0.0:6969";
    let listener = TcpListener::bind(bind_address)?;

    let storage: Arc<RwLock<HashMap<String, StorageType>>> = Arc::new(RwLock::new(HashMap::new()));

    println!("Listening On: {}", bind_address);

    for stream in listener.incoming() {
        println!("Got Stream: {:#?}", stream);

        match stream {
            Ok(tcp_stream) => handle_tcp_stream(tcp_stream, Arc::clone(&storage)).unwrap(),
            Err(err) => println!("Error TCP Data: {:#?}", err),
        }
    }

    Ok(())
}
