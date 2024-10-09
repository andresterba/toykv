# toykv

Experiments to implement the [Redis serialization protocol specification](https://redis.io/docs/latest/develop/reference/protocol-spec/).
Additionally, my first experiments in Rust.


Run with `cargo run` will start the server.
Execute `redis-cli ping hello` should return whatever was provided with `ping`,
in this case `hello`.

Inspired by https://github.com/ahmedash95/build-redis-from-scratch.