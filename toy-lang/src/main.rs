use std::{env, fs};

#[derive(Debug, Clone, PartialEq)]
enum Token {
    Let,
    Print,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Comma,
    Semicolon,
    Equal,
    Ident(String),
    Str(String),
    Eof,
}

struct Lexer {
    src: Vec<char>,
    i: usize,
}
impl Lexer {
    fn new(s: &str) -> Self {
        Self {
            src: s.chars().collect(),
            i: 0,
        }
    }
    fn peek(&self) -> Option<char> {
        self.src.get(self.i).copied()
    }
    fn bump(&mut self) -> Option<char> {
        let c = self.peek()?;
        self.i += 1;
        Some(c)
    }
    fn skip_ws(&mut self) {
        while matches!(self.peek(), Some(c) if c.is_whitespace()) {
            self.i += 1;
        }
    }

    fn string(&mut self) -> Result<String, String> {
        // assumes opening quote already consumed
        let mut out = String::new();
        while let Some(c) = self.bump() {
            match c {
                '"' => return Ok(out),
                '\\' => {
                    let esc = self.bump().ok_or("Unfinished escape in string")?;
                    out.push(match esc {
                        'n' => '\n',
                        't' => '\t',
                        '"' => '"',
                        '\\' => '\\',
                        _ => return Err(format!("Unsupported escape: \\{esc}")),
                    });
                }
                _ => out.push(c),
            }
        }
        Err("Unterminated string".into())
    }

    fn ident_or_kw(&mut self, first: char) -> String {
        let mut s = String::new();
        s.push(first);
        while let Some(c) = self.peek() {
            if c.is_alphanumeric() || c == '_' {
                s.push(c);
                self.i += 1;
            } else {
                break;
            }
        }
        s
    }

    fn next_token(&mut self) -> Result<Token, String> {
        self.skip_ws();
        let Some(c) = self.bump() else {
            return Ok(Token::Eof);
        };
        Ok(match c {
            '(' => Token::LParen,
            ')' => Token::RParen,
            '{' => Token::LBrace,
            '}' => Token::RBrace,
            ',' => Token::Comma,
            ';' => Token::Semicolon,
            '=' => Token::Equal,
            '"' => Token::Str(self.string()?),
            c if c.is_alphabetic() || c == '_' => {
                let s = self.ident_or_kw(c);
                match s.as_str() {
                    "let" => Token::Let,
                    "print" => Token::Print,
                    _ => Token::Ident(s),
                }
            }
            _ => return Err(format!("Unexpected char: '{c}' at {}", self.i - 1)),
        })
    }

    fn tokenize(mut self) -> Result<Vec<Token>, String> {
        let mut ts = Vec::new();
        loop {
            let t = self.next_token()?;
            let end = t == Token::Eof;
            ts.push(t);
            if end {
                break;
            }
        }
        Ok(ts)
    }
}

#[derive(Debug)]
enum Stmt {
    Let { name: String, value: String },
    Print { format: String, args: Vec<String> },
}

#[derive(Debug)]
struct Program {
    body: Vec<Stmt>,
}

struct Parser {
    ts: Vec<Token>,
    i: usize,
}
impl Parser {
    fn new(ts: Vec<Token>) -> Self {
        Self { ts, i: 0 }
    }
    fn peek(&self) -> &Token {
        self.ts.get(self.i).unwrap_or(&Token::Eof)
    }
    fn bump(&mut self) -> &Token {
        let t = self.peek() as *const Token;
        self.i += 1;
        unsafe { &*t }
    }
    fn expect(&mut self, expected: &Token) -> Result<(), String> {
        let t = self.bump().clone();
        if &t == expected {
            Ok(())
        } else {
            Err(format!("Expected {:?}, found {:?}", expected, t))
        }
    }

    fn parse(&mut self) -> Result<Program, String> {
        let mut body = Vec::new();
        while !matches!(self.peek(), Token::RBrace | Token::Eof) {
            body.push(self.parse_stmt()?);
        }
        Ok(Program { body })
    }

    fn parse_stmt(&mut self) -> Result<Stmt, String> {
        match self.peek() {
            Token::Let => self.parse_let(),
            Token::Print => self.parse_print(),
            other => Err(format!("Unexpected token in statement: {:?}", other)),
        }
    }

    fn parse_let(&mut self) -> Result<Stmt, String> {
        self.expect(&Token::Let)?;
        let name = match self.bump().clone() {
            Token::Ident(s) => s,
            t => return Err(format!("Expected identifier after let, got {:?}", t)),
        };
        self.expect(&Token::Equal)?;
        let value = match self.bump().clone() {
            Token::Str(s) => s,
            t => return Err(format!("Expected string literal after '=', got {:?}", t)),
        };
        self.expect(&Token::Semicolon)?;
        Ok(Stmt::Let { name, value })
    }

    fn parse_print(&mut self) -> Result<Stmt, String> {
        self.expect(&Token::Print)?;
        self.expect(&Token::LParen)?;
        let format = match self.bump().clone() {
            Token::Str(s) => s,
            t => {
                return Err(format!(
                    "Expected string literal in print(...), got {:?}",
                    t
                ));
            }
        };

        let mut args: Vec<String> = Vec::new();
        while matches!(self.peek(), Token::Comma) {
            self.expect(&Token::Comma)?;
            match self.bump().clone() {
                Token::Ident(s) => args.push(s),
                t => return Err(format!("Expected identifier as print arg, got {:?}", t)),
            }
        }

        self.expect(&Token::RParen)?;
        self.expect(&Token::Semicolon)?;
        Ok(Stmt::Print { format, args })
    }
}

use std::collections::HashMap;

struct Interpreter {
    env: HashMap<String, String>,
}
impl Interpreter {
    fn new() -> Self {
        Self {
            env: HashMap::new(),
        }
    }

    fn run(&mut self, prog: Program) -> Result<(), String> {
        for s in prog.body {
            match s {
                Stmt::Let { name, value } => {
                    self.env.insert(name, value);
                }
                Stmt::Print { format, args } => self.exec_print(format, args)?,
            }
        }
        Ok(())
    }

    fn exec_print(&self, format: String, args: Vec<String>) -> Result<(), String> {
        // Sustituye secuencialmente cada "{}" por el valor de cada nombre en args.
        let mut out = String::new();
        let mut fmt = format.as_str();
        let mut remaining_args = args.iter();

        loop {
            match fmt.find("{}") {
                // Option<usize>: Some(pos) o None
                Some(pos) => {
                    // copiar hasta el marcador
                    out.push_str(&fmt[..pos]);
                    // tomar el próximo argumento
                    let name = remaining_args
                        .next()
                        .ok_or_else(|| "print: missing arguments for placeholders".to_string())?;
                    let val = self
                        .env
                        .get(name)
                        .ok_or_else(|| format!("Undefined variable: {name}"))?;
                    out.push_str(val);
                    // avanzar después de "{}"
                    fmt = &fmt[pos + 2..];
                }
                None => {
                    // no hay más "{}", copiar el resto y salir
                    out.push_str(fmt);
                    break;
                }
            }
        }

        // Si sobraron args, también es error (más args que "{}")
        if remaining_args.next().is_some() {
            return Err("print: too many arguments for placeholders".to_string());
        }

        println!("{out}");
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = env::args().nth(1).ok_or("usage: mini_x <file.x>")?;
    let code = fs::read_to_string(&path)?;
    let tokens = Lexer::new(&code)
        .tokenize()
        .map_err(|e| format!("Lex error: {e}"))?;
    println!("Tokens: {:?}", tokens);
    let program = Parser::new(tokens)
        .parse()
        .map_err(|e| format!("Parse error: {e}"))?;
    println!("Program: {:?}", program);
    let mut vm = Interpreter::new();
    vm.run(program).map_err(|e| format!("Runtime error: {e}"))?;
    Ok(())
}
