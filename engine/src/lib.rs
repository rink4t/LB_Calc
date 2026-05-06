use std::{collections::HashMap};

use crate::{ast::ExpressionAST, parser::{Diagnostic, Lexer, Parser}, properties::Properties, truth_table::{ResTables, TruthTable}};

mod parser;
mod ast;
mod truth_table;
mod properties;

#[derive(Default)]
pub struct ExprRes{
    main_vars: Vec<ResTables>,
    res_vars: Vec<ResTables>,
    err_msg: String,
    properties: Properties,
}

impl ExprRes {
    
}

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
            for errs in self.diags.get_msgs() {
                println!("{errs}");
            }
        }
        expr_res
    }

    fn evaluate(&mut self, entry: String, expr_res: &mut ExprRes){
        let mut parser: Parser = Parser::new(Lexer::new(entry.clone()));
        let (asts, is_equiv) = parser.build_asts();

        if !parser.plexer.diags.is_empty() {
            match parser.diags.get_msgs().first() {
                Some(msg) => {expr_res.err_msg = msg.clone()},
                None => {}
            }
            return;
        }

        if !parser.diags.is_empty() {
            match parser.diags.get_msgs().first() {
                Some(msg) => {expr_res.err_msg = msg.clone()},
                None => {}
            }
            return;
        }

        let vars_buff = parser.generate_var_buffer();
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
            expr_res.properties.set_properties(table, true);
        }

        expr_res.res_vars = expre_table.map_table_to_restable(&vars_buff, res_expr_table, Some(expressions));//returns the 
        expr_res.main_vars = expre_table.map_table_to_restable(&vars_buff, variables, None);//returns the operat
    }

    fn simpl_eval(&mut self, asts: Vec<ExpressionAST>, mut expre_table: TruthTable, variables: HashMap<String, Vec<bool>>, expr_res: &mut ExprRes, vars_buff: Vec<String>, expressions: Vec<String>){
        let res_expr_table = expre_table.generate_table_from_ast(asts, &variables); 

        if let Some(table) = res_expr_table.get("A"){
            expr_res.properties.set_properties(table, false);
        }

        expr_res.res_vars = expre_table.map_table_to_restable(&vars_buff, res_expr_table, Some(expressions)); // the expressions truth result values  
        expr_res.main_vars = expre_table.map_table_to_restable(&vars_buff, variables, None); //the vars of the main table
    }

}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, iter::Map};

    use crate::{ast::ExpressionAST, parser::{Lexer, Parser, Token}, truth_table::TruthTable};

    use super::*;

    #[test]
    fn engine_test() {
        let mut engine = Engine::new();
        let res = engine.solve_expr("(a&b)".to_string());

        println!("{}", res.err_msg);

        println!("Working...");
        for x in res.res_vars {
            match x {
                ResTables::Table(id, data) => {
                    println!("{id}");
                    for y in data {
                        println!("{y}");
                    }
                }
            }
        }
    }
}
