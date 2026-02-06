use crate::dom;
use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_while, take_while1},
    character::complete::{alpha1, char, multispace0, multispace1, space0},
    combinator::{opt, recognize, verify},
    multi::{many0, fold_many0},
    sequence::{delimited, pair, preceded, tuple},
    IResult,
};

pub fn parse(html_input: String) -> dom::Node {
    let result: IResult<&str, Vec<dom::Node>> = many0(parse_node)(&html_input);

    match result {
        Ok((_, mut nodes)) => {
            if nodes.len() == 1 {
                nodes.remove(0)
            } else {
                dom::Node::element("root".to_string(), HashMap::new(), nodes)
            }
        },
        Err(e) => {
            eprintln!("Detailed Parse Error: {:?}", e);
            dom::Node::text("Parse Error".to_string())
        }
    }
}

// ---------------------
// parse attributes
// ---------------------

fn parse_attributes(input: &str) -> IResult<&str, HashMap<String, String>> {
    // class="foo" id="bar" ...
    fold_many0(
        preceded(multispace1, parse_attribute),
        HashMap::new,
        |mut acc: HashMap<String, String>, (k, v)| {
            acc.insert(k, v);
            acc
        }
    )(input)
}

fn parse_attribute(input: &str) -> IResult<&str, (String, String)> {
    // key="value" ...
    let (input, key) = parse_identifier(input)?;
    let (input, _) = space0(input)?;
    let (input, val) = opt(preceded(
        tuple((char('='), space0)), 
        alt((
            delimited(char('"'), take_until("\""), char('"')),
            delimited(char('\''), take_until("'"), char('\'')),
        ))
    ))(input)?;

    // or true as default value
    let value = val.unwrap_or("").to_string();
    
    Ok((input, (key.to_string(), value)))
}

// ---------------------
// parse tags
// ---------------------

fn is_void_tag(tag_name: &str) -> bool {
    matches!(tag_name.to_lowercase().as_str(), 
        "area"   |
        "base"   |
        "br"     |
        "col"    |
        "embed"  |
        "hr"     |
        "img"    |
        "input"  | 
        "link"   |
        "meta"   |
        "param"  |
        "source" |
        "track"  |
        "wbr"
    )
}

fn parse_open_tag(input: &str) -> IResult<&str, (String, HashMap<String, String>, bool)> {
    let (input, _) = char('<')(input)?;
    let (input, tag_name) = parse_identifier(input)?;
    let (input, attrs) = parse_attributes(input)?;
    let (input, _) = multispace0(input)?;

    let (input, self_closing) = opt(char('/'))(input)?; 
    let (input, _) = char('>')(input)?;

    let is_void = is_void_tag(tag_name) || self_closing.is_some();

    Ok((input, (tag_name.to_string(), attrs, is_void)))
}

fn parse_close_tag(expected_name: String) -> impl FnMut(&str) -> IResult<&str, &str> {
    move |input: &str| {
        let (input, _) = tag("</")(input)?;
        // 使用 verify 确保解析出的标识符必须等于我们预期的标签名
        let (input, name) = verify(parse_identifier, |s: &str| s == expected_name)(input)?;
        let (input, _) = preceded(multispace0, char('>'))(input)?;
        Ok((input, name))
    }
}

// ---------------------
// parse text
// ---------------------

fn parse_text(input: &str) -> IResult<&str, dom::Node> {
    let (input, text) = take_while1(|c| c != '<')(input)?;
    Ok((input, dom::Node::text(text.to_string())))
}

fn parse_comment(input: &str) -> IResult<&str, dom::Node> {
    let (input, content) = tag("")(input)?;
    let (input, _) = tag("-->")(input)?;
    Ok((input, dom::Node::comment(content.to_string())))
}

// ---------------------
// parse elements
// ---------------------

fn parse_element(input: &str) -> IResult<&str, dom::Node> {
    let (input, (tag_name, attrs, is_void)) = parse_open_tag(input)?;

    if is_void {
        // void_tag no children
        return Ok((input, dom::Node::element(tag_name, attrs, Vec::new())));
    }

    // many0 to repeat until fail
    let (input, children) = many0(parse_node)(input)?;

    let (input, _) = multispace0(input)?;
    let (input, _) = parse_close_tag(tag_name.clone())(input)?;

    Ok((input, dom::Node::element(tag_name, attrs, children)))
}

fn parse_node(input: &str) -> IResult<&str, dom::Node> {
    // consum whitespaces
    let (input, _) = multispace0(input)?; 
    
    if input.is_empty() {
        return Err(nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Eof)));
    }

    alt((
        parse_comment,
        parse_element,
        parse_text,
    ))(input)
}

// ---------------------
// assistant functions
// ---------------------

fn parse_identifier(input: &str) -> IResult<&str, &str> {
    // div, class, data-id, ...
    recognize(
        pair(
            alpha1,
            take_while(|c: char| c.is_alphanumeric() || c == '-' || c == '_')
        )
    )(input)
}
