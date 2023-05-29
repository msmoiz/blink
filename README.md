# Blink

This is a tiny command line utility that reminds you to blink from time to time.
Useful for those days when you are deep in a problem and forget to take care of
yourself. It sends a reminder every 15 minutes.

To install the tool, clone this repository and run the following command from
the root directory. This requires the [Cargo
CLI](https://doc.rust-lang.org/cargo/getting-started/installation.html).

```shell
cargo install --path . 
```

To run the tool, use the following command.

```shell
Start-Process blink -WindowStyle Hidden
```

This will start the process in the background.
