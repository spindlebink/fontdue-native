# Fontdue Native

A C FFI for [Fontdue](https://github.com/mooman219/fontdue).

Fontdue is an ultra-fast, easy-to-use font renderer written in Rust. The code in this repository allows you to write bindings for it in any language with C interoperability.

* [Codeberg repo](https://codeberg.org/spindlebink/fontdue-native)
* [GitHub mirror](https://github.com/spindlebink/fontdue-native)

## Bindings

* [Odin bindings](https://codeberg.org/spindlebink/fontdue-odin)
* [Scopes bindings](https://github.com/ScopesCommunity/eo-packages/blob/main/bindings/fontdue.sc)

## Status

The C interface should be complete. Running `cargo build` will generate a C header in `ffi/fontdue.h` which includes documentation. The API is pretty self explanatory.

## Wrapping Conventions

In general, I've stuck closely to Fontdue's original naming. I've translated struct functions using the C library naming convention of `lib_struct_operation`, using in most cases a direct copy of the original function's name. For example, `fontdue::Font.rasterize_indexed(index, px)` has become `ftd_font_rasterize_indexed(font, index, px, &bitmap)`.

A couple of exceptions to the 1-1 naming are:

* The `ftd_font_new_from_bytes` function, which wraps `Font::from_bytes`. I added `new` to better telegraph that the function allocates for the structure.

For return values that don't map easily to C--tuples and structs, generally--I've taken a pointer to a struct instead and populated it with the results of the corresponding Fontdue call.

A couple of functions in Fontdue return an `Option`, and for these I use a pointer argument and return a `bool` for whether the function would've returned `Some`.

## License

Apache or MIT.
