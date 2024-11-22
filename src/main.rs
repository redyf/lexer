use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Identifier(String),
    Number(i64),
    String(String),
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
    Ampersand,
    Exclamation,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    LeftParen,
    RightParen,
    Circumflex,
    QuestionMark,
    DollarSign,
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
        lexer.next_char(); // Inicializa o primeiro caractere
        lexer
    }

    fn next_char(&mut self) -> Option<char> {
        if self.position < self.input.len() {
            let ch = self.input[self.position..].chars().next().unwrap();
            self.position += ch.len_utf8();

            self.current_char = Some(ch);
            self.current_char
        } else {
            self.current_char = None;
            self.current_char
        }
    }

    fn skip_line(&mut self) -> () {
        while let Some(ch) = self.current_char {
            self.next_char();
            if ch == '\n' {
                break;
            }
        }
    }

    fn skip_multiline_comment(&mut self) -> () {
        self.next_char();
        while let Some(ch) = self.next_char() {
            if ch == '*' {
                if let Some('/') = self.seek_offset(1) {
                    self.next_char();
                    self.next_char();
                    break;
                }
            }
        }
    }

    fn seek_offset(&self, offset: usize) -> Option<char> {
        self.input.chars().nth(self.position + offset - 1)
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current_char {
            if ch.is_whitespace() {
                if ch == '\n' {
                    self.line_number += 1;
                }
                self.next_char();
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

    fn read_string(&mut self) -> String {
        self.current_char = self.next_char();
        let start_pos = self.position - 1; // Posição atual é o próximo caractere
        while let Some(ch) = self.current_char {
            self.current_char = self.next_char();
            if ch == '"' {
                break;
            }

            // pula o próximo caractere caso seja uma sequência de escape
            if ch == '\\' {
                self.current_char = self.next_char();
            }
        }
        let str_lit = &self.input[start_pos..self.position - 2];
        String::from(str_lit)
    }

    pub fn next_token(&mut self) -> Token {
        while let Some(ch) = self.current_char {
            match ch {
                '0'..='9' => return Token::Number(self.integer()),
                '#' => {
                    self.skip_line();
                }
                '<' => {
                    self.next_char();
                    return Token::LessThan;
                }
                '>' => {
                    self.next_char();
                    return Token::BiggerThan;
                }
                '.' => {
                    self.next_char();
                    return Token::Dot;
                }
                '+' => {
                    self.next_char();
                    return Token::Plus;
                }
                '-' => {
                    self.next_char();
                    return Token::Minus;
                }
                '=' => {
                    self.next_char();
                    return Token::Equal;
                }
                '*' => {
                    self.next_char();
                    return Token::Multiply;
                }
                '/' => {
                    let nch = self.seek_offset(1);

                    // pula o restante da linha se encontrar um comentário
                    if let Some('/') = nch {
                        self.skip_line();
                        continue;
                    }

                    if let Some('*') = nch {
                        self.skip_multiline_comment();
                        continue;
                    }

                    self.next_char();
                    return Token::Slash;
                }
                '\\' => {
                    self.next_char();
                    return Token::BackwardSlash;
                }
                '|' => {
                    self.next_char();
                    return Token::Pipe;
                }
                ':' => {
                    self.next_char();
                    return Token::Colon;
                }
                ';' => {
                    self.next_char();
                    return Token::Semicolon;
                }
                ',' => {
                    self.next_char();
                    return Token::Comma;
                }
                '%' => {
                    self.next_char();
                    return Token::Percent;
                }
                '\'' => {
                    self.next_char();
                    return Token::SingleQuotationMark;
                }
                '"' => {
                    return Token::String(self.read_string());
                }
                '&' => {
                    self.next_char();
                    return Token::Ampersand;
                }
                '!' => {
                    self.next_char();
                    return Token::Exclamation;
                }
                '[' => {
                    self.next_char();
                    return Token::LeftBracket;
                }
                ']' => {
                    self.next_char();
                    return Token::RightBracket;
                }
                '{' => {
                    self.next_char();
                    return Token::LeftBrace;
                }
                '}' => {
                    self.next_char();
                    return Token::RightBrace;
                }
                '(' => {
                    self.next_char();
                    return Token::LeftParen;
                }
                ')' => {
                    self.next_char();
                    return Token::RightParen;
                }
                '^' => {
                    self.next_char();
                    return Token::Circumflex;
                }
                '?' => {
                    self.next_char();
                    return Token::QuestionMark;
                }
                '$' => {
                    self.next_char();
                    return Token::DollarSign;
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
                        self.symbol_table
                            .add(ident.clone(), Token::Identifier(ident.clone()));
                        Token::Identifier(ident.clone())
                    };
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
                self.next_char();
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
                self.next_char();
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
