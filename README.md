<div align="center">
  <h1>Speculare Server</h1>
  <p>
    <strong>Metrics for your servers</strong>
  </p>
  <p>

[![Apache 2 License](https://img.shields.io/badge/license-Apache%202-blue.svg)](LICENSE)
[![CI](https://github.com/Martichou/speculare-server/workflows/CI/badge.svg)](https://github.com/Martichou/speculare-server/actions)
[![Docs](https://img.shields.io/badge/Docs-latest-green.svg)](https://docs.speculare.cloud)

  </p>
</div>

Speculare server is intended to recieve data from speculare-client childrens.

This project is meant to evolve in something more complete and more complexe in a somewhat near future.

Server setup / Dev setup
--------------------------

- Install all deps
```bash
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
$ sudo apt-get install libssl-dev libpq-dev pkg-config build-essential
```

- Create a .env file based on .env.example

> **⚠ WARNING: The TimescaleDB instance you're going to use need to be configured for logical replication, check the [docs](https://docs.speculare.cloud).**

- (Solution A) Setup the database without doing anything
```
When running the binary, it will automatically check if all available migrations have been applied.
So you don't have to do anything, just launch and enjoy.
```

- (Solution B) Setup the database based on diesel
```bash
# You first need to setup a postgresql 13 instance
# And you also need to install diesel cli
$ cargo install diesel_cli --no-default-features --features postgres
# For diesel setup to works you need to be at the root of the project
$ diesel setup
```

Information
--------------------------
By default (and until configuration is implemented) the data retention is set to 7 days. If you've configured everything and each service are running correctly, you don't have to do anything. Partman will take care of cleaning old PARTITION and creating new one.

Using PARTITION instead of regular DELETE ... WHERE created_at ... is better to avoid any performance impact. DELETE on a big table can take several minutes and block incoming transaction, whereas DROPing a TABLE is faster and don't block incoming transaction.

Run in Docker
--------------------------

You might want to take advantage of Docker to run the server inside a container.

You first need to build the image:
```bash
$ docker build -t speculare_server .
```

And then you can simply launch it:
```bash
$ docker run -d -p 8080:8080 --env-file .env speculare_server
```

_Note: the Dockerfile is configured to use the *port* 8080 by default and is running in *debug* mode._

Contributing
--------------------------

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.
