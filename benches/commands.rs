use std::{
    collections::{HashMap, VecDeque},
    sync::{Arc, RwLock},
};

use criterion::{black_box, criterion_group, criterion_main, Criterion};
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

fn bench_set_op(c: &mut Criterion) {
    let storage: Arc<RwLock<HashMap<String, StorageType>>> = Arc::new(RwLock::new(HashMap::new()));
    let set_command = to_stream("*3\r\n$3\r\nSET\r\n$3\r\nHII\r\n$11\r\nHELLO WORLD");

    c.bench_function("SET command", move |b| {
        b.iter(|| {
            handle_command_stream(black_box(set_command.clone()), black_box(storage.clone()))
                .unwrap();
        })
    });
}

fn bench_get_op(c: &mut Criterion) {
    let storage: Arc<RwLock<HashMap<String, StorageType>>> = Arc::new(RwLock::new(HashMap::from(
        [("HII".into(), StorageType::String("AAA".into()))],
    )));
    let get_command = to_stream("*2\r\n$3\r\nGET\r\n$3\r\nHII");

    c.bench_function("GET command", move |b| {
        b.iter(|| {
            handle_command_stream(black_box(get_command.clone()), black_box(storage.clone()))
                .unwrap();
        })
    });
}

fn bench_del_op(c: &mut Criterion) {
    let storage: Arc<RwLock<HashMap<String, StorageType>>> = Arc::new(RwLock::new(HashMap::from(
        [("HII".into(), StorageType::String("AAA".into()))],
    )));

    let del_command = to_stream("*2\r\n$3\r\nDEL\r\n$3\r\nHII");

    c.bench_function("DEL command", move |b| {
        b.iter(|| {
            handle_command_stream(black_box(del_command.clone()), black_box(storage.clone()))
                .unwrap();
        })
    });
}

criterion_group!(benches, bench_set_op, bench_get_op, bench_del_op);
criterion_main!(benches);
