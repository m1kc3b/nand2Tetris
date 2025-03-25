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

        // engine.compile_class()?;

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

    // TODO: Implement this
    // Compiles a complete class
    // 'class' className '{' classVarDec* subroutineDec* '}'
    pub fn compile_class(&mut self) -> io::Result<()> {
        self.write("<class>")?;

        // Pass the first token
        self.advance();

        // Handle class name
        self.write_token()?; // <keyword> class </keyword>
        self.write_token()?; // <identifier> className </identifier>
        self.write_token()?; // <symbol> { </symbol>

        // Handle classVarDec | subroutineDec
        while self.peek().unwrap() != END_TOKEN {
            if self.peek().unwrap() == STATIC || self.peek().unwrap() == FIELD {
                self.compile_class_var_dec()?;
            } else if self.peek().unwrap() == CONSTRUCTOR
                || self.peek().unwrap() == FUNCTION
                || self.peek().unwrap() == METHOD
            {
                self.compile_subroutine()?;
            }
        }

        Ok(())
    }

    // Compiles a complete method, function or constructor
    // ('constructor' | 'function' | 'method') ('void' | type) subroutineName '(' parameterList ')' subroutineBody
    pub fn compile_subroutine(&mut self) -> io::Result<()> {
        self.write("<subroutineDec>")?;

        while self.peek().unwrap() != OPEN_PARENTHESIS {
            self.write_token()?;
        }

        // Handle parameter list
        if self.peek().unwrap() == CLOSE_PARENTHESIS {
            self.write_token()?; // <symbol> ( </symbol>
            self.compile_parameter_list()?;
            self.write_token()?; // <symbol> ) </symbol>
        }

        self.compile_subroutine_body()?;
        self.write("</subroutineDec>")?;

        Ok(())
    }

    // Compiles a static declaration or a field declaration
    // 'static' | 'field' type varName (',' varName)* ';'
    pub fn compile_class_var_dec(&mut self) -> io::Result<()> {
        self.write("<classVarDec>")?;

        if self.peek().unwrap() == STATIC {
            self.write_token()?; // <keyword> static </keyword>
        } else if self.peek().unwrap() == FIELD {
            self.write_token()?; // <keyword> field </keyword>
        }

        // static | field declaration end with ";"
        while self.peek().unwrap() != SEMICOLON_SYMBOL {
            self.write_token()?;
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

        // Handle vardDec | statements
        while self.peek().unwrap() != END_TOKEN {
            //
            if self.peek().unwrap() == "<keyword> var </keyword>" {
                self.compile_var_dec()?;
            }
            // letStatement | ifStatement | whileStatement | doStatement | returneStatement
            else {
                self.compile_statements()?;
            }

            self.advance();
        }

        self.write_token()?; // <symbol> } </symbol>
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

    // TODO: Implement this
    // Compiles a sequence of statements. Does not handle the enclosing "{}"
    // letStatement | ifStatement | whileStatement | doStatement | returneStatement
    pub fn compile_statements(&mut self) -> io::Result<()> {
        self.write("<statements>")?;

        // Loop while next token is not statement
        while self.peek_next_token().unwrap() != LET_STATEMENT
            || self.peek_next_token().unwrap() == IF_STATEMENT
            || self.peek_next_token().unwrap() == WHILE_STATEMENT
            || self.peek_next_token().unwrap() == DO_STATEMENT
            || self.peek_next_token().unwrap() == RETURN_STATEMENT
        {
            // Handle let, if, while, do, return
            if self.peek().unwrap() == LET_STATEMENT {
                self.compile_let()?;
            } else if self.peek().unwrap() == IF_STATEMENT {
                self.compile_if()?;
            } else if self.peek().unwrap() == WHILE_STATEMENT {
                self.compile_while()?;
            } else if self.peek().unwrap() == DO_STATEMENT {
                self.compile_do()?;
            } else if self.peek().unwrap() == RETURN_STATEMENT {
                self.compile_return()?;
            }
        }

        self.write("</statements>")?;
        Ok(())
    }

    // Compiles a let statement
    // 'let' varName ('[' expression ']')? '=' expression ';'
    pub fn compile_let(&mut self) -> io::Result<()> {
        self.write("<letStatement>")?;

        // let statement ends with ";"
        if self.peek().unwrap() != SEMICOLON_SYMBOL {
            self.write_token()?;

            if self.peek().unwrap() == EQUAL_SYMBOL {
                self.compile_expression()?;
            }
        }

        self.write("</letStatement>")?;

        Ok(())
    }

    // Compiles an if statement, possibly with a trailing else clause
    // 'if' '(' expression ')' '{' statements '}' ('else' '{' statements '}')?
    pub fn compile_if(&mut self) -> io::Result<()> {
        Ok(())
    }

    // Compiles a while statement
    // 'while' '(' expression ')' '{' statements '}'
    pub fn compile_while(&mut self) -> io::Result<()> {
        self.write("<whileStatement>")?;

        // while statement ends with "}"
        if self.peek().unwrap() != CLOSE_BRACKET {
            self.write_token()?;

            // (expression)
            if self.peek().unwrap() == OPEN_PARENTHESIS {
                self.write_token()?;
                self.compile_expression()?;
                self.write_token()?;
            }

            // statements
            self.compile_statements()?;
        }

        self.write("</whileStatement>")?;
        Ok(())
    }

    // Compiles a do statement
    // 'do' subroutineCall ';'
    pub fn compile_do(&mut self) -> io::Result<()> {
        Ok(())
    }

    // Compiles a return statement
    // 'return' expression? ';'
    pub fn compile_return(&mut self) -> io::Result<()> {
        self.write_token()?; // <keyword> return </keyword>
        self.write_token()?; // <symbol> ; </symbol>
        self.write_token()?; // <symbol> } </symbol>

        Ok(())
    }

    // Compiles an expression
    // term (op term)*
    pub fn compile_expression(&mut self) -> io::Result<()> {
        self.write("<expression>")?;
        self.compile_term()?;
        self.write("</expression>")?;

        Ok(())
    }

    // TODO: Implement this
    // Compiles a term
    // intergerConstant | stringConstant |keywordConstant | varName | varName '[' expression ']' | '(' expression ')' | (unaryOp term) | subroutineCall
    pub fn compile_term(&mut self) -> io::Result<()> {
        self.write("<term>")?;

        while self.peek().unwrap() != SEMICOLON_SYMBOL {
            // If () -> expressionList
            if self.peek().unwrap() == OPEN_PARENTHESIS {
                self.write_token()?;
                self.compile_expression_list()?;
                self.write_token()?;
            } else {
                self.write_token()?;
            }
        }

        self.write("</term>")?;

        Ok(())
    }

    // TODO: Implement this
    // Compiles an expression list
    // (expression(',' expression)*)?
    pub fn compile_expression_list(&mut self) -> io::Result<()> {
        self.write("<expressionList>")?;
        self.compile_expression()?;
        self.write("</expressionList>")?;
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::{self, BufReader, Read};

    #[test]
    fn compile_var_dec_test() {
        let path = "tests/unitTests";
        let mut engine = CompilationEngine::new(
            vec![
                "<keyword> var </keyword>".to_string(),
                "<keyword> int </keyword>".to_string(),
                "<identifier> x </identifier>".to_string(),
                "<symbol> ; </symbol>".to_string(),
            ],
            "tests/unitTests/compile_var_dec.xml",
        ).unwrap();

        engine.compile_var_dec().unwrap();

        let mut output_file = File::open("tests/unitTests/compile_var_dec.xml").expect("Failed to open output file");
        let mut output_buffer = String::new();
        BufReader::new(&mut output_file).read_to_string(&mut output_buffer).unwrap();

        let expected = "<varDec>\n<keyword> var </keyword>\n<keyword> int </keyword>\n<identifier> x </identifier>\n<symbol> ; </symbol>\n</varDec>\n";

        assert_eq!(output_buffer, expected.to_string());
    }
}
