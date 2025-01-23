# Poly1305 implementation

Please check the latest version on [GitHub](github.com/leo-leesco/Crypto-TD2). Some bugs might be fixed there :)

## Build

`cargo build` produces `poly1305-gen` and `poly1305-check` in `target/debug`.

If you want the optimized version, run `cargo build --release`, and the executables can then be found in `target/release`.

## Requirements

`poly1305-gen` expects :
- a `r||s` 64 hexadecimal characters key
- the path to the file to authenticate

`poly1305-check` expects :
- a `r||s` 64 hexadecimal characters key
- the path to the file to authenticate
- a 32 hexadecimal characters authenticator
