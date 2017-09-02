Odbc-safe
=========

[![Build Status](https://travis-ci.org/pacman82/odbc-safe.svg?branch=master)](https://travis-ci.org/pacman82/odbc-safe)
[![Docs](https://docs.rs/odbc-safe/badge.svg)](https://docs.rs/odbc-safe/)
[![MIT licensed](https://img.shields.io/github/license/mashape/apistatus.svg)](https://github.com/pacman82/odbc-safe/blob/master/LICENSE)
[![Published](http://meritbadge.herokuapp.com/odbc-safe)](https://crates.io/crates/odbc-safe)

Introduction
------------

This Rust crate builds upon the FFI (Foreign Function Interface) bindings for [ODBC] (Open Database
Connectivity) provided by [odbc-sys]. It enables you to write ODBC Applications entirely in safe
Rust. While this crate tries to prevent all kinds of erros it does very little to hide the
complexity of ODBC, as it tries to be the smallest safe layer around [odbc-sys]. Therfore code
written with this library is likely to be safe, but unlikely to look like idiomatic Rust. For a
library offering greater convinience have a look at [odbc-rs].

Design Goals
------------

* Zero cost abstraction
* Catch Invalid Handle Errors at compile time
* Prevent bound buffers or columns from going out of scope
* Catch Function Sequence Errors at compile time
* Not to abstract away any power of the underlying API

Current State
-------------

This library currently supports:

* Direct execution of queries
* Prepared execution of queries
* Binding parameters
* Retrieving Result Sets (kind of slow via `SQLGetData`)

Currently unsupported:

* Binding columns to Result Sets
* Multi threading and asynchronous capabilities

Documentation
-------------

Thanks to the folks of [docs.rs] for building and hosting the [documentation]!

Contributing
------------

Want to help out? Just create an issue, pull request or contact markus.klein@live.de.

[ODBC]: https://docs.microsoft.com/en-us/sql/odbc/microsoft-open-database-connectivity-odbc
[docs.rs]: https://docs.rs
[documentation]: https://docs.rs/odbc-safe/
[odbc-sys]: https://github.com/pacman82/odbc-sys
[odbc-rs]: https://github.com/koka/odbc-rs