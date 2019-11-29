# nix-base32

[![Crate](https://img.shields.io/crates/v/nix-base32.svg)](https://crates.io/crates/nix-base32)

This crate encodes a `[u8]` byte slice in a nix-compatible way.
SHA256 hash codes in [nix](https://nixos.org/nix/) are usually encoded in base32 with
an unusual set of characters (without E O U T).

```rust
    assert_eq!(
        to_nix_base32(
            &hex::decode("ab335240fd942ab8191c5e628cd4ff3903c577bda961fb75df08e0303a00527b")
                .unwrap()
        ),
        "0ysj00x31q08vxsznqd9pmvwa0rrzza8qqjy3hcvhallzm054cxb"
    );
```
