use std::{
    collections::{HashMap, VecDeque},
    sync::{Arc, RwLock},
};

use criterion::{criterion_group, criterion_main, Criterion};
use rust_eez::{handle_command_stream, storage::StorageType};

/// Hacky way to "mock" TcpStream, but it work to see perf
fn to_stream(command: &str) -> VecDeque<u8> {
    // NOTE: This is the one that will be parsed.
    let mut stream = VecDeque::from(command.as_bytes().to_vec());

    // NOTE: As it is done parsing, the pointer will live at the end of the command,
    // as our program will then write the response to it. We need to resize extra for it.
    // So for this application, this will be enough to give room to the writer.
    stream.resize_with(1_000, || 0u8);

    stream
}

fn bench_simple_set_get_del(c: &mut Criterion) {
    c.bench_function("Test", |b| {
        b.iter(|| {
            // NOTE: All of this initialization might took a few cycle, need to find a better way
            // to do it.
            let storage: Arc<RwLock<HashMap<String, StorageType>>> =
                Arc::new(RwLock::new(HashMap::new()));

            let set_command = to_stream("*3\r\n$3\r\nSET\r\n$3\r\nHII\r\n$11\r\nHELLO WORLD");
            let get_command = to_stream("*2\r\n$3\r\nGET\r\n$3\r\nHII");
            let del_command = to_stream("*2\r\n$3\r\nDEL\r\n$3\r\nHII");

            handle_command_stream(set_command, storage.clone()).unwrap();
            handle_command_stream(get_command, storage.clone()).unwrap();
            handle_command_stream(del_command, storage.clone()).unwrap();
        })
    });
}

criterion_group!(benches, bench_simple_set_get_del);
criterion_main!(benches);
