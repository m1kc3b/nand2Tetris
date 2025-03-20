use crate::lexical::{KEYWORDS, SYMBOLS};
use std::io;

pub enum Token {
    Keyword(Keyword),
    Symbol,
    Identifier,
    IntConst,
    StringConst,
}

#[derive(Debug, Clone, Copy)]
pub enum Keyword {
    Class,
    Method,
    Function,
    Constructor,
    Int,
    Boolean,
    Char,
    Void,
    Var,
    Static,
    Field,
    Let,
    Do,
    If,
    Else,
    While,
    Return,
    True,
    False,
    Null,
    This,
}

pub fn parse_file(file: &str) -> Result<Vec<String>, io::Error> {
    let mut tokens: Vec<String> = Vec::new();
    tokens.push("<tokens>".to_string());

    for line in file.lines() {
        // removes empty line and comments
        let line = line.trim_start();
        let line = if line.starts_with("//") {
            ""
        } else if line.starts_with("/**") {
            ""
        } else {
            line
        };

        // split line in tokens
        for word in line.split_whitespace() {

            match token_type(word) {
                Some(Token::Keyword(_)) => {
                  tokens.push(format!("<keyword> {} </keyword>", word));
                }
                Some(Token::Symbol) => {
                  tokens.push(format!("<symbol> {} </symbol>", word));
                }
                Some(Token::Identifier) => {
                  let tags = handle_identifier(word);
                  for tag in tags {
                    tokens.push(tag);
                  }
                }
                Some(Token::IntConst) => {
                  tokens.push(format!("<integerConstant> {} </integerConstant>", word));
                }
                Some(Token::StringConst) => {
                  tokens.push(format!("<stringConstant> {} </stringConstant>", word));
                }
                None => {}
            }
        }
    }

    tokens.push("</tokens>".to_string());
    Ok(tokens)
}

// Returns the type of the current token
fn token_type(token: &str) -> Option<Token> {
    if KEYWORDS.contains(&token) {
        if let Some(key) = keyword_type(token) {
            return Some(Token::Keyword(key));
        }
    } else if token.chars().all(|c| SYMBOLS.contains(&c))  {
        return Some(Token::Symbol);
    } else if is_valid_identifier(token) {
        return Some(Token::Identifier);
    } else if is_valid_integer(token) {
        return Some(Token::IntConst);
    } else if token.starts_with('"') & token.ends_with('"') {
        return Some(Token::StringConst);
    }

    None
}

// Returns the keyword which is the current token
fn keyword_type(token: &str) -> Option<Keyword> {
    let keywords = [
        ("class", Keyword::Class),
        ("method", Keyword::Method),
        ("function", Keyword::Function),
        ("constructor", Keyword::Constructor),
        ("int", Keyword::Int),
        ("boolean", Keyword::Boolean),
        ("char", Keyword::Char),
        ("void", Keyword::Void),
        ("var", Keyword::Var),
        ("static", Keyword::Static),
        ("field", Keyword::Field),
        ("let", Keyword::Let),
        ("do", Keyword::Do),
        ("if", Keyword::If),
        ("else", Keyword::Else),
        ("while", Keyword::While),
        ("return", Keyword::Return),
        ("true", Keyword::True),
        ("false", Keyword::False),
        ("null", Keyword::Null),
        ("this", Keyword::This),
    ];

    // Vérifie si le token commence par l'un des mots-clés
    for &(pat, keyword) in keywords.iter() {
        if token.starts_with(pat) {
            return Some(keyword);
        }
    }

    None
}

// TODO: symbol: returns the character which is the current token. Only for Symbol tokenType
fn handle_symbol(token: &str) -> Option<&str> {
  // example: main()
  // returns (

  Some("()")
}

// TODO: identifier: returns the string which is the current token. Only for Identifier tokenType
fn handle_identifier(token: &str) -> Vec<String>{
  let mut identifiers: Vec<String> = Vec::new();

  // Check if token contains any symbols
  if token.chars().any(|c| SYMBOLS.contains(&c)) {
      let elements = split_token_by_symbols(token);

      for element in elements {
        match token_type(&element) {
          Some(Token::Keyword(_)) => {
            identifiers.push(format!("<keyword> {} </keyword>", element));
          }
          Some(Token::Symbol) => {
            identifiers.push(format!("<symbol> {} </symbol>", element));
          }
          Some(Token::Identifier) => {
            identifiers.push(format!("<identifier> {} </identifier>", element));
          }
          Some(Token::IntConst) => {
            identifiers.push(format!("<integerConstant> {} </integerConstant>", element));
          }
          Some(Token::StringConst) => {
            identifiers.push(format!("<stringConstant> {} </stringConstant>", element));
          }
          None => {}
      }
      }
  } else {
      identifiers.push(token.to_string());
  }
  
  identifiers
}

// TODO: intVal: returns the integer value of the current token. Only for IntConst
fn integer(token: &str) -> Option<usize> {
  // example: let i = 0;
  // returns 0

    Some(token.parse::<usize>().unwrap())
}

// TODO: stringVal: returns the string value of the current token without "". Only for StringConst tokenType
fn string(token: &str) -> Option<&str> {
  // example: let a[i] = Keyboard.readInt("ENTER THE NEXT NUMBER: ");
  // returns ENTER THE NEXT NUMBER:

    //  Find the index of the first double quote
    if let Some(start_index) = token.find('"') {
        // Find the index of the second double quote after the first
        if let Some(end_index) = token[start_index + 1..].find('"') {
            // Extract the substring between the quotes
            let content = &token[start_index + 1..start_index + 1 + end_index];
            return Some(content);
        }
    }
    // Retourner None si les guillemets ne sont pas trouvés
    None
}

/**
 * HELPERS
 */

// Check if token is an Identifier TokenType
fn is_valid_identifier(token: &str) -> bool {
    // Check token is not empty
    if token.is_empty() {
        return false;
    }
    // Check the first char is a letter or an underscore
    let first_char = token.chars().next().unwrap();
    if !first_char.is_alphabetic() && first_char != '_' {
        return false;
    }
    // Check the next chars are letters, digits or underscores
    for ch in token.chars().skip(1) {
        if !ch.is_alphanumeric() && ch != '_' {
            return false;
        }
    }

    true
}

// Check if token is an IntConst TokenType
fn is_valid_integer(token: &str) -> bool {
    match token.parse::<u32>() {
        Ok(number) => number <= 32767,
        Err(_) => false,
    }
}

fn get_tag(token_type: Token, word: &str) -> String {
  match token_type {
      Token::Keyword(_) => format!("<keyword> {} </keyword>", word),
      Token::Identifier => format!("<identifier> {} </identifier>", word),
      Token::IntConst => format!("<integerConstant> {} </integerConstant>", word),
      Token::StringConst => format!("<stringConstant> {} </stringConstant>", word),
      Token::Symbol => format!("<symbol> {} </symbol>", word),
  }
}

fn split_token_by_symbols(token: &str) -> Vec<String> {
  let mut result = Vec::new();
  let mut current_segment = String::new();

  for c in token.chars() {
      if SYMBOLS.contains(&c) {
          // Si un symbole est rencontré, ajoute le segment actuel au résultat
          if !current_segment.is_empty() {
              result.push(current_segment.clone());
              current_segment.clear();
          }
          // Ajoute le symbole comme son propre segment
          result.push(c.to_string());
      } else {
          // Ajoute le caractère au segment actuel
          current_segment.push(c);
      }
  }

  // Ajoute le dernier segment s'il n'est pas vide
  if !current_segment.is_empty() {
      result.push(current_segment);
  }

  result
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_split_token_by_symbols() {
    let token = "Keyboard.readInt(\"ENTER THE NEXT NUMBER: \");";
    let mut expected: Vec<String> = Vec::new();
    expected.push("Keyboard".to_string());
    expected.push(".".to_string());
    expected.push("readInt".to_string());
    expected.push("(".to_string());
    expected.push("\"ENTER THE NEXT NUMBER: \"".to_string());
    expected.push(")".to_string());
    expected.push(";".to_string());
    

    let r = split_token_by_symbols(token);

    assert_eq!(r, expected)
  }
}