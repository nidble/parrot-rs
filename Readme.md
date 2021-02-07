# Parrot-rs

Parrot-rs is a simple restful service built with Rust and Warp.

## Installation

### Prerequisites
Please refer to offical guide for installing [Rust](https://www.rust-lang.org/tools/install) toolchain.

### Building from sources
For building a production ready version, issue the following command:

```bash
make build
```

## Usage
Start execution with:

```bash
make run
```

On another terminal try to perform some Curl requests with:

```bash
make e2e
```

### Docker & containers
You can ship a container version with the following comands:


```bash
make docker-build 0.1.0
```

```bash
make docker-run 0.1.0
```

### Test
The full tests suite can be runned with:

```bash
make test
```

Or if you only want to run integration test, raise the follow:

```bash
make integration
```

### Todo

- [ ] Reconsidering the introduction of .env file (i.e. [dotenv](https://crates.io/crates/dotenv)) if the project growths

## License
[MIT](https://choosealicense.com/licenses/mit/)

