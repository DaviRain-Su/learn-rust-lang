include!(concat!(env!("OUT_DIR"), "/commit_id.rs"));

fn main() {
    println!("Current commit id is: {}", CURRENT_COMMIT_ID);
}