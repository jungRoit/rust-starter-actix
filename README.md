# rust-starter-actix

### Setup
1. Setup [rust](https://www.rust-lang.org/tools/install) and `cargo`.
1. Setup [mongodb](https://docs.mongodb.com/manual/installation/).
1. Clone the repository.

    ```bash
    $ git clone https://github.com/jungRoit/rust-starter-actix.git
    ```

1. Create `.env` file.

    ```bash
    $ cp .env.example .env
    ```

### Environemt Variables

| Variable    | Description                                              |
| ----------- | -------------------------------------------------------- |
| HOST        | Base Server URL e.g. `127.0.0.1:8000`                    |
| DB_URL      | DB URL for mongodb e.g. `mongodb://127.0.0.1:27017/`     |
| DB_NAME     | MongoDB database name. e.g. `local`                      |
| RUST_LOG    | Configure log levels. e.g. `debug`, `info`               |

### Build

For development.

```bash
$ cargo build
```

For production.

```
$ cargo build --release
```

## Run
```bash
$ cargo run
```

## Formatter

*Note: Make sure you have [rustfmt](https://github.com/rust-lang/rustfmt).*

Check code formatting.

```bash
$ cargo fmt -- --check
```

Fix code formatting.

```bash
$ cargo fmt
```
