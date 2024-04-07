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

## Problems

Here are some problem that I'm aware, might not be correct, but that's what I think is an issue in this code base.

- [ ] Storage size aren't limited, so after a while, it can just not insert new keys. Might need some kind of LRU to be implemented (?).
- [ ] Stream are copied for writing in case of any error on deserialization (see [main.rs](./src/main.rs)). Probably should think of how to return the `stream` on error as well.

If you found anything, please do tell me. I'm actively learning Rust. Why else would I try to rewrite Redis?

## License

See [LICENSE](./LICENSE).
