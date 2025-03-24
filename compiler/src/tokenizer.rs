use crate::lexical::{KEYWORDS, SYMBOLS};
use std::{
    fs::File,
    io::{self, BufRead},
};

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

pub fn tokenize(path: &str) -> Result<Vec<String>, io::Error> {
    let mut tokens: Vec<String> = Vec::new();
    tokens.push("<tokens>".to_string());

    let f = File::open(path)?;
    let reader = io::BufReader::new(f);
    let mut inside_comment_block = false;

    for line in reader.lines() {
        let line = line?;
        let line = line.trim_start();

        // Ignore comments like // xxxx
        if line.starts_with("//") {
            continue;
        }

        // Ignore comment like /** xxxx */
        if line.starts_with("/**") {
            inside_comment_block = true;
        }
        if inside_comment_block {
            if line.ends_with("*/") {
                inside_comment_block = false;
            }
            continue;
        }

        let words: Vec<String> = split_line_into_tokens(&line);

        for word in words {
            match token_type(&word) {
                Some(Token::Keyword(keyword)) => {
                    tokens.push(format!(
                        "<keyword> {} </keyword>",
                        keyword_to_string(&keyword)
                    ));
                }
                Some(Token::Symbol) => {
                    let escape_word = escape_special_chars(&word);
                    tokens.push(format!("<symbol> {} </symbol>", escape_word));
                }
                Some(Token::Identifier) => {
                    tokens.push(format!("<identifier> {} </identifier>", word));
                }
                Some(Token::IntConst) => {
                    tokens.push(format!("<integerConstant> {} </integerConstant>", word));
                }
                Some(Token::StringConst) => {
                    let clean_str = word.trim_matches('"');
                    tokens.push(format!("<stringConstant> {} </stringConstant>", clean_str));
                }
                None => {}
            }
        }
    }

    tokens.push("</tokens>".to_string());
    Ok(tokens)
}

fn token_type(token: &str) -> Option<Token> {
    if KEYWORDS.contains(&token) {
        if let Some(key) = keyword_type(token) {
            return Some(Token::Keyword(key));
        }
    } else if token.chars().all(|c| SYMBOLS.contains(&c)) {
        return Some(Token::Symbol);
    } else if is_valid_identifier(token) {
        return Some(Token::Identifier);
    } else if is_valid_integer(token) {
        return Some(Token::IntConst);
    } else if token.starts_with('"') && token.ends_with('"') {
        return Some(Token::StringConst);
    }

    None
}

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

    for &(pat, keyword) in keywords.iter() {
        if token == pat {
            return Some(keyword);
        }
    }

    None
}

fn keyword_to_string(keyword: &Keyword) -> &str {
    match keyword {
        Keyword::Class => "class",
        Keyword::Method => "method",
        Keyword::Function => "function",
        Keyword::Constructor => "constructor",
        Keyword::Int => "int",
        Keyword::Boolean => "boolean",
        Keyword::Char => "char",
        Keyword::Void => "void",
        Keyword::Var => "var",
        Keyword::Static => "static",
        Keyword::Field => "field",
        Keyword::Let => "let",
        Keyword::Do => "do",
        Keyword::If => "if",
        Keyword::Else => "else",
        Keyword::While => "while",
        Keyword::Return => "return",
        Keyword::True => "true",
        Keyword::False => "false",
        Keyword::Null => "null",
        Keyword::This => "this",
    }
}

fn is_valid_identifier(token: &str) -> bool {
    if token.is_empty() {
        return false;
    }
    let first_char = token.chars().next().unwrap();
    if !first_char.is_alphabetic() && first_char != '_' {
        return false;
    }
    for ch in token.chars().skip(1) {
        if !ch.is_alphanumeric() && ch != '_' {
            return false;
        }
    }
    true
}

fn is_valid_integer(token: &str) -> bool {
    match token.parse::<u32>() {
        Ok(number) => number <= 32767,
        Err(_) => false,
    }
}


fn split_line_into_tokens(line: &str) -> Vec<String> {
  let mut result = Vec::new();
  let mut current_token = String::new();
  let mut inside_string = false;
  let mut chars = line.chars().peekable();

  while let Some(ch) = chars.next() {
      if ch == '/' && chars.peek() == Some(&'/') {
          break;
      }

      if ch == '/' && chars.peek() == Some(&'*') {
          while let Some(c) = chars.next() {
              if c == '*' && chars.peek() == Some(&'/') {
                  chars.next();
                  break;
              }
          }
          continue;
      }

      if inside_string {
          current_token.push(ch);
          if ch == '"' {
              result.push(current_token.clone());
              current_token.clear();
              inside_string = false;
          }
          continue;
      }

      match ch {
          '"' => {
              if !current_token.is_empty() {
                  result.push(current_token.clone());
                  current_token.clear();
              }
              inside_string = true;
              current_token.push(ch);
          }
          '(' | ')' | '{' | '}' | ',' | '=' | '.' | ';' | '[' | ']' => {
              if !current_token.is_empty() {
                  result.push(current_token.clone());
                  current_token.clear();
              }
              result.push(ch.to_string());
          }
          '-' => {
              if !current_token.is_empty() {
                  result.push(current_token.clone());
                  current_token.clear();
              }
              result.push(ch.to_string());
          }
          _ if ch.is_alphanumeric() || (ch == '.' && !current_token.is_empty()) => {
              current_token.push(ch);
          }
          ' ' => {
              if !current_token.is_empty() {
                  result.push(current_token.clone());
                  current_token.clear();
              }
          }
          _ => {
              if !current_token.is_empty() {
                  result.push(current_token.clone());
                  current_token.clear();
              }
              result.push(ch.to_string());
          }
      }
  }

  if !current_token.is_empty() {
      result.push(current_token);
  }

  result
}

// Replace &, <, >, \ by &amp;, &lt;, &gt;, &quot;
fn escape_special_chars(input: &str) -> String {
    input
        .replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
        .replace("\"", "&quot;")
}

