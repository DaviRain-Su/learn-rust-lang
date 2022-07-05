use crate::ast::expression::infix_expression::InfixExpression;
use crate::ast::expression::integer_literal::IntegerLiteral;
use crate::ast::expression::prefix_expression::PrefixExpression;
use crate::ast::expression::Expression;
use crate::ast::statement::expression_statement::ExpressionStatement;
use crate::ast::statement::let_statement::LetStatement;
use crate::ast::statement::return_statement::ReturnStatement;
use crate::ast::statement::Statement;
use crate::ast::Identifier;
use crate::ast::Node;
use crate::lexer::Lexer;
use crate::parser::Parser;
use std::fmt::format;

fn test_let_statements() -> anyhow::Result<()> {
    struct LetStatementTest {
        input: String,
        expected_identifier: String,
        expected_value: String,
    }

    let input = "
let x  5;
let  = 19;
let  838383;
    ";

    let lexer = Lexer::new(input)?;
    let mut parser = Parser::new(lexer)?;

    let program = parser.parse_program()?;

    let program_len = program.len();

    if program_len != 3 {
        panic!(
            "program statements does not contain 3 statements. got = {}",
            program_len
        );
    }

    let tests = vec!["x", "y", "foobar"];

    for (i, tt) in tests.into_iter().enumerate() {
        let stmt = program
            .statements
            .get(i)
            .ok_or(anyhow::anyhow!("read statements error"))?;

        if !test_let_statement(stmt, tt.into()) {
            return Ok(());
        }
    }

    Ok(())
}

fn test_let_statement(s: &Statement, name: String) -> bool {
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
fn test_return_statements() -> anyhow::Result<()> {
    let input = "
return 3;
return 10;
return 233;
    ";

    let lexer = Lexer::new(input)?;
    let mut parser = Parser::new(lexer)?;

    let program = parser.parse_program()?;

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

    Ok(())
}

fn test_identifier_expression() -> anyhow::Result<()> {
    let input = "foobar;";

    let lexer = Lexer::new(input)?;

    let mut parser = Parser::new(lexer)?;

    let program = parser.parse_program()?;

    println!("program: {}", program);

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

    Ok(())
}

fn test_integer_literal_expression() -> anyhow::Result<()> {
    let input = "5;";

    let lexer = Lexer::new(input)?;

    let mut parser = Parser::new(lexer)?;

    let program = parser.parse_program()?;

    println!("program: {}", program);

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

    Ok(())
}

fn test_parsing_prefix_expression() -> anyhow::Result<()> {
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
        // PrefixTest::new("!foobar;".into(), "!".into(), 15),
        // PrefixTest::new("-foobar;".into(), "-".into(), 15),
        // PrefixTest::new("!true;".into(), "!".into(), ""),
        // PrefixTest::new("!false;".into(), "!".into(), "false"),
    ];

    for tt in prefix_tests.iter() {
        let lexer = Lexer::new(tt.input.as_str())?;
        let mut parser = Parser::new(lexer)?;
        let program = parser.parse_program()?;

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

        let exp = PrefixExpression::try_from(stmt.unwrap())?;

        println!("PrefixExpression = {:#?}", exp);

        if exp.operator != tt.operator {
            eprintln!(
                "exp.operator is no '{}'. got = {}",
                tt.operator, exp.operator
            );
        }

        let ret = test_integer_literal(*exp.right.clone(), tt.integer_value)?;

        if ret == false {
            eprintln!("test_integer_literal error!");
        }
    }

    Ok(())
}

fn test_integer_literal(il: Expression, value: i64) -> anyhow::Result<bool> {
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

fn test_parsing_infix_expression() -> anyhow::Result<()> {
    struct InfixTest {
        input: String,
        left_value: i64,
        operator: String,
        right_value: i64,
    }

    impl InfixTest {
        fn new(input: String, left_value: i64, operator: String, right_value: i64) -> Self {
            Self {
                input,
                left_value,
                operator,
                right_value,
            }
        }
    }

    let infix_tests = vec![
        InfixTest::new("5 + 5;".into(), 5, "+".into(), 5),
        InfixTest::new("5 - 5;".into(), 5, "-".into(), 5),
        InfixTest::new("5 * 5;".into(), 5, "*".into(), 5),
        InfixTest::new("5 / 5;".into(), 5, "/".into(), 5),
        InfixTest::new("5 > 5;".into(), 5, ">".into(), 5),
        InfixTest::new("5 < 5;".into(), 5, "<".into(), 5),
        InfixTest::new("5 == 5;".into(), 5, "==".into(), 5),
        InfixTest::new("5 != 5;".into(), 5, "!=".into(), 5),
    ];

    for tt in infix_tests.iter() {
        let lexer = Lexer::new(tt.input.as_str())?;

        let mut parser = Parser::new(lexer)?;

        let program = parser.parse_program()?;

        println!(" program: {}", program);

        if program.statements.len() != 1 {
            eprintln!(
                "program statements does not contain {} statemtns. got = {}",
                1,
                program.statements.len()
            );
        }

        let stmt: Option<ExpressionStatement> = program.statements.get(0).map(|value| value.into());

        if stmt.is_none() {
            eprintln!("program statements[0] is not ExpressionStatement. got = None");
        }

        let exp = InfixExpression::try_from(stmt.unwrap())?;

        let ret = test_integer_literal(*exp.left.clone(), tt.left_value)?;

        if ret == false {
            eprintln!("test_integer_literal error!");
        }

        if exp.operator != tt.operator {
            eprintln!(
                "exp.operator is not `{}`. got = {}",
                tt.operator, exp.operator
            );
        }

        let ret = test_integer_literal(*exp.right.clone(), tt.right_value)?;
        if ret == false {
            eprintln!("test_integer_literal error!");
        }
    }
    Ok(())
}

fn test_operator_precedence_parsing() -> anyhow::Result<()> {
    struct TempTest {
        input: String,
        expected: String,
    }

    let tests = vec![
        TempTest {
            input: "-a * b".into(),
            expected: "((-a) * b)".into(),
        },
        TempTest {
            input: "!-a".into(),
            expected: "(!(-a))".into(),
        },
        TempTest {
            input: "a + b + c".into(),
            expected: "((a + b) + c)".into(),
        },
        TempTest {
            input: "a * b * c".into(),
            expected: "((a * b) * c)".into(),
        },
        TempTest {
            input: "a * b / c".into(),
            expected: "((a * b) / c)".into(),
        },
        TempTest {
            input: "a + b / c".into(),
            expected: "(a + (b /c))".into(),
        },
        TempTest {
            input: "a + b * c + d / e - f".into(),
            expected: "(((a + (b * c)) + (d / e) - f)".into(),
        },
        TempTest {
            input: "3 + 4; -5 * 5".into(),
            expected: "(3 + 4)((-5) * 5)".into(),
        },
        TempTest {
            input: "5 > 4 == 3 < 4".into(),
            expected: "((5 > 4) == (3 < 4))".into(),
        },
        TempTest {
            input: "3 + 4 * 5 == 3 * 1 + 4 * 5".into(),
            expected: "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))".into(),
        },
    ];

    for tt in tests.into_iter() {
        let lexer = Lexer::new(tt.input.as_str())?;
        let mut parser = Parser::new(lexer)?;
        let program = parser.parse_program()?;

        if format!("{}", program) != tt.expected {
            eprintln!(
                "expected = {}, got = {}",
                tt.expected,
                format!("{}", program)
            );
        }
    }

    Ok(())
}

#[test]
#[ignore]
fn test_test_let_statements() {
    let ret = test_let_statements();
    println!("test_test_let_statements : Ret = {:?}", ret);
}

#[test]
#[ignore]
fn test_test_return_statements() {
    let ret = test_return_statements();
    println!("test_test_return_statements : Ret = {:?}", ret);
}

#[test]
#[ignore]
fn test_test_identifier_expression() {
    let ret = test_identifier_expression();
    println!("test_test_identifier_expression: Ret = {:?}", ret);
}

#[test]
#[ignore]
fn test_test_integer_literal_expression() {
    let ret = test_integer_literal_expression();
    println!("test_test_integer_literal_expression : Ret = {:?}", ret);
}

#[test]
#[ignore]
fn test_test_parsing_prefix_expression() {
    let ret = test_parsing_prefix_expression();
    println!("test_test_parsing_prefix_expression : Ret = {:?}", ret);
}

#[test]
#[ignore]
fn test_test_parsing_infix_expression() {
    let ret = test_parsing_infix_expression();
    println!("test_parsing_infix_expression: Ret = {:?}", ret);
}

#[test]
fn test_test_operator_precedence_parsing() {
    let ret = test_operator_precedence_parsing();
    println!("test_operator_precedence_parsing: Ret = {:?}", ret);
}
