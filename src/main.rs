use std::{
    io::Write,
    net::{TcpListener, TcpStream},
};

use rust_eez::resp::RespType;

fn handle_tcp_stream(mut stream: TcpStream) -> std::io::Result<()> {
    // NOTE: Might not be the best way to do it? Clone might be VERY expensive here!
    let resp_result = RespType::deserialize(stream.try_clone()?);

    println!("TCP Packet reading: `{:?}`", resp_result);

    stream.write("+OK\r\n".as_bytes())?;

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
