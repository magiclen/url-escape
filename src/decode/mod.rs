use std::{
    borrow::Cow,
    io::{self, Write},
    str::from_utf8_unchecked,
};

use crate::percent_encoding::percent_decode_str;

/// Decode percent-encoded bytes in a given string.
#[inline]
pub fn decode<S: ?Sized + AsRef<str>>(text: &S) -> Cow<str> {
    let pd = percent_decode_str(text.as_ref());

    pd.decode_utf8_lossy()
}

/// Decode percent-encoded bytes in a given string to a mutable `String` reference and return the decoded string slice.
#[inline]
pub fn decode_to_string<S: AsRef<str>>(text: S, output: &mut String) -> &str {
    unsafe { from_utf8_unchecked(decode_to_vec(text, output.as_mut_vec())) }
}

/// Decode percent-encoded bytes in a given string to a mutable `Vec<u8>` reference and return the decoded data slice.
#[inline]
pub fn decode_to_vec<S: AsRef<str>>(text: S, output: &mut Vec<u8>) -> &[u8] {
    let text = text.as_ref();
    let text_bytes = text.as_bytes();
    let text_length = text_bytes.len();

    output.reserve(text_length);

    let current_length = output.len();

    let pd = percent_decode_str(text);

    output.extend(pd);

    &output[current_length..]
}

/// Decode percent-encoded bytes in a given string to a writer.
#[inline]
pub fn decode_to_writer<S: AsRef<str>, W: Write>(text: S, output: &mut W) -> Result<(), io::Error> {
    let pd = percent_decode_str(text.as_ref());

    for s in pd {
        output.write_all(&[s])?;
    }

    Ok(())
}
