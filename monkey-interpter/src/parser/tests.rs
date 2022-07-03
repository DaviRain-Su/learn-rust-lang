use crate::ast::{LetStatement, Node, ReturnStatement, Statement, ExpressionStatement};
use crate::lexer::Lexer;
use crate::parser::Parser;

fn test_let_statements() {
    let input = "
let x  5;
let  = 19;
let  838383;
    ";

    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);

    let program = parser.parse_program();
    check_parser_errors(parser);

    if program.is_none() {
        panic!("parse_program() returned None!")
    }
    let program = program.unwrap();
    let program_len = program.len();

    if program_len != 3 {
        panic!(
            "program statements does not contain 3 statements. got = {}",
            program_len
        );
    }

    let tests = vec!["x", "y", "foobar"];

    for (i, tt) in tests.into_iter().enumerate() {
        let stmt = program.statements.get(i).unwrap();

        if !test_let_statement(stmt, tt.into()) {
            return;
        }
    }
}

fn test_let_statement(s: &Box<dyn Statement>, name: String) -> bool {
    if s.token_literal() != "let" {
        eprint!(
            "Statement token_literal not 'let'. got = {}",
            s.token_literal()
        );
        return false;
    }

    // HOW TODO this convert from box to concept type
    let let_stmt: LetStatement = s.into();

    if let_stmt.name.value != name {
        eprint!(
            "let_stmt.name.value not `{}`. got = {}",
            name, let_stmt.name.value
        );
        return false;
    }

    if let_stmt.name.token_literal() != name {
        eprint!(
            "let_stmt.name.token_literal() not `{}`. got = {}",
            name,
            let_stmt.name.token_literal()
        );
        return false;
    }

    true
}
fn test_return_statements() {
    let input = "
return 3;
return 10;
return 233;
    ";

    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);

    let program = parser.parse_program();
    check_parser_errors(parser);

    if program.is_none() {
        panic!("parse_program() returned None!")
    }
    let program = program.unwrap();
    let program_len = program.len();

    if program_len != 3 {
        panic!(
            "program statements does not contain 3 statements. got = {}",
            program_len
        );
    }

    for (_, stmt) in program.statements.into_iter().enumerate() {
        let return_stmt: ReturnStatement = stmt.into();
        println!("return statement: {:?}", return_stmt);

        if return_stmt.token_literal() != "return" {
            eprintln!(
                "return statement token literal not `return`, got {}",
                return_stmt.token_literal()
            );
        }
    }
}

fn check_parser_errors(p: Parser) {
    let errors = p.errors();
    if errors.is_empty() {
        return;
    }

    eprintln!("parser has {} errors", errors.len());

    for (_index, msg) in errors.iter().enumerate() {
        eprintln!("parser error: {:?}", msg);
    }
}

#[test]
pub fn test_identifier_expression() {
    let input = "foobar;";

    let lexer = Lexer::new(input);

    let mut  parser = Parser::new(lexer);

    let program = parser.parse_program().unwrap();

    check_parser_errors(parser);

    if program.statements.len() != 1 {
        eprintln!("program has not enough statements. got = {}", program.statements.len());
    }

    let stmt: Option<ExpressionStatement> = program.statements
        .get(0)
        .map(|value| value.into());

    if stmt.is_none() {
        eprintln!("program statement[0] is None");
    }

    let identifier = stmt.unwrap().expression;

    if identifier.value != "foobar" {
        eprintln!("ident.value not {}. got = {}", "foobar", identifier.value);
    }

    if identifier.token_literal() != "foobar" {
        eprintln!("ident.token_literal not {}. got = {}", "foobar", identifier.token_literal());
    }
}

#[test]
#[ignore]
fn test_test_let_statements() {
    test_let_statements();
}

#[test]
#[ignore]
fn test_test_return_statements() {
    test_return_statements();
}
