use std::collections::HashSet;

pub const __TOKENS: [&str; 47] = [
    "//", "*/", "/*", "(", ")", "{", "}", "[", "]", ":", ";", ",", "=", "&", "*", "..", "let",
    "var", "const", "struct", "qbit", "void", "#", "macro", "gate", "H", "PX", "PY", "PZ", "CNT",
    "CY", "ID", "TOF", "RX", "RY", "RZ", "S", "T", "SDG", "TDG", ".", "if", "for", "in", "return",
    "break", "$",
];

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token: i32,
    pub value: String,
}

pub fn rm_comments(input: &str) -> String {
    let mut output = String::new();
    let mut in_comment = false;
    let mut in_multiline_comment = false;
    let mut i = 0;

    while i < input.len() {
        if in_multiline_comment {
            if i + 1 < input.len() && &input[i..i + 2] == "*/" {
                in_multiline_comment = false;
                i += 2;
            } else {
                i += 1;
            }
        } else if in_comment {
            if input.chars().nth(i).unwrap() == '\n' {
                in_comment = false;
                output.push('\n');
            }
            i += 1;
        } else if i + 1 < input.len() && &input[i..i + 2] == "/*" {
            in_multiline_comment = true;
            i += 2;
        } else if i + 1 < input.len() && &input[i..i + 2] == "//" {
            in_comment = true;
            i += 2;
        } else {
            output.push(input.chars().nth(i).unwrap());
            i += 1;
        }
    }

    output
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let input = rm_comments(input);
    let mut tokens: Vec<Token> = Vec::new();
    let mut i = 0;
    let mut _str = String::new();

    while i < input.len() {
        let mut found = false;
        let mut longest_match_len = 0;
        let mut longest_match_token = None;

        // Check for predefined tokens
        for (j, token) in __TOKENS.iter().enumerate() {
            if i + token.len() <= input.len() && &input[i..i + token.len()] == *token {
                if token.len() > longest_match_len {
                    longest_match_len = token.len();
                    longest_match_token = Some((j as i32, token));
                }
                found = true;
            }
        }

        if found {
            if let Some((token_index, token_value)) = longest_match_token {
                if !_str.is_empty() {
                    tokens.push(Token {
                        token: 50,
                        value: _str.clone(),
                    });
                    _str.clear();
                }
                tokens.push(Token {
                    token: token_index,
                    value: token_value.to_string(),
                });
                i += token_value.len();
            }
        } else {
            let current_char = input.chars().nth(i).unwrap();
            if current_char.is_whitespace()
                || "!@#$%^&*()-=+[]{}|;:'\",.<>?/".contains(current_char)
            {
                if !_str.is_empty() {
                    tokens.push(Token {
                        token: 50,
                        value: _str.clone(),
                    });
                    _str.clear();
                }
                if !current_char.is_whitespace() {
                    tokens.push(Token {
                        token: 55, // Assign a specific token for punctuation if needed
                        value: current_char.to_string(),
                    });
                }
                i += 1;
            } else {
                _str.push(current_char);
                i += 1;
            }
        }
    }

    if !_str.is_empty() {
        tokens.push(Token {
            token: 50,
            value: _str,
        });
    }

    tokens
}

pub fn is_num(s: String) -> bool {
    s.parse::<f64>().is_ok()
}

pub fn filter50s(mut tokens: Vec<Token>) -> Vec<Token> {
    let mut lookup: HashSet<(u32, String)> = HashSet::new();
    let mut num_lookup: HashSet<String> = HashSet::new();
    let mut current_scope: u32 = 0;
    for tok in tokens.iter_mut() {
        match tok.token {
            5 => current_scope += 1,
            6 => {
                current_scope -= 1;
                lookup = lookup
                    .iter()
                    .filter(|x| x.0 <= current_scope)
                    .map(|x| x.to_owned())
                    .collect();
            }
            50 => {
                if lookup.iter().position(|s| s.1 == tok.value).is_some() {
                    tok.token = 51; // is a reference, not declaration
                } else if num_lookup.contains(&tok.value) {
                    tok.token = 52;
                } else {
                    if !is_num(tok.value.clone()) {
                        lookup.insert((current_scope, tok.value.clone()));
                    } else {
                        num_lookup.insert(tok.value.clone());
                    }
                }
            }
            _ => {}
        }
    }
    tokens
}
