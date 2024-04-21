use std::{
    collections::HashMap,
    net::TcpListener,
    sync::{Arc, RwLock},
};

use rust_eez::{handle_command_stream, storage::StorageType};

fn main() -> std::io::Result<()> {
    let bind_address = "0.0.0.0:6969";
    let listener = TcpListener::bind(bind_address)?;

    let storage: Arc<RwLock<HashMap<String, StorageType>>> = Arc::new(RwLock::new(HashMap::new()));

    println!("Listening On: {}", bind_address);

    for stream in listener.incoming() {
        println!("Got Stream: {:#?}", stream);

        match stream {
            Ok(tcp_stream) => {
                if let Err(err) = handle_command_stream(tcp_stream, Arc::clone(&storage)) {
                    println!(
                        "[Main Handler] Error handling the command stream: {:#?}",
                        err
                    )
                }
            }
            Err(err) => println!("Error TCP Data: {:#?}", err),
        }
    }

    Ok(())
}
