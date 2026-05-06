mod parser;
mod ast;

#[cfg(test)]
mod tests {
    use crate::{ast::ExpressionAST, parser::{Lexer, Parser}};

use super::*;

    #[test]
    fn engine_test(){

        let mut parser = Parser::new(Lexer::new("(a&b&)".to_string()));
        let (asts, is) = parser.build_asts();

        if parser.plexer.diags.is_empty() {
            if parser.diags.is_empty() {
                for x in asts {
                    match x {
                        ExpressionAST::Expression(id, tree) => {
                            println!("expr id: {id}");
                            tree.print_tree(" ", false, true);
                        }
                    }
                }

            }else {
                for x in parser.diags.get_msgs() {
                    println!("{x}");
                }
            }
        }else {
            for x in parser.plexer.diags.get_msgs() {
                println!("{x}");
            }
        }

    }
    
}
