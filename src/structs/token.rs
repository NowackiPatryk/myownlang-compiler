pub enum TokenType {
  INT,
  FLOAT,
  STRING,
  BOOL,
}

pub struct Token {
  typed_as: TokenType,
  value: Option<&'static str>
}