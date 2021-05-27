<div align="center">
  <h1>Speculare Server</h1>
  <p>
    <strong>Metrics for your servers</strong>
  </p>
  <p>

[![Apache 2 License](https://img.shields.io/badge/license-Apache%202-blue.svg)](LICENSE)
[![CI](https://github.com/Martichou/speculare-server/workflows/CI/badge.svg)](https://github.com/Martichou/speculare-server/actions)

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
$ cargo install diesel_cli --no-default-features --features postgres
```

- Create a .env file based on .env.example

- (Solutation A) Setup the database based on the migrations files (bash file)
```bash
# For database_setup.sh to works, you need to have a folder migrations at the same
# level of database_setup, containing folders containting up.sql and down.sql.
$ env PGHOST=localhost PGUSER=postgres PGPASSWORD=password ./database_setup.sh
```

- (Solutation B) Setup the database based on diesel
```bash
# You first need to setup a postgresql 13 instance
# And you also need to install diesel cli
$ cargo install diesel_cli
# For diesel setup to works you need to be at the root of the project
$ diesel setup
```

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
