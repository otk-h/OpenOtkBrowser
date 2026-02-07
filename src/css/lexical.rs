#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Ident(String),          // div, body, color
    Hash(String),           // #ff0000, #main
    AtKeyword(String),      // @media, @import
    String(String),         // "hello"
    Number(f32),            // 10.5
    Percentage(f32),        // 50%
    Dimension(f32, String), // 10px, 2em (数值, 单位)
    
    Colon,          // :
    SemiColon,      // ;
    Comma,          // ,
    CurlyOpen,      // {
    CurlyClose,     // }
    BracketOpen,    // [
    BracketClose,   // ]
    ParenOpen,      // (
    ParenClose,     // )
    
    Delim(char),    // ., >, +, ~, *
    EOF,
}

pub struct Tokenizer {
    pos: usize,
    input: Vec<char>,
}

impl Tokenizer {
    fn new(css_input: String) -> Self {
        Tokenizer { pos:0, input: css_input.chars().collect(), }
    }
    
    pub fn parse_token(css_input: String) -> Vec<Token> {
        let mut tokenizer = Tokenizer::new(css_input);
        let mut tokens = Vec::new();
        loop {
            let token = tokenizer.next_token();
            if token == Token::EOF { break; }
            tokens.push(token);
        }
        return tokens;
    }

    fn next_token(&mut self) -> Token {
        self.consume_whitespace();
        if self.eof() { return Token::EOF; }

        let c = self.curr_char();
        match c {
            ':' => { self.consume(); Token::Colon }
            ';' => { self.consume(); Token::SemiColon }
            ',' => { self.consume(); Token::Comma }
            '{' => { self.consume(); Token::CurlyOpen }
            '}' => { self.consume(); Token::CurlyClose }
            '(' => { self.consume(); Token::ParenOpen }
            ')' => { self.consume(); Token::ParenClose }
            '[' => { self.consume(); Token::BracketOpen }
            ']' => { self.consume(); Token::BracketClose }
            '#' => { self.consume(); Token::Hash(self.consume_ident()) }
            '.' => {
                if self.next_char().is_numeric() {
                    self.consume_numeric()
                } else {
                    self.consume();
                    Token::Delim('.')
                }
            }
            '0'..='9' => { self.consume_numeric() }
            '"' | '\'' => { self.consume_string(c) }
            c if is_ident_start(c) => { Token::Ident(self.consume_ident()) }
            _ => { self.consume(); Token::Delim(c) }
            // '@' => {
            //     self.consume();
            //     Token::AtKeyword(self.consume_ident())
            // }
        }
    }


// ---------------------
// assistant functions
// ---------------------

    fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }

    fn curr_char(&self) -> char {
        if self.eof() { '\0' } else { self.input[self.pos] }
    }

    fn next_char(&self) -> char {
        if self.pos + 1 >= self.input.len() { '\0' } else { self.input[self.pos + 1] }
    }

    fn consume_numeric(&mut self) -> Token {
        let num_str = self.consume_while(|c| c.is_numeric() || c == '.');
        let num: f32 = num_str.parse().unwrap_or(0.0);

        if self.curr_char() == '%' {
            self.consume();
            Token::Percentage(num)
        } else if is_ident_start(self.curr_char()) {
            let unit = self.consume_ident();
            Token::Dimension(num, unit)
        } else {
            Token::Number(num)
        }
    }

    fn consume_ident(&mut self) -> String {
        self.consume_while(is_ident_char)
    }

    fn consume_string(&mut self, quote: char) -> Token {
        self.consume();
        let s = self.consume_while(|c| c != quote);
        self.consume();
        Token::String(s)
    }

    fn consume_whitespace(&mut self) {
        self.consume_while(|c| c.is_whitespace());
    }

    fn consume_while<F>(&mut self, test: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let mut result = String::new();
        while !self.eof() && test(self.curr_char()) {
            result.push(self.consume());
        }
        return result;
    }

    fn consume(&mut self) -> char {
        let c = self.input[self.pos];
        self.pos += 1;
        return c;
    }
}

fn is_ident_start(c: char) -> bool {
    c.is_alphabetic() || c == '-' || c == '_'
}

fn is_ident_char(c: char) -> bool {
    c.is_alphanumeric() || c == '-' || c == '_'
}
