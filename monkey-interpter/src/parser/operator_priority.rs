#[derive(Debug)]
#[repr(C)]
pub enum OperatorPriority {
    LOWEST = 1,
    EQUALS = 2,      // ==
    LESSGREATER = 3, // < or >
    SUM = 4,         // +
    PRODUCT = 5,     // *
    PREFIX = 6,      // -X or !x
    CALL = 7,        // myFcuntion(x)
}

#[test]
#[ignore]
fn test_operator_priority_type() {
    let lowest = OperatorPriority::LOWEST;
    println!("lowest: {:?}", lowest as u8);
    let equals = OperatorPriority::EQUALS;
    println!("equals: {:?}", equals as u8);
    let lessgreater = OperatorPriority::LESSGREATER;
    println!("lessgreater: {:?}", lessgreater as u8);
    let sum = OperatorPriority::SUM;
    println!("sum: {:?}", sum as u8);
    let product = OperatorPriority::PRODUCT;
    println!("product: {:?}", product as u8);
    let prefix = OperatorPriority::PREFIX;
    println!("prefix: {:?}", prefix as u8);
    let call = OperatorPriority::CALL;
    println!("call: {:?}", call as u8);
}
