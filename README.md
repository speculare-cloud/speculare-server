Speculare server
========
[![AGPL License](https://img.shields.io/badge/license-AGPL-blue.svg)](LICENSE)
[![CI](https://github.com/Martichou/speculare-server/workflows/CI/badge.svg)](https://github.com/Martichou/speculare-server/actions)

Speculare server is intended to recieve data from speculare-client childrens.

This project is meant to evolve in something more complete and more complexe in a somewhat near future.

Server setup / Dev setup
--------------------------

- Install all deps
```bash
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
$ sudo apt-get install postgresql-12 libpq-dev
$ (optional) pg_ctlcluster 12 main start
$ cargo install diesel_cli --no-default-features --features postgres
```

- Create a .env file based on .env.exemple
```bash
$ diesel setup
```

Contributing
--------------------------

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.
