use std::{
    borrow::Cow,
    io::{self, Write},
};

use crate::percent_encoding::percent_decode_str;

/// Decode percent-encoded bytes in a given string.
#[inline]
pub fn decode<S: ?Sized + AsRef<str>>(text: &S) -> Cow<'_, str> {
    let pd = percent_decode_str(text.as_ref());

    pd.decode_utf8_lossy()
}

/// Decode percent-encoded bytes in a given string to a mutable `String` reference with lossy UTF-8 and return the decoded string slice.
#[inline]
pub fn decode_to_string<S: AsRef<str>>(text: S, output: &mut String) -> &str {
    let current_length = output.len();
    let pd = percent_decode_str(text.as_ref());

    output.push_str(&pd.decode_utf8_lossy());

    &output[current_length..]
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
    let decoded: Cow<'_, [u8]> = Cow::from(pd);

    output.write_all(decoded.as_ref())?;

    Ok(())
}

/// Decode a www-form-urlencoded text, converting `+` to SPACE before percent-decoding.
#[inline]
pub fn decode_www_form_urlencoded<S: ?Sized + AsRef<str>>(text: &S) -> Cow<'_, str> {
    let text = text.as_ref();

    if text.as_bytes().contains(&b'+') {
        let mut output = String::new();

        decode_www_form_urlencoded_to_string(text, &mut output);

        Cow::Owned(output)
    } else {
        decode(text)
    }
}

/// Decode a www-form-urlencoded text to a mutable `String` reference and return the decoded string slice.
#[inline]
pub fn decode_www_form_urlencoded_to_string<S: AsRef<str>>(text: S, output: &mut String) -> &str {
    let current_length = output.len();
    let mut v = Vec::new();

    decode_www_form_urlencoded_to_vec(text, &mut v);
    output.push_str(&String::from_utf8_lossy(&v));

    &output[current_length..]
}

/// Decode a www-form-urlencoded text to a mutable `Vec<u8>` reference and return the decoded data slice.
#[inline]
pub fn decode_www_form_urlencoded_to_vec<S: AsRef<str>>(text: S, output: &mut Vec<u8>) -> &[u8] {
    let text = text.as_ref();
    let text_bytes = text.as_bytes();

    if !text_bytes.contains(&b'+') {
        if !text_bytes.contains(&b'%') {
            let current_length = output.len();

            output.extend_from_slice(text_bytes);

            return &output[current_length..];
        }

        return decode_to_vec(text, output);
    }

    output.reserve(text.len());

    let current_length = output.len();

    for (index, part) in text.split('+').enumerate() {
        if index > 0 {
            output.push(b' ');
        }

        let pd = percent_decode_str(part);

        output.extend(pd);
    }

    &output[current_length..]
}

/// Decode a www-form-urlencoded text to a writer.
#[inline]
pub fn decode_www_form_urlencoded_to_writer<S: AsRef<str>, W: Write>(
    text: S,
    output: &mut W,
) -> Result<(), io::Error> {
    for (index, part) in text.as_ref().split('+').enumerate() {
        if index > 0 {
            output.write_all(b" ")?;
        }

        let pd = percent_decode_str(part);

        for s in pd {
            output.write_all(&[s])?;
        }
    }

    Ok(())
}
