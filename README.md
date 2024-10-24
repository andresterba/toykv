# toykv

Experiments to implement the [Redis serialization protocol specification](https://redis.io/docs/latest/develop/reference/protocol-spec/).
Additionally, my first experiments in Rust.


Run with `cargo run` will start the server.
Execute `redis-cli ping hello` should return whatever was provided with `ping`,
in this case `hello`.

Possible commands are:
```sh
redis-cli ping hello
redis-cli set andre 1337
redis-cli get andre
```

Inspired by https://github.com/ahmedash95/build-redis-from-scratch.