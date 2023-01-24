#[test]
fn test_fsb_hash_function() {
    use fsb::{Digest, Fsb256};

    // create a FSB-256 object
    let mut hasher = Fsb256::new();

    // write input message
    hasher.update(b"hello");

    // read hash digest
    let result = hasher.finalize();
    let encode_result = hex::encode(result);
    println!("encode fsb hash result = {}", encode_result);
    // 0f036dc3761aed2cba9de586a85976eedde6fa8f115c0190763decc02f28edbc
}
