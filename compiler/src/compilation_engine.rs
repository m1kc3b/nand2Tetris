use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

const LET_STATEMENT: &str = "<keyword> let </keyword>";
const IF_STATEMENT: &str = "<keyword> if </keyword>";
const WHILE_STATEMENT: &str = "<keyword> while </keyword>";
const DO_STATEMENT: &str = "<keyword> do </keyword>";
const RETURN_STATEMENT: &str = "<keyword> return </keyword>";
const SEMICOLON_SYMBOL: &str = "<symbol> ; </symbol>";
const CONSTRUCTOR: &str = "<keyword> constructor </keyword>";
const FUNCTION: &str = "<keyword> function </keyword>";
const METHOD: &str = "<keyword> method </keyword>";
const STATIC: &str = "<keyword> static </keyword>";
const FIELD: &str = "<keyword> field </keyword>";
const OPEN_BRACKET: &str = "<symbol> { </symbol>";
const CLOSE_BRACKET: &str = "<symbol> } </symbol>";
const OPEN_PARENTHESIS: &str = "<symbol> ( </symbol>";
const CLOSE_PARENTHESIS: &str = "<symbol> ) </symbol>";
const OPEN_SQUARE_BRACKET: &str = "<symbol> [ </symbol>";
const CLOSE_SQUARE_BRACKET: &str = "<symbol> ] </symbol>";
const DOT_SYMBOL: &str = "<symbol> . </symbol>";
const COMMA_SYMBOL: &str = "<symbol> , </symbol>";
const PLUS_SYMBOL: &str = "<symbol> + </symbol>";
const MINUS_SYMBOL: &str = "<symbol> - </symbol>";
const ASTERISK_SYMBOL: &str = "<symbol> * </symbol>";
const SLASH_SYMBOL: &str = "<symbol> / </symbol>";
const EQUAL_SYMBOL: &str = "<symbol> = </symbol>";
const START_TOKEN: &str = "<tokens>";
const END_TOKEN: &str = "</tokens>";

#[derive(Debug)]
pub struct CompilationEngine {
    tokens: Vec<String>,
    position: usize,
    output: File,
}

impl CompilationEngine {
    pub fn new(tokens: Vec<String>, source_path: &str) -> io::Result<Self> {
        let output_path = Path::new(source_path)
            .with_extension("xml")
            .to_str()
            .unwrap()
            .to_string();

        let file = File::create(output_path)?;

        let mut engine = Self {
            tokens,
            position: 0,
            output: file,
        };

        engine.compile_class()?;

        Ok(engine)
    }

    // Get current token
    fn peek(&self) -> Option<&String> {
        self.tokens.get(self.position)
    }

    // Get next token
    fn peek_next_token(&self) -> Option<&String> {
        self.tokens.get(self.position + 1)
    }

    // Advance to next token
    fn advance(&mut self) -> Option<&String> {
        let token = self.tokens.get(self.position);
        self.position += 1;
        token
    }

    // Write to output file
    fn write(&mut self, content: &str) -> io::Result<()> {
        writeln!(self.output, "{}", content)
    }

    // Write current token to output file
    fn write_token(&mut self) -> io::Result<()> {
        if let Some(token) = self.peek().cloned() {
            self.advance();
            self.write(&token)
        } else {
            Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "Unexpected end of tokens",
            ))
        }
    }

    // TEST:
    // Compiles a complete class
    // 'class' className '{' classVarDec* subroutineDec* '}'
    pub fn compile_class(&mut self) -> io::Result<()> {
        self.write("<class>")?;

        // Ignore "<tokens>"
        if let Some(token) = self.peek() {
            if token == START_TOKEN {
                self.advance();
            }
        }

        // Check "class"
        if let Some(token) = self.peek() {
            if token == "<keyword> class </keyword>" {
                self.write_token()?;
            } else {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Expected 'class' keyword",
                ));
            }
        }

        // Class name
        self.write_token()?; // <identifier> className </identifier>

        // "{"
        if self.peek() == Some(&OPEN_BRACKET.to_string()) {
            self.write_token()?;
        } else {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Expected '{'"));
        }

        // ClassVardec or subroutineDec
        while let Some(token) = self.peek() {
            if token == STATIC || token == FIELD {
                self.compile_class_var_dec()?;
            } else if token == CONSTRUCTOR || token == FUNCTION || token == METHOD {
                self.compile_subroutine()?;
            } else {
                break;
            }
        }

        // Check class ends with "}"
        if let Some(token) = self.peek() {
            if token == CLOSE_BRACKET {
                self.write_token()?; // adds "}"
            } else {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Expected '}}' but found {:?}", self.peek()),
                ));
            }
        } else {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "Unexpected end of tokens",
            ));
        }

        self.write("</class>")?;
        Ok(())
    }

    // TEST
    // Compiles a complete method, function or constructor
    // ('constructor' | 'function' | 'method') ('void' | type) subroutineName '(' parameterList ')' subroutineBody
    pub fn compile_subroutine(&mut self) -> io::Result<()> {
        self.write("<subroutineDec>")?;

        // Listen to "("
        while let Some(token) = self.peek() {
            if token == OPEN_PARENTHESIS {
                break;
            }
            self.write_token()?;
        }

        // Parenthesis "("
        if let Some(token) = self.peek() {
            if token == OPEN_PARENTHESIS {
                self.write_token()?; // <symbol> ( </symbol>
                self.compile_parameter_list()?;
                self.write_token()?; // <symbol> ) </symbol>
            } else {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Expected '(' after function name",
                ));
            }
        }

        self.compile_subroutine_body()?;

        self.write("</subroutineDec>")?;
        Ok(())
    }

    // TEST
    // Compiles a static declaration or a field declaration
    // 'static' | 'field' type varName (',' varName)* ';'
    pub fn compile_class_var_dec(&mut self) -> io::Result<()> {
        self.write("<classVarDec>")?;

        if let Some(token) = self.peek() {
            if token == STATIC || token == FIELD {
                self.write_token()?; // <keyword> static ou field </keyword>
            } else {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Expected 'static' or 'field'",
                ));
            }
        }

        // Listen to find ";"
        while let Some(token) = self.peek() {
            if token == SEMICOLON_SYMBOL {
                break;
            }
            self.write_token()?;
        }

        // Check and write ";"
        if let Some(token) = self.peek() {
            if token == SEMICOLON_SYMBOL {
                self.write_token()?;
            } else {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Expected ';' at end of var declaration",
                ));
            }
        }

        self.write("</classVarDec>")?;
        Ok(())
    }

    // Compiles a (possibly empty) parameter list, not including the enclosing "()"
    // ((type varName) (',' type varName)*)?
    pub fn compile_parameter_list(&mut self) -> io::Result<()> {
        self.write("<parameterList>")?;

        // Loop while next token is not ")"
        // TODO: handle this case: (((y + size) < 254) & ((x + size) < 510))
        while self.peek().unwrap() != CLOSE_PARENTHESIS {
            self.write_token()?;
        }

        self.write("</parameterList>")?;
        Ok(())
    }

    // Compiles a subroutine's body
    // '{' varDec* statements '}'
    pub fn compile_subroutine_body(&mut self) -> io::Result<()> {
        self.write("<subroutineBody>")?;
        self.write_token()?; // <symbol> { </symbol>

        // Handle varDec
        while let Some(token) = self.peek() {
            if token == "<keyword> var </keyword>" {
                self.compile_var_dec()?;
            } else {
                break;
            }
        }

        self.compile_statements()?;


        // If it remains tokens = Error!
        if let Some(token) = self.peek() {
            if token == CLOSE_BRACKET {
                self.write_token()?; // write "}"
            } else {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!(
                        "Expected '}}' but found {:?} (Tokens restants non consommés !)",
                        token
                    ),
                ));
            }
        } else {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "Unexpected end of tokens",
            ));
        }

        self.write("</subroutineBody>")?;
        Ok(())
    }

    // Compiles a var declaration
    // 'var' type varName (',' varName)* ';'
    pub fn compile_var_dec(&mut self) -> io::Result<()> {
        self.write("<varDec>")?;

        // Var dec ends with ";"
        while self.peek().unwrap() != SEMICOLON_SYMBOL {
            self.write_token()?;
        }
        // Adds ";" to the end of varDec
        self.write_token()?;

        self.write("</varDec>")?;

        Ok(())
    }

    // Compiles a sequence of statements. Does not handle the enclosing "{}"
    // letStatement | ifStatement | whileStatement | doStatement | returneStatement
    pub fn compile_statements(&mut self) -> io::Result<()> {
        self.write("<statements>")?;
        
        while let Some(token) = self.peek() {
            match token.as_str() {
                LET_STATEMENT => self.compile_let()?,
                IF_STATEMENT => self.compile_if()?,
                WHILE_STATEMENT => self.compile_while()?,
                DO_STATEMENT => self.compile_do()?,
                RETURN_STATEMENT => self.compile_return()?,
                _ => break, // Stop if there no longer statement
            }
        }
    
        self.write("</statements>")?;
        Ok(())
    }
    

    // Compiles a let statement
    // 'let' varName ('[' expression ']')? '=' expression ';'
    pub fn compile_let(&mut self) -> io::Result<()> {
        self.write("<letStatement>")?;
        
        if let Some(token) = self.peek() {
            if token == LET_STATEMENT {
                self.write_token()?; // <keyword> let </keyword>
            } else {
                return Err(io::Error::new(io::ErrorKind::InvalidData, "Expected 'let' keyword"));
            }
        }
    
        self.write_token()?; // <identifier> varName </identifier>
    
        // Check "let a[i] = value;"
        if let Some(token) = self.peek() {
            if token == OPEN_SQUARE_BRACKET {
                self.write_token()?; // "["
    
    
                self.compile_expression()?; // Compile array index
    
    
                if let Some(next_token) = self.peek() {
                    if next_token == CLOSE_SQUARE_BRACKET {
                        self.write_token()?; // "]"
                    } else {
                        return Err(io::Error::new(io::ErrorKind::InvalidData, "Expected ']' after array index"));
                    }
                }
            }
        }
    
        // Check "="
        if let Some(token) = self.peek() {
            if token == EQUAL_SYMBOL {
                self.write_token()?; // "="
    
                self.compile_expression()?; // Compile this expression
    
            } else {
                return Err(io::Error::new(io::ErrorKind::InvalidData, "Expected '=' in let statement"));
            }
        }
    
    
        if let Some(token) = self.peek() {
            if token == SEMICOLON_SYMBOL {
                self.write_token()?; // ";"
            } else {
                return Err(io::Error::new(io::ErrorKind::InvalidData, "Expected ';'"));
            }
        }
    
        self.write("</letStatement>")?;
        Ok(())
    }
    

    // Compiles an if statement, possibly with a trailing else clause
    // 'if' '(' expression ')' '{' statements '}' ('else' '{' statements '}')?
    pub fn compile_if(&mut self) -> io::Result<()> {
        self.write("<ifStatement>")?;

        // Lire "if"
        if let Some(token) = self.peek() {
            if token == IF_STATEMENT {
                self.write_token()?; // <keyword> if </keyword>
            } else {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Expected 'if' keyword",
                ));
            }
        }

        // Check "("
        if let Some(token) = self.peek() {
            if token == OPEN_PARENTHESIS {
                self.write_token()?; // <symbol> ( </symbol>
                self.compile_expression()?; // Compiler l'expression conditionnelle
                self.write_token()?; // <symbol> ) </symbol>
            } else {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Expected '(' in if statement",
                ));
            }
        }

        // Check "{"
        if let Some(token) = self.peek() {
            if token == OPEN_BRACKET {
                self.write_token()?; // <symbol> { </symbol>
                self.compile_statements()?; // Compiler les statements
                self.write_token()?; // <symbol> } </symbol>
            } else {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Expected '{' in if statement",
                ));
            }
        }

        // Check if there is an "else"
        if let Some(token) = self.peek() {
            if token == "<keyword> else </keyword>" {
                self.write_token()?; // <keyword> else </keyword>
                self.write_token()?; // <symbol> { </symbol>
                self.compile_statements()?; // Compiler les statements du else
                self.write_token()?; // <symbol> } </symbol>
            }
        }

        self.write("</ifStatement>")?;
        Ok(())
    }

    // Compiles a while statement
    // 'while' '(' expression ')' '{' statements '}'
    pub fn compile_while(&mut self) -> io::Result<()> {
        self.write("<whileStatement>")?;
        self.write_token()?; // "while"
    
        // "("
        if self.peek() == Some(&OPEN_PARENTHESIS.to_string()) {
            self.write_token()?; // "("
            self.compile_expression()?; // Condition du while
            self.write_token()?; // ")"
        } else {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Expected '(' after while"));
        }
    
        // "{"
        if self.peek() == Some(&OPEN_BRACKET.to_string()) {
            self.write_token()?; // "{"
            self.compile_statements()?; // Instructions dans le while
        } else {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Expected '{' after while condition"));
        }
    
        // "}"
        if self.peek() == Some(&CLOSE_BRACKET.to_string()) {
            self.write_token()?; // "}"
        } else {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Expected '}' at end of while statement"));
        }
    
        self.write("</whileStatement>")?;
        Ok(())
    }
    
    

    // Compiles a do statement
    // 'do' subroutineCall ';'
    pub fn compile_do(&mut self) -> io::Result<()> {
        self.write("<doStatement>")?;

        if let Some(token) = self.peek() {
            if token == DO_STATEMENT {
                self.write_token()?; // <keyword> do </keyword>
            } else {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Expected 'do' keyword",
                ));
            }
        }

        self.write_token()?; // <identifier> functionName </identifier>

        // Check if it is a method call with "."
        if let Some(token) = self.peek() {
            if token == DOT_SYMBOL {
                self.write_token()?; // <symbol> . </symbol>
                self.write_token()?; // <identifier> methodName </identifier>
            }
        }

        // "("
        if let Some(token) = self.peek() {
            if token == OPEN_PARENTHESIS {
                self.write_token()?; // <symbol> ( </symbol>
                self.compile_expression_list()?; // Compiler les arguments
                self.write_token()?; // <symbol> ) </symbol>
            } else {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Expected '(' in do statement",
                ));
            }
        }

        if let Some(token) = self.peek() {
            if token == SEMICOLON_SYMBOL {
                self.write_token()?; // <symbol> ; </symbol>
            } else {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Expected ';' in do statement",
                ));
            }
        }

        self.write("</doStatement>")?;
        Ok(())
    }

    // Compiles a return statement
    // 'return' expression? ';'
    pub fn compile_return(&mut self) -> io::Result<()> {
        self.write("<returnStatement>")?;
        self.write_token()?; // <keyword> return </keyword>

        // If there is an expression before ";"
        if let Some(token) = self.peek() {
            if token != SEMICOLON_SYMBOL {
                self.compile_expression()?;
            }
        }

        // ";"
        if self.peek() == Some(&SEMICOLON_SYMBOL.to_string()) {
            self.write_token()?;
        } else {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Expected ';'"));
        }

        self.write("</returnStatement>")?;
        Ok(())
    }

    // Compiles an expression
    // term (op term)*
    pub fn compile_expression(&mut self) -> io::Result<()> {
        self.write("<expression>")?;
        
        self.compile_term()?; // Compile first one
    
        while let Some(token) = self.peek() {
            if token == PLUS_SYMBOL || token == MINUS_SYMBOL || token == ASTERISK_SYMBOL ||
               token == SLASH_SYMBOL || token == EQUAL_SYMBOL || token == "<symbol> &lt; </symbol>" {
                self.write_token()?; // write operator
                self.compile_term()?; // Compile the next one
            }
            // `sum[i]`
            else if token == OPEN_SQUARE_BRACKET {
                self.write_token()?; //  "["
                self.compile_expression()?;
                if let Some(close_bracket) = self.peek() {
                    if close_bracket == CLOSE_SQUARE_BRACKET {
                        self.write_token()?; // "]"
                    } else {
                        return Err(io::Error::new(io::ErrorKind::InvalidData, "Expected ']' after array index"));
                    }
                }
            }
            else {
                break;
            }
        }
    
        self.write("</expression>")?;
        Ok(())
    }
    
    

    // Compiles a term
    // intergerConstant | stringConstant |keywordConstant | varName | varName '[' expression ']' | '(' expression ')' | (unaryOp term) | subroutineCall
    pub fn compile_term(&mut self) -> io::Result<()> {
        self.write("<term>")?;
        
        if let Some(token) = self.peek() {
            self.write_token()?;
    
            if let Some(next_token) = self.peek() {
                if next_token == DOT_SYMBOL {
                    self.write_token()?; // "."
    
                    if let Some(method_name) = self.peek() {
                        if method_name.starts_with("<identifier>") {
                            self.write_token()?; // Method name
                        } else {
                            return Err(io::Error::new(io::ErrorKind::InvalidData, "Expected method name after '.'"));
                        }
                    }
    
                    // "("
                    if let Some(paren) = self.peek() {
                        if paren == OPEN_PARENTHESIS {
                            self.write_token()?; // "("
                            self.compile_expression_list()?; // Compile arguments
                            if let Some(close_paren) = self.peek() {
                                if close_paren == CLOSE_PARENTHESIS {
                                    self.write_token()?; // ")"
                                } else {
                                    return Err(io::Error::new(io::ErrorKind::InvalidData, "Expected ')' after method arguments"));
                                }
                            }
                        } else {
                            return Err(io::Error::new(io::ErrorKind::InvalidData, "Expected '(' after method name"));
                        }
                    }
                }
            }
        } else {
            return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Unexpected end of tokens"));
        }
    
        self.write("</term>")?;
        Ok(())
    }
    
    

    // Compiles an expression list
    // (expression(',' expression)*)?
    pub fn compile_expression_list(&mut self) -> io::Result<()> {
        self.write("<expressionList>")?;

        if let Some(token) = self.peek() {
            if token != CLOSE_PARENTHESIS {
                self.compile_expression()?;

                while let Some(token) = self.peek() {
                    if token == COMMA_SYMBOL {
                        self.write_token()?; //","
                        self.compile_expression()?;
                    } else {
                        break;
                    }
                }
            }
        }

        if let Some(token) = self.peek() {
            if token != CLOSE_PARENTHESIS {
                return Err(io::Error::new(io::ErrorKind::InvalidData, "Expected ')'"));
            }
        }

        self.write("</expressionList>")?;
        Ok(())
    }
}
