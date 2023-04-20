use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum TokenType {
  Variable,
  Identifier,
  Int,
  Float,
  String,
  Equals,
  IsEqualTo,
}

pub fn get_keyword_hashes() -> HashMap<TokenType, & 'static str> {
  let mut token_hashes = HashMap::new();

  token_hashes.insert(TokenType::Variable, "VAR");
  token_hashes.insert(TokenType::Equals, "=");
  token_hashes.insert(TokenType::IsEqualTo, "==");

  token_hashes
}

#[derive(Debug)]
pub struct Token {
  pub typed_as: TokenType,
  pub value: Option<String>
}
