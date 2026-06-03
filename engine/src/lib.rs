use std::{collections::HashMap};

use crate::{ast::ExpressionAST, parser::{Diagnostic, Lexer, Parser}, properties::Properties, truth_table::{TruthTable}};

mod parser;
mod ast;
mod truth_table;
pub mod properties;

//|-----------------{ExprRes >ᴗ<}------------------|

#[derive(Default)]
pub struct ExprRes{
    pub colums: Vec<Vec<bool>>,
    pub ids: Vec<String>,
    pub err_msg: String,
    pub properties: Properties,
}

//|-----------------{Engine ( . .)φ}------------------|

pub struct Engine{
    diags: Diagnostic,
}

impl Engine {

    pub fn new() -> Engine{
        Engine { diags: Diagnostic::new("Engine".to_string()) }
    }

    fn entry_hndl(&mut self, mut entry: String) -> Option<String>{
        if entry.is_empty() {
            self.diags.add_msg("Empty entry!");
            return None;
        }
        entry = entry.trim().to_string();

        Some(entry)
    }

    pub fn solve_expr(&mut self, entry: String) -> ExprRes{

        let mut expr_res: ExprRes = ExprRes::default();

        if let Some(entry) = self.entry_hndl(entry) {
            self.evaluate(entry, &mut expr_res);
        }else {
            if let Some(msg) = self.diags.get_msgs().first() {
                expr_res.err_msg = msg.clone();   
            }
        }
        expr_res
    }

    fn evaluate(&mut self, entry: String, expr_res: &mut ExprRes){
        let mut parser: Parser = Parser::new(Lexer::new(entry.clone()));
        let (asts, is_equiv) = parser.build_asts();

        if !parser.plexer.diags.is_empty() {
            match parser.plexer.diags.get_msgs().first() {
                Some(msg) => {
                    return expr_res.err_msg = msg.clone();
                },
                None => {}
            }
            return;
        }

        if !parser.diags.is_empty() {
            match parser.diags.get_msgs().first() {
                Some(msg) => {
                    return expr_res.err_msg = msg.clone()
                },
                None => {}
            }
            return;
        }

        let mut vars_buff = parser.generate_var_buffer();
        vars_buff.sort();

        let mut vars_expr_table = TruthTable::new(vars_buff.len());
        let main_vars_table = vars_expr_table.table_init(&vars_buff);

        let expressions: Vec<String> = parser.get_expressions();

        if is_equiv{
            self.equiv_eval(asts, vars_expr_table, main_vars_table, expr_res, vars_buff, expressions);
        }else{
            self.simpl_eval(asts,vars_expr_table, main_vars_table, expr_res, vars_buff, expressions);
        }
    }
 
    fn equiv_eval(&mut self, asts: Vec<ast::ExpressionAST>, mut expre_table: TruthTable, variables: HashMap<String, Vec<bool>>, expr_res: &mut ExprRes, vars_buff: Vec<String>, expressions: Vec<String>){
        let res_expr_table = expre_table.generate_table_from_ast(asts, &variables);

        let mut equivparser = Parser::new(Lexer::new("A<->B".to_string()));
        let (ast, _) = equivparser.build_asts();
        let mut equivtable = TruthTable::new(equivparser.generate_var_buffer().len());

        let res_table = equivtable.generate_table_from_ast(ast, &res_expr_table);

        if let Some(table) = res_table.get("A") {
            let expr_a = if let Some(expr) = res_expr_table.get("A") {expr} else {unreachable!()};
            let expr_b = if let Some(expr) = res_expr_table.get("B") {expr} else {unreachable!()};
            expr_res.properties.set_properties_equivexpr(table, expr_a, expr_b);
        }

        expr_res.colums = expre_table.get_colums(&vars_buff, variables, res_expr_table);
        expr_res.ids = expre_table.get_ids(vars_buff, expressions);
    }

    fn simpl_eval(&mut self, asts: Vec<ExpressionAST>, mut expre_table: TruthTable, variables: HashMap<String, Vec<bool>>, expr_res: &mut ExprRes, vars_buff: Vec<String>, expressions: Vec<String>){
        let res_expr_table = expre_table.generate_table_from_ast(asts, &variables); 

        if let Some(table) = res_expr_table.get("A"){
            expr_res.properties.set_properties(table);
        }

        //expr_res.res_vars = expre_table.map_table_to_restable(&vars_buff, res_expr_table, Some(expressions)); // the expressions truth result values  
        //expr_res.main_vars = expre_table.map_table_to_restable(&vars_buff, variables, None); //the vars of the main table
        expr_res.colums = expre_table.get_colums(&vars_buff, variables, res_expr_table);
        expr_res.ids = expre_table.get_ids(vars_buff, expressions);
    }

}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, io::{self, Write}, iter::Map};

    use crate::{ast::ExpressionAST, parser::{Lexer, Parser, Token}, truth_table::TruthTable};

    use super::*;

    #[test]
    ///Performance test with 19 Vars
    /*fn engine_test() {
        let mut engine = Engine::new();
        let res = engine.solve_expr("a&b&c&d&e&f&g&h&i&j&k&m&n&o&p&q&r&s&t".to_string());
        if res.err_msg.is_empty() {
            let rows = usize::pow(2, 19);
            let items = rows * res.colums[0].len();
            println!("Ready rows: {}, items: {}", rows, items);
        }else {
            println!("Error");
        }
    }*/

    //test1
    fn test1() {
        let mut engine = Engine::new();
        let res = engine.solve_expr("a-b".to_string());
        
    }
}
