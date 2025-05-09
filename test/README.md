# rlibc test

This directory contains a collection of standalone C test files designed to validate POSIX-compliant functionality, particularly in the context of lightweight libc implementations or custom system libraries. Most of these test cases were generated with assistance from [ChatGPT](https://chat.openai.com), then reviewed and modified for correctness and clarity.

## Contents

The test suite includes coverage for a wide range of system-level APIs, including:

- Threading primitives: `pthread_mutex`, `pthread_cond`, `pthread_barrier`, `pthread_spinlock`, etc.
- Time functions: `clock`, `clock_gettime`, `localtime`, `strftime`, `mktime`, etc.
- Networking: `inet_pton`, `inet_ntop`, `inet_aton`, `inet_ntoa`, `bind`, etc.
- Miscellaneous utilities: `lock/unlock`, `strspn`, `atfork` (fake), and others.

Each file is self-contained and intended to be compiled and executed independently.

## Build Instructions

### Compile single test case

To build a specific test, use:

```sh
make test_foo
```

For example:

```sh
make test_mutex_lock
```

Each test will compile to a corresponding ELF file with the suffix "_rlibc" and "_musl" (e.g., `test_mutex_lock_rlibc` and `test_mutex_lock_musl`) that can be executed directly.

### Compile all test cases

You can run the following command to compile all test cases with rlibc-gcc:

```sh
make rlibc
```

or this one with musl-gcc:

```sh
make musl
```

**But you'd better rename `test_cond.c` to `foo.c` that not beginning with `test_` prefix, because it call `fdprintf()` which is not contained in POSIX.

### More Convinient

You can directly compile, run and compare the versions of rlibc and musl. The result will be written to the file and the console will also output human-friendly information. The command can be:

```sh
make run_test_mutex_lock
```

with the prefix `run_test_`.

To list all available test targets, run:

```
make list
```

### Clean

To clean up generated executables and written files, run:

```sh
make clean
```

### test_cond

`test_cond.c` is inspired by jyy[jyywiki.cn]. This is a producer-consumer problem.

Usage:

```sh
./test_cond_rlibc 4 2
```

or use Python script for check:

```sh
./test_cond_rlibc 4 2 | ./check_cond.py 4
```

If no assertion fail, it will run continuously.

### For more details, read the source code. 🙂