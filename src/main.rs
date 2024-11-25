use lazy_static::lazy_static;
use regex::Regex;
use std::env;
use std::fs;
use std::path::Path;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // Keywords
    Auto,
    Break,
    Case,
    Char,
    Const,
    Continue,
    Default,
    Do,
    Double,
    Else,
    Enum,
    Extern,
    Float,
    For,
    Goto,
    If,
    Int,
    Long,
    Register,
    Return,
    Short,
    Signed,
    Sizeof,
    Static,
    Struct,
    Switch,
    Typedef,
    Union,
    Unsigned,
    Void,
    Volatile,
    While,

    // Identifiers and literals
    Identifier(usize),
    Number(i64),
    String(String),

    // Operators and punctuation
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Assign,
    PlusAssign,
    MinusAssign,
    StarAssign,
    SlashAssign,
    PercentAssign,
    Increment,
    Decrement,
    Equals,
    NotEquals,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    LogicalAnd,
    LogicalOr,
    LogicalNot,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    BitwiseNot,
    LeftShift,
    RightShift,

    // Punctuation
    Semicolon,
    Comma,
    Dot,
    Arrow,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Question,
    Colon,

    // Special tokens
    EOF,
    Error(String),
}

lazy_static! {
    static ref IDENTIFIER_REGEX: Regex = Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*").unwrap();
    static ref NUMBER_REGEX: Regex =
        Regex::new(r"^(?:0[xX][0-9a-fA-F]+|0[0-7]*|[1-9][0-9]*)").unwrap();
    static ref OPERATOR_REGEX: Regex = Regex::new(
        r"^(?:->|>=|<=|==|!=|\+=|-=|\*=|/=|%=|>>|<<|\+\+|--|&&|\|\||[+\-*/%=<>&|^!~?:;,\.\[\]{}()])"
    )
    .unwrap();
    static ref WHITESPACE_REGEX: Regex = Regex::new(r"^[\s\t\n\r]+").unwrap();
    static ref PREPROCESSOR_LINE_REGEX: Regex = Regex::new(r"^#[^\n]*\n?").unwrap();
    static ref STRING_LITERAL_REGEX: Regex = Regex::new(r#"^"(.*)[^\\]""#).unwrap();
}

#[derive(Default)]
pub struct SymbolTable {
    symbols: Vec<String>,
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable {
            symbols: Vec::new(),
        }
    }

    pub fn add(&mut self, identifier: &str) -> usize {
        if self.symbols.contains(&identifier.to_string()) {
            for (i, name) in self.symbols.iter().enumerate() {
                if name == &identifier {
                    return i;
                }
            }
        }
        self.symbols.push(identifier.to_string());
        self.symbols.len()
    }

    pub fn display(&self) {
        for (i, name) in self.symbols.iter().enumerate() {
            println!("ID: {:02} | {}", i + 1, name);
        }
    }
}

pub struct Lexer {
    input: String,
    position: usize,
    line_number: usize,
    symbol_table: SymbolTable,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Lexer {
            input,
            position: 0,
            line_number: 1,
            symbol_table: SymbolTable::new(),
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        self.skip_preprocessor_line();

        let remaining = &self.input[self.position..];
        if remaining.is_empty() {
            return Token::EOF;
        }

        // Check for preprocessor line at the current position
        if remaining.starts_with('#') {
            self.skip_preprocessor_line();
            return self.next_token();
        }

        // Check for multiline comments
        if remaining.starts_with("/*") {
            self.skip_until("/*", "*/");
            return self.next_token(); // Restart tokenization after skipping
        }

        if remaining.starts_with("//") {
            self.skip_until("//", "\n");
            return self.next_token();
        }

        // Match numbers (including hex and octal)
        if let Some(mat) = NUMBER_REGEX.find(remaining) {
            let number_str = &remaining[..mat.end()];
            self.position += mat.end();

            // Parse hex numbers
            if number_str.starts_with("0x") || number_str.starts_with("0X") {
                if let Ok(num) = i64::from_str_radix(&number_str[2..], 16) {
                    return Token::Number(num);
                }
            }
            // Parse octal numbers
            else if number_str.starts_with('0') && number_str.len() > 1 {
                if let Ok(num) = i64::from_str_radix(&number_str[1..], 8) {
                    return Token::Number(num);
                }
            }
            // Parse decimal numbers
            else if let Ok(num) = number_str.parse() {
                return Token::Number(num);
            }
            Token::Error(format!("Invalid number format: {}", number_str))
        }
        // Match identifiers and keywords
        else if let Some(mat) = IDENTIFIER_REGEX.find(remaining) {
            let word = &remaining[..mat.end()];
            self.position += mat.end();

            if let Some(keyword_token) = self.match_keyword(word) {
                keyword_token
            } else {
                let id = self.symbol_table.add(word);
                Token::Identifier(id)
            }
        }
        // Match operators and punctuation
        else if let Some(mat) = OPERATOR_REGEX.find(remaining) {
            let op = &remaining[..mat.end()];
            self.position += mat.end();
            self.match_operator(op)
        } else if let Some(mat) = STRING_LITERAL_REGEX.find(remaining) {
            let lit_str = &remaining[1..mat.end() - 1];
            self.position += mat.end();
            Token::String(lit_str.to_string())
        }
        // Handle unrecognized characters
        else {
            let ch = remaining.chars().next().unwrap();
            self.position += 1;
            Token::Error(format!("Unexpected character: {}", ch))
        }
    }

    fn skip_until(&mut self, start: &str, end: &str) {
        if self.input[self.position..].starts_with(start) {
            self.position += start.len(); // Skip the starting pattern

            while self.position < self.input.len() {
                let ch = self.input[self.position..].chars().next().unwrap();
                self.position += 1;

                if ch == '\n' {
                    self.line_number += 1;
                }

                if self.input[self.position..].starts_with(end) {
                    self.position += end.len(); // Skip the ending pattern
                    return;
                }
            }

            // If we reach here, the end pattern was not found
            println!(
                "Error: Unterminated pattern '{}' starting at line {}",
                start, self.line_number
            );
        }
    }

    fn skip_preprocessor_line(&mut self) {
        while let Some(mat) = PREPROCESSOR_LINE_REGEX.find(&self.input[self.position..]) {
            let preprocessor_line = &self.input[self.position..][..mat.end()];
            // Count newlines in the preprocessor directive
            self.line_number += preprocessor_line.chars().filter(|&c| c == '\n').count();
            self.position += mat.end();
            // Skip any following whitespace
            self.skip_whitespace();
        }
    }

    fn match_keyword(&self, word: &str) -> Option<Token> {
        match word {
            "auto" => Some(Token::Auto),
            "break" => Some(Token::Break),
            "case" => Some(Token::Case),
            "char" => Some(Token::Char),
            "const" => Some(Token::Const),
            "continue" => Some(Token::Continue),
            "default" => Some(Token::Default),
            "do" => Some(Token::Do),
            "double" => Some(Token::Double),
            "else" => Some(Token::Else),
            "enum" => Some(Token::Enum),
            "extern" => Some(Token::Extern),
            "float" => Some(Token::Float),
            "for" => Some(Token::For),
            "goto" => Some(Token::Goto),
            "if" => Some(Token::If),
            "int" => Some(Token::Int),
            "long" => Some(Token::Long),
            "register" => Some(Token::Register),
            "return" => Some(Token::Return),
            "short" => Some(Token::Short),
            "signed" => Some(Token::Signed),
            "sizeof" => Some(Token::Sizeof),
            "static" => Some(Token::Static),
            "struct" => Some(Token::Struct),
            "switch" => Some(Token::Switch),
            "typedef" => Some(Token::Typedef),
            "union" => Some(Token::Union),
            "unsigned" => Some(Token::Unsigned),
            "void" => Some(Token::Void),
            "volatile" => Some(Token::Volatile),
            "while" => Some(Token::While),
            _ => None,
        }
    }

    fn match_operator(&self, op: &str) -> Token {
        match op {
            "+" => Token::Plus,
            "-" => Token::Minus,
            "*" => Token::Star,
            "/" => Token::Slash,
            "%" => Token::Percent,
            "=" => Token::Assign,
            "+=" => Token::PlusAssign,
            "-=" => Token::MinusAssign,
            "*=" => Token::StarAssign,
            "/=" => Token::SlashAssign,
            "%=" => Token::PercentAssign,
            "++" => Token::Increment,
            "--" => Token::Decrement,
            "==" => Token::Equals,
            "!=" => Token::NotEquals,
            "<" => Token::Less,
            ">" => Token::Greater,
            "<=" => Token::LessEqual,
            ">=" => Token::GreaterEqual,
            "&&" => Token::LogicalAnd,
            "||" => Token::LogicalOr,
            "!" => Token::LogicalNot,
            "&" => Token::BitwiseAnd,
            "|" => Token::BitwiseOr,
            "^" => Token::BitwiseXor,
            "~" => Token::BitwiseNot,
            "<<" => Token::LeftShift,
            ">>" => Token::RightShift,
            ";" => Token::Semicolon,
            "," => Token::Comma,
            "." => Token::Dot,
            "->" => Token::Arrow,
            "(" => Token::LeftParen,
            ")" => Token::RightParen,
            "{" => Token::LeftBrace,
            "}" => Token::RightBrace,
            "[" => Token::LeftBracket,
            "]" => Token::RightBracket,
            "?" => Token::Question,
            ":" => Token::Colon,
            _ => Token::Error(format!("Unknown operator: {}", op)),
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(mat) = WHITESPACE_REGEX.find(&self.input[self.position..]) {
            let whitespace = &self.input[self.position..][..mat.end()];
            self.line_number += whitespace.chars().filter(|&c| c == '\n').count();
            self.position += mat.end();
        }
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

    let mut lexer = Lexer::new(input);

    println!("Tokenizing file: {}", file_path);
    println!("-------------------");

    loop {
        let token = lexer.next_token();
        println!("Line {}: {:?}", lexer.line_number, token);

        if token == Token::EOF {
            break;
        }
    }

    println!("\nSymbol Table:");
    println!("------------");
    lexer.symbol_table.display();
}
