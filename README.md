# rusted

Tool/framework for learning Rust/Python development

Rust CLI based on `sarahbx/help-cli`  
Python integration based on `pyo3`

### Development
    $ sudo dnf install make rustc cargo rustup rust-src llvm
    $ python3 -m pip install --user pipx
    $ python3 -m pipx ensurepath
    $ pipx install maturin    # for python support
    $ cargo install rustfilt  # for rust coverage

### Config
    $ cp config-example.toml ~/.rusted-config.toml
    # Edit local config ~/.rusted-config.toml

### Build/Test
    $ make clean
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

### Build with test coverage
    $ make coverage
