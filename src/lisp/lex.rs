pub fn tokenize(input : &str) -> Vec<String> {
    let formatted = format_braces(input);
    let mut tokens : Vec<String> = Vec::new();

    for token in formatted.split(' ') {
        tokens.push(token.trim().to_string());
    }

    tokens
}

fn format_braces(input : &str) -> String {
    input.replace("(", " ( ").replace(")", " ) ").trim().to_string()
}

#[test]
fn braces_are_formatted() {
    let input = "(())";
    let s = format_braces(input);
    assert_eq!(s, " ( ( ) ) ");
}
