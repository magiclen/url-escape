URL Escape
====================

[![CI](https://github.com/magiclen/url-escape/actions/workflows/ci.yml/badge.svg)](https://github.com/magiclen/url-escape/actions/workflows/ci.yml)

This library is for encoding/escaping special characters in URLs and decoding/unescaping URLs as well.

## Usage

### Encoding

This crate provides some `encode_*` functions to encode URL text in different situations.

For example, to put a text to a fragment, use the `encode_fragment` function.

```rust
extern crate url_escape;

assert_eq!("a%20%3E%20b?", url_escape::encode_fragment("a > b?"));
```

The functions suffixed with `_to_writer`, `_to_vec` or `_to_string` are useful to generate URL text.

```rust
extern crate url_escape;

let mut url = String::from("https://");
assert_eq!("admin%40example.com", url_escape::encode_userinfo_to_string("admin@example.com", &mut url));
url.push_str("@127.0.0.1/");
assert_eq!("%E4%B8%AD%E6%96%87%E5%AD%97/eng/12%2034", url_escape::encode_path_to_string("中文字/eng/12 34", &mut url));
url.push('/');
assert_eq!(r"56%2F78", url_escape::encode_component_to_string("56/78", &mut url));
url.push('?');
assert_eq!(r"a=1&b=a%20b%20c", url_escape::encode_query_to_string("a=1&b=a b c", &mut url));

assert_eq!("https://admin%40example.com@127.0.0.1/%E4%B8%AD%E6%96%87%E5%AD%97/eng/12%2034/56%2F78?a=1&b=a%20b%20c", url);
```

### Decoding

```rust
extern crate url_escape;

assert_eq!("中文字/eng/12 34", url_escape::decode("%E4%B8%AD%E6%96%87%E5%AD%97/eng/12%2034"));
```

## Crates.io

https://crates.io/crates/url-escape

## Documentation

https://docs.rs/url-escape

## License

[MIT](LICENSE)