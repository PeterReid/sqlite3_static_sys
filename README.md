This crate provides raw rust bindings to SQLite 3. 

Building requires GCC. [The GCC package repository](https://github.com/alexcrichton/gcc-rs) explains installation steps for Windows users that do not already have it.

An alternative to this set of bindings is [libsqlite-sys](https://crates.io/crates/libsqlite3-sys), which depends on the sqlite3 library provided by the build system. This makes for a smaller download when compiling, but makes what version of sqlite3 you are linking against uncertain.
