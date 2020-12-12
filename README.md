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
# postgresql-12 is optional
$ sudo apt-get install libpq-dev postgresql-12
# start the pg server only if you installed it in the prev step
$ pg_ctlcluster 12 main start
$ cargo install diesel_cli --no-default-features --features postgres
```

- Create a .env file based on .env.example
```bash
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
