# Rust-eez

Redis written in Rust. Thus to not make it sound like Redis, Red-is, Rust-is, Rust-eez! TaDa!

## Available Commands

Currently there's only a small subset of Redis that's supported. The following commands are available,

### **SET**
Synopsis: Set a string to be stored on a key.

Syntax: `SET key value`

### **GET**
Synopsis: Get a string stored on a key.

Syntax: `GET key`

### **DEL**
Synopsis: Delete value in key(s).

Syntax: `DEL key [key ...]`

### **PING**
Synopsis: Ping the server.

Syntax: `PING [message]`

### **PING**
Synopsis: Switch protocol, though only allowing RESP2 (lol).

Syntax: `HELLO [protover]`

## Supported Protocol

The only supported protocol are [RESP2](https://redis.io/docs/reference/protocol-spec).

## License

See [LICENSE](./LICENSE).
