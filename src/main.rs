use std::iter;

#[derive(Debug)]
enum Token {
    Number(i64),
    Plus,
    Dash,
    Star,
    Slash,
    LeftParen,
    RightParen,
    EOF,
}

#[derive(Debug)]
struct SyntaxError {
    message: String,
}

impl SyntaxError {
    fn new(message: String) -> Self {
        SyntaxError { message }
    }
}

fn tokenizer(input: String) -> Result<Vec<Token>, SyntaxError> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut iter = input.chars().peekable();
    while let Some(ch) = iter.next() {
        match ch {
            ch if ch.is_whitespace() => continue,
            '(' => tokens.push(Token::LeftParen),
            ')' => tokens.push(Token::RightParen),
            '+' => tokens.push(Token::Plus),
            '-' => tokens.push(Token::Dash),
            '*' => tokens.push(Token::Star),
            '/' => tokens.push(Token::Slash),
            '0'..='9' => {
                let n: i64 = iter::once(ch)
                    .chain(iter::from_fn(|| {
                        iter.by_ref().next_if(|s| s.is_ascii_digit())
                    }))
                    .collect::<String>()
                    .parse()
                    .unwrap();
                tokens.push(Token::Number(n));
            }
            _ => return Err(SyntaxError::new(format!("unrecognized character {}", ch))),
        }
    }
    tokens.push(Token::EOF);
    Ok(tokens)
}

fn main() {
    let result = tokenizer("2 + 2 * 2".to_string());
    println!("{:?}", result);
}
