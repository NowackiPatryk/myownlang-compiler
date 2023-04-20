mod structs;
use self::structs::token::Token;
use self::structs::token::TokenType;
use self::structs::token::get_keyword_hashes;

fn get_code_lines(code: &str) -> Vec<&str>{
  let lines: Vec<&str> = code.split(";").collect();
  lines.iter().filter(|str| !str.is_empty()).copied().collect()
}

fn normalize_line(code_line: &str) -> String {
  code_line.split_whitespace().collect()
}

fn get_tokens_from_line(code_line: &str) -> Vec<Token> {
  let mut tokens: Vec<Token> = Vec::new();
  let token_hashes = get_keyword_hashes();

  let variable_keyword = token_hashes.get(&TokenType::Variable).unwrap();
  let is_variable_declaration = code_line.split_at(variable_keyword.len()).0 == variable_keyword.to_owned();


  if is_variable_declaration {
    tokens.push(Token { typed_as: TokenType::Variable, value: None });

    let parts: Vec<&str> = code_line.split("=").collect();
    let declaration_part: Vec<&str> = parts[0].split(variable_keyword).collect();

    let variable_value = parts[1];
    let identifier = declaration_part[1];

    tokens.push(Token {typed_as: TokenType::Identifier, value: Some(identifier.to_string())});
    tokens.push(Token {typed_as: TokenType::Equals, value: None});
    tokens.push(Token{ typed_as: TokenType::Int, value: Some(variable_value.to_string())});
  }

    tokens
}

pub fn process(code: &str) -> Vec<Vec<Token>> {
  let mut tokens: Vec<Vec<Token>> = Vec::new();
  let code_lines: Vec<&str> = get_code_lines(code);

  for code_line in code_lines {
    let line_tokens = get_tokens_from_line(normalize_line(code_line).as_str());
    tokens.push(line_tokens);
  }

  tokens
}