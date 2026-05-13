
use std::collections::HashMap;

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
    pub fn _print_tree(&self, prefix: &str, is_left: bool, is_root: bool){
        if is_root{
            println!("──{:?}", self.token);
        }else {
            println!("{}{}{:?}", prefix, if is_left {"├──"} else {"└──"}, self.token);
        }
        
        let new_prefix = format!("{prefix}{}", if is_root {"   "} else {if is_left {"│  "} else {"   "}});

        if let Some(lnode) = &self.left { lnode._print_tree(new_prefix.as_str(), true, false); }

        if let Some(rnode) = &self.right { rnode._print_tree(new_prefix.as_str(), false, false); }
    }

    fn value_from_variable(&self, var: &String, variables: &HashMap<String, Vec<bool>>, pos: usize) -> bool{
        match variables.get(var).cloned() {
            Some(value) => {
                match value.get(pos) {
                    Some(val) => *val,
                    None => false,
                }
            },
            None => false,
        }
    }

    pub fn evaluate(&self, variables: &HashMap<String, Vec<bool>>, pos: usize) -> bool{

        match &self.token {
            Token::Var(x) => {
                self.value_from_variable(x, &variables, pos)
            },
            
            Token::Ope(op) => {
                let l_eval: bool = self.left.as_ref().map(|left| left.evaluate(&variables, pos)).unwrap_or(false);
                let r_eval: bool = self.right.as_ref().map(|right| right.evaluate(&variables, pos)).unwrap_or(false);

                match op.as_str() {
                    "∧" => l_eval && r_eval,
                    "∨" => l_eval || r_eval,
                    "¬" => !r_eval,
                    "↔" => l_eval == r_eval,
                    "→" => !l_eval || r_eval,
                    _ => unreachable!()
                }
            }
            _ => {unreachable!()}
        }
    }
}