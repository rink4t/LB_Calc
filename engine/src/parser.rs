// Notes(o･ω･o): I use the following pattern only for order
// Some struct and impl
// |------------{Some info ( . .)φ}----------------|
// another struct and impl


//|-----------------{Diagnostics ( . .)φ}------------------|

use std::fmt::format;

use crate::ast::{AST, ExpressionAST};

pub struct Diagnostic {
    msgs: Vec<String>,
    owner: String,
}

impl Diagnostic {
    pub fn new(owner_str: String) -> Diagnostic{
        Diagnostic{msgs: Vec::new(), owner: owner_str}
    }

    pub fn add_msg(&mut self, msg: &str){
        let mut err_msg: String = self.owner.clone();
        err_msg.push_str(format!(" {msg}").as_str());
    }

    fn add_err_msg(&mut self, msg: &str, token: Token){
        let mut token_info: String = self.owner.clone();
        token_info.push_str(match token {
            Token::Var(data) => format!(": {msg} {data}"),
            Token::Ope(data) => format!(": {msg} {data}"),
            Token::Bad(data) => format!(": {msg} {data}"),
            Token::Equivl => format!("{msg} {:?}", token),
            Token::OpenPar | Token::ClosePar => format!("{msg} {:?}", token),
            Token::Eof => format!(": Premature EOF"),
        }.as_str());
        
        self.msgs.push(token_info);
    }

    pub fn is_empty(&self) -> bool{
        self.msgs.is_empty()
    }

    pub fn get_msgs(&self) -> Vec<String>{
        self.msgs.clone()
    }

} //I plan to change Diagnostic to its own rs only for simplicity( . .)φ

//|-----------------{Lexer and Token( . .)φ}------------------|

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Var(String),
    Ope(String),
    Bad(String),
    OpenPar,
    ClosePar,
    Equivl,
    Eof,
}

pub struct Lexer {
    expression: Vec<char>,
    pos: usize,
    pub diags: Diagnostic,
}

impl Lexer {
    pub fn new(entry: String) -> Lexer{
        let ch_buffer: Vec<char> = entry.chars().collect();
        Lexer { expression: ch_buffer, pos: 0, diags: Diagnostic::new("Lexer".to_string()) }
    }

    pub fn lex(&mut self) -> Token{
        let item_ch: char = self.next();

        if item_ch.is_ascii_whitespace(){
            self.diags.add_err_msg("whitespaces aren't allowed", Token::Bad(item_ch.to_string()));
            return Token::Bad(item_ch.to_string());
        }

        if item_ch.is_ascii_digit(){
            self.diags.add_err_msg("Numbers aren't allowed as a variable", Token::Bad(item_ch.to_string()));
            return Token::Bad(item_ch.to_string());
        }

        match item_ch {
            'a'..='z' | 'A'..='Z' => {
                let mut variable: String = item_ch.to_string();
                loop {
                    let tmp_ch = self.peek();
                    match tmp_ch {
                        'a'..='z' | 'A'..='Z' => {
                            self.next();
                            variable.push(tmp_ch);
                        }
                        _ => {break;}
                    }
                }
                return Token::Var(variable);
            }
            '&' => Token::Ope("∧".to_string()),
            '|' => Token::Ope("∨".to_string()),
            '!' => Token::Ope("¬".to_string()),
            '<' =>{ match self.next() {
                        '-' => {
                            let tmp = self.next();
                            if tmp == '>'{
                                return Token::Ope("↔".to_string());
                            }else {
                                self.diags.add_err_msg("Bad token expected '>' no: ", Token::Bad(tmp.to_string()));
                                return Token::Bad(tmp.to_string());
                            }
                        },
                        err => {
                            self.diags.add_err_msg("Bad token expected '-' no: ", Token::Bad(err.to_string()));
                            return Token::Bad(self.peek().to_string());
                        },
                    }
                },
            '-' => {
                match self.next() {
                    '>' => return Token::Ope(String::from("→")),
                    err => {
                        self.diags.add_err_msg("Bad token expected '>' no: ", Token::Ope(err.to_string()));
                        return Token::Bad(err.to_string());
                    }
                }
            },
            '=' => Token::Equivl,
            '(' => Token::OpenPar,
            ')' => Token::ClosePar,
            '\0' => Token::Eof,
            bad => {self.diags.add_err_msg("Invalid operator, simbol or variable:", Token::Bad(bad.to_string())); return Token::Bad(bad.to_string());},    
        }
    }

    pub fn next(&mut self) -> char{
        let ch = self.peek();
        self.pos += 1;
        ch
    }

    pub fn peek(&mut self) -> char{
        match self.expression.get(self.pos) {
            Some(ch) => *ch,
            None => '\0',
        }
    }

    pub fn reset(&mut self){
        self.pos = 0;
    }

}

//|-----------------{Parser ( . .)φ}------------------|

pub struct Parser{
    pub plexer: Lexer, 
    pub diags: Diagnostic, //I plan build a fn who cares about the errors from the lexer and parser(in the parser)
    current: Token,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Parser{
        let mut parser = Parser{plexer: lexer, diags: Diagnostic::new("Parser".to_string()), current: Token::Eof};
        parser.initparser();
        parser
    }

    fn initparser(&mut self){
        self.next_token();
    }

    fn next_token(&mut self){
        self.current = self.plexer.lex();
    }

    fn parse_to_ast(&mut self, min_op: u8) -> AST{

        let mut node: AST = match self.current.clone() {
            Token::Var(var) => {
                self.next_token();
                AST { left: None, right: None, token: Token::Var(var) }
            },
            Token::OpenPar => {
                self.next_token();
                let node: AST = self.parse_to_ast(0);
                match self.current.clone() {
                    Token::ClosePar => {self.next_token(); node}
                    _ => {
                        self.diags.add_err_msg("Error missing:", Token::ClosePar);
                        node
                    }
                }
            },
            Token::Ope(op) if self.prefix_operators(op.as_str()) == ((), 9) => {
                self.next_token();
                AST { left: None, right: Some(Box::new(self.parse_to_ast(9))), token: Token::Ope(op) }
            },

            bad => {self.next_token(); self.diags.add_err_msg("Invalid token expected a var or valid prefix operant: ", bad.clone()); AST { left: None, right: None, token: bad}},
        };

        loop {
            let op: String = match self.current.clone() {
                Token::Eof => break,
                Token::Equivl => break,
                Token::ClosePar => break,
                Token::Ope(op) => op,
                Token::Var(bad) => {self.next_token(); self.diags.add_err_msg("Invalid token operator expected! ", Token::Var(bad)); break;},
                bad => {self.next_token(); self.diags.add_err_msg("Invalid token expected a infix operator", bad); break;},
            };
            
            if let Some((lop, rop)) = self.infix_operators(op.as_str()){
                if lop < min_op{ break; }

                self.next_token();
                
                let rhs = self.parse_to_ast(rop);
                node = AST { left: Some(Box::new(node)), right: Some(Box::new(rhs)), token: Token::Ope(op.to_string())};
                continue;
            }
            break;
        }
        return node;
    }

    fn infix_operators(&self, op: &str) -> Option<(u8, u8)>{
        match op {
            "↔" => Some((1, 2)),
            "→" => Some((3, 4)),
            "∨" => Some((5, 6)),
            "∧" => Some((7, 8)),
            _ => None,
        }
    }

    fn prefix_operators(&self, op: &str) -> ((), u8){
        match op {
            "¬" => ((), 9),
            _ => ((), 0),
        }
    }

    pub fn build_asts(&mut self) -> (Vec<ExpressionAST>, bool){
        let mut expr_asts: Vec<ExpressionAST> = Vec::new();
        let mut equiv_expr = false;

        expr_asts.push(ExpressionAST::Expression("A".to_string(), self.parse_to_ast(0)));

        if self.current == Token::Equivl{
            self.next_token();

            expr_asts.push(ExpressionAST::Expression("B".to_string(), self.parse_to_ast(0)));
            if self.current == Token::Equivl{
                self.diags.add_err_msg("Error multiple equivalence operators!", Token::Equivl);
            }
            equiv_expr = true;
        }
        (expr_asts, equiv_expr)
    }

        pub fn generate_var_buffer(&mut self) -> Vec<String>{
        self.plexer.reset();
        self.next_token();

        let mut vars_buff: Vec<String> = Vec::new();

        loop {
            match self.current.clone() {
                Token::Var(v) => {
                    if !vars_buff.contains(&v) {vars_buff.push(v);}
                }
                Token::Eof => {break;},
                _ => {},
            }
            self.next_token();
        }

        return vars_buff;
    }

    pub fn get_expressions(&mut self) -> Vec<String>{
        self.plexer.reset();
        self.next_token();

        let mut expressions: Vec<String> = Vec::new();

        loop {
            let mut tmp_expr = String::new();

            if self.current == Token::Eof {
                break;
            }else {
                loop {
                    match self.current.clone() {
                        Token::Var(data) => {tmp_expr.push_str(&data);},
                        Token::Equivl => {break;},
                        Token::Ope(data) => {tmp_expr.push_str(&data);},
                        Token::Eof => {break;}
                        _ => {},
                    }
                    self.next_token();
                }
                expressions.push(tmp_expr);
            }
            self.next_token();
        }

        expressions
    }

}