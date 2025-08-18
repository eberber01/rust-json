use std::error::Error;
use std::fs;
use std::iter::Peekable;
use std::collections::HashMap;
use std::slice::Iter;
use std::str::Chars;


#[derive(Debug)]
enum JsonValue{
    JsonString(String),
    JsonNumber(u32),
    JsonObject(HashMap<String, JsonValue>),
    JsonArray(Vec<JsonValue>),
    JsonBool(bool),
    JsonNull
}

#[derive(Debug)]
enum Token {
    TokenString(String),
    TokenNumber(u32),
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

fn parse(iterator: &mut Peekable<Iter<'_, Token>>) -> JsonValue {
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

fn lex(contents: String) -> Vec<Token> {
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
            digit @ '0'..='9' => {
               let mut num: u32 = 0; 
               num += digit.to_digit(10).unwrap();
                while iterator.peek().is_some(){
                    let next_char = iterator.peek().unwrap();
                    
                    if next_char.is_digit(10) {
                        num *= 10;
                        num += iterator.next().unwrap().to_digit(10).unwrap();
                    }
                    else{
                        break;
                    }
               }
               tokens.push(Token::TokenNumber(num));
            },
            ' ' | '\n' => continue,
            ','=> tokens.push(Token::Comma),
            _ => {todo!("not impl: {:?}", c)},
        }
    }

    tokens
}

fn main() -> Result<(), Box<dyn Error>> {
    let contents: String = fs::read_to_string("./src/test.json")?;
    println!("{}", contents);
    let tokens = lex(contents);
    for token in tokens.iter().clone(){
        println!("{:?}",token);
    }
    let mut iterator = tokens.iter().peekable();
    let json = parse(&mut iterator);
    println!("{:?}", json);
    Ok(())
}