rust-du
-------

This is just an implementation of `du -sb` in Rust so that I can play around
with learning Rust.

Some things that doing this project allowed me touch on in Rust:

- [Error Handling](https://doc.rust-lang.org/book/error-handling.html)

    - with the `try!` macro calls in lib.rs
    - with pattern matching inside the main function
    - and some `unwrap`s, sprinkled around the place

- Argument Parsing

    - with [getopts](https://doc.rust-lang.org/getopts/getopts/index.html)

- Generic Code

    - using [AsRef](https://doc.rust-lang.org/book/borrow-and-asref.html)

- Testing

- Packaging
