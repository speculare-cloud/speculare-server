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

This repo is the place for both server & alerts of Speculare, doing it this way to share models and some common code.

Speculare server is intended to recieve data from speculare-client childrens.

Speculare alerts is intended to create and send notifications about incidents.

This project is meant to evolve in something more complete and more complexe in a somewhat near future.

Server setup / Dev setup
--------------------------

- Install all deps
```bash
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
$ sudo apt-get install libssl-dev libpq-dev pkg-config build-essential
```

- Create both Alerts.toml and Server.toml files based on Example.toml

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

Contributing
--------------------------

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.
