use std::env;
use std::fs;
use std::path::Path;

#[derive(Debug, PartialEq)]
pub enum Token {
    Identifier(String),
    Number(i64),
    Hash,
    LessThan,
    BiggerThan,
    Dot,
    Plus,
    Minus,
    Multiply,
    Divide,
    Equal,
    Semicolon,
    LeftBrace,
    RightBrace,
    LeftParen,
    RightParen,
    If,
    Else,
    Return,
    EOF, // End of File
}

pub struct Lexer<'a> {
    input: &'a str,
    position: usize,
    current_char: Option<char>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Lexer {
            input,
            position: 0,
            current_char: None,
        };
        lexer.current_char = lexer.next_char(); // Inicializa o primeiro caractere
        lexer
    }

    fn next_char(&mut self) -> Option<char> {
        if self.position < self.input.len() {
            let ch = self.input[self.position..].chars().next().unwrap();
            self.position += ch.len_utf8();
            Some(ch)
        } else {
            None
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current_char {
            if ch.is_whitespace() {
                self.current_char = self.next_char();
            } else {
                break;
            }
        }
    }

    fn keyword(&mut self, word: &str) -> Option<Token> {
        match word {
            "if" => Some(Token::If),
            "else" => Some(Token::Else),
            "return" => Some(Token::Return),
            _ => None,
        }
    }

    pub fn next_token(&mut self) -> Token {
        while let Some(ch) = self.current_char {
            match ch {
                '0'..='9' => return Token::Number(self.integer()),
                '#' => {
                    self.current_char = self.next_char();
                    return Token::Hash;
                }
                '<' => {
                    self.current_char = self.next_char();
                    return Token::LessThan;
                }
                '>' => {
                    self.current_char = self.next_char();
                    return Token::BiggerThan;
                }
                '.' => {
                    self.current_char = self.next_char();
                    return Token::Dot;
                }
                '+' => {
                    self.current_char = self.next_char();
                    return Token::Plus;
                }
                '-' => {
                    self.current_char = self.next_char();
                    return Token::Minus;
                }
                '=' => {
                    self.current_char = self.next_char();
                    return Token::Equal;
                }
                '*' => {
                    self.current_char = self.next_char();
                    return Token::Multiply;
                }
                '/' => {
                    self.current_char = self.next_char();
                    return Token::Divide;
                }
                ';' => {
                    self.current_char = self.next_char();
                    return Token::Semicolon;
                }
                '{' => {
                    self.current_char = self.next_char();
                    return Token::LeftBrace;
                }
                '}' => {
                    self.current_char = self.next_char();
                    return Token::RightBrace;
                }
                '(' => {
                    self.current_char = self.next_char();
                    return Token::LeftParen;
                }
                ')' => {
                    self.current_char = self.next_char();
                    return Token::RightParen;
                }
                _ => {
                    if ch.is_whitespace() {
                        self.skip_whitespace();
                        continue;
                    } else if ch.is_alphabetic() || ch == '_' {
                        // Identificadores e palavras-chave
                        let ident = self.identifier();

                        let kw = self.keyword(ident.as_str());
                        match kw {
                            Some(kw) => return kw,
                            None => return Token::Identifier(ident),
                        }
                    } else {
                        panic!("Unexpected character: {}", ch);
                    }
                }
            }
        }
        Token::EOF // Retorna EOF quando não há mais caracteres
    }

    fn integer(&mut self) -> i64 {
        let start_pos = self.position - 1; // Posição atual é o próximo caractere
        while let Some(ch) = self.current_char {
            if ch.is_digit(10) {
                self.current_char = self.next_char();
            } else {
                break;
            }
        }
        let num_str = &self.input[start_pos..self.position - 1];
        num_str.parse::<i64>().unwrap()
    }

    fn identifier(&mut self) -> String {
        let start_pos = self.position - 1; // Posição atual é o próximo caractere
        while let Some(ch) = self.current_char {
            if ch.is_alphanumeric() || ch == '_' {
                self.current_char = self.next_char();
            } else {
                break;
            }
        }
        String::from(&self.input[start_pos..self.position - 1])
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];
    let path = Path::new(&file_path);
    let input = fs::read_to_string(path).expect("Could not read file");

    let mut lexer = Lexer::new(&input);

    loop {
        let token = lexer.next_token();
        println!("{:?}", token);
        if token == Token::EOF {
            break;
        }
    }
}
