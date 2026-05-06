
use crate::parser::Token;

//|-----------------{AST ( . .)φ}------------------|

pub enum ExpressionAST {
    Expression(String, AST)
}

pub struct AST{
    pub left: Option<Box<AST>>,
    pub right: Option<Box<AST>>,
    pub token: Token,
}

impl AST {
    //only for test purposes! (o･ω･o)
    pub fn print_tree(&self, prefix: &str, is_left: bool, is_root: bool){
        if is_root{
            println!("──{:?}", self.token);
        }else {
            println!("{}{}{:?}", prefix, if is_left {"├──"} else {"└──"}, self.token);
        }
        
        let new_prefix = format!("{prefix}{}", if is_root {"   "} else {if is_left {"│  "} else {"   "}});

        if let Some(lnode) = &self.left { lnode.print_tree(new_prefix.as_str(), true, false); }

        if let Some(rnode) = &self.right { rnode.print_tree(new_prefix.as_str(), false, false); }
    }
}