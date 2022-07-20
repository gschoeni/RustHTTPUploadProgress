# Rust HTTP upload with progress

This example uses [reqwest](https://docs.rs/reqwest/0.11.11/reqwest/index.html) and [indicatif](https://docs.rs/indicatif/0.16.2/indicatif/index.html) to upload binary data to an [actix-web](https://docs.rs/actix-web/4.0.0/actix_web/index.html) server with progress. Building the project will result in two binaries, one for the client and one for the server.

# Build and Run

Build the binaries with cargo

`cargo build`

Run the server

```bash
$ ./target/debug/upload-server

Server running on 0.0.0.0:3030 🚀
```

Run the client on a decent sized file and watch the progress bar cruise.

```bash
./target/debug/upload-client upload ~/Downloads/archive.zip
```

