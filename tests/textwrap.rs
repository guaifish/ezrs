use ezrs::*;

#[test]
fn test_textwrap() {
    let text = "textwrap: a small library for wrapping text.";
    let result = "textwrap: a small\nlibrary for\nwrapping text.";
    assert_eq!(textwrap::fill(text, 18), result);
}
