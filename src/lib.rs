//! # Encodes `[u8]` as base32 like nix
//!
//! This crate encodes a `[u8]` byte slice in a nix-compatible way.
//! SHA256 hash codes in [nix](https://nixos.org/nix/) are usually encoded in base32 with
//! an unusual set of characters (without E O U T).

// omitted: E O U T
const BASE32_CHARS: &[u8] = b"0123456789abcdfghijklmnpqrsvwxyz";

/// Converts the given byte slice to a nix-compatible base32 encoded String.
pub fn to_nix_base32(bytes: &[u8]) -> String {
    let len = (bytes.len() * 8 - 1) / 5 + 1;

    (0..len)
        .rev()
        .map(|n| {
            let b: usize = (n as usize) * 5;
            let i: usize = b / 8;
            let j: usize = b % 8;
            // bits from the lower byte
            let v1 = bytes[i].checked_shr(j as u32).unwrap_or(0);
            // bits from the upper byte
            let v2 = if i >= bytes.len() - 1 {
                0
            } else {
                bytes[i + 1].checked_shl(8 - j as u32).unwrap_or(0)
            };
            let v: usize = (v1 | v2) as usize;
            char::from(BASE32_CHARS[v % BASE32_CHARS.len()])
        })
        .collect()
}

#[test]
fn test_to_nix_base32() {
    assert_eq!(
        to_nix_base32(
            &hex::decode("ab335240fd942ab8191c5e628cd4ff3903c577bda961fb75df08e0303a00527b")
                .unwrap()
        ),
        "0ysj00x31q08vxsznqd9pmvwa0rrzza8qqjy3hcvhallzm054cxb"
    );
}
