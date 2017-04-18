# Whatlang-py
bindings by Cathal Garvey, Copyright 2017, released under MIT license.

## About
This simply wraps the [Whatlang-rs library][orig] by [Sergey Potapov][sergey],
exposing a single function to Python which returns a tuple of (language code, script code).

## Installation
You will require:

* [A Rust toolchain][rust]
* `pip install setuptools_rust`

Then: `python3 setup.py install --user`. The `--user` part is essential, as
Rustup doesn't perform a system installation of Rust. Possibly, you could use
a system-managed Rust toolchain (e.g. `apt-get install cargo`) to permit a
system-wide install of the library. Or, you could just use `python3 setup.py build`
and then manually install the library system-wide.

I might make a binary wheel for linux/amd64 available later on PyPI if it proves useful.

## Usage

There is only one function in the library: `detect_language`, which accepts a
single unicode string and returns a pair of strings or raises a `ValueError` on failure.

The strings returned correspond to the three-character language code, and three-character
script code. These are enumerated in [Whatlang's docs][wldocs] as the Enums `Lang` and
`Script` and are returned as strings in the same leading-uppercase format.

[orig]: https://github.com/greyblake/whatlang-rs
[sergey]: https://github.com/greyblake
[rust]: http://rustup.rs
[wldocs]: https://docs.rs/whatlang/0.3.1/whatlang
