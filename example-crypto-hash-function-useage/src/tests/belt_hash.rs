#[test]
fn test_belt_hash_function() {
    use belt_hash::{BeltHash, Digest};

    // create a BelT hasher instance
    let mut hasher = BeltHash::new();

    // process input message
    hasher.update(b"hello world");

    // acquire hash digest in the form of GenericArray,
    // which in this case is equivalent to [u8; 32]
    let result = hasher.finalize();
    let encode_result = hex::encode(result);
    println!("encode belt hash function = {}", encode_result);
    // afb175816416fbadad4629ecbd78e1887789881f2d2e5b80c22a746b7ac7ba88
}
