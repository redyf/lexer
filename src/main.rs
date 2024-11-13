use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;

#[derive(Debug, PartialEq, Clone)]
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
    Slash,
    BackwardSlash,
    Pipe,
    Equal,
    Colon,
    Semicolon,
    Comma,
    Percent,
    SingleQuotationMark,
    DoubleQuotationMark,
    Ampersand,
    Exclamation,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    LeftParen,
    RightParen,
    EOF, // End of File

    // Palavras Chave
    Auto,
    Const,
    Default,
    Extern,
    Goto,
    Register,
    Signed,
    Sizeof,
    Static,
    Union,
    Unsigned,
    Volatile,
    If,
    Else,
    Return,
    Do,
    While,
    For,
    Switch,
    Case,
    Break,
    Continue,
    Enum,
    Struct,
    Typedef,
    Int,
    Long,
    Short,
    Char,
    Float,
    Double,
    Void,
}

#[derive(Default)]
pub struct SymbolTable {
    symbols: HashMap<String, Token>,
}

impl SymbolTable {
    fn add(&mut self, identifier: String, token: Token) {
        self.symbols.entry(identifier).or_insert(token);
    }

    fn display(&self) {
        for (idx, (name, token)) in self.symbols.iter().enumerate() {
            println!("ID, {:02} | {}: {:?}", idx + 1, name, token);
        }
    }
}

pub struct Lexer {
    input: String,
    position: usize,
    current_char: Option<char>,
    line_number: usize,
    symbol_table: SymbolTable,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut lexer = Lexer {
            input,
            position: 0,
            current_char: None,
            line_number: 1,
            symbol_table: SymbolTable::default(),
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
                if ch == '\n' {
                    self.line_number += 1;
                }
                self.current_char = self.next_char();
            } else {
                break;
            }
        }
    }

    fn keyword(&mut self, word: &str) -> Option<Token> {
        match word {
            "auto" => Some(Token::Auto),
            "const" => Some(Token::Const),
            "default" => Some(Token::Default),
            "extern" => Some(Token::Extern),
            "goto" => Some(Token::Goto),
            "register" => Some(Token::Register),
            "signed" => Some(Token::Signed),
            "sizeof" => Some(Token::Sizeof),
            "static" => Some(Token::Static),
            "union" => Some(Token::Union),
            "unsigned" => Some(Token::Unsigned),
            "volatile" => Some(Token::Volatile),
            "if" => Some(Token::If),
            "else" => Some(Token::Else),
            "return" => Some(Token::Return),
            "do" => Some(Token::Do),
            "while" => Some(Token::While),
            "for" => Some(Token::For),
            "switch" => Some(Token::Switch),
            "case" => Some(Token::Case),
            "break" => Some(Token::Break),
            "continue" => Some(Token::Continue),
            "enum" => Some(Token::Enum),
            "struct" => Some(Token::Struct),
            "typedef" => Some(Token::Typedef),
            "int" => Some(Token::Int),
            "long" => Some(Token::Long),
            "short" => Some(Token::Short),
            "char" => Some(Token::Char),
            "float" => Some(Token::Float),
            "double" => Some(Token::Double),
            "void" => Some(Token::Void),
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
                    return Token::Slash;
                }
                '\\' => {
                    self.current_char = self.next_char();
                    return Token::BackwardSlash;
                }
                '|' => {
                    self.current_char = self.next_char();
                    return Token::Pipe;
                }
                ':' => {
                    self.current_char = self.next_char();
                    return Token::Colon;
                }
                ';' => {
                    self.current_char = self.next_char();
                    return Token::Semicolon;
                }
                ',' => {
                    self.current_char = self.next_char();
                    return Token::Comma;
                }
                '%' => {
                    self.current_char = self.next_char();
                    return Token::Percent;
                }
                '\'' => {
                    self.current_char = self.next_char();
                    return Token::SingleQuotationMark;
                }
                '"' => {
                    self.current_char = self.next_char();
                    return Token::DoubleQuotationMark;
                }
                '&' => {
                    self.current_char = self.next_char();
                    return Token::Ampersand;
                }
                '!' => {
                    self.current_char = self.next_char();
                    return Token::Exclamation;
                }
                '[' => {
                    self.current_char = self.next_char();
                    return Token::LeftBracket;
                }
                ']' => {
                    self.current_char = self.next_char();
                    return Token::RightBracket;
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

                _ if ch.is_whitespace() => {
                    self.skip_whitespace();
                    continue;
                }

                _ if ch.is_alphabetic() || ch == '_' => {
                    let ident = self.identifier();
                    let token = if let Some(keyword_token) = self.keyword(ident.as_str()) {
                        keyword_token
                    } else {
                        Token::Identifier(ident.clone())
                    };
                    self.symbol_table.add(ident, token.clone());
                    return token;
                }

                _ => panic!("Unexpected character: {}", ch),
            }
        }

        Token::EOF // Retorna EOF quando não há mais caracteres
    }

    fn integer(&mut self) -> i64 {
        let start_pos = self.position - 1;
        while let Some(ch) = &self.current_char {
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
        let start_pos = self.position - 1;
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

    if args.len() < 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];

    let path = Path::new(file_path);

    let input = fs::read_to_string(path).expect("Could not read file");

    let mut lexer = Lexer::new(input); // Passa o ownership do String

    loop {
        let token = lexer.next_token();

        println!("{:?}", token);

        if token == Token::EOF {
            break;
        }
    }

    // Display the symbol table
    println!("\nSymbol Table:");
    lexer.symbol_table.display();
}
