extern crate gcc;

fn main() {
    gcc::compile_library("libsqlite3.a", &["src/sqlite3.c"]);
}