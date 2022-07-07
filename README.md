# Fontdue Native

The beginnings of a C FFI for [Fontdue](https://github.com/mooman219/fontdue).

Fontdue is an ultra-fast, easy-to-use font renderer written in Rust. The code in this repository allows you to write bindings for it in any language with C interoperability.

* [Codeberg repo](https://codeberg.org/spindlebink/fontdue-native)
* [GitHub mirror](https://github.com/spindlebink/fontdue-native)

## Status

The interface is very much incomplete. I've bound the Font struct and its associated functions and nothing else.

To-dos, besides wrapping the rest of the library:
* Remove dependencies on stdlib, since Fontdue doesn't need it
* Write documentation on how the calls have been translated
* Pull stuff in `font.rs` out into multiple files, find a home for structs, in general organize the code better
* Improve building so we don't need manual `cbindgen`

## License

Apache or MIT.
