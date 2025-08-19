use std::iter::Peekable;
use std::collections::HashMap;
use std::slice::Iter;
use std::str::Chars;

#[derive(Debug)]
pub enum JsonValue{
    JsonString(String),
    JsonNumber(f64),
    JsonObject(HashMap<String, JsonValue>),
    JsonArray(Vec<JsonValue>),
    JsonBool(bool),
    JsonNull
}

#[derive(Debug)]
pub enum Token {
    TokenString(String),
    TokenNumber(f64),
    TokenNull,
    TokenTrue,
    TokenFalse,
    Comma,
    SemiColon,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket
}

fn parse_object(iterator: &mut Peekable<Iter<'_, Token>>) -> JsonValue {
    let mut entries: HashMap<String, JsonValue> = HashMap::new();
        while let Some(token) = iterator.next() {
            match token {
                Token::TokenString(s) => {

                    if let Some(Token::SemiColon) = iterator.next() {
                        let value = parse(iterator);

                        entries.insert(s.clone(),  
                                value);
                    }
                },
                Token::Comma => {
                    match iterator.peek(){
                        Some(peek) => {
                            match peek {
                                Token::TokenString(_) => continue,
                                _ => panic!("Expected key after object")
                            }
                        },
                        None => panic!("Unexpected end of object"),
                    }
                },
                Token::RightBrace => break,
                _ => {
                    println!("Unexpected token in object: {:?}", token);
                }
            }
    }

    
    JsonValue::JsonObject(entries)
 
}

fn parse_array(iterator: &mut Peekable<Iter<'_, Token>>) -> JsonValue {
    let mut items: Vec<JsonValue> = Vec::new();
    while let Some(token) = iterator.next(){
                    
        match token {
            Token::LeftBrace => items.push(parse_object(iterator)),
            Token::RightBracket => break,
            Token::Comma => {
                match iterator.peek(){
                    Some(peek) => {
                        match peek {
                            Token::LeftBrace => continue,
                            _ => panic!("Expected new object")
                        }
                    },
                    None => panic!("Unexpected end of array"),
                }

            }
            _ => panic!("Unexpected token in array: {:?}", token),
        }
    }

    JsonValue::JsonArray(items)
    
}

pub fn parse(iterator: &mut Peekable<Iter<'_, Token>>) -> JsonValue {
    while let Some(token) = iterator.next() {
        match token {
            Token::TokenString(s) => {
                return JsonValue::JsonString(s.clone());
            },
            Token::TokenNumber(n) => {
                return JsonValue::JsonNumber(n.clone());
            },
            Token::TokenTrue => {
                return JsonValue::JsonBool(true);
            },            
            Token::TokenFalse => {
                return JsonValue::JsonBool(false);
            },            
            Token::TokenNull => {
                return JsonValue::JsonNull;
            },
            Token::LeftBrace => {
                return parse_object(iterator);
            },
            Token::LeftBracket => {
                return parse_array(iterator);
            },
            _ => {
                todo!();
            }
        }
    }
    return JsonValue::JsonNull; 
}

fn expect_rest_of_constant(rest: &str, iterator: &mut Peekable<Chars<'_>>){
    for t in rest.chars() {
        if let Some(next_char) = iterator.next(){
            if next_char == t{
                continue;
            }
            else{
                panic!("Unexpected char: {:?}", t);
            }
        }
    }
}

fn tokenize_number(start : char, iterator: &mut Peekable<Chars<'_>>) -> Token {
    let mut num_str = String::new();
    let mut has_dec = false;

    num_str.push(start);
    while iterator.peek().is_some(){
        let next_char = iterator.peek().unwrap();

        match next_char {
            '0'..='9' => {
                num_str.push(iterator.next().unwrap());
            },
            '.' =>{
                if has_dec {
                    panic!("Invalid chararacter while parsing number: {:?}", next_char)
                }
                has_dec = true;
                num_str.push(iterator.next().unwrap());
            },
            _ => break
        }

    }
    match num_str.parse::<f64>() {
        Ok(n) => Token::TokenNumber(n),
        Err(e) => panic!("Error parsing number: {:?}", e),
    }
}


pub fn lex(contents: String) -> Vec<Token> {
    let chars= contents.chars();
    let mut tokens: Vec<Token> = Vec::new();
    let mut iterator = chars.peekable();
    while let Some(c) = iterator.next(){
        match c {
            '{' => tokens.push(Token::LeftBrace),
            '}' => tokens.push(Token::RightBrace),
            '[' => tokens.push(Token::LeftBracket),
            ']' => tokens.push(Token::RightBracket),
            ':' => tokens.push(Token::SemiColon),
            '"' => {
                let mut s = String::new();
                while let Some(next) = iterator.next(){
                    if next == '"'{
                        tokens.push(Token::TokenString(s.clone()));
                        break;
                    }
                    s.push(next); 
                }
            },
            'n'  => {
                expect_rest_of_constant("ull", &mut iterator);
                tokens.push(Token::TokenNull);
            },
            't'  => {
                expect_rest_of_constant("rue", &mut iterator);
                tokens.push(Token::TokenTrue);
            },
            'f'  => {
                expect_rest_of_constant("alse", &mut iterator);
                tokens.push(Token::TokenFalse);
            },
            digit @ '0'..='9'   => {
               let num= tokenize_number(digit, &mut iterator);
               tokens.push(num);
            },
            sub @'-' => {
               let num= tokenize_number(sub, &mut iterator);
               tokens.push(num);
            }
            ' ' | '\n' => continue,
            ','=> tokens.push(Token::Comma),
            _ => panic!("Unexpected character: {:?}", c),
        }
    }
    tokens
}