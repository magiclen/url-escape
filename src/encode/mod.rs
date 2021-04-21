// Ref: https://url.spec.whatwg.org/

use std::borrow::Cow;
use std::io::{self, Write};
use std::str::from_utf8_unchecked;

use crate::percent_encoding::{utf8_percent_encode, AsciiSet};

/// The C0 control percent-encode set are the C0 controls and U+007F (DEL).
pub use percent_encoding::CONTROLS;

/// Not an ASCII letter or digit.
pub use percent_encoding::NON_ALPHANUMERIC;

/// The fragment percent-encode set is the C0 control percent-encode set and U+0020 SPACE, U+0022 ("), U+003C (<), U+003E (>), and U+0060 (`).
pub const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

/// The query percent-encode set is the C0 control percent-encode set and U+0020 SPACE, U+0022 ("), U+0023 (#), U+003C (<), and U+003E (>).
///
/// The query percent-encode set cannot be defined in terms of the fragment percent-encode set due to the omission of U+0060 (`).
pub const QUERY: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'#').add(b'<').add(b'>');

/// The special-query percent-encode set is the query percent-encode set and U+0027 (').
pub const SPECIAL_QUERY: &AsciiSet = &QUERY.add(b'\'');

/// The path percent-encode set is the query percent-encode set and U+003F (?), U+0060 (`), U+007B ({), and U+007D (}).
pub const PATH: &AsciiSet = &QUERY.add(b'?').add(b'`').add(b'{').add(b'}');

/// The userinfo percent-encode set is the path percent-encode set and U+002F (/), U+003A (:), U+003B (;), U+003D (=), U+0040 (@), U+005B ([) to U+005E (^), inclusive, and U+007C (|).
pub const USERINFO: &AsciiSet = &PATH
    .add(b'/')
    .add(b':')
    .add(b';')
    .add(b'=')
    .add(b'@')
    .add(b'[')
    .add(b'\\')
    .add(b']')
    .add(b'^')
    .add(b'|');

/// The component percent-encode set is the userinfo percent-encode set and U+0024 ($) to U+0026 (&), inclusive, U+002B (+), and U+002C (,).
pub const COMPONENT: &AsciiSet = &USERINFO.add(b'$').add(b'%').add(b'&').add(b'+').add(b',');

/// The application/x-www-form-urlencoded percent-encode set is the component percent-encode set and U+0021 (!), U+0027 (') to U+0029 RIGHT PARENTHESIS, inclusive, and U+007E (~).
pub const X_WWW_FORM_URLENCODED: &AsciiSet =
    &COMPONENT.add(b'!').add(b'\'').add(b'(').add(b')').add(b'~');

/// Encode text.
#[inline]
pub fn encode<'a, S: ?Sized + AsRef<str>>(
    text: &'a S,
    ascii_set: &'static AsciiSet,
) -> Cow<'a, str> {
    Cow::from(utf8_percent_encode(text.as_ref(), ascii_set))
}

/// Write text to a mutable `String` reference and return the encoded string slice.
#[inline]
pub fn encode_to_string<'a, S: AsRef<str>>(
    text: S,
    ascii_set: &'static AsciiSet,
    output: &'a mut String,
) -> &'a str {
    unsafe { from_utf8_unchecked(encode_to_vec(text, ascii_set, output.as_mut_vec())) }
}

/// Write text to a mutable `Vec<u8>` reference and return the encoded data slice.
pub fn encode_to_vec<'a, S: AsRef<str>>(
    text: S,
    ascii_set: &'static AsciiSet,
    output: &'a mut Vec<u8>,
) -> &'a [u8] {
    let text = text.as_ref();
    let text_bytes = text.as_bytes();
    let text_length = text_bytes.len();

    output.reserve(text_length);

    let current_length = output.len();

    let pe = utf8_percent_encode(text, ascii_set);

    output.extend(pe.flat_map(|e| e.bytes()));

    &output[current_length..]
}

/// Write text to a writer.
#[inline]
pub fn encode_to_writer<S: AsRef<str>, W: Write>(
    text: S,
    ascii_set: &'static AsciiSet,
    output: &mut W,
) -> Result<(), io::Error> {
    let pe = utf8_percent_encode(text.as_ref(), ascii_set);

    for s in pe {
        output.write_all(s.as_bytes())?;
    }

    Ok(())
}

macro_rules! encode_impl {
    ($(#[$attr: meta])* $escape_set:ident; $(#[$encode_attr: meta])* $encode_name: ident; $(#[$encode_to_string_attr: meta])* $encode_to_string_name: ident; $(#[$encode_to_vec_attr: meta])* $encode_to_vec_name: ident; $(#[$encode_to_writer_attr: meta])* $encode_to_writer_name: ident $(;)*) => {
        $(#[$encode_attr])*
        ///
        $(#[$attr])*
        #[inline]
        pub fn $encode_name<S: ?Sized + AsRef<str>>(text: &S) -> Cow<str> {
            encode(text, $escape_set)
        }

        $(#[$encode_to_string_attr])*
        ///
        $(#[$attr])*
        #[inline]
        pub fn $encode_to_string_name<S: AsRef<str>>(text: S, output: &mut String) -> &str {
            encode_to_string(text, $escape_set, output)
        }

        $(#[$encode_to_vec_attr])*
        ///
        $(#[$attr])*
        #[inline]
        pub fn $encode_to_vec_name<S: AsRef<str>>(text: S, output: &mut Vec<u8>) -> &[u8] {
            encode_to_vec(text, $escape_set, output)
        }

        $(#[$encode_to_writer_attr])*
        ///
        $(#[$attr])*
        #[inline]
        pub fn $encode_to_writer_name<S: AsRef<str>, W: Write>(text: S, output: &mut W) -> Result<(), io::Error> {
            encode_to_writer(text, $escape_set, output)
        }
    };
}

encode_impl! {
    /// The following characters are escaped:
    ///
    /// C0 controls and,
    ///
    /// * SPACE
    /// * `"`
    /// * `<`
    /// * `>`
    /// * <code>&#096;</code>
    ///
    /// and all code points greater than `~` (U+007E) are escaped.
    FRAGMENT;
    /// Encode text used in a fragment part.
    encode_fragment;
    /// Write text used in a fragment part to a mutable `String` reference and return the encoded string slice.
    encode_fragment_to_string;
    /// Write text used in a fragment part to a mutable `Vec<u8>` reference and return the encoded data slice.
    encode_fragment_to_vec;
    /// Write text used in a fragment part to a writer.
    encode_fragment_to_writer;
}

encode_impl! {
    /// The following characters are escaped:
    ///
    /// C0 controls and,
    ///
    /// * SPACE
    /// * `"`
    /// * `#`
    /// * `<`
    /// * `>`
    ///
    /// and all code points greater than `~` (U+007E) are escaped.
    QUERY;
    /// Encode text used in the query part.
    encode_query;
    /// Write text used in the query part to a mutable `String` reference and return the encoded string slice.
    encode_query_to_string;
    /// Write text used in the query part to a mutable `Vec<u8>` reference and return the encoded data slice.
    encode_query_to_vec;
    /// Write text used in the query part to a writer.
    encode_query_to_writer;
}

encode_impl! {
    /// The following characters are escaped:
    ///
    /// C0 controls and,
    ///
    /// * SPACE
    /// * `"`
    /// * `#`
    /// * `'`
    /// * `<`
    /// * `>`
    ///
    /// and all code points greater than `~` (U+007E) are escaped.
    ///
    /// The term "special" means whether a URL is special. A URL is special is the scheme of that URL is **ftp**, **file** , **http**, **https**, **ws**, or **wss**.
    SPECIAL_QUERY;
    /// Encode text used in the query part.
    encode_special_query;
    /// Write text used in the query part to a mutable `String` reference and return the encoded string slice.
    encode_special_query_to_string;
    /// Write text used in the query part to a mutable `Vec<u8>` reference and return the encoded data slice.
    encode_special_query_to_vec;
    /// Write text used in the query part to a writer.
    encode_special_query_to_writer;
}

encode_impl! {
    /// The following characters are escaped:
    ///
    /// C0 controls and,
    ///
    /// * SPACE
    /// * `"`
    /// * `#`
    /// * `<`
    /// * `>`
    /// * `?`
    /// * <code>&#096;</code>
    /// * `{`
    /// * `}`
    ///
    /// and all code points greater than `~` (U+007E) are escaped.
    PATH;
    /// Encode text used in the path part.
    encode_path;
    /// Write text used in the path part to a mutable `String` reference and return the encoded string slice.
    encode_path_to_string;
    /// Write text used in the path part to a mutable `Vec<u8>` reference and return the encoded data slice.
    encode_path_to_vec;
    /// Write text used in the path part to a writer.
    encode_path_to_writer;
}

encode_impl! {
    /// The following characters are escaped:
    ///
    /// C0 controls and,
    ///
    /// * SPACE
    /// * `"`
    /// * `#`
    /// * `/`
    /// * `:`
    /// * `;`
    /// * `<`
    /// * `=`
    /// * `>`
    /// * `?`
    /// * `@`
    /// * `[`
    /// * `\`
    /// * `]`
    /// * `^`
    /// * <code>&#096;</code>
    /// * `{`
    /// * `}`
    /// * `|`
    ///
    /// and all code points greater than `~` (U+007E) are escaped.
    USERINFO;
    /// Encode text used in the userinfo part.
    encode_userinfo;
    /// Write text used in the userinfo part to a mutable `String` reference and return the encoded string slice.
    encode_userinfo_to_string;
    /// Write text used in the userinfo part to a mutable `Vec<u8>` reference and return the encoded data slice.
    encode_userinfo_to_vec;
    /// Write text used in the userinfo part to a writer.
    encode_userinfo_to_writer;
}

encode_impl! {
    /// The following characters are escaped:
    ///
    /// C0 controls and,
    ///
    /// * SPACE
    /// * `"`
    /// * `#`
    /// * `$`
    /// * `%`
    /// * `&`
    /// * `+`
    /// * `,`
    /// * `/`
    /// * `:`
    /// * `;`
    /// * `<`
    /// * `=`
    /// * `>`
    /// * `?`
    /// * `@`
    /// * `[`
    /// * `\`
    /// * `]`
    /// * `^`
    /// * <code>&#096;</code>
    /// * `{`
    /// * `}`
    /// * `|`
    ///
    /// and all code points greater than `~` (U+007E) are escaped.
    ///
    /// It gives identical results to JavaScript's `encodeURIComponent()`.
    COMPONENT;
    /// Encode text used in a component.
    encode_component;
    /// Write text used in a component to a mutable `String` reference and return the encoded string slice.
    encode_component_to_string;
    /// Write text used in a component to a mutable `Vec<u8>` reference and return the encoded data slice.
    encode_component_to_vec;
    /// Write text used in a component to a writer.
    encode_component_to_writer;
}

encode_impl! {
    /// The following characters are escaped:
    ///
    /// C0 controls and,
    ///
    /// * SPACE
    /// * `!`
    /// * `"`
    /// * `#`
    /// * `$`
    /// * `%`
    /// * `&`
    /// * `'`
    /// * `(`
    /// * `)`
    /// * `+`
    /// * `,`
    /// * `/`
    /// * `:`
    /// * `;`
    /// * `<`
    /// * `=`
    /// * `>`
    /// * `?`
    /// * `@`
    /// * `[`
    /// * `\`
    /// * `]`
    /// * `^`
    /// * <code>&#096;</code>
    /// * `{`
    /// * `}`
    /// * `|`
    /// * `~`
    ///
    /// and all code points greater than `~` (U+007E) are escaped.
    X_WWW_FORM_URLENCODED;
    /// Encode text as a www-form-urlencoded text.
    encode_www_form_urlencoded;
    /// Write text as a urlencoded text to a mutable `String` reference and return the encoded string slice.
    encode_www_form_urlencoded_to_string;
    /// Write text as a www-form-urlencoded text to a mutable `Vec<u8>` reference and return the encoded data slice.
    encode_www_form_urlencoded_to_vec;
    /// Write text as a www-form-urlencoded text to a writer.
    encode_www_form_urlencoded_to_writer;
}
