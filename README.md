# rusted

Tool/framework for learning Rust/Python development

### Development

    $ sudo dnf install rustc cargo rustup rust-src
    $ python3 -m pip install --user pipx
    $ python3 -m pipx ensurepath
    $ pipx install maturin

### Build

    $ make build

### Run

    $ ./target/debug/rusted --help

### Try running Rust from Python or Python from Rust

    $ maturin develop
    ...
    $ python3
    >>> from rusted import hello_world
    >>> hello_world.hello_world_python_test()
    PYTHON: Hello world

    >>> from rusted.rusted import Rusted
    >>> Rusted().hello_world_rust_test()
    RUST: Hello world

    >>> from rusted.rusted import run_python_from_rust
    >>> run_python_from_rust()
    PYTHON: Hello world
