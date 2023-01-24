#[test]
fn test_blake2_hash_function() {
    use blake2::{Blake2b512, Blake2s256, Digest};

    // create a Blake2b512 object
    let mut hasher = Blake2b512::new();

    // write input message
    hasher.update(b"hello world");

    // read hash digest and consume hasher
    let res = hasher.finalize();
    let encode_res = hex::encode(res);
    println!("encode blake2b512 hash function = {}", encode_res);
    // 021ced8799296ceca557832ab941a50b4a11f83478cf141f51f933f653ab9fbc
    // c05a037cddbed06e309bf334942c4e58cdf1a46e237911ccd7fcf9787cbc7fd0

    // same example for Blake2s256:
    let mut hasher = Blake2s256::new();
    hasher.update(b"hello world");
    let res = hasher.finalize();
    let encode_res = hex::encode(res);
    println!("encode blake2s256 hash function = {}", encode_res);
    // 9aec6806794561107e594b1f6a8a6b0c92a0cba9acf5e5e93cca06f781813b0b
}
