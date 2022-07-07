use crate::ast::expression::boolean::Boolean;
use crate::ast::expression::function_literal::FunctionLiteral;
use crate::ast::expression::if_expression::IfExpression;
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
        integer_value: Box<dyn Interface>,
    }

    impl PrefixTest {
        fn new(input: String, operator: String, integer_value: Box<dyn Interface>) -> Self {
            Self {
                input,
                operator,
                integer_value,
            }
        }
    }

    let prefix_tests = vec![
        PrefixTest::new("!5;".into(), "!".into(), 5.into()),
        PrefixTest::new("-15;".into(), "-".into(), 15.into()),
        // PrefixTest::new("!foobar;".into(), "!".into(), 15),
        // PrefixTest::new("-foobar;".into(), "-".into(), 15),
        PrefixTest::new("!true;".into(), "!".into(), true.into()),
        PrefixTest::new("!false;".into(), "!".into(), false.into()),
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

        let ret = test_literal_expression(exp.into(), &*tt.integer_value)?;

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
        InfixTest::new(
            "foobar - barfoo;".into(),
            "foobar".into(),
            "-".into(),
            "barfoo".into(),
        ),
        InfixTest::new(
            "foobar * barfoo;".into(),
            "foobar".into(),
            "*".into(),
            "barfoo".into(),
        ),
        InfixTest::new(
            "foobar / barfoo;".into(),
            "foobar".into(),
            "/".into(),
            "barfoo".into(),
        ),
        InfixTest::new(
            "foobar < barfoo;".into(),
            "foobar".into(),
            "<".into(),
            "barfoo".into(),
        ),
        InfixTest::new(
            "foobar > barfoo;".into(),
            "foobar".into(),
            ">".into(),
            "barfoo".into(),
        ),
        InfixTest::new(
            "foobar == barfoo;".into(),
            "foobar".into(),
            "==".into(),
            "barfoo".into(),
        ),
        InfixTest::new(
            "foobar != barfoo;".into(),
            "foobar".into(),
            "!=".into(),
            "barfoo".into(),
        ),
        InfixTest::new("true == true".into(), true.into(), "==".into(), true.into()),
        InfixTest::new(
            "true != false".into(),
            true.into(),
            "!=".into(),
            false.into(),
        ),
        InfixTest::new(
            "false == false".into(),
            false.into(),
            "==".into(),
            false.into(),
        ),
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
        TempTest {
            input: "true".into(),
            expected: "true".into(),
        },
        TempTest {
            input: "3 < 5 == false".into(),
            expected: "((3 < 5) == false)".into(),
        },
        TempTest {
            input: "false".into(),
            expected: "false".into(),
        },
        TempTest {
            input: "3 > 5 == false".into(),
            expected: "((3 > 5) == false".into(),
        },
        TempTest {
            input: "1 + (2 + 3) + 4".into(),
            expected: "((1 + (2 + 3)) + 4)".into(),
        },
        TempTest {
            input: "(5 + 5) * 2".into(),
            expected: "((5 + 5) * 2)".into(),
        },
        TempTest {
            input: "2 / ( 5 + 5)".into(),
            expected: "(2 / (5 + 5))".into(),
        },
        TempTest {
            input: "-( 5 + 5)".into(),
            expected: "(-( 5 + 5))".into(),
        },
        TempTest {
            input: "!(true == true)".into(),
            expected: "(!(true == true))".into(),
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

fn test_if_expression() -> anyhow::Result<()> {
    let input = "if (x < y) { x }";

    let lexer = Lexer::new(input)?;
    let mut parser = Parser::new(lexer)?;

    let program = parser.parse_program()?;

    if program.statements.len() != 1 {
        eprintln!(
            "program statements does not contain {} statements. got = {}",
            1,
            program.statements.len()
        );
    }

    let stmt = program
        .statements
        .get(0)
        .map(|value| ExpressionStatement::from(value));

    if stmt.is_none() {
        eprintln!("program statements[0] is not ExpressionStatement. got = None");
    }

    let exp = IfExpression::try_from(stmt.unwrap().expression)?;
    println!("IfExpression Debug is = {:#?}", exp);
    println!("IfExpression Display is = {}", exp);

    if !test_infix_expression(
        *exp.condition,
        &"x".to_string(),
        "<".into(),
        &"y".to_string(),
    )? {
        eprintln!("test_infix_expression error");
    }

    if exp.consequence.is_none() {
        eprintln!(
            "exp consequence statements was not nil. got = {:?}",
            exp.consequence
        );
    }

    if exp.consequence.as_ref().unwrap().statements.len() != 1 {
        eprintln!(
            "consequence is not 1 statements. got = {}",
            exp.consequence.as_ref().unwrap().statements.len()
        );
    }

    let consequence = exp
        .consequence
        .unwrap()
        .statements
        .get(0)
        .map(|value| ExpressionStatement::from(value));

    if consequence.is_none() {
        eprintln!("statements[0] is not ExpressionStatement. got = None");
    }

    if !test_identifier(consequence.as_ref().unwrap().expression.clone(), "x".into())? {
        eprintln!("test identifier error");
    }

    if exp.alternative.is_some() {
        eprintln!(
            "exp alternative statements was not nil. got = {:?}",
            exp.alternative
        );
    }

    Ok(())
}

fn test_if_else_expression() -> anyhow::Result<()> {
    let input = "if (x < y) { x } else { y }";

    let lexer = Lexer::new(input)?;
    let mut parser = Parser::new(lexer)?;

    let program = parser.parse_program()?;

    if program.statements.len() != 1 {
        eprintln!(
            "program statements does not contain {} statements. got = {}",
            1,
            program.statements.len()
        );
    }

    let stmt = program
        .statements
        .get(0)
        .map(|value| ExpressionStatement::from(value));

    if stmt.is_none() {
        eprintln!("program statements[0] is not ExpressionStatement. got = None");
    }

    let exp = IfExpression::try_from(stmt.unwrap().expression)?;

    if !test_infix_expression(
        *exp.condition,
        &"x".to_string(),
        "<".into(),
        &"y".to_string(),
    )? {
        eprintln!("test infix expression error");
    }

    if exp.consequence.is_none() {
        eprintln!(
            "exp consequence statements was not nil. got = {:?}",
            exp.consequence
        );
    }

    if exp.consequence.as_ref().unwrap().statements.len() != 1 {
        eprintln!(
            "consequence is not 1 statements. got = {}",
            exp.consequence.as_ref().unwrap().statements.len()
        );
    }

    let alternative = exp
        .alternative
        .unwrap()
        .statements
        .get(0)
        .map(|value| ExpressionStatement::from(value));

    if alternative.is_none() {
        eprintln!("statements[0] is not ExpressionStatement. got = None");
    }

    if !test_identifier(alternative.as_ref().unwrap().expression.clone(), "y".into())? {
        eprintln!("test identifier error");
    }

    Ok(())
}

fn test_function_literal_parsing() -> anyhow::Result<()> {
    let input = "fn(x, y) { x + y; }";

    let lexer = Lexer::new(input)?;

    let mut parser = Parser::new(lexer)?;

    let program = parser.parse_program()?;

    if program.statements.len() != 1 {
        eprintln!(
            "program statements does not contain {} statments. got = {}",
            1,
            program.statements.len()
        );
    }

    let stmt = program
        .statements
        .get(0)
        .map(|value| ExpressionStatement::from(value));
    if stmt.is_none() {
        eprintln!("program statements[0] is not  expression statement. got = None");
    }

    let function = FunctionLiteral::try_from(stmt.unwrap().expression)?;

    if function.parameters.len() != 2 {
        eprintln!(
            "function literals parameters wrong. want 2, got = {}",
            function.parameters.len()
        );
    }

    test_literal_expression(function.parameters[0].clone().into(), &"x".to_string())
        .expect("test literals expression error");
    test_literal_expression(function.parameters[1].clone().into(), &"y".to_string())
        .expect("test literals expression error");

    if function.body.statements.len() != 1 {
        eprintln!(
            "function body statements wrong. want 1, got = {}",
            function.body.statements.len()
        );
    }

    let body_stmt = function
        .body
        .statements
        .get(0)
        .map(|value| ExpressionStatement::from(value));
    if body_stmt.is_none() {
        eprintln!("function body stmt is not ExpressionStatement. got = None");
    }

    test_infix_expression(
        body_stmt.unwrap().expression,
        &"x".to_string(),
        "+".into(),
        &"y".to_string(),
    )
    .expect("test infix expression error");

    Ok(())
}

fn test_function_parameter_parsing() -> anyhow::Result<()> {
    struct Test {
        input: String,
        expected_params: Vec<String>,
    }

    let tests = vec![
        Test {
            input: "fn() {};".into(),
            expected_params: vec![],
        },
        Test {
            input: "fn(x) {};".into(),
            expected_params: vec!["x".into()],
        },
        Test {
            input: "fn(x, y, z) {};".into(),
            expected_params: vec!["x".into(), "y".into(), "z".into()],
        },
    ];

    for tt in tests.into_iter() {
        let lexer = Lexer::new(tt.input.as_str())?;
        let mut parser = Parser::new(lexer)?;

        let program = parser.parse_program()?;

        let stmt = program
            .statements
            .get(0)
            .map(|value| ExpressionStatement::from(value));
        let function = FunctionLiteral::try_from(stmt.unwrap().expression)?;

        if function.parameters.len() != tt.expected_params.len() {
            eprintln!(
                "length parameters wrong. want {}. got = {}",
                tt.expected_params.len(),
                function.parameters.len()
            );
        }

        for (i, ident) in tt.expected_params.into_iter().enumerate() {
            test_literal_expression(function.parameters[i].clone().into(), &ident)?;
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
#[ignore]
fn test_test_operator_precedence_parsing() {
    let ret = test_operator_precedence_parsing();
    println!("test_operator_precedence_parsing: Ret = {:?}", ret);
}

#[test]
#[ignore]
fn test_test_if_expression() {
    let ret = test_if_expression();
    println!("test_if_expression: Ret = {:?}", ret);
}

#[test]
#[ignore]
fn test_test_if_else_expression() {
    let ret = test_if_else_expression();
    println!("test_if_else_expression: Ret = {:?}", ret);
}

#[test]
#[ignore]
fn test_test_function_literal_parsing() {
    let ret = test_function_literal_parsing();
    println!("test_function_literal_parsing: ret = {:?}", ret);
}

#[test]
fn test_test_function_parameter_parsing() {
    let ret = test_function_parameter_parsing();
    println!("test_function_parameter_parsing: ret = {:?}", ret);
}
