use core::panic;

use super::structs::token::{Token, TokenType};

pub fn get_tokens_from_variable_declaration(code_line: &str, variable_keyword: &str) -> Vec<Token> {
  let mut tokens: Vec<Token> = Vec::new();
    tokens.push(Token { typed_as: TokenType::Variable, value: None });

    let parts: Vec<&str> = code_line.split("=").collect();
    let declaration_part: Vec<&str> = parts[0].split(variable_keyword).collect();

    let variable_value = parts[1];
    let identifier = declaration_part[1];

    tokens.push(Token {typed_as: TokenType::Identifier, value: Some(identifier.to_string())});
    tokens.push(Token {typed_as: TokenType::Equals, value: None});

    if is_string(variable_value){
      tokens.push(Token{ typed_as: TokenType::String, value: Some(variable_value.to_string())})
    } else if is_float(variable_value){
        tokens.push(Token{ typed_as: TokenType::Float, value: Some(variable_value.to_string())})
    } else if is_int(variable_value){
      tokens.push(Token{ typed_as: TokenType::Int, value: Some(variable_value.to_string())})
      //  @TODO: Implement for boolean
    } else {
      panic!("Unrecognized value type: {}", variable_value)
    }

    tokens
}

fn is_correct_number(c: char) -> bool {
  "0123456789".contains(c)
}

fn is_string(variable_value: &str) -> bool {
  variable_value.contains('"') || variable_value.contains("'")
}

fn is_float(variable_value: &str) -> bool {
  if variable_value.contains(".") {
    let parts: Vec<&str> = variable_value.split(".").collect();

    for part in parts.iter() {
      for c in part.chars() {
        if !is_correct_number(c) {
          return false;
        }
      }
    }
  }

  true
}

fn is_int(variable_value: &str) -> bool {
  for c in variable_value.chars() {
    if !is_correct_number(c) {
      return false;
    }
  }

  true
}