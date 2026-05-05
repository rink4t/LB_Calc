// Notes(o･ω･o): I use the following pattern only for order
// Some struct and impl
// |------------{Some info ( . .)φ}----------------|
// another struct and impl


//|-----------------{Diagnostics ( . .)φ}------------------|

pub struct Diagnostic{
    msgs: Vec<String>,
    owner: String,
}

impl Diagnostic{
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

}