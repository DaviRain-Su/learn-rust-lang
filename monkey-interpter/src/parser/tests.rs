use crate::ast::statement::expression_statement::ExpressionStatement;
use crate::ast::statement::integer_literal::IntegerLiteral;
use crate::ast::statement::let_statement::LetStatement;
use crate::ast::statement::prefix_expression::PrefixExpression;
use crate::ast::statement::return_statement::ReturnStatement;
use crate::ast::statement::{Expression, Node, Statement};
use crate::ast::Identifier;
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

fn test_identifier_expression() {
    let input = "foobar;";

    let lexer = Lexer::new(input);

    let mut parser = Parser::new(lexer);

    let program = parser.parse_program().unwrap();

    println!("program: {}", program);

    check_parser_errors(parser);

    if program.statements.len() != 1 {
        eprintln!(
            "program has not enough statements. got = {}",
            program.statements.len()
        );
    }

    let stmt: Option<ExpressionStatement> = program.statements.get(0).map(|value| value.into());

    println!("expression statement: {:#?}", stmt);

    if stmt.is_none() {
        eprintln!("program statement[0] is None");
    }

    let identifier: Identifier = Identifier::from(stmt.unwrap().expression);

    if identifier.value != "foobar" {
        eprintln!("ident.value not {}. got = {}", "foobar", identifier.value);
    }

    if identifier.token_literal() != "foobar" {
        eprintln!(
            "ident.token_literal not {}. got = {}",
            "foobar",
            identifier.token_literal()
        );
    }
}

fn test_integer_literal_expression() {
    let input = "5;";

    let lexer = Lexer::new(input);

    let mut parser = Parser::new(lexer);

    let program = parser.parse_program().unwrap();

    println!("program: {}", program);

    check_parser_errors(parser);

    if program.statements.len() != 1 {
        eprintln!(
            "program has not enough statements. got = {}",
            program.statements.len()
        );
    }

    let stmt: Option<ExpressionStatement> = program.statements.get(0).map(|value| value.into());

    println!("expression statement: {:#?}", stmt);

    if stmt.is_none() {
        eprintln!("program statement[0] is None");
    }

    let literal = IntegerLiteral::try_from(stmt.unwrap()).unwrap();

    if literal.value != 5 {
        eprintln!("ident.value not {}. got = {}", "foobar", literal.value);
    }

    if literal.token_literal() != "5" {
        eprintln!(
            "ident.token_literal not {}. got = {}",
            "foobar",
            literal.token_literal()
        );
    }
}

fn test_parsing_prefix_expression() {
    struct PrefixTest {
        input: String,
        operator: String,
        integer_value: i64,
    }

    impl PrefixTest {
        fn new(input: String, operator: String, integer_value: i64) -> Self {
            Self {
                input,
                operator,
                integer_value,
            }
        }
    }

    let prefix_tests = vec![
        PrefixTest::new("!5;".into(), "!".into(), 5),
        PrefixTest::new("-15;".into(), "-".into(), 15),
    ];

    for tt in prefix_tests.iter() {
        let lexer = Lexer::new(tt.input.as_str());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        check_parser_errors(parser);

        let program = program.unwrap();

        println!("Program = {:#?}", program);
        println!("Program = {}", program);

        let program_statements_len = program.statements.len();
        if program_statements_len != 1 {
            eprintln!(
                "program statements does not contain {} statements. got = {}",
                1, program_statements_len
            );
        }

        let stmt: Option<ExpressionStatement> = program.statements.get(0).map(|value| value.into());
        if stmt.is_none() {
            eprintln!(
                "program statements[0] is not expression statement. got = {:?}",
                stmt
            );
        }

        let exp = PrefixExpression::try_from(stmt.unwrap());
        if exp.is_err() {
            eprintln!("stmt is not prefix_expression. got = {:?}", exp);
        }

        let exp = exp.unwrap();

        if exp.operator != tt.operator {
            eprintln!(
                "exp.operator is no '{}'. got = {}",
                tt.operator, exp.operator
            );
        }

        let ret = test_integer_literal(exp.right, tt.integer_value);
        if ret.is_err() {
            println!("test_integer_literal error = {:?}", ret);
        } else {
            if ret.unwrap() == false {
                eprintln!("test_integer_literal error!");
            }
        }
    }
}

fn test_integer_literal(il: Box<dyn Expression>, value: i64) -> anyhow::Result<bool> {
    let integ = IntegerLiteral::try_from(il)?;
    if integ.value != value {
        eprintln!("integ value not {}. got = {}", value, integ.value);
        return Ok(false);
    }

    if integ.token_literal() != format!("{}", value) {
        eprintln!(
            "integ token_literal not {}. got = {}",
            value,
            integ.token_literal()
        );
        return Ok(false);
    }

    Ok(true)
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

#[test]
#[ignore]
fn test_test_identifier_expression() {
    test_identifier_expression();
}

#[test]
#[ignore]
fn test_test_integer_literal_expression() {
    test_integer_literal_expression();
}

#[test]
// #[ignore]
fn test_test_parsing_prefix_expression() {
    test_parsing_prefix_expression();
}
