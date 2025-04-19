# Rust-based C Library (rlibc)

rlibc is a portable POSIX C standard library written in Rust and is under heavy development.

This project is part of [tst](https://github.com/ShitaoTang)'s undergraduate thesis work, focusing on the implementation of core POSIX components including `thread`, `network`, and `time`, in Rust.

Currently, only **static linking** is supported.

## repository layout

```sh
.
├── C/                      # C test & helper code
│   ├── Makefile
│   ├── src/
│   │   ├── assert.c
│   │   ├── exit.c
│   │   ├── stdio.c
│   │   └── string.c
│   └── test/
│       └── test.c
├── include/               # ported musl headers (installed headers)
├── src/                   # Rust implementation of libc modules
│   ├── aio/
│   ├── arch/
│   ├── crt/               # C Run-Time
│   ├── env/
│   ├── exit/
│   ├── include/
│   ├── internal/
│   ├── ldso/              # dynamic link (not supported yet)
│   ├── lib.rs
│   ├── linux/
│   ├── locale/
│   ├── malloc/
│   ├── mman/
│   ├── network/
│   ├── signal/
│   ├── stat/
│   ├── stdio/
│   ├── stdlib/
│   ├── string/
│   ├── thread/
│   └── time/
├── arch/                  # architecture-specific headers
├── tools/
├── configure
├── Makefile
├── Cargo.toml
└── README.md
```

## Build Instructions

### Dependencies

Make sure the following tools are installed:

- `build-essential` (for `gcc`, `make`, etc.)
- `git`
- `rustup` and `cargo`
- ...

You can download rust by following this [tutorial](https://www.rust-lang.org/tools/install).

### Rust Toolchain

This project requires the Rust nightly toolchain. 

Install and set up via rustup:

```sh
rustup install nightly
rustup default nightly
```

To check if it is nightly version, you can run the following command:

```sh
rustup show
# or
rustc --version
```

The output can be:

```bash
$ rustc --version
rustc 1.87.0-nightly (f8c27dfe1 2025-03-24)
```

### Configure and Build

Use the standard configure-make-install flow:

```sh
cd rlibc
./configure --prefix=/your/install/path
make && make install
```

If you encounter a permission error related to `cargo`, try running the command with `sudo`.

## Usage

To use `rlibc-gcc`, add the installation prefix’s bin directory to your PATH.

```sh
# fish shell
echo 'set -x PATH /usr/local/rlibc/bin $PATH' >> ~/.config/fish/config.fish
source ~/.config/fish/config.fish
```

Now you can compile C programs using:

```sh
rlibc-gcc a.c -static
```

## Supported Operating Systems

- Linux
- macOS (Intel-based) *(not tested)*

## Supported Architectures

- `x86_64` (Intel / AMD)
- `aarch64` (ARM64)
