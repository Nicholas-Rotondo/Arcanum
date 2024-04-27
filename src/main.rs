use sha2::{Sha256, Digest};
pub mod arcanum_block;

fn main() {
    let block = arcanum_block::ArcanumBlock::new("0000".to_string(), "123".to_string(), 1, 1);
    println!("{:?}", block);
}
