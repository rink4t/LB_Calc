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
                '=' => Token::Ope(String::from("≡")),
                '(' | ')' => Token::Ope(item_ch.to_string()),
                '\0' => Token::Eof,
            bad => {self.diags.add_err_msg("Invalid opertor, simbol or variable:", Token::Bad(bad.to_string())); return Token::Bad(bad.to_string());},    
        };
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