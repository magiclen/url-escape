extern crate url_escape;

const TEXT_WWW_FORM_URLENCODED_CASES: [(&str, &str); 7] = [
    ("", ""),
    ("%E5%93%88%E5%9B%89%EF%BC%8C%E4%B8%AD%E6%96%87%EF%BC%81", "哈囉，中文！"),
    ("%E9%BA%B5%E5%8C%85%20%26%20butter", "麵包 & butter"),
    ("%22bread%22%20%26%20%E5%A5%B6%E6%B2%B9", "\"bread\" & 奶油"),
    ("%3C%20less%20than", "< less than"),
    ("greater%20than%20%3E", "greater than >"),
    ("https%3A%2F%2Fmagiclen.org", "https://magiclen.org"),
];

#[test]
fn encode_www_form_urlencoded() {
    for (expect, text) in TEXT_WWW_FORM_URLENCODED_CASES.iter().copied() {
        assert_eq!(expect, url_escape::encode_www_form_urlencoded(text));
    }
}

#[test]
fn encode_www_form_urlencoded_to_string() {
    for (expect, text) in TEXT_WWW_FORM_URLENCODED_CASES.iter().copied() {
        assert_eq!(
            expect,
            url_escape::encode_www_form_urlencoded_to_string(text, &mut String::new())
        );
    }
}

#[test]
fn encode_www_form_urlencoded_to_writer() {
    for (expect, text) in TEXT_WWW_FORM_URLENCODED_CASES.iter().copied() {
        let mut v = Vec::new();
        url_escape::encode_www_form_urlencoded_to_writer(text, &mut v).unwrap();

        assert_eq!(expect.as_bytes(), v.as_slice());
    }
}

#[test]
fn decode_www_form_urlencoded() {
    for (text, expect) in TEXT_WWW_FORM_URLENCODED_CASES.iter().copied() {
        assert_eq!(expect, url_escape::decode(text));
    }
}

#[test]
fn decode_www_form_urlencoded_to_string() {
    for (text, expect) in TEXT_WWW_FORM_URLENCODED_CASES.iter().copied() {
        assert_eq!(expect, url_escape::decode_to_string(text, &mut String::new()));
    }
}

#[test]
fn decode_www_form_urlencoded_to_writer() {
    for (text, expect) in TEXT_WWW_FORM_URLENCODED_CASES.iter().copied() {
        let mut v = Vec::new();
        url_escape::decode_to_writer(text, &mut v).unwrap();

        assert_eq!(expect.as_bytes(), v.as_slice());
    }
}
