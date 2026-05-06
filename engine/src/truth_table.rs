use std::{collections::HashMap};

use crate::ast::ExpressionAST;

//|-----------------{Res Tables ( . .)φ}------------------|
pub enum ResTables {
    Table(String, Vec<bool>)
}

impl Default for ResTables {
    fn default() -> Self {
        ResTables::Table(String::new(), Vec::new())
    }
}

//|-----------------{Truth Table ( . .)φ}------------------|

pub struct TruthTable{
    pub rows: usize,
    pub colums: usize,   
}

impl TruthTable {
    pub fn new(vars_buff_len: usize) -> TruthTable{
        let tmp_rows: usize = 2usize.pow(vars_buff_len as u32);
        let tmp_cols: usize = vars_buff_len;
        TruthTable { rows: tmp_rows, colums: tmp_cols }
    }

    pub fn table_init(&mut self, vars_buff: &Vec<String>) -> HashMap<String, Vec<bool>>{
        let vars: Vec<String> = vars_buff.clone();
        let mut true_table: HashMap<String, Vec<bool>> = HashMap::new();
        let mut colums = self.table_gen();
        colums.reverse();

        for var in vars {
            match colums.pop() {
                Some(col) => {
                    true_table.insert(var, col);
                },
                None => {},
            }
        }

        true_table
    }

    fn table_gen(&mut self) -> Vec<Vec<bool>>{
        let mut vect: Vec<Vec<bool>> = Vec::new();

        let rows = self.rows;
        let colms = self.colums;

        for x in 0..=colms{
            let mut tmpv: Vec<bool> = Vec::new();
            tmpv.reserve(self.rows as usize);

            if x == 0 { // this case handles the firs colum with the forg pattern
                vect.push(self.first_colm(rows));
                continue;
            }
            
            if x >= colms {//generates the last colum this is half true half false
                let subr = rows / 2;
                tmpv.extend(std::iter::repeat(false).take(subr));
                tmpv.extend(std::iter::repeat(true).take(subr));
                vect.push(tmpv);
                break;
            }
            if x > 0 && x < colms {
                let bloks_size: usize = 2usize.pow(x as u32);
                for y in 0..rows / bloks_size {
                    if y % 2 == 0{
                        tmpv.extend(std::iter::repeat(false).take(bloks_size));
                    }else {
                        tmpv.extend(std::iter::repeat(true).take(bloks_size));
                    }
                }
                vect.push(tmpv);
                continue;
            }
        }
        vect
    }

    fn first_colm(&self, rows: usize) -> Vec<bool>{
        let mut colm: Vec<bool> = Vec::new();
        for y in 0..rows{
            if y % 2 == 0{
                colm.push(false);
            }else {
                colm.push(true);
            }
        }
        return colm;
    }

    pub fn generate_table_from_ast(&mut self, expr_ast: Vec<ExpressionAST>, vars: &HashMap<String, Vec<bool>>) -> HashMap<String, Vec<bool>> {
        let mut exprmap = HashMap::new();

        for expre in expr_ast {
                match expre {
                ExpressionAST::Expression(id, ast ) => {
                    let mut tmp_vect: Vec<bool> = Vec::new();
                    for x in 0..self.rows {
                        tmp_vect.push(ast.evaluate(vars, x));
                    }
                    exprmap.insert(id, tmp_vect);
                }
            }
        }
        exprmap
    }

    pub fn map_table_to_restable(&self, vars_buffer: &Vec<String>, map_table: HashMap<String, Vec<bool>>, exprname: Option<Vec<String>>) -> Vec<ResTables>{
        let mut restable: Vec<ResTables> = Vec::new();
        
        if let Some(names_buff) = exprname{
            self.using_exprname(&mut restable, map_table, names_buff);
        }else {
            self.using_varsname(&mut restable, &vars_buffer, map_table);
        }
        restable
    }
    

    /// # using_exprname 
    /// This function is only use to pass the expression name in place of the default var name assigned during the parsing
    /// to the expressions the defaults are 'A' and 'B' 
    /// The resultant table is similar to:
    /// | a&b |
    fn using_exprname(&self, restable: &mut Vec<ResTables>, map_table: HashMap<String, Vec<bool>>, mut names: Vec<String>){
        
        let exprids = vec!["A", "B"];
        
        for vars in exprids {
            if let Some(colum) = map_table.get(vars){
                match names.pop() {
                    Some(name) => {restable.push(ResTables::Table(name, colum.clone()));},
                    None => {break;}
                }
            }
        }
    }

    fn using_varsname(&self, restable: &mut Vec<ResTables>, vars_buffer: &Vec<String>, map_table: HashMap<String, Vec<bool>>){
        for vars in vars_buffer {
            if let Some(colum) = map_table.get(vars){
                restable.push(ResTables::Table(vars.clone(), colum.clone()));
            }
        }
    }

}