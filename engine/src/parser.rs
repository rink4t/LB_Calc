// Notes(o･ω･o): I use the following pattern only for order
// Some struct and impl
// |------------{Some info ( . .)φ}----------------|
// another struct and impl


//|-----------------{Diagnostics ( . .)φ}------------------|

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

pub enum Token {
    Var(String),
    Ope(String),
    Bad(String),
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
        Token::Eof
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