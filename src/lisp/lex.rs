// use regex::Regex;

pub fn tokenize(input : &str) -> Vec<String> {
    let formatted = format_braces(input);
    let mut tk : Vec<String> = Vec::new();

    for token in formatted.split(' ') {
        tk.push(token.to_string());
    }

    tk
}

fn format_braces(input : &str) -> String {
    // let re = Regex::new(r"(?P<left>\S\(\S|?P<right>\S\)\S)");
    input.replace("(", " ( ").replace(")", " ) ").trim().to_string()
}


#[test]
fn braces_are_formatted() {
    let input = "(())";
    let s = format_braces(input);
    assert_eq!(s, " ( ( ) ) ");
}
