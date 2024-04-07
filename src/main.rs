use std::{
    io::Write,
    net::{TcpListener, TcpStream},
};

use rust_eez::{commands::commands::handle_commands, resp::RespType};

fn handle_tcp_stream(mut stream: TcpStream) -> std::io::Result<()> {
    // NOTE: Might not be the best way to do it? Clone might be VERY expensive here!
    let resp_result = RespType::deserialize(stream.try_clone()?);

    match resp_result {
        Ok((resp, mut stream)) => {
            println!("[Main Handler] Parsed TCP packet: `{:?}`", resp);
            let mut response = RespType::Error("WRONGTYPE array was expected".into());

            if let RespType::Array(commands) = resp {
                response = handle_commands(commands);
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

    println!("Listening On: {}", bind_address);

    for stream in listener.incoming() {
        println!("Got Stream: {:#?}", stream);

        match stream {
            Ok(tcp_stream) => handle_tcp_stream(tcp_stream).unwrap(),
            Err(err) => println!("Error TCP Data: {:#?}", err),
        }
    }

    Ok(())
}
