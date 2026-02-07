use super::stylesheet::*;
use super::lexical::*;

pub struct CssParser {
    pos: usize,
    tokens: Vec<Token>,
}

impl CssParser {
    fn new(css_input: String) -> Self {
        let tokens = Tokenizer::parse_token(css_input);
        CssParser { pos: 0, tokens: tokens, }
    }

    pub fn parse_stylesheet(css_input: String) -> Stylesheet {
        let mut css_parser = Self::new(css_input);
        let mut rules = Vec::new();
        loop {
            if css_parser.eof() { break; }
            rules.push(css_parser.parse_rule());
        }
        Stylesheet { rules }
    }

    fn parse_rule(&mut self) -> Rule {
        Rule {
            selectors: self.parse_selectors(),
            declarations: self.parse_declarations(),
        }
    }

    fn parse_selectors(&mut self) -> Vec<Selector> {
        let mut selectors = Vec::new();
        loop {
            selectors.push(self.parse_selector_chain());
            match self.curr_token() {
                Token::Comma => { self.consume(); },
                Token::CurlyOpen => break,
                _ => panic!("Unexpected token in selector list: {:?}", self.curr_token()),
            }
        }
        selectors.sort_by(|a, b| b.specificity().cmp(&a.specificity()));
        return selectors;
    }

    fn parse_selector_chain(&mut self) -> Selector {
        let mut selector = Selector::Simple(self.parse_simple_selector());

        loop {
            if self.eof() { break; }
            
            match self.curr_token() {
                Token::Comma | Token::CurlyOpen => break,
                
                Token::Delim('>') => {
                    self.consume();
                    let next = self.parse_simple_selector();
                    selector = Selector::Child(Box::new(selector), Box::new(Selector::Simple(next)));
                }
                Token::Delim('+') => {
                    self.consume();
                    let next = self.parse_simple_selector();
                    selector = Selector::AdjacentSibling(Box::new(selector), Box::new(Selector::Simple(next)));
                }
                Token::Delim('~') => {
                    self.consume();
                    let next = self.parse_simple_selector();
                    selector = Selector::GeneralSibling(Box::new(selector), Box::new(Selector::Simple(next)));
                }
                token => {
                    match token {
                        Token::Hash(_) | Token::Delim('.') | Token::Delim('*') | Token::Ident(_) => {
                            let next = self.parse_simple_selector();
                            selector = Selector::Descendant(Box::new(selector), Box::new(Selector::Simple(next)));
                        }
                        _ => panic!("Unexpected token in selector chain: {:?}", token),
                    }
                }
            }
        }
        return selector;
    }

    fn parse_simple_selector(&mut self) -> SimpleSelector {
        let mut selector = SimpleSelector {
            tag: None,
            id: None,
            class: Vec::new(),
        };
        loop {
            match self.curr_token() {
                Token::Hash(id) => {
                    self.consume();
                    selector.id = Some(id.clone());
                }
                Token::Delim('.') => {
                    self.consume();
                    if let Token::Ident(class_name) = self.consume() {
                        selector.class.push(class_name.clone());
                    } else {
                        panic!("Expected identifier after .");
                    }
                }
                Token::Delim('*') => {
                    self.consume();
                }
                Token::Ident(tag) => {
                    self.consume();
                    selector.tag = Some(tag.clone());
                }
                _ => break, 
            }
        }
        return selector;
    }

    fn parse_declarations(&mut self) -> Vec<Declaration> {
        assert_eq!(self.consume(), Token::CurlyOpen);
        let mut declarations = Vec::new();
        loop {
            if self.curr_token() == Token::CurlyClose { break; }
            declarations.push(self.parse_declaration());
            match self.curr_token() {
                Token::SemiColon => { self.consume(); }
                Token::CurlyClose => { break; }
                _ => panic!("Expected ; or }} after declaration, found {:?}", self.curr_token()),
            }
        }
        assert_eq!(self.consume(), Token::CurlyClose);
        return declarations;
    }

    fn parse_declaration(&mut self) -> Declaration {
        let name = match self.consume() {
            Token::Ident(name) => name,
            _ => panic!("Expected property name"),
        };
        assert_eq!(self.consume(), Token::Colon, "Expected ':' after property name");
        let value = self.parse_value();
        Declaration { name: name, value: value, }
    }

    fn parse_value(&mut self) -> Value {
         match self.consume() {
            Token::Dimension(v, unit) => {
                match unit.as_str() {
                    "px" => Value::Length(v, Unit::Px),
                    _ => panic!("Unknown unit"),
                }
            }
            Token::Number(v) => { 
                Value::Length(v, Unit::Px)
            }
            Token::Hash(color) => {
                let r = u8::from_str_radix(&color[0..2], 16).unwrap();
                let g = u8::from_str_radix(&color[2..4], 16).unwrap();
                let b = u8::from_str_radix(&color[4..6], 16).unwrap();
                Value::ColorValue(Color { r, g, b, a: 255 })
            }
            Token::Ident(s) => {
                Value::Keyword(s)
            }
            _ => {
                panic!("Unsupported value")
            }
        }
    }

// ---------------------
// assistant functions
// ---------------------

    fn eof(&self) -> bool {
        self.pos >= self.tokens.len()
    }

    fn curr_token(&self) -> Token {
        if self.pos >= self.tokens.len() { Token::EOF } else { self.tokens[self.pos].clone() }
    }

    fn consume(&mut self) -> Token {
        let token = self.tokens[self.pos].clone();
        self.pos += 1;
        return token;
    }
}
