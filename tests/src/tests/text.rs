use crate::test;
use gooseboy::text::{get_formatted_text_width, get_text_height, get_text_width};

pub fn test_text() {
    test!("text:width-3chars", get_text_width("abc") == 24);
    test!("text:width-empty", get_text_width("") == 0);
    test!("text:height-1line", get_text_height("hello") == 8);
    test!("text:height-3lines", get_text_height("a\nb\nc") == 24);

    let plain = "hello";
    let formatted = "[red]hello";
    test!(
        "text:formatted-equals-plain",
        get_formatted_text_width(plain) == get_formatted_text_width(formatted)
    );

    test!("text:escaped-bracket", get_formatted_text_width("[[") == 8);
}
