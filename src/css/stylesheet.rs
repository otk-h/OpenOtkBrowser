// ---------------------
// stylesheet
// ---------------------

#[derive(Debug)]
pub struct Stylesheet {
    pub rules: Vec<Rule>,
}

#[derive(Debug)]
pub struct Rule {
    pub selectors: Vec<Selector>,
    pub declarations: Vec<Declaration>,
}

#[derive(Debug, Clone)]
pub struct Declaration {
    pub name: String,
    pub value: Value,
    // pub important: bool,
}

// ---------------------
// selector
// ---------------------

pub type Specificity = (u32, u32, u32);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Selector {
    Simple(SimpleSelector),
    Descendant(Box<Selector>, Box<Selector>),
    Child(Box<Selector>, Box<Selector>),
    AdjacentSibling(Box<Selector>, Box<Selector>),
    GeneralSibling(Box<Selector>, Box<Selector>),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SimpleSelector {
    pub tag: Option<String>,
    pub id: Option<String>,
    pub class: Vec<String>,
}

impl Selector {
    pub fn specificity(&self) -> Specificity {
        match self {
            Selector::Simple(simple) => {
                simple.specificity()
            },
            Selector::Descendant(a, b) |
            Selector::Child(a, b) |
            Selector::AdjacentSibling(a, b) |
            Selector::GeneralSibling(a, b) => {
                let (a1, b1, c1) = a.specificity();
                let (a2, b2, c2) = b.specificity();
                (a1 + a2, b1 + b2, c1 + c2)
            },
        }
    }
}

impl SimpleSelector {
    fn specificity(&self) -> Specificity {
        let a = self.id.iter().count() as u32;
        let b = self.class.len() as u32;
        let c = self.tag.iter().count() as u32;
        (a, b, c)
    }
}

// ---------------------
// value
// ---------------------

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Keyword(String),
    Length(f32, Unit),
    // Percentage(f32),
    ColorValue(Color),
    // Url(String),
    // Function(String, Vec<Value>),
    // String(String),
    // Number(f32),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Unit {
    Px,
    // Em,
    // Rem,
    // Percent,
    // Vw,
    // Vh,
    // Vmin,
    // Vmax,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}




// use nom::{
//     IResult,
//     bytes::complete::{tag, take_while, take_while1},
//     multi::{many0, separated_list0},
//     character::complete::{char, space0},
//     combinator::{map, opt, recognize},
//     sequence::{delimited, pair, preceded, tuple},
// };

// // ---------------------
// // stylesheet
// // ---------------------

// pub fn parse_stylesheet(css_input: &str) -> Stylesheet {
//     let mut rules = Vec::new();
//     let mut input = css_input;

//     while let Ok((remaining, rule)) = parse_rule(input.trim_start()) {
//         rules.push(rule);
//         input = remaining;
//         if input.trim().is_empty() { break; }
//     }
    
//     Stylesheet { rules }
// }



// // ---------------------
// // rule
// // ---------------------



// impl Rule {
//     pub fn new(selectors: Vec<Selector>, declarations: Vec<Declaration>) -> Self {
//         let mut sorted_selectors = selectors;
//         sorted_selectors.sort_by(|a, b| b.specificity().cmp(&a.specificity()));
        
//         Self {
//             selectors: sorted_selectors,
//             declarations,
//         }
//     }
// }

// // ---------------------
// // declaration
// // ---------------------



// impl Declaration {
//     fn new(name: String, value: Value, important: bool) -> Self {
//         Self { name, value, important }
//     }
// }

// // ---------------------
// // parse functions
// // ---------------------

// pub fn parse_rule(input: &str) -> IResult<&str, Rule> {
//     let (remaining, (selectors, declarations)) = tuple((
//         separated_list0(
//             preceded(space0, char(',')),
//             preceded(space0, parse_selector)
//         ),
//         parse_declarations
//     ))(input)?;
    
//     Ok((remaining, Rule::new(selectors, declarations)))
// }

// fn parse_declarations(input: &str) -> IResult<&str, Vec<Declaration>> {
//     delimited(
//         preceded(space0, char('{')),
//         map(
//             many0(preceded(space0, parse_declaration)),
//             |declarations| { declarations }
//         ),
//         preceded(space0, char('}')),
//     )(input)
// }

// fn parse_declaration(input: &str) -> IResult<&str, Declaration> {
//     let (remaining, (property, _, value, important)) = tuple((
//         parse_identifier,
//         preceded(space0, char(':')),
//         preceded(space0, parse_value),
//         opt(preceded(
//             space0,
//             preceded(char('!'), tag("important"))
//         )),
//     ))(input)?;
    
//     let (remaining, _) = opt(preceded(space0, char(';')))(remaining)?;

//     Ok((remaining, Declaration {
//         name: property,
//         value,
//         important: important.is_some(),
//     }))
// }

// fn parse_identifier(input: &str) -> IResult<&str, String> {
//     map(
//         recognize(pair(
//             take_while1(|c: char| c.is_ascii_alphabetic() || c == '_' || c == '-'),
//             take_while(|c: char| c.is_ascii_alphanumeric() || c == '_' || c == '-'),
//         )),
//         |s: &str| s.to_string()
//     )(input)
// }

impl Value {
    pub fn to_px(&self) -> f32 {
        match *self {
            Value::Length(f, Unit::Px) => f,
            _ => 0.0,
        }
    }
}

// impl Value {
//     pub fn to_px(&self) -> Option<&f32> {
//         match self {
//             Value::Length(value, Unit::Px) => Some(value),
//             _ => None,
//         }
//     }

//      pub fn as_number(&self) -> Option<&f32> {
//         match self {
//             Value::Length(val, _) => Some(val),
//             Value::Percentage(val) => Some(val),
//             Value::Number(val) => Some(val),
//             _ => None,
//         }
//     }

//     pub fn as_color(&self) -> Option<&Color> {
//         match self {
//             Value::ColorValue(color) => Some(color),
//             _ => None,
//         }
//     }
// }