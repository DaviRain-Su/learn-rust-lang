use crate::ast::expression::boolean::Boolean;
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
use std::any::{Any, TypeId};

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

fn test_parsing_infix_expression() -> anyhow::Result<()> {
    struct InfixTest {
        input: String,
        left_value: Box<dyn Interface>,
        operator: String,
        right_value: Box<dyn Interface>,
    }

    impl InfixTest {
        fn new(
            input: String,
            left_value: Box<dyn Interface>,
            operator: String,
            right_value: Box<dyn Interface>,
        ) -> Self {
            Self {
                input,
                left_value,
                operator,
                right_value,
            }
        }
    }

    // {"5 + 5;", 5, "+", 5},
    // {"5 - 5;", 5, "-", 5},
    // {"5 * 5;", 5, "*", 5},
    // {"5 / 5;", 5, "/", 5},
    // {"5 > 5;", 5, ">", 5},
    // {"5 < 5;", 5, "<", 5},
    // {"5 == 5;", 5, "==", 5},
    // {"5 != 5;", 5, "!=", 5},
    // {"foobar + barfoo;", "foobar", "+", "barfoo"},
    // {"foobar - barfoo;", "foobar", "-", "barfoo"},
    // {"foobar * barfoo;", "foobar", "*", "barfoo"},
    // {"foobar / barfoo;", "foobar", "/", "barfoo"},
    // {"foobar > barfoo;", "foobar", ">", "barfoo"},
    // {"foobar < barfoo;", "foobar", "<", "barfoo"},
    // {"foobar == barfoo;", "foobar", "==", "barfoo"},
    // {"foobar != barfoo;", "foobar", "!=", "barfoo"},
    // {"true == true", true, "==", true},
    // {"true != false", true, "!=", false},
    // {"false == false", false, "==", false},

    let infix_tests = vec![
        InfixTest::new("5 + 5;".into(), 5.into(), "+".into(), 5.into()),
        InfixTest::new("5 - 5;".into(), 5.into(), "-".into(), 5.into()),
        InfixTest::new("5 * 5;".into(), 5.into(), "*".into(), 5.into()),
        InfixTest::new("5 / 5;".into(), 5.into(), "/".into(), 5.into()),
        InfixTest::new("5 > 5;".into(), 5.into(), ">".into(), 5.into()),
        InfixTest::new("5 < 5;".into(), 5.into(), "<".into(), 5.into()),
        InfixTest::new("5 == 5;".into(), 5.into(), "==".into(), 5.into()),
        InfixTest::new("5 != 5;".into(), 5.into(), "!=".into(), 5.into()),
        InfixTest::new(
            "foobar + barfoo;".into(),
            "foobar".into(),
            "+".into(),
            "barfoo".into(),
        ),
        InfixTest::new("true == true".into(), true.into(), "==".into(), true.into()),
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

        if !test_infix_expression(
            stmt.unwrap().expression,
            &*tt.left_value,
            tt.operator.clone(),
            &*tt.right_value,
        )? {
            return Err(anyhow::anyhow!("test_infix_expression error"));
        }

        // let exp = InfixExpression::try_from(stmt.unwrap())?;
        //
        // let ret = test_integer_literal(*exp.left.clone(), tt.left_value)?;
        //
        // if ret == false {
        //     eprintln!("test_integer_literal error!");
        // }
        //
        // if exp.operator != tt.operator {
        //     eprintln!(
        //         "exp.operator is not `{}`. got = {}",
        //         tt.operator, exp.operator
        //     );
        // }
        //
        // let ret = test_integer_literal(*exp.right.clone(), tt.right_value)?;
        // if ret == false {
        //     eprintln!("test_integer_literal error!");
        // }
    }
    Ok(())
}

fn test_operator_precedence_parsing() -> anyhow::Result<()> {
    struct TempTest {
        input: String,
        expected: String,
    }

    let tests = vec![
        // TempTest {
        //     input: "-a * b".into(),
        //     expected: "((-a) * b)".into(),
        // },
        // TempTest {
        //     input: "!-a".into(),
        //     expected: "(!(-a))".into(),
        // },
        TempTest {
            input: "a + b + c".into(),
            expected: "((a + b) + c)".into(),
        },
        // TempTest {
        //     input: "a * b * c".into(),
        //     expected: "((a * b) * c)".into(),
        // },
        // TempTest {
        //     input: "a * b / c".into(),
        //     expected: "((a * b) / c)".into(),
        // },
        // TempTest {
        //     input: "a + b / c".into(),
        //     expected: "(a + (b /c))".into(),
        // },
        // TempTest {
        //     input: "a + b * c + d / e - f".into(),
        //     expected: "(((a + (b * c)) + (d / e) - f)".into(),
        // },
        // TempTest {
        //     input: "3 + 4; -5 * 5".into(),
        //     expected: "(3 + 4)((-5) * 5)".into(),
        // },
        // TempTest {
        //     input: "5 > 4 == 3 < 4".into(),
        //     expected: "((5 > 4) == (3 < 4))".into(),
        // },
        // TempTest {
        //     input: "3 + 4 * 5 == 3 * 1 + 4 * 5".into(),
        //     expected: "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))".into(),
        // },
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

fn test_identifier(exp: Expression, value: String) -> anyhow::Result<bool> {
    let ident = Identifier::try_from(exp)?;

    if ident.value != value {
        eprintln!("identifier value not {}. got = {}", value, ident.value);
        return Ok(false);
    }

    if ident.token_literal() != value {
        eprintln!(
            "identifier token_literal not {}. got = {}",
            value,
            ident.token_literal()
        );
        return Ok(false);
    }
    Ok(true)
}

fn test_boolean_literal(exp: Expression, value: bool) -> anyhow::Result<bool> {
    let boolean = Boolean::try_from(exp)?;

    if boolean.value != value {
        eprintln!("boolean value not {}. got = {}", value, boolean.value);
        return Ok(false);
    }

    if boolean.token_literal() != format!("{}", value) {
        eprintln!(
            "boolean token_literal not {}. got = {}",
            value,
            boolean.token_literal()
        );
        return Ok(false);
    }
    Ok(true)
}

trait Interface {
    fn as_any(&self) -> &dyn Any;
}

impl Interface for i64 {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl From<i64> for Box<dyn Interface> {
    fn from(value: i64) -> Self {
        Box::new(value)
    }
}

impl Interface for String {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl From<String> for Box<dyn Interface> {
    fn from(value: String) -> Self {
        Box::new(value)
    }
}

impl Interface for &'static str {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl From<&'static str> for Box<dyn Interface> {
    fn from(value: &'static str) -> Self {
        Box::new(value)
    }
}

impl Interface for bool {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl From<bool> for Box<dyn Interface> {
    fn from(value: bool) -> Self {
        Box::new(value)
    }
}

fn test_literal_expression(exp: Expression, expected: &dyn Interface) -> anyhow::Result<bool> {
    let t = expected.as_any().type_id();
    if TypeId::of::<i64>() == t {
        let value = expected
            .as_any()
            .downcast_ref::<i64>()
            .expect("downcast_ref error");
        test_integer_literal(exp, *value)
    } else if TypeId::of::<String>() == t {
        let value = expected
            .as_any()
            .downcast_ref::<String>()
            .expect("downcast_ref error");
        test_identifier(exp, value.clone())
    } else if TypeId::of::<&str>() == t {
        let value = expected
            .as_any()
            .downcast_ref::<&str>()
            .expect("downcast_ref error");
        test_identifier(exp, value.to_string())
    } else if TypeId::of::<bool>() == t {
        let value = expected
            .as_any()
            .downcast_ref::<bool>()
            .expect("downcast_ref error");
        test_boolean_literal(exp, value.clone())
    } else {
        eprintln!("type of exp not handle.got = {}", exp);
        Ok(false)
    }
}

fn test_infix_expression(
    exp: Expression,
    left: &dyn Interface,
    operator: String,
    right: &dyn Interface,
) -> anyhow::Result<bool> {
    let op_exp = InfixExpression::try_from(exp)?;

    if !test_literal_expression(*op_exp.left, left)? {
        return Ok(false);
    }

    if op_exp.operator != operator {
        eprintln!(
            "exp.operator is not '{}'. got = {}",
            operator, op_exp.operator
        );
        return Ok(false);
    }

    if !test_literal_expression(*op_exp.right, right)? {
        return Ok(false);
    }

    Ok(true)
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
// #[ignore]
fn test_test_parsing_infix_expression() {
    let ret = test_parsing_infix_expression();
    println!("test_parsing_infix_expression: Ret = {:?}", ret);
}

#[test]
#[ignore]
fn test_test_operator_precedence_parsing() {
    env_logger::init();
    let ret = test_operator_precedence_parsing();
    println!("test_operator_precedence_parsing: Ret = {:?}", ret);
}
